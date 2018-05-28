use point::Point;
use ray::{Glimmer, HitRecord, Ray};

pub struct Sphere {
    center_: Point,
    radius_: Point,
}

impl Sphere {
    pub fn new(c: Point, r: f64) -> Self {
        Sphere { c, r }
    }

    pub fn center(&self) -> &Point {
        self.center_
    }

    pub fn radius(&self) -> f64 {
        self.radius_
    }
}

impl Glimmer<T> for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center();
        let rd = r.direction();
        // a, b, c correspond to quadratic equation terms
        let a = rd.self_dot();
        let b = oc.dot(&rd);
        let c = oc.self_dot() - radius.powi(2);
        let disc = b.powi(2) - (a * c); // b^2 - ac
        if disc > 0.0 {
            let temp = -b - disc.sqrt() / a;
            if temp < t_max && temp >= t_min {
                record.t = temp;
                record.p = r.pt_at_param(temp);
                record.n = (record.p - self.center());
                true
            } else {
                let temp = -b + disc.sqrt() / a;
                if temp < t_max && temp >= t_min {
                    record.t = temp;
                    record.p = r.pt_at_param(temp);
                    record.n = (record.p - self.center());
                    true
                }
            }
        } else {
            false
        }
    }
}
