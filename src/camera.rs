use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera {
    pub pos: Vec3,
    pub focal_length: f64,
    pub viewport: (f64, f64),
    pub resolution: (u32, u32),
    pub samples_per_pixel: u32,
    pub max_bounces: u8,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;

        let image_width: u32 = 800;
        let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;

        let viewport_height = 1.0;
        let viewport_width = viewport_height * aspect_ratio;

        Camera {
            pos: Vec3::ZERO,
            focal_length: 1.0,
            viewport: (viewport_width, viewport_height),
            resolution: (image_width, image_height),
            samples_per_pixel: 4,
            max_bounces: 4,
        }
    }

    pub fn horizontal(&self) -> Vec3 {
        self.pos + Vec3::RIGHT * self.viewport.0
    }
    pub fn vertical(&self) -> Vec3 {
        self.pos + Vec3::UP * self.viewport.1
    }
    pub fn lower_left_corner(&self) -> Vec3 {
        self.pos - self.horizontal() / 2.0 - self.vertical() / 2.0 - Vec3::FORW * self.focal_length
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.pos,
            dir: self.lower_left_corner() + self.horizontal() * u + self.vertical() * v - self.pos,
        }
    }
}
