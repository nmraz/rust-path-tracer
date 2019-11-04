use crate::math::{Ray, Vec3};

pub struct Intersection {
    pub point: Vec3,
    pub normal: Vec3,
    pub dist: f64,
}

pub trait Geom {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}
