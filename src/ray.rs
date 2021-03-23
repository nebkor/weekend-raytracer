use crate::{Point3, Vec3};

#[derive(Clone, Copy, PartialEq)]
pub struct Ray {
    origin: Point3,
    dir: Point3,
}

impl Ray {
    pub fn new(origin: Point3, dir: Point3) -> Self {
        Ray { origin, dir }
    }

    pub fn direction(&self) -> &Point3 {
        &self.dir
    }

    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + (self.dir * t)
    }
}

// "hit record", though that's a good pun-y name too
pub struct Glint {
    pub p: Point3,
    pub n: Vec3,
    pub t: f64,
}

// "hittable"
pub trait Illumable {
    fn shine(&self, r: &Ray, t_range: std::ops::Range<f64>) -> Option<Glint>;
}
