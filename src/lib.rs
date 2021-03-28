// use std::f64::MAX as FMAX;

use euclid::{Point3D, UnknownUnit, Vector3D};
pub use rand::prelude::*;

pub type Color64 = Vector3D<f64, UnknownUnit>;
pub type Color8 = Vector3D<u8, UnknownUnit>;
pub type Point3 = Point3D<f64, UnknownUnit>;
pub type Vec3 = Vector3D<f64, UnknownUnit>;

pub const PI: f64 = std::f64::consts::PI;

mod ray;
pub use ray::*;
mod sphere;
pub use sphere::*;

pub fn d2r(d: f64) -> f64 {
    (d * PI) / 180.0
}

pub fn random_unit_point(r: &mut SmallRng) -> Point3 {
    let mut p: Vec3;
    let one = Vec3::one();
    loop {
        p = (Vec3::new(r.gen(), r.gen(), r.gen()) * 2.0) - one;
        if p.square_length() < 1.0 {
            break;
        }
    }
    p.to_point()
}

pub fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = *r.origin() - *center;
    let a = r.direction().square_length();
    let half_b = oc.dot(*r.direction());
    let c = oc.square_length() - radius.powi(2);
    let discrm = half_b.powi(2) - a * c;
    if discrm < 0.0 {
        -1.0
    } else {
        (-half_b - discrm.sqrt()) / a
    }
}

pub fn color(r: &Ray) -> Color64 {
    let t = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, &r);
    if t > 0.0 {
        let n = (r.at(t) - Vec3::new(0.0, 0.0, -1.0))
            .to_vector()
            .normalize();
        // the reason to add one to each component and then divide the vector by two is to make sure all values
        // are positive when it gets cast to a color8.
        Color64::new(n.x + 1.0, n.y + 1.0, n.z + 1.0) / 2.0
    } else {
        let unit = r.direction().normalize();
        let t = 0.5 * (unit.y + 1.);
        // interpolate between blue at the top and white at the bottom
        (Color64::new(1., 1., 1.) * (1.0 - t)) + (Color64::new(0.5, 0.7, 1.0) * t)
    }
}
