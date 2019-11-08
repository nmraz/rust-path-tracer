mod geom;
mod math;
mod renderer;
mod sample;
mod scene;

use std::fs::File;
use std::io::*;

use geom::*;
use math::*;
use renderer::*;
use scene::*;

fn main() {
    let opts = CameraOptions {
        pos: Default::default(),
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
        vert_fov: 60.0,
    };

    let width = 300;
    let height = 200;

    let prim = Primitive::new(
        Sphere::new(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            2.0,
        ),
        Material {
            diffuse_color: Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            emittance_color: Default::default(),
        },
    );
    let cam = Camera::new(&opts, width, height);
    let mut pixels: Box<[Vec3]> =
        vec![Default::default(); (width * height) as usize].into_boxed_slice();

    for y in 0..height {
        for x in 0..width {
            let ray = cam.cast_ray(x, y);
            if prim.geom().intersect(&ray).is_some() {
                pixels[(x + y * width) as usize] = prim.material().diffuse_color;
            }
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
