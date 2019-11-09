mod geom;
mod math;
mod renderer;
mod sample;

use std::fs::File;
use std::io::*;

use geom::*;
use math::*;
use renderer::*;

fn main() {
    let scene = Scene::with_primitives(vec![
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

    let width = 400;
    let height = 200;

    let opts = RenderOptions {
        width,
        height,
        max_depth: 10,
        samples_per_pixel: 1000,
        camera_options: CameraOptions {
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
        },
    };
    let pixels = render(&scene, &opts);

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
