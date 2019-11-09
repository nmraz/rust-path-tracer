mod geom;
mod math;
mod renderer;
mod sample;
mod scene;

use std::fs::File;
use std::io::*;

use rand::Rng;

use geom::*;
use math::*;
use renderer::*;
use scene::*;

fn main() {
    let width = 400;
    let height = 200;

    let max_depth = 10;
    let spp = 1000;

    let s = Scene::with_primitives(vec![
        Primitive::new(
            Sphere::new(
                Vec3 {
                    x: 2.5,
                    y: 0.0,
                    z: -5.0,
                },
                2.0,
            ),
            Material::Diffuse(Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }),
        ),
        Primitive::new(
            Sphere::new(
                Vec3 {
                    x: -2.5,
                    y: 0.0,
                    z: -5.0,
                },
                2.0,
            ),
            Material::Diffuse(Vec3 {
                x: 1.0,
                y: 0.0,
                z: 1.0,
            }),
        ),
        Primitive::new(
            Sphere::new(
                Vec3 {
                    x: 0.0,
                    y: -12.0,
                    z: -3.0,
                },
                10.0,
            ),
            Material::Diffuse(Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }),
        ),
        Primitive::new(
            Sphere::new(
                Vec3 {
                    x: -3.0,
                    y: -5.0,
                    z: -2.0,
                },
                3.0,
            ),
            Material::Light(Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.5,
            }),
        ),
        Primitive::new(
            Sphere::new(
                Vec3 {
                    x: 0.0,
                    y: 8.0,
                    z: -3.0,
                },
                6.0,
            ),
            Material::Light(Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }),
        ),
        Primitive::new(
            Sphere::new(
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 7.0,
                },
                6.0,
            ),
            Material::Light(Vec3 {
                x: 1.0,
                y: 1.0,
                z: 0.0,
            }),
        ),
    ]);

    let opts = CameraOptions {
        pos: Vec3::default(),
        target: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        up: Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        vert_fov: 55.0,
    };
    let cam = Camera::new(&opts, width, height);
    let mut pixels = vec![Vec3::default(); (width * height) as usize].into_boxed_slice();

    let mut rng = rand::thread_rng();

    for y in 0..height {
        for x in 0..width {
            let total_sampled = (0..spp)
                .map(|_| {
                    let ray = cam.cast_ray(
                        f64::from(x) + rng.gen::<f64>(),
                        f64::from(y) + rng.gen::<f64>(),
                    );
                    trace_ray(&s, &ray, &mut rng, 0, max_depth)
                })
                .fold(Vec3::default(), |acc, val| acc + val);
            pixels[(x + y * width) as usize] = total_sampled / f64::from(spp);
        }
    }

    let mut ppm = BufWriter::new(File::create("image.ppm").unwrap());
    writeln!(ppm, "P3").unwrap();
    writeln!(ppm, "{} {}", width, height).unwrap();
    writeln!(ppm, "255").unwrap();

    for pixel in pixels.iter() {
        let r = (pixel.x * 255.0) as u8;
        let g = (pixel.y * 255.0) as u8;
        let b = (pixel.z * 255.0) as u8;
        writeln!(ppm, "{} {} {}", r, g, b).unwrap();
    }

    ppm.flush().unwrap();
}
