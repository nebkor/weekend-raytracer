use crate::{random_unit_point, Color64, Glint, Ray, Vec3};
use rand::rngs::SmallRng;
use rand::Rng;

use std::sync::Arc;

pub type MatPtr = Arc<Box<dyn Material + Send + Sync>>;

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
        Arc::new(Box::new(self))
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray_in: &Ray,
        glint: &Glint,
        rng: &mut SmallRng,
    ) -> std::option::Option<Scatter> {
        let mut dir = glint.normal + random_unit_point(rng).to_vector().normalize();
        if dir.square_length() < 1.0e-12 {
            // it'll fuck things up later if you have a near-zero-len scatter direction
            dir = glint.normal / 2.0;
        }
        let ray = Ray::new(glint.p, dir, ray_in.time());
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
        Arc::new(Box::new(self))
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
            ray_in.time(),
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

pub struct Dialectric {
    // index of refraction
    pub i_o_r: f64,
}

impl Dialectric {
    pub fn mat_ptr(self) -> MatPtr {
        Arc::new(Box::new(self))
    }
}

fn refract(incident: &Vec3, normal: &Vec3, etais: f64) -> Vec3 {
    // incident is a unit vector
    let cos = ((*incident * -1.0).dot(*normal)).min(1.0);
    let perp: Vec3 = (*incident + (*normal * cos)) * etais;
    let para: Vec3 = *normal * (1.0 - perp.square_length()).abs().sqrt() * -1.0;
    perp + para
}

fn reflectance(cos: f64, i_o_r: f64) -> f64 {
    let r = ((1.0 - i_o_r) / (1.0 + i_o_r)).powi(2);
    r + (1.0 - r) * (1.0 - cos).powi(5)
}

impl Material for Dialectric {
    fn scatter(&self, ray_in: &Ray, glint: &Glint, rng: &mut SmallRng) -> Option<Scatter> {
        let ratio = if glint.front_facing {
            self.i_o_r.powi(-1)
        } else {
            self.i_o_r
        };
        let unit_incident = ray_in.direction().normalize();
        let cos = ((unit_incident * -1.0).dot(glint.normal)).min(1.0);
        let sin = (1.0 - cos.powi(2)).sqrt();
        let ref_prob = reflectance(cos, ratio);

        let dir = if (ratio * sin) > 1.0 || ref_prob > rng.gen() {
            // we can't refract, we're internally reflecting
            reflect(&unit_incident, &glint.normal)
        } else {
            refract(&unit_incident, &glint.normal, ratio)
        };

        let ray_out = Ray::new(glint.p, dir, ray_in.time());

        Some(Scatter {
            attenuation: Color64::one(),
            ray: ray_out,
        })
    }
}
