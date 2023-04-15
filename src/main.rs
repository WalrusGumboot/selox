#![allow(dead_code)]
mod objects;
mod ray;
mod vec3;

use image::{Rgb, RgbImage};
use indicatif::ProgressIterator;
use objects::{HitResult, Renderable, Sphere};
use rand::prelude::*;
use ray::Ray;
use std::path::Path;
use vec3::Vec3;

type Colour = Rgb<u8>;

struct Camera {
    pos: Vec3,
    focal_length: f64,
    viewport: (f64, f64),
    resolution: (u32, u32),
    samples_per_pixel: u32,
}

impl Camera {
    fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;

        let image_width: u32 = 800;
        let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;

        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;

        Camera {
            pos: Vec3::ZERO,
            focal_length: 1.0,
            viewport: (viewport_width, viewport_height),
            resolution: (image_width, image_height),
            samples_per_pixel: 20,
        }
    }

    fn horizontal(&self) -> Vec3 {
        self.pos + Vec3::RIGHT * self.viewport.0
    }
    fn vertical(&self) -> Vec3 {
        self.pos + Vec3::UP * self.viewport.1
    }
    fn lower_left_corner(&self) -> Vec3 {
        self.pos - self.horizontal() / 2.0 - self.vertical() / 2.0 - Vec3::FORW * self.focal_length
    }

    fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.pos,
            dir: self.lower_left_corner() + self.horizontal() * u + self.vertical() * v - self.pos,
        }
    }
}

struct Scene {
    camera: Camera,
    objects: Vec<Box<dyn Renderable>>,
}

impl Scene {
    fn empty() -> Self {
        Scene {
            camera: Camera::new(),
            objects: vec![
                /*
                Box::new(Sphere::new(Vec3(1.0, 0.0, -2.0), 1.0)),
                Box::new(Sphere::new(Vec3(-1.0, 0.5, -3.0), 1.3)),
                Box::new(Sphere::new(Vec3(0.0, -21.0, 0.0), 20.0)),
                */
                Box::new(Sphere::new(Vec3::BACKW * 3.0, 1.0)),
            ],
        }
    }

    fn render_to_png<P: AsRef<Path>>(&self, path: P) -> Result<(), image::ImageError> {
        let width = self.camera.resolution.0;
        let height = self.camera.resolution.1;

        let mut colours = vec![vec![Vec3::ZERO; width as usize]; height as usize];

        let mut rng = rand::thread_rng();

        for y in (0..height as usize).progress() {
            for x in 0..width as usize {
                for _ in 0..self.camera.samples_per_pixel {
                    let rand_u_offset: f64 = rng.gen();
                    let rand_v_offset: f64 = rng.gen();

                    let u = (x as f64 + rand_u_offset) / width as f64;
                    let v = ((height - y as u32) as f64 + rand_v_offset) / height as f64;

                    let ray = self.camera.get_ray(u, v);
                    // println!("u: {u}, v: {v}, ray dir: {:?}", ray.dir);

                    let maybe_hit =
                        self.objects
                            .iter()
                            .fold(HitResult::infinitely_far_hr(), |best_hr, obj| {
                                if let Some(hit) = obj.hit_test(&ray, 0.0, 1000.0) {
                                    if best_hr.distance > hit.distance {
                                        return hit;
                                    } else {
                                        return best_hr;
                                    }
                                } else {
                                    return best_hr;
                                }
                            });

                    let colour = if maybe_hit.distance.is_infinite() {
                        let t = ray.dir.normalise().1 * 0.5 + 0.5;
                        Vec3::ONES * (1.0 - t) + Vec3(0.5, 0.7, 1.0) * t
                    } else {
                        maybe_hit.normal
                    };

                    let reduced_colour = colour / self.camera.samples_per_pixel as f64;
                    colours[y][x] += reduced_colour;
                }
            }
        }

        let img = RgbImage::from_fn(width as u32, height as u32, |x, y| {
            image::Rgb(colours[y as usize][x as usize].to_colour())
        });

        img.save(path)
    }
}

fn main() {
    let scene = Scene::empty();
    scene.render_to_png("test.png").unwrap();
}
