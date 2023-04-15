use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

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

    pub fn to_colour(&self) -> [u8; 3] {
        let multiplied = *self * 255.0;
        let r = multiplied.0.clamp(0.0, 255.0) as u8;
        let g = multiplied.1.clamp(0.0, 255.0) as u8;
        let b = multiplied.2.clamp(0.0, 255.0) as u8;

        [r, g, b]
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
