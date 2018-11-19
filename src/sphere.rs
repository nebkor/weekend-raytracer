use crate::material::*;
use crate::ray::*;
use crate::Point;

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub mat: BoxMat,
}

impl Sphere {
    pub fn new(center: Point, radius: f32, mat: MatSpec) -> Self {
        Sphere {
            center,
            radius,
            mat: mat.box_mat(),
        }
    }

    pub fn center(&self) -> &Point {
        &self.center
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}

impl Visible for Sphere {
    fn bounce(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Bounce> {
        let oc = r.origin() - *self.center();
        let rd = r.direction();
        // a, b, c correspond to quadratic equation terms
        let a = rd.square_length();
        let b = oc.dot(rd);
        let c = oc.square_length() - self.radius.powi(2);
        let disc = b.powi(2) - (a * c); // b^2 - ac
        if disc > 0.0 {
            let temp = (-b - disc.sqrt()) / a;
            if temp > t_min && temp < t_max {
                let p = r.pt_at_param(temp);
                let n = (p - *self.center()) / self.radius;
                Some(Bounce::new(temp, p, n, &self.mat))
            } else {
                None
            }
        } else {
            None
        }
    }
}
