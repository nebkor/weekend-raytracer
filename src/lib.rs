extern crate rand;
pub use rand::prelude::*;
pub use rand::FromEntropy;

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
    fn glimmer(&self, r: &Ray) -> Option<HitRecord> {
        let mut record: Option<HitRecord> = None;
        for thing in self {
            if let Some(hr) = thing.glimmer(r) {
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

pub fn make_ppm_header(w: usize, h: usize, max: usize) -> String {
    format!("P3\n{} {}\n{}\n", w, h, max)
}

pub fn random_unit_point<R: Rng>(r: &mut R) -> Point {
    let mut p: Point;
    loop {
        p = (2.0 * Point::p3(r.gen(), r.gen(), r.gen())) - Point::p3(1.0, 1.0, 1.0);
        if p.len_sq() < 1.0 {
            break;
        }
    }
    p
}

pub fn color<G: Glimmer, R: Rng>(r: Ray, world: &Vec<G>, rng: &mut R) -> Color {
    if let Some(rec) = world.glimmer(&r) {
        let target = rec.p + rec.n + random_unit_point(rng);
        0.5 * color(Ray::new(rec.p, target - rec.p), world, rng)
    } else {
        let unit = r.direction().unit();
        let t = 0.5 * (unit.y() + 1.);
        // interpolate between blue at the top and white at the bottom
        (1. - t) * Color::c3(1., 1., 1.) + t * Color::c3(0.5, 0.7, 1.0)
    }
}
