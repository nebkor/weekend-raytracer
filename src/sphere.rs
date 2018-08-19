use crate::ray::*;
use crate::{Color, Material, Point, Rng};

pub struct Sphere<M: Material> {
    pub center: Point,
    pub radius: f64,
    pub material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Point, radius: f64, material: M) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }

    pub fn center(&self) -> &Point {
        &self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn material(&self) -> &M {
        &self.material
    }
}

impl<M: Material> Material for Sphere<M> {
    fn scatter(&mut self, record: &HitRecord) -> Option<(Color, Ray)> {
        self.material.scatter(record)
    }
}

impl<M: Material> Glimmer for Sphere<M> {
    fn glimmer(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
                Some(HitRecord::new(temp, p, n))
            } else {
                None
            }
        } else {
            None
        }
    }
}
