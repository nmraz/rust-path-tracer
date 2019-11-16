mod geom;
mod img;
mod math;
mod renderer;
mod sample;

use std::fs::File;
use std::io::*;
use std::time::Instant;

use quicli::prelude::*;
use structopt::StructOpt;

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

#[derive(StructOpt)]
struct CliArgs {
    /// Maximum bounce depth
    #[structopt(long, default_value = "5")]
    pub max_depth: u32,

    /// Number of samples to gather per pixel
    #[structopt(long, short)]
    pub samples_per_pixel: u32,

    /// Number of threads to use when rendering in parallel.
    /// If this argument is 0, the number of cores will be used.
    #[structopt(short = "j", default_value = "0")]
    pub threads: u32,

    /// Output filename
    #[structopt(short, default_value = "render.png")]
    pub output_filename: String,
}

fn main() {
    let cli = CliArgs::from_args();

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

        max_depth: cli.max_depth,
        samples_per_pixel: cli.samples_per_pixel,
        threads: cli.threads,
    };

    println!(
        "Rendering {}x{} at {}spp with max depth {}",
        opts.width, opts.height, opts.samples_per_pixel, opts.max_depth
    );

    let start = Instant::now();
    let pixels = render(&scene, &opts).unwrap();
    let elapsed = Instant::now() - start;

    println!("Rendered in {}s", elapsed.as_secs_f64());

    let raw_pixels = img::pixels_to_raw_rgb(pixels.as_ref());

    let mut png = BufWriter::new(File::create(&cli.output_filename).unwrap());
    img::write_png(&mut png, raw_pixels.as_ref(), opts.width, opts.height).unwrap();
    png.flush().unwrap();
}
