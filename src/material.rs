use crate::{random_unit_point, Color, HitRecord, Ray, SmallRng};

pub trait Material {
    fn scatter(&mut self, record: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
    rng: SmallRng,
}
pub struct Metal {
    albedo: Color,
}

impl Lambertian {
    fn new(albedo: Color, rng: SmallRng) -> Self {
        Lambertian { albedo, rng }
    }
}
impl Material for Lambertian {
    fn scatter(&mut self, rec: &HitRecord) -> Option<(Color, Ray)> {
        let target = rec.p + rec.n + random_unit_point(&mut self.rng);
        let scattered = Ray::new(rec.p, target - rec.p);
        Some((self.albedo.clone(), scattered))
    }
}

impl Metal {
    fn new(albedo: Color) -> Self {
        Metal { albedo }
    }
}

//impl Material for Metal {}
