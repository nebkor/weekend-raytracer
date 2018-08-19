use crate::{Color, HitRecord, Ray};

pub trait Material {
    fn scatter(&self, record: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}
pub struct Metal {
    albedo: Color,
}

impl Lambertian {
    fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}
impl Material for Lambertian {
    fn scatter(&self, rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }
}

impl Metal {
    fn new(albedo: Color) -> Self {
        Metal { albedo }
    }
}

//impl Material for Metal {}
