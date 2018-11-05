use crate::{Sparkle, Point, Ray};

pub struct ScatterRecord {
    attenuation: Point,
    scattered: Ray,
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, sparkle: &Sparkle) -> Option<ScatterRecord>
}

pub struct Metal {
    albedo: Point,
}

pub struct Lambertian {
    albedo: Point,
}
