// use std::f64::MAX as FMAX;

use euclid::Vector3D;
pub use rand::prelude::*;

pub type Color = Vector3D<f32, ()>;
pub type Coloru8 = Vector3D<u8, ()>;
pub type Point3 = Vector3D<f64, ()>;
pub type Vec3 = Vector3D<f64, ()>;

mod ray;
pub use ray::*;

pub fn random_unit_point<R: Rng>(r: &mut R) -> Point3 {
    let mut p: Point3;
    let one = Point3::new(1.0, 1.0, 1.0);
    loop {
        p = (Point3::new(r.gen(), r.gen(), r.gen()) * 2.0) - one;
        if p.square_length() < 1.0 {
            break;
        }
    }
    p
}

pub fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> bool {
    let oc = *r.origin() - *center;
    let a = r.direction().square_length();
    let b = oc.dot(*r.direction()) * 2.0;
    let c = oc.square_length() - radius.powi(2);
    let discrm = b.powi(2) - 4.0 * a * c;
    discrm > 0.0
}

pub fn color(r: &Ray) -> Color {
    if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, &r) {
        Color::new(1.0, 0.0, 0.0)
    } else {
        let unit = r.direction().normalize();
        let t = 0.5 * (unit.y + 1.) as f32;
        // interpolate between blue at the top and white at the bottom
        (Color::new(1., 1., 1.) * (1.0 - t)) + (Color::new(0.5, 0.7, 1.0) * t)
    }
}
