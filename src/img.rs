use std::io::Write;

use crate::math::Vec3;

fn channel_to_raw(chan: f64) -> u8 {
    (chan.max(0.0).min(1.0) * 255.0) as u8
}

pub fn pixels_to_raw_rgb(pixels: &[Vec3]) -> Box<[u8]> {
    let mut raw_buf = Vec::with_capacity(pixels.len() * 3);

    for pixel in pixels {
        raw_buf.push(channel_to_raw(pixel.x));
        raw_buf.push(channel_to_raw(pixel.y));
        raw_buf.push(channel_to_raw(pixel.z));
    }

    raw_buf.into_boxed_slice()
}

pub fn write_png<W: Write>(
    writer: &mut W,
    raw_pixels: &[u8],
    width: u32,
    height: u32,
) -> Result<(), png::EncodingError> {
    assert_eq!(raw_pixels.len(), (width * height * 3) as usize);

    let mut enc = png::Encoder::new(writer, width, height);
    enc.set_color(png::ColorType::RGB);
    enc.set_depth(png::BitDepth::Eight);

    enc.write_header()?.write_image_data(raw_pixels)
}
