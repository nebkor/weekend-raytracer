use crate::{Point3, Vec3};

#[derive(Clone, Copy, PartialEq)]
pub struct Ray {
    origin: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, dir: Vec3) -> Self {
        Ray { origin, dir }
    }

    pub fn direction(&self) -> &Vec3 {
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
#[derive(Default, Debug)]
pub struct Glint {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_facing: bool,
}

impl Glint {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        if ray.direction().dot(outward_normal) < 0.0 {
            // we're front-facing
            self.normal = outward_normal;
            self.front_facing = true;
        } else {
            // we were inside the surface
            self.normal = outward_normal * -1.0;
            self.front_facing = false;
        }
    }
}

// "hittable"
pub trait Illumable {
    fn shine(&self, r: &Ray, t_range: std::ops::Range<f64>) -> Option<Glint>;
}
