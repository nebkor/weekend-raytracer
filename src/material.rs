use crate::{get_rng, random_unit_point, Bounce, Color, Point, Ray};
use rand::prelude::*;
use std::cell::RefCell;

pub type BoxMat = Box<dyn Material>;

pub struct ScatterRecord {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, bounce: &Bounce) -> Option<ScatterRecord>;
}

pub enum MatSpec {
    Lambertian(Color),
    Metal(Color, f64),
    Dialectric(f64),
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
    rng: RefCell<SmallRng>,
}

pub struct Lambertian {
    albedo: Color,
    rng: RefCell<SmallRng>,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzz: match fuzz {
                fuzz if fuzz < 0.0 => 0.0,
                fuzz if fuzz < 1.0 => fuzz,
                _ => 1.0,
            },
            rng: RefCell::new(get_rng()),
        }
    }
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian {
            albedo,
            rng: RefCell::new(get_rng()),
        }
    }
}

impl Material for Lambertian {
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

impl Material for Metal {
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

fn schlick(cosine: f64, refractive_index: f64) -> f64 {
    let r0 = ((1.0 - refractive_index) / (1.0 + refractive_index)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

pub struct Dialectric {
    refractive_index: f64,
    rng: RefCell<SmallRng>,
}

impl Dialectric {
    pub fn new(refractive_index: f64) -> Self {
        Dialectric {
            refractive_index,
            rng: RefCell::new(get_rng()),
        }
    }
}

impl Material for Dialectric {
    fn scatter(&self, ray_in: &Ray, bounce: &Bounce) -> Option<ScatterRecord> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let reflected = reflect(&ray_in.direction(), &bounce.n);
        let norm_dot = ray_in.direction().dot(bounce.n);
        let (ni_nt, outward_normal, cosine) = if norm_dot > 0.0 {
            let cosine = self.refractive_index * norm_dot / ray_in.direction().length();
            (self.refractive_index, bounce.n * -1.0, cosine)
        } else {
            let cosine = -1.0 * norm_dot / ray_in.direction().length();
            (1.0 / self.refractive_index, bounce.n, cosine)
        };

        match refract(&ray_in.direction(), &outward_normal, ni_nt) {
            Some(refracted) => Some(ScatterRecord {
                attenuation,
                scattered: if self.rng.borrow_mut().gen::<f64>()
                    < schlick(cosine, self.refractive_index)
                {
                    Ray::new(bounce.p, reflected)
                } else {
                    Ray::new(bounce.p, refracted)
                },
            }),
            None => Some(ScatterRecord {
                attenuation,
                scattered: Ray::new(bounce.p, reflected),
            }),
        }
    }
}
