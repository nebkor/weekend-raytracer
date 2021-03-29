use crate::{random_unit_point, Color64, Glint, Ray, Vec3};
use rand::rngs::SmallRng;

use std::rc::Rc;

pub type MatPtr = Rc<Box<dyn Material>>;

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Color64,
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, glint: &Glint, rng: &mut SmallRng) -> Option<Scatter>;
}

pub struct Lambertian {
    pub albedo: Color64,
}

impl Lambertian {
    pub fn new(albedo: Color64) -> Self {
        Lambertian { albedo }
    }

    pub fn mat_ptr(self) -> MatPtr {
        Rc::new(Box::new(self))
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray_in: &Ray,
        glint: &Glint,
        rng: &mut SmallRng,
    ) -> std::option::Option<Scatter> {
        let mut dir = glint.normal + random_unit_point(rng).to_vector().normalize();
        if dir.square_length() < 1.0e-12 {
            // it'll fuck things up later if you have a near-zero-len scatter direction
            dir = glint.normal / 2.0;
        }
        let ray = Ray::new(glint.p, dir);
        Some(Scatter {
            ray,
            attenuation: self.albedo,
        })
    }
}

pub struct Metal {
    pub albedo: Color64,
    pub fuzz: f64,
}

impl Metal {
    pub fn mat_ptr(self) -> MatPtr {
        Rc::new(Box::new(self))
    }
}

fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    *incident - (*normal * (2.0 * incident.dot(*normal)))
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, glint: &Glint, rng: &mut SmallRng) -> Option<Scatter> {
        let reflected = reflect(&ray_in.direction().normalize(), &glint.normal);
        let scattered = Ray::new(
            glint.p,
            reflected + random_unit_point(rng).to_vector() * self.fuzz,
        );
        if reflected.dot(glint.normal) > 0.0 {
            Some(Scatter {
                ray: scattered,
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}
