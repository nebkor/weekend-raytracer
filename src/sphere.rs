use crate::{Glint, Illumable, MatPtr, Point3, Ray};

use std::sync::Arc;

pub struct Sphere {
    pub center_start: Point3,
    pub center_end: Point3,
    pub radius: f64,
    pub material: MatPtr,
    pub t_start: f64,
    pub t_end: f64,
}

impl Sphere {
    pub fn center(&self, t: f64) -> Point3 {
        self.center_start
            + ((self.center_end - self.center_start) * (t - self.t_start)
                / (self.t_end - self.t_start))
    }
}

impl Illumable for Sphere {
    fn shine(&self, r: &Ray, t_range: std::ops::Range<f64>) -> Option<Glint> {
        let r_time = r.time();
        let oc = *r.origin() - self.center(r_time);
        let a = r.direction().square_length();
        let half_b = oc.dot(*r.direction());
        let c = oc.square_length() - self.radius.powi(2);

        let discrim = half_b.powi(2) - a * c;
        if discrim < 0.0 {
            return None;
        }
        // discrim is always positive here, we can take the square root
        let sqrtd = discrim.sqrt();
        let nroot = (-half_b - sqrtd) / a;
        let proot = (-half_b + sqrtd) / a;

        let root = if t_range.contains(&nroot) {
            nroot
        } else if t_range.contains(&proot) {
            proot
        } else {
            return None;
        };

        let pnt = r.at(root);
        let normal = (pnt - self.center(r_time)) / self.radius;
        let mut glint = Glint::new(pnt, root, Arc::clone(&self.material));
        glint.set_face_normal(&r, normal);

        Some(glint)
    }
}

impl Illumable for &[Sphere] {
    fn shine(&self, r: &Ray, t_range: std::ops::Range<f64>) -> Option<Glint> {
        let mut t_range = t_range;
        let mut glint: Option<Glint> = None;
        for sphere in self.iter() {
            if let Some(glnt) = sphere.shine(r, t_range.clone()) {
                t_range.end = glnt.t;
                glint = Some(glnt);
            }
        }
        glint
    }
}
