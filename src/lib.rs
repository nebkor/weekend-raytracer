use std::f64::MAX as FMAX;

pub use rand::prelude::*;
pub use rand::FromEntropy;

use euclid::*;
pub type Color = Vector3D<f32>;
pub type Coloru8 = Vector3D<u8>;
pub type Point = Vector3D<f64>;
pub type Vec3D = Vector3D<f64>;

mod sphere;
pub use crate::sphere::Sphere;

mod camera;
pub use crate::camera::Camera;

mod ray;
pub use crate::ray::*;

mod material;
pub use crate::material::*;

pub type World<'w> = &'w [&'w dyn Glimmer];
pub type ImageBuf = Vec<u8>;

pub trait Gamma {
    fn gamma_correct(&self, factor: f32) -> Self;
}

impl Gamma for Color {
    fn gamma_correct(&self, factor: f32) -> Self {
        let pow = 1.0 / factor;
        Color::new(self.x.powf(pow), self.y.powf(pow), self.z.powf(pow))
    }
}

impl Glimmer for World<'_> {
    fn glimmer(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut record: Option<HitRecord> = None;
        for thing in self.iter() {
            if let Some(hr) = thing.glimmer(r, t_min, t_max) {
                match record {
                    None => record = Some(hr),
                    Some(prev) => if hr.t < prev.t {
                        record = Some(hr)
                    },
                }
            }
        }
        record
    }
}

pub fn random_unit_point<R: Rng>(r: &mut R) -> Point {
    let mut p: Point;
    let one = Point::new(1.0, 1.0, 1.0);
    loop {
        p = (Point::new(r.gen(), r.gen(), r.gen()) * 2.0) - one;
        if p.square_length() < 1.0 {
            break;
        }
    }
    p
}

pub fn color<R: Rng>(r: Ray, world: World<'_>, rng: &mut R) -> Color {
    if let Some(rec) = world.glimmer(&r, 0.001, FMAX) {
        let target = rec.p + rec.n + random_unit_point(rng);
        color(Ray::new(rec.p, target - rec.p), world, rng) * 0.5
    } else {
        let unit = r.direction().normalize();
        let t = 0.5 * (unit.y + 1.) as f32;
        // interpolate between blue at the top and white at the bottom
        (Color::new(1., 1., 1.) * (1.0 - t) + Color::new(0.5, 0.7, 1.0)) * t
    }
}
