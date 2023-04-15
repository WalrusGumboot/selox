use crate::objects::{Renderable, Sphere, Material};
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::camera::Camera;

use image::{RgbImage};
use indicatif::ProgressIterator;
use rand::prelude::*;
use std::path::Path;

pub struct Scene {
    camera: Camera,
    objects: Vec<Box<dyn Renderable>>,
}

impl Scene {
    pub fn test_scene() -> Self {
        Scene {
            camera: Camera::new(),
            objects: vec![
                Box::new(Sphere::new(Vec3::BACKW * 3.0, 1.0, Material::with_colour(Vec3(0.3, 0.7, 0.9)))),
                Box::new(Sphere::new(Vec3::DOWN * 30.0, 29.0, Material::with_colour(Vec3(1.0, 0.7, 0.2)))),
                Box::new(Sphere::new(Vec3(-10.0, 0.0, -30.0), 15.0, Material::white_lamp())),

            ],
        }
    }

    pub fn cast(&self, rng: &mut ThreadRng, ray: &Ray) -> Vec3 {
        let mut ray_colour = Vec3::ONES;
        let mut incoming_light = Vec3::ZERO;

        let mut working_ray = ray.clone();

        for _ in 0..self.camera.max_bounces {
            let best_hit =
                self.objects
                    .iter()
                    .fold(None, |best_hr, obj| {
                        if best_hr.is_none() {
                            // whatever the case, (no hit OR a hit) will be better than (no hit).
                            return obj.hit_test(&working_ray, 0.0, f64::INFINITY);
                        } else {
                            let best_hr = best_hr.unwrap();
                            let maybe_hit = obj.hit_test(&working_ray, 0.0, f64::INFINITY);
    
                            if maybe_hit.is_none() {
                                Some(best_hr)
                            } else {
                                let hit = maybe_hit.unwrap();
                                if hit.distance < best_hr.distance {
                                    Some(hit)
                                } else {
                                    Some(best_hr)
                                }
                            }
                        }
                    });
    
            if let Some(hit) = best_hit {
                let new_bounce: Vec3 = hit.pos + hit.normal + Vec3::random_on_hemisphere(rng, hit.normal);
                working_ray = Ray { dir: new_bounce, origin: hit.pos };
    
                incoming_light += (hit.material.emission_colour * hit.material.emission_strength).hadamard(&ray_colour);
                ray_colour = ray_colour.hadamard(&hit.material.base_colour);
            } else {
                incoming_light = incoming_light.hadamard(&Vec3(1.0, 0.0, 0.0));
                break;
            }
        }

        incoming_light
    }

    pub fn render_to_png<P: AsRef<Path>>(&self, path: P) -> Result<(), image::ImageError> {
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

                    let raw_cast_result = self.cast(&mut rng, &ray);
                    colours[y][x] += raw_cast_result;
                }
            }
        }

        let img = RgbImage::from_fn(width as u32, height as u32, |x, y| {
            image::Rgb(colours[y as usize][x as usize].to_colour(self.camera.samples_per_pixel))
        });

        img.save(path)
    }
}