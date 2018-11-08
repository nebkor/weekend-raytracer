use crate::{Point, Ray, Bounce};

pub struct ScatterRecord {
    attenuation: Point,
    scattered: Ray,
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, bounce: &Bounce) -> Option<ScatterRecord>
}

pub struct Metal {
    albedo: Point,
}

pub struct Lambertian {
    albedo: Point,
}
