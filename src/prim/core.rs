use crate::math::{Ray, Vec3};

pub struct Intersection {
    pub point: Vec3,
    pub normal: Vec3,
    pub dist: f32,
}

pub trait Prim {
    fn try_intersect(&self, ray: &mut Ray) -> Option<Intersection>;
}
