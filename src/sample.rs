use std::f64;

use rand::Rng;

use crate::math::{Unit3, Vec3};

struct Basis {
    pub x: Vec3,
    pub y: Vec3,
    pub z: Vec3,
}

impl Basis {
    pub fn from_normal(normal: Unit3) -> Basis {
        let z: Vec3 = normal.into();

        // We need another vector that isn't collinear with the normal.
        let other = if z.x.abs() > 0.1 {
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            }
        } else {
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            }
        };

        let x = Vec3::from(other.cross(z).to_unit());
        let y = z.cross(x);

        Basis { x, y, z }
    }
}

pub fn sample_cos_weighted_hemisphere<R: Rng + ?Sized>(normal: Unit3, rng: &mut R) -> Unit3 {
    let basis = Basis::from_normal(normal);

    let radius_squared: f64 = rng.gen();
    let phi = rng.gen_range(0.0, 2.0 * f64::consts::PI);

    let radius = radius_squared.sqrt();
    let x = radius * phi.cos();
    let y = radius * phi.sin();
    let z = (1.0 - radius_squared).sqrt();

    Unit3::from_unit_vec3(x * basis.x + y * basis.y + z * basis.z)
}

pub fn sample_uniform_cone<R: Rng + ?Sized>(normal: Unit3, alpha: f64, rng: &mut R) -> Unit3 {
    let basis = Basis::from_normal(normal);

    let u: f64 = rng.gen();
    let phi = rng.gen_range(0.0, 2.0 * f64::consts::PI);

    let z = 1.0 + u * (alpha.cos() - 1.0);
    let radius = (1.0 - z * z).sqrt();
    let x = radius * phi.cos();
    let y = radius * phi.sin();

    Unit3::from_unit_vec3(x * basis.x + y * basis.y + z * basis.z)
}
