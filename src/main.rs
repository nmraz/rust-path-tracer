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

struct BuiltScene(pub Scene<'static>, pub CameraOptions);

fn build_spec_spheres_scene() -> BuiltScene {
    BuiltScene(
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
    )
}

fn build_mirror_scene() -> BuiltScene {
    BuiltScene(
        Scene::with_primitives(vec![
            Primitive::new(
                Sphere::new(
                    Vec3 {
                        x: 0.0,
                        y: -100.0,
                        z: -8.0,
                    },
                    100.0,
                ),
                Material::make_reflective(
                    Vec3 {
                        x: 0.8,
                        y: 0.8,
                        z: 0.8,
                    },
                    0.5,
                    0.9,
                ),
            ),
            Primitive::new(
                Sphere::new(
                    Vec3 {
                        x: 0.0,
                        y: 1.0,
                        z: -4.0,
                    },
                    0.75,
                ),
                Material::make_reflective(
                    Vec3 {
                        x: 0.7,
                        y: 0.7,
                        z: 0.7,
                    },
                    0.9,
                    0.99,
                ),
            ),
            Primitive::new(
                Sphere::new(
                    Vec3 {
                        x: -1.7,
                        y: 1.0,
                        z: -4.0,
                    },
                    0.5,
                ),
                Material::make_diffuse(Vec3 {
                    x: 0.5,
                    y: 0.0,
                    z: 0.0,
                }),
            ),
            Primitive::new(
                Sphere::new(
                    Vec3 {
                        x: 1.7,
                        y: 1.0,
                        z: -4.0,
                    },
                    0.5,
                ),
                Material::make_diffuse(Vec3 {
                    x: 0.0,
                    y: 0.5,
                    z: 0.0,
                }),
            ),
            Primitive::new(
                Sphere::new(
                    Vec3 {
                        x: -0.7,
                        y: 2.3,
                        z: -4.0,
                    },
                    0.3,
                ),
                Material::make_diffuse(Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                }),
            ),
            Primitive::new(
                Sphere::new(
                    Vec3 {
                        x: 1.0,
                        y: 1.8,
                        z: -3.7,
                    },
                    0.1,
                ),
                Material::make_light(
                    Vec3 {
                        x: 1.0,
                        y: 1.0,
                        z: 0.0,
                    } * 10.0,
                ),
            ),
            Primitive::new(
                Sphere::new(
                    Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 1.0,
                    },
                    1.0,
                ),
                Material::make_light(
                    Vec3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    } * 30.0,
                ),
            ),
        ]),
        CameraOptions {
            pos: Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            target: Vec3 {
                x: 0.0,
                y: 0.0,
                z: -10.0,
            },
            up: Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            vert_fov: 55.0,
        },
    )
}

fn build_scene(name: &str) -> Option<BuiltScene> {
    match name {
        "spec-spheres" => Some(build_spec_spheres_scene()),
        "mirror" => Some(build_mirror_scene()),
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

    /// Name of the scene to render. Must be one of spec-spheres or mirror.
    pub scene: String,
}

fn main() -> Result<(), Box<dyn error::Error + 'static>> {
    let cli = CliArgs::from_args();

    let BuiltScene(scene, camera_options) = match build_scene(&cli.scene) {
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
