use crate::math::Vec3;

pub struct CameraOptions {
    pub pos: Vec3,
    pub looking_at: Vec3,
    pub up: Vec3,
    pub vert_fov: f64,
}

pub struct RenderOptions {
    pub width: usize,
    pub height: usize,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub camera_options: CameraOptions,
}
