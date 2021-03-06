use std::error;
use std::f64;

use rand::Rng;
use rayon::prelude::*;

use crate::geom::*;
use crate::math::*;
use crate::sample::*;

#[derive(Debug, Copy, Clone)]
pub struct CameraOptions {
    pub pos: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub vert_fov: f64,
}

#[derive(Debug, Copy, Clone)]
pub struct RenderOptions {
    pub camera_options: CameraOptions,
    pub width: u32,
    pub height: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub threads: u32,
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

#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub emittance: Vec3,
    pub albedo: Vec3,
    pub reflectance: f64,
    pub gloss: f64,
}

impl Material {
    pub fn make_light(color: Vec3) -> Material {
        Material {
            emittance: color,
            albedo: Vec3::default(),
            reflectance: 0.0,
            gloss: 0.0,
        }
    }

    pub fn make_diffuse(color: Vec3) -> Material {
        Material {
            emittance: Vec3::default(),
            albedo: color,
            reflectance: 0.0,
            gloss: 0.0,
        }
    }

    pub fn make_reflective(color: Vec3, reflectance: f64, gloss: f64) -> Material {
        Material {
            emittance: Vec3::default(),
            albedo: color,
            reflectance,
            gloss,
        }
    }
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

struct IntersectionInfo<'a> {
    pub prim: &'a Primitive<'a>,
    pub point: Vec3,
    pub normal: Unit3,
    pub inside: bool,
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

    fn intersect(&'a self, ray: &Ray) -> Option<IntersectionInfo<'a>> {
        let mut intersected = None;
        for prim in self.primitives() {
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
            let point = ray.interp(dist);
            let normal = prim.geom().normal_at(point);
            // Note: == 0 means tangent, still outside.
            let inside = Vec3::from(normal).dot(ray.dir.into()) > 0.0;
            IntersectionInfo {
                prim,
                point,
                normal: if inside {
                    (-Vec3::from(normal)).to_unit()
                } else {
                    normal
                },
                inside,
            }
        })
    }

    fn trace_reflection<R: Rng + ?Sized>(
        &self,
        ray: &Ray,
        info: &IntersectionInfo,
        rng: &mut R,
        depth: u32,
        max_depth: u32,
    ) -> Vec3 {
        let material = info.prim.material();

        if material.reflectance > 0.0 && rng.gen::<f64>() < material.reflectance {
            let alpha = (1.0 - material.gloss) * f64::consts::FRAC_PI_2;
            let cos_alpha = alpha.cos();

            let ray_dir: Vec3 = ray.dir.into();
            let normal: Vec3 = info.normal.into();

            let reflection_dir = ray_dir - 2.0 * ray_dir.dot(normal) * normal;

            let dir = sample_uniform_cone(Unit3::from_unit_vec3(reflection_dir), alpha, rng);
            let cos_theta = Vec3::from(dir).dot(normal);

            if cos_theta < 0.0 {
                // TODO: take into account in PDF, BDRF?
                return Vec3::default();
            }

            let incoming = self.trace_ray(
                &Ray {
                    origin: info.point,
                    dir,
                },
                rng,
                depth,
                max_depth,
            );

            let coeff = if cos_alpha < EPSILON {
                1.0
            } else {
                // PDF = 1 / (2pi * (1 - cos(alpha))), BDRF = 1 / (pi * (1 - cos^2(alpha)))
                2.0 * (1.0 - cos_alpha) / (1.0 - cos_alpha * cos_alpha)
            };

            return coeff * cos_theta * incoming;
        }

        if material.albedo.mag_squared() > EPSILON {
            let dir = sample_cos_weighted_hemisphere(info.normal, rng);
            let incoming = self.trace_ray(
                &Ray {
                    origin: info.point,
                    dir,
                },
                rng,
                depth,
                max_depth,
            );
            return material.albedo.component_mul(incoming);
        }

        Vec3::default()
    }

    pub fn trace_ray<R: Rng + ?Sized>(
        &self,
        ray: &Ray,
        rng: &mut R,
        depth: u32,
        max_depth: u32,
    ) -> Vec3 {
        if depth >= max_depth {
            return Vec3::default();
        }

        let info = match self.intersect(ray) {
            None => {
                return Vec3::default();
            }
            Some(info) => info,
        };

        let material = info.prim.material();

        material.emittance + self.trace_reflection(ray, &info, rng, depth + 1, max_depth)
    }
}

pub fn render_to(
    scene: &Scene,
    pixels: &mut [Vec3],
    opts: &RenderOptions,
) -> Result<(), Box<dyn error::Error + 'static>> {
    assert_eq!(pixels.len(), (opts.width * opts.height) as usize);

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(opts.threads as usize)
        .build()?;

    let cam = Camera::new(&opts.camera_options, opts.width, opts.height);

    pool.install(|| {
        pixels.par_iter_mut().enumerate().for_each(|(idx, pixel)| {
            let x = (idx as u32) % opts.width;
            let y = (idx as u32) / opts.width;

            let mut rng = rand::thread_rng();

            let total_sampled = (0..opts.samples_per_pixel)
                .map(|_| {
                    let ray = cam.cast_ray(
                        f64::from(x) + rng.gen::<f64>(),
                        f64::from(y) + rng.gen::<f64>(),
                    );
                    scene.trace_ray(&ray, &mut rng, 0, opts.max_depth)
                })
                .fold(Vec3::default(), |a, b| a + b);

            *pixel = total_sampled / f64::from(opts.samples_per_pixel);
        })
    });

    Ok(())
}

pub fn render(
    scene: &Scene,
    opts: &RenderOptions,
) -> Result<Box<[Vec3]>, Box<dyn error::Error + 'static>> {
    let mut pixels = vec![Vec3::default(); (opts.width * opts.height) as usize].into_boxed_slice();
    render_to(scene, &mut pixels, opts)?;
    Ok(pixels)
}
