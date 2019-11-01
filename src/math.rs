use std::ops::{Add, Div, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3f {
    pub fn mag_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn mag(self) -> f32 {
        self.mag_squared().sqrt()
    }

    pub fn dot(self, rhs: Vec3f) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Add for Vec3f {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vec3f {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Mul<f32> for Vec3f {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Vec3f {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3f> for f32 {
    type Output = Vec3f;

    fn mul(self, rhs: Vec3f) -> Vec3f {
        Vec3f {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl Div<f32> for Vec3f {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Vec3f {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
