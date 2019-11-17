mod geom;
mod img;
mod math;
mod renderer;
mod sample;

use std::error;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::process;
use std::time::Instant;

use structopt::StructOpt;

use geom::Sphere;
use math::Vec3;
use renderer::*;

fn build_scene(name: &str) -> Option<(Scene<'static>, CameraOptions)> {
    match name {
        "spec-balls" => Some((
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
            ]),
            CameraOptions {
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
        )),
        _ => None,
    }
}

#[derive(StructOpt)]
struct CliArgs {
    /// Width of rendered image, in pixels
    #[structopt(long, short)]
    pub width: u32,

    /// Height of rendered image, in pixels
    #[structopt(long, short)]
    pub height: u32,

    /// Maximum bounce depth
    #[structopt(long, default_value = "5")]
    pub max_depth: u32,

    /// Number of samples to gather per pixel
    #[structopt(long = "spp")]
    pub samples_per_pixel: u32,

    /// Number of threads to use when rendering in parallel.
    /// If this argument is 0, the number of cores will be used.
    #[structopt(short = "j", default_value = "0")]
    pub threads: u32,

    /// Output filename
    #[structopt(short, default_value = "render.png")]
    pub output_filename: String,

    /// Name of scene to render. Currently must be spec-balls.
    pub scene: String,
}

fn main() -> Result<(), Box<dyn error::Error + 'static>> {
    let cli = CliArgs::from_args();

    let (scene, camera_options) = match build_scene(&cli.scene) {
        Some(scene) => scene,
        None => {
            eprintln!(
                "error: Unknown scene '{}'\n\nFor more information try --help",
                cli.scene
            );
            process::exit(1);
        }
    };

    let opts = RenderOptions {
        camera_options,

        width: cli.width,
        height: cli.height,

        max_depth: cli.max_depth,
        samples_per_pixel: cli.samples_per_pixel,
        threads: cli.threads,
    };

    println!(
        "Rendering {} at {}x{} {}spp with max depth {}",
        cli.scene, opts.width, opts.height, opts.samples_per_pixel, opts.max_depth
    );

    let start = Instant::now();
    let pixels = render(&scene, &opts)?;
    let elapsed = Instant::now() - start;

    println!("Rendered in {}s", elapsed.as_secs_f64());

    let raw_pixels = img::pixels_to_raw_rgb(pixels.as_ref());

    let mut png = BufWriter::new(File::create(&cli.output_filename)?);
    img::write_png(&mut png, raw_pixels.as_ref(), opts.width, opts.height)?;
    png.flush()?;

    Ok(())
}
