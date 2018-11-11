use crate::{random_unit_point, Bounce, Color, Point, Ray, Rng};
use std::cell::RefCell;

pub type BoxMat = Box<dyn Material>;

pub struct ScatterRecord {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, bounce: &Bounce) -> Option<ScatterRecord>;
}

pub struct Metal<R: Rng> {
    albedo: Color,
    fuzz: f64,
    rng: RefCell<R>,
}

pub struct Lambertian<R: Rng> {
    albedo: Color,
    rng: RefCell<R>,
}

impl<R: Rng> Metal<R> {
    pub fn new(albedo: Color, fuzz: f64, rng: R) -> Self {
        Metal {
            albedo,
            fuzz: match fuzz {
                fuzz if fuzz < 0.0 => 0.0,
                fuzz if fuzz < 1.0 => fuzz,
                _ => 1.0,
            },
            rng: RefCell::new(rng),
        }
    }
}

impl<R: Rng> Lambertian<R> {
    pub fn new(albedo: Color, rng: R) -> Self {
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
            attenuation: self.albedo,
            scattered,
        })
    }
}

fn reflect(v: &Point, n: &Point) -> Point {
    *v - *n * 2.0 * v.dot(*n)
}

impl<R: Rng> Material for Metal<R> {
    fn scatter(&self, ray_in: &Ray, bounce: &Bounce) -> Option<ScatterRecord> {
        let reflected = reflect(&(ray_in.direction().normalize()), &bounce.n);
        let scattered = Ray::new(
            bounce.p,
            reflected + (random_unit_point(&mut *(self.rng.borrow_mut())) * self.fuzz),
        );
        if scattered.direction().dot(bounce.n) > 0.0 {
            Some(ScatterRecord {
                attenuation: self.albedo,
                scattered,
            })
        } else {
            None
        }
    }
}

fn refract(v: &Point, n: &Point, ni_nt: f64) -> Option<Point> {
    let unit = v.normalize();
    let dt = unit.dot(*n);
    let discrim: f64 = 1.0 - ni_nt.powi(2) * (1.0 - dt.powi(2));
    if discrim > 0.0 {
        let refracted = (unit - *n * dt) * ni_nt - *n * discrim.sqrt();
        Some(refracted)
    } else {
        None
    }
}

pub struct Dialectric<R: Rng> {
    refractive_index: f64,
    rng: RefCell<R>,
}

impl<R: Rng> Dialectric<R> {
    pub fn new(refractive_index: f64, rng: R) -> Self {
        Dialectric {
            refractive_index,
            rng: RefCell::new(rng),
        }
    }
}

impl<R: Rng> Material for Dialectric<R> {
    fn scatter(&self, ray_in: &Ray, bounce: &Bounce) -> Option<ScatterRecord> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let reflected = reflect(&ray_in.direction(), &bounce.n);
        let (ni_nt, outward_normal) = if ray_in.direction().dot(bounce.n) > 0.0 {
            (self.refractive_index, bounce.n * -1.0)
        } else {
            (1.0 / self.refractive_index, bounce.n)
        };

        if let Some(refracted) = refract(&ray_in.direction(), &outward_normal, ni_nt) {
            Some(ScatterRecord {
                attenuation,
                scattered: Ray::new(bounce.p, refracted),
            })
        } else {
            None
        }
    }
}
