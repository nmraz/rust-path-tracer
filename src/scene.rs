use crate::geom::Geom;
use crate::math::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub diffuse_color: Vec3,
    pub emittance_color: Vec3,
}

pub struct Primitive {
    geom: Box<dyn Geom>,
    material: Material,
}

impl Primitive {
    pub fn new<G: Geom + 'static>(geom: G, material: Material) -> Primitive {
        Primitive {
            geom: Box::new(geom),
            material,
        }
    }
}

pub struct Scene {
    primitives: Vec<Primitive>,
}

impl Scene {
    pub fn new(primitives: Vec<Primitive>) -> Scene {
        Scene { primitives }
    }

    pub fn primitives(&self) -> &[Primitive] {
        self.primitives.as_slice()
    }

    pub fn add_primitive(&mut self, primitive: Primitive) {
        self.primitives.push(primitive);
    }
}
