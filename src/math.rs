use std::ops::{Add, Div, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn mag_squared(self) -> f32 {
        self.dot(self)
    }

    pub fn mag(self) -> f32 {
        self.mag_squared().sqrt()
    }

    pub fn dot(self, rhs: Vec3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn to_unit(self) -> Vec3 {
        self / self.mag()
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Ray {
    origin: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: &Vec3, dir: &Vec3) -> Ray {
        Ray {
            origin: *origin,
            dir: dir.to_unit(),
        }
    }

    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }

    pub fn dir(&self) -> &Vec3 {
        &self.dir
    }

    pub fn interp(&self, t: f32) -> Vec3 {
        self.origin + t * self.dir
    }
}
