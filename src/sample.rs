use std::f64;

use rand::Rng;

use crate::math::{nearly_equal, Unit3, Vec3};

pub fn sample_cos_weighted_hemisphere<R: Rng + ?Sized>(normal: Unit3, rng: &mut R) -> Unit3 {
    let basis_z: Vec3 = normal.into();

    // We need another vector that isn't collinear with the normal.
    let other = if nearly_equal(basis_z.x, 1.0) {
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

    let basis_x = Vec3::from(other.cross(basis_z).to_unit());
    let basis_y = basis_z.cross(basis_x);

    let radius_squared: f64 = rng.gen();
    let theta: f64 = rng.gen_range(0.0, 2.0 * f64::consts::PI);

    let radius = radius_squared.sqrt();
    let x = radius * theta.cos();
    let y = radius * theta.sin();
    let z = (1.0 - radius_squared).sqrt();

    Unit3::from_unit_vec3(x * basis_x + y * basis_y + z * basis_z)
}
