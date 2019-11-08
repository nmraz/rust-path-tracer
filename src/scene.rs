use crate::geom::*;
use crate::math::*;

#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub diffuse_color: Vec3,
    pub emittance_color: Vec3,
}

pub struct Primitive<'a> {
    geom: Box<dyn Geom + 'a>,
    material: Material,
}

impl<'a> Primitive<'a> {
    pub fn new<G: Geom + 'a>(geom: G, material: Material) -> Primitive<'a> {
        Primitive {
            geom: Box::new(geom),
            material,
        }
    }

    pub fn geom(&self) -> &dyn Geom {
        self.geom.as_ref()
    }

    pub fn material(&self) -> &Material {
        &self.material
    }
}

pub struct Scene<'a> {
    primitives: Vec<Primitive<'a>>,
}

impl<'a> Scene<'a> {
    pub fn new() -> Scene<'a> {
        Scene { primitives: vec![] }
    }

    pub fn with_primitives(primitives: Vec<Primitive<'a>>) -> Scene<'a> {
        Scene { primitives }
    }

    pub fn primitives(&self) -> &[Primitive] {
        self.primitives.as_slice()
    }

    pub fn add_primitive(&mut self, primitive: Primitive<'a>) {
        self.primitives.push(primitive);
    }
}
