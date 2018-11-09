use crate::{random_unit_point, Bounce, Point, Ray, Rng};
use std::cell::RefCell;

pub type BoxMat = Box<dyn Material>;

pub struct ScatterRecord {
    pub attenuation: Point,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, bounce: &Bounce) -> Option<ScatterRecord>;
}

pub struct Metal {
    albedo: Point,
}

pub struct Lambertian<R: Rng> {
    albedo: Point,
    rng: RefCell<R>,
}

impl Metal {
    pub fn new(albedo: Point) -> Self {
        Metal { albedo }
    }
}

impl<R: Rng> Lambertian<R> {
    pub fn new(albedo: Point, rng: R) -> Self {
        Lambertian {
            albedo,
            rng: RefCell::new(rng),
        }
    }
}

impl<R: Rng> Material for Lambertian<R> {
    fn scatter(&self, _ray_in: &Ray, bounce: &Bounce) -> Option<ScatterRecord> {
        let target = bounce.p + bounce.n + random_unit_point(&mut *(self.rng.borrow_mut()));
        let scattered = Ray::new(bounce.p, target - bounce.p);
        Some(ScatterRecord {
            attenuation: self.albedo.clone(),
            scattered,
        })
    }
}

fn reflect(v: &Point, n: &Point) -> Point {
    *v - *n * 2.0 * v.dot(*n)
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, bounce: &Bounce) -> Option<ScatterRecord> {
        let reflected = reflect(&(ray_in.direction().normalize()), &bounce.n);
        let scattered = Ray::new(bounce.p, reflected);
        if scattered.direction().dot(bounce.n) > 0.0 {
            Some(ScatterRecord {
                attenuation: self.albedo.clone(),
                scattered,
            })
        } else {
            None
        }
    }
}
