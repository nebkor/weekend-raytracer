use std::f64::MAX as FMAX;

extern crate rand;
pub use rand::prelude::*;
pub use rand::FromEntropy;

extern crate euclid;

mod color;
pub use color::Color;

mod sphere;
pub use sphere::Sphere;

mod camera;
pub use camera::Camera;

mod point;
pub use point::Point;

mod ray;
pub use ray::*;

impl<G: Glimmer> Glimmer for Vec<G> {
    fn glimmer(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut record: Option<HitRecord> = None;
        for thing in self {
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

pub fn c2u8(color: &Color) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::new();
    v.reserve_exact(4);
    for i in color.iter() {
        v.push(i.floor() as u8);
    }
    v
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

pub fn color<G: Glimmer, R: Rng>(r: Ray, world: &Vec<G>, rng: &mut R) -> Color {
    if let Some(rec) = world.glimmer(&r, 0.001, FMAX) {
        let target = rec.p + rec.n + random_unit_point(rng);
        0.5 * color(Ray::new(rec.p, target - rec.p), world, rng)
    } else {
        let unit = r.direction().normalize();
        let t = 0.5 * (unit.y + 1.);
        // interpolate between blue at the top and white at the bottom
        (1. - t) * Color::c3(1., 1., 1.) + t * Color::c3(0.5, 0.7, 1.0)
    }
}
