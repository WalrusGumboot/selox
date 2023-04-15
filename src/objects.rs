use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitResult {
    pub pos: Vec3,
    pub normal: Vec3,
    pub distance: f64,

    pub front_face: bool,
}

impl HitResult {
    pub fn infinitely_far_hr() -> Self {
        HitResult {
            pos: Vec3::ZERO,
            normal: Vec3::ZERO,
            distance: f64::INFINITY,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.dir.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            *outward_normal * -1.0
        };
    }
}

pub trait Renderable {
    fn hit_test(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitResult>;
}

pub struct Sphere {
    pub centre: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(centre: Vec3, radius: f64) -> Self {
        Sphere { centre, radius }
    }
}

impl Renderable for Sphere {
    fn hit_test(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitResult> {
        let offset_centre = ray.origin - self.centre;
        let a = ray.dir.magnitude_squared();
        let half_b = offset_centre.dot(&ray.dir);
        let c = offset_centre.magnitude_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0f64 {
            None
        } else {
            let sqrt_d = discriminant.sqrt();

            let mut root = (-half_b - sqrt_d) / a;

            // We want to use the nearest root possible.
            if root < t_min || root > t_max {
                return None;
            } else {
                root = (-half_b + sqrt_d) / a;
                if root < t_min || root > t_max {
                    return None;
                }
            }

            let mut hr = HitResult {
                distance: root,
                pos: ray.at(root),
                normal: ((ray.at(root) - self.centre) / self.radius).normalise(),
                front_face: false,
            };

            hr.set_face_normal(ray, &hr.normal.clone());

            Some(hr)
        }
    }
}
