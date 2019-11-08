use std::f64;

use crate::math::*;

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

    pub fn cast_ray(&self, pixel_x: u32, pixel_y: u32) -> Ray {
        // Map to [-1, 1], cast through pixel centers.
        let ndc_x = 2.0 * ((f64::from(pixel_x) + 0.5) * self.inv_width) - 1.0;
        let ndc_y = 2.0 * ((f64::from(pixel_y) + 0.5) * self.inv_height) - 1.0;

        let dir = ((-ndc_x * self.aspect_ratio) * Vec3::from(self.u) - ndc_y * Vec3::from(self.v)
            + self.n_with_plane_dist)
            .to_unit();

        Ray {
            origin: self.pos,
            dir,
        }
    }
}
