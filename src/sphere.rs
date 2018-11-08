use crate::ray::*;
use crate::Point;

pub struct Sphere {
    center_: Point,
    radius_: f64,
}

impl Sphere {
    pub fn new(c: Point, r: f64) -> Self {
        Sphere {
            center_: c,
            radius_: r,
        }
    }

    pub fn center(&self) -> &Point {
        &self.center_
    }

    pub fn radius(&self) -> f64 {
        self.radius_
    }
}

impl Visible for Sphere {
    fn bounce(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Bounce> {
        let oc = r.origin() - *self.center();
        let rd = r.direction();
        // a, b, c correspond to quadratic equation terms
        let a = rd.square_length();
        let b = oc.dot(rd);
        let c = oc.square_length() - self.radius_.powi(2);
        let disc = b.powi(2) - (a * c); // b^2 - ac
        if disc > 0.0 {
            let temp = (-b - disc.sqrt()) / a;
            if temp > t_min && temp < t_max {
                let p = r.pt_at_param(temp);
                let n = (p - *self.center()) / self.radius_;
                Some(Bounce::new(temp, p, n))
            } else {
                None
            }
        } else {
            None
        }
    }
}
