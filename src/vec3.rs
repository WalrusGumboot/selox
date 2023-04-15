use rand::{Rng, rngs::ThreadRng};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

fn random_normal_dist(rng: &mut ThreadRng) -> f64 {
    let theta = std::f64::consts::TAU * rng.gen::<f64>();
    let rho = (-2.0 * rng.gen::<f64>().log(10.0)).sqrt();

    rho * theta.cos()
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub const ZERO: Self = Vec3(0.0, 0.0, 0.0);
    pub const ONES: Self = Vec3(1.0, 1.0, 1.0);
    pub const UP: Self = Vec3(0.0, 1.0, 0.0);
    pub const DOWN: Self = Vec3(0.0, -1.0, 0.0);
    pub const LEFT: Self = Vec3(-1.0, 0.0, 0.0);
    pub const RIGHT: Self = Vec3(1.0, 0.0, 0.0);
    pub const FORW: Self = Vec3(0.0, 0.0, 1.0);
    pub const BACKW: Self = Vec3(0.0, 0.0, -1.0);

    pub fn magnitude(&self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    pub fn magnitude_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn normalise(&self) -> Self {
        self.clone() / self.magnitude()
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Vec3(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }

    pub fn hadamard(&self, rhs: &Self) -> Self {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }

    pub fn to_colour(&self, samples_per_pixel: u32) -> [u8; 3] {
        let r = self.0;
        let g = self.1;
        let b = self.2;

        let scale = 1.0 / samples_per_pixel as f64;
        let gamma_corrected_r = (scale * r).sqrt().clamp(0.0, 1.0 - f64::EPSILON);
        let gamma_corrected_g = (scale * g).sqrt().clamp(0.0, 1.0 - f64::EPSILON);
        let gamma_corrected_b = (scale * b).sqrt().clamp(0.0, 1.0 - f64::EPSILON);

        let pix_r = (gamma_corrected_r * 256.0).floor() as u8;
        let pix_g = (gamma_corrected_g * 256.0).floor() as u8;
        let pix_b = (gamma_corrected_b * 256.0).floor() as u8;

        [pix_r, pix_g, pix_b]
    }

    pub fn random_range(rng: &mut ThreadRng, min: f64, max: f64) -> Self {
        Vec3(rng.gen_range(min..max), rng.gen_range(min..max), rng.gen_range(min..max))
    }

    pub fn random_unit(rng: &mut ThreadRng) -> Self {
        Self::random_range(rng, 0.0, 1.0)
    }

    pub fn random_in_unit_sphere(rng: &mut ThreadRng) -> Self {
        // simple rejection algorithm
        let x = random_normal_dist(rng);
        let y = random_normal_dist(rng);
        let z = random_normal_dist(rng);

        Self(x, y, z).normalise()
    }

    pub fn random_on_hemisphere(rng: &mut ThreadRng, normal: Vec3) -> Self {
        let dir = Self::random_in_unit_sphere(rng);
        return dir * normal.dot(&dir).signum();
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}
