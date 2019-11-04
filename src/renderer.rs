use crate::math::Vec3;

struct SampledPixel {
    total: Vec3,
    count: usize,
}

impl SampledPixel {
    pub fn new() -> SampledPixel {
        SampledPixel {
            total: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            count: 0,
        }
    }

    pub fn add_sample(&mut self, color: Vec3) {
        self.total = self.total + color; // TODO: std::ops::AddAssign et al for Vec3
        self.count += 1;
    }

    pub fn result(&self) -> Vec3 {
        if self.count == 0 {
            self.total
        } else {
            self.total / (self.count as f64)
        }
    }
}

pub struct RenderOptions {
    pub width: usize,
    pub height: usize,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
}
