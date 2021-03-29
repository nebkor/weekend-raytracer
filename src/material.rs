use crate::{random_unit_point, Color64, Glint, Ray};
use rand::Rng;

use std::cell::RefCell;
use std::rc::Rc;

pub type BoxMat = Box<dyn Material>;
pub type MatPtr = Rc<RefCell<BoxMat>>;

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Color64,
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, glint: &Glint) -> Option<Scatter>;
}

pub struct Lambertian<R: Rng> {
    pub albedo: Color64,
    rng: RefCell<R>,
}

impl<R: Rng + 'static> Lambertian<R> {
    pub fn new(albedo: Color64, rng: R) -> Self {
        Lambertian {
            albedo,
            rng: RefCell::new(rng),
        }
    }

    pub fn mat_ptr(self) -> MatPtr {
        Rc::new(RefCell::new(Box::new(self)))
    }
}

impl<R: Rng> Material for Lambertian<R> {
    fn scatter(&self, _ray_in: &Ray, glint: &Glint) -> std::option::Option<Scatter> {
        let mut dir = glint.normal
            + random_unit_point(&mut *(self.rng.borrow_mut()))
                .to_vector()
                .normalize();
        if dir.square_length() < 1.0e-9 {
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
