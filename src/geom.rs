use crate::math::*;

pub trait Geom: Sync {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
    fn normal_at(&self, point: Vec3) -> Unit3;
}

#[derive(Copy, Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        assert!(radius > 0.0, "Invalid sphere radius");
        Sphere { center, radius }
    }

    pub fn center(&self) -> Vec3 {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Geom for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        // t^2 + 2t * (origin - center) . dir + |origin - center|^2 - r^2 = 0
        // Divided by 2 here for stability
        let oc = ray.origin - self.center;
        let b = oc.dot(ray.dir.into());
        let discriminant = b * b - oc.mag_squared() + self.radius * self.radius;
        if discriminant < 0.0 {
            return None;
        }
        let radical = discriminant.sqrt();
        let t1 = -b - radical;
        let t2 = -b + radical;

        // Prefer intersections closer to the origin first (but always ignore those behind the ray).

        if t1 > EPSILON {
            return Some(t1);
        }

        if t2 > EPSILON {
            return Some(t2);
        }

        None
    }

    fn normal_at(&self, point: Vec3) -> Unit3 {
        let outward = point - self.center;
        debug_assert!(
            nearly_equal(outward.mag_squared(), self.radius * self.radius),
            "Point not on sphere"
        );
        outward.to_unit()
    }
}
