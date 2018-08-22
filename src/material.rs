use crate::{random_unit_point, reflect, Color, HitRecord, Ray, Rng};

pub trait Material {
    fn scatter(&mut self, ray_in: &Ray, record: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian<R: Rng> {
    albedo: Color,
    rng: R,
}
pub struct Metal {
    albedo: Color,
}

impl<R: Rng> Lambertian<R> {
    fn new(albedo: Color, rng: R) -> Self {
        Lambertian { albedo, rng }
    }
}

impl<R: Rng> Material for Lambertian<R> {
    fn scatter(&mut self, _: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
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

impl Material for Metal {
    fn scatter(&mut self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = reflect(&(ray_in.direction().normalize()), &(rec.n));
        let scattered = Ray::new(rec.p, reflected);
        if scattered.direction().dot(rec.n) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
