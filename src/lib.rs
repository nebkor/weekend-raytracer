// use std::f64::MAX as FMAX;

use euclid::Vector3D;
pub use rand::prelude::*;

pub type Color = Vector3D<f32, ()>;
pub type Coloru8 = Vector3D<u8, ()>;
pub type Point3 = Vector3D<f64, ()>;
pub type Vec3 = Vector3D<f64, ()>;

mod ray;
pub use ray::*;

mod illumable;
pub use illumable::*;

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

pub fn color(r: &Ray) -> Color {
    let t = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, &r);
    if t > 0.0 {
        let n = (r.at(t) - Vec3::new(0.0, 0.0, -1.0)).normalize();
        // the reason to add one to each component and then divide the vector by two is to make sure all values
        // are positive when it gets cast to a coloru8.
        Color::new(n.x as f32 + 1.0, n.y as f32 + 1.0, n.z as f32 + 1.0) / 2.0
    } else {
        let unit = r.direction().normalize();
        let t = 0.5 * (unit.y + 1.) as f32;
        // interpolate between blue at the top and white at the bottom
        (Color::new(1., 1., 1.) * (1.0 - t)) + (Color::new(0.5, 0.7, 1.0) * t)
    }
}
