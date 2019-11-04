use std::ops::{Add, Div, Mul, Neg, Sub};

pub const EPSILON: f64 = 1e-9;

pub fn nearly_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON * a.abs()
}

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Copy, Clone)]
pub struct Unit3 {
    vec: Vec3,
}

impl Unit3 {
    pub fn from_unit_vec3(vec: Vec3) -> Unit3 {
        debug_assert!(
            nearly_equal(vec.mag_squared(), 1.0),
            "Illegal construction of Unit3"
        );
        Unit3 { vec }
    }

    pub fn x(&self) -> f64 {
        self.vec.x
    }
    pub fn y(&self) -> f64 {
        self.vec.y
    }
    pub fn z(&self) -> f64 {
        self.vec.z
    }
}

impl Vec3 {
    pub fn mag_squared(self) -> f64 {
        self.dot(self)
    }

    pub fn mag(self) -> f64 {
        self.mag_squared().sqrt()
    }

    pub fn dot(self, rhs: Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn to_unit(self) -> Unit3 {
        let mag = self.mag();
        assert!(mag < EPSILON, "Normalizing zero vector");
        Unit3::from_unit_vec3(self / mag)
    }
}

impl From<Unit3> for Vec3 {
    fn from(u: Unit3) -> Vec3 {
        Vec3 {
            x: u.x(),
            y: u.y(),
            z: u.z(),
        }
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

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Unit3,
}

impl Ray {
    pub fn interp(&self, t: f64) -> Vec3 {
        self.origin + t * Vec3::from(self.dir)
    }
}
