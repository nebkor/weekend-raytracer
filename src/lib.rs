extern crate rand;
use rand::distributions::StandardNormal;
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
    loop {
        p = (2.0 * Point::p3(r.gen(), r.gen(), r.gen())) - Point::p3(1.0, 1.0, 1.0);
        if p.len_sq() < 1.0 {
            break;
        }
    }
    p
}

pub fn normalize_cube_point<R: Rng>(r: &mut R) -> Point {
    let scale: f64 = r.gen();
    ((2.0 * Point::p3(r.gen(), r.gen(), r.gen())) - Point::p3(1.0, 1.0, 1.0)).unit() * scale
}

fn incorrectly_scaled_point<R: Rng>(r: &mut R) -> Point {
    Point::p3(
        r.sample(StandardNormal),
        r.sample(StandardNormal),
        r.sample(StandardNormal),
    ).unit() * r.gen::<f64>()
}

fn correctly_scaled_point<R: Rng>(r: &mut R) -> Point {
    Point::p3(
        r.sample(StandardNormal),
        r.sample(StandardNormal),
        r.sample(StandardNormal),
    ).unit() * r.gen::<f64>().cbrt()
}

fn fast_cbrt(x: f32) -> f32 {
    let i0: u32 = unsafe { std::mem::transmute(x) };
    let i1 = i0 / 4 + i0 / 16;
    let i2 = i1 + i1 / 16;
    let i3 = i2 + i2 / 256;
    let j = 0x2a511cd0 + i3;
    unsafe { std::mem::transmute(j) }
}

fn gauss_fast_cbrt<R: Rng>(r: &mut R) -> Point {
    let scale = fast_cbrt(r.gen::<f32>()) as f64;
    Point::p3(
        r.sample(StandardNormal),
        r.sample(StandardNormal),
        r.sample(StandardNormal),
    ).unit() * scale
}

pub fn color<G: Glimmer, R: Rng>(r: Ray, world: &Vec<G>, rng: &mut R) -> Color {
    if let Some(rec) = world.glimmer(&r) {
        let target = rec.p + rec.n + gauss_fast_cbrt(rng);
        0.5 * color(Ray::new(rec.p, target - rec.p), world, rng)
    } else {
        let unit = r.direction().unit();
        let t = 0.5 * (unit.y() + 1.);
        // interpolate between blue at the top and white at the bottom
        (1. - t) * Color::c3(1., 1., 1.) + t * Color::c3(0.5, 0.7, 1.0)
    }
}
