mod geom;
mod img;
mod math;
mod renderer;
mod sample;

use std::fs::File;
use std::io::*;
use std::time::Instant;

use geom::Sphere;
use math::Vec3;
use renderer::*;

fn main() {
    let scene = Scene::with_primitives(vec![
        Primitive::new(
            Sphere::new(
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: -6.0,
                },
                1.0,
            ),
            Material::make_reflective(
                Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
                0.4,
                0.9,
            ),
        ),
        Primitive::new(
            Sphere::new(
                Vec3 {
                    x: 0.0,
                    y: 2.05,
                    z: -6.0,
                },
                0.75,
            ),
            Material::make_reflective(
                Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                0.4,
                0.9,
            ),
        ),
        Primitive::new(
            Sphere::new(
                Vec3 {
                    x: 2.05,
                    y: 0.0,
                    z: -6.0,
                },
                0.75,
            ),
            Material::make_reflective(
                Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 0.0,
                },
                0.4,
                0.9,
            ),
        ),
        Primitive::new(
            Sphere::new(
                Vec3 {
                    x: 0.0,
                    y: -2.05,
                    z: -6.0,
                },
                0.75,
            ),
            Material::make_reflective(
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                0.4,
                0.9,
            ),
        ),
        Primitive::new(
            Sphere::new(
                Vec3 {
                    x: -2.05,
                    y: 0.0,
                    z: -6.0,
                },
                0.75,
            ),
            Material::make_reflective(
                Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 1.0,
                },
                0.4,
                0.9,
            ),
        ),
        Primitive::new(
            Sphere::new(
                Vec3 {
                    x: -7.5,
                    y: 5.0,
                    z: -3.0,
                },
                1.0,
            ),
            Material::make_light(
                Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                } * 240.0,
            ),
        ),
        Primitive::new(
            Sphere::new(
                Vec3 {
                    x: 7.5,
                    y: -5.0,
                    z: -9.0,
                },
                1.0,
            ),
            Material::make_light(
                Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                } * 240.0,
            ),
        ),
    ]);

    let opts = RenderOptions {
        width: 800,
        height: 800,
        max_depth: 10,
        samples_per_pixel: 20000,
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

    let start = Instant::now();
    let pixels = render(&scene, &opts);
    let elapsed = Instant::now() - start;

    println!(
        "Rendered {}x{} at {}spp in {}s",
        opts.width,
        opts.height,
        opts.samples_per_pixel,
        elapsed.as_secs_f64()
    );

    let raw_pixels = img::pixels_to_raw_rgb(pixels.as_ref());

    let mut png = BufWriter::new(File::create("image.png").unwrap());
    img::write_png(&mut png, raw_pixels.as_ref(), opts.width, opts.height).unwrap();
    png.flush().unwrap();
}
