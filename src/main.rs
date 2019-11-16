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

fn build_scene() -> Scene<'static> {
    Scene::with_primitives(vec![
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
                0.83,
                0.95,
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
                0.5,
                0.95,
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
                0.7,
                0.95,
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
                0.6,
                0.95,
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
                0.95,
            ),
        ),
        Primitive::new(
            Sphere::new(
                Vec3 {
                    x: 3.0,
                    y: 3.0,
                    z: 1.1,
                },
                1.0,
            ),
            Material::make_light(
                Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                } * 80.0,
            ),
        ),
    ])
}

fn main() {
    let scene = build_scene();

    let opts = RenderOptions {
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

        width: 800,
        height: 800,

        max_depth: 10,
        samples_per_pixel: 20000,
        threads: 0, // Use number of cpus
    };

    let start = Instant::now();
    let pixels = render(&scene, &opts).unwrap();
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
