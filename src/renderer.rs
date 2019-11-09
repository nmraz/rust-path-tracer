use std::f64;

use rand::Rng;

use crate::geom::*;
use crate::math::*;
use crate::sample::sample_hemisphere;
use crate::scene::*;

#[derive(Debug, Copy, Clone)]
pub struct CameraOptions {
    pub pos: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub vert_fov: f64,
}

#[derive(Debug, Copy, Clone)]
pub struct RenderOptions {
    pub width: u32,
    pub height: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub camera_options: CameraOptions,
}

pub struct Camera {
    pos: Vec3,
    u: Unit3,
    v: Unit3,
    n_with_plane_dist: Vec3,
    aspect_ratio: f64,
    inv_width: f64,
    inv_height: f64,
}

impl Camera {
    pub fn new(options: &CameraOptions, width: u32, height: u32) -> Camera {
        let n = (options.target - options.pos).to_unit();
        let u = options.up.cross(n.into()).to_unit();
        let v = Unit3::from_unit_vec3(Vec3::from(n).cross(u.into()));

        // cot(vert_fov/2)
        let plane_dist = 1.0 / (options.vert_fov * f64::consts::PI / 360.0).tan();

        let fwidth = f64::from(width);
        let fheight = f64::from(height);

        Camera {
            pos: options.pos,
            u,
            v,
            n_with_plane_dist: plane_dist * Vec3::from(n),
            aspect_ratio: fwidth / fheight,
            inv_width: 1.0 / fwidth,
            inv_height: 1.0 / fheight,
        }
    }

    pub fn cast_ray(&self, pixel_x: f64, pixel_y: f64) -> Ray {
        // Map to [-1, 1]
        let ndc_x = 2.0 * (pixel_x * self.inv_width) - 1.0;
        let ndc_y = 2.0 * (pixel_y * self.inv_height) - 1.0;

        let dir = ((-ndc_x * self.aspect_ratio) * Vec3::from(self.u) - ndc_y * Vec3::from(self.v)
            + self.n_with_plane_dist)
            .to_unit();

        Ray {
            origin: self.pos,
            dir,
        }
    }
}

fn intersect<'a>(scene: &'a Scene<'a>, ray: &Ray) -> Option<(&'a Primitive<'a>, IntersectionInfo)> {
    let mut intersected = None;
    for prim in scene.primitives() {
        if let Some(dist) = prim.geom().intersect(ray) {
            match intersected {
                Some((_, min_dist)) => {
                    if dist < min_dist {
                        intersected = Some((prim, dist));
                    }
                }
                None => {
                    intersected = Some((prim, dist));
                }
            }
        }
    }
    intersected.map(|(prim, dist)| {
        (
            prim,
            prim.geom().intersection_info_at(ray.interp(dist), ray),
        )
    })
}

pub fn trace_ray<R: Rng + ?Sized>(
    scene: &Scene,
    ray: &Ray,
    rng: &mut R,
    depth: u32,
    max_depth: u32,
) -> Vec3 {
    if depth >= max_depth {
        return Vec3::default();
    }

    let (prim, intersection_info) = match intersect(scene, ray) {
        None => {
            return Vec3::default();
        }
        Some(info) => info,
    };

    match prim.material() {
        Material::Light(color) => *color,
        Material::Diffuse(color) => {
            let dir = sample_hemisphere(intersection_info.normal, rng);
            let incoming = trace_ray(
                scene,
                &Ray {
                    origin: intersection_info.point,
                    dir,
                },
                rng,
                depth + 1,
                max_depth,
            );
            Vec3::from(dir).dot(intersection_info.normal.into()) * color.component_mul(incoming)
        }
    }
}
