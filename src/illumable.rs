use crate::{Point3, Ray, Vec3};

// "hit record", though that's a good pun-y name too
pub struct Glint {
    pub p: Point3,
    pub n: Vec3,
    pub t: f64,
}

// "hittable"
pub trait Illumable {
    fn shine(r: &Ray, t_range: std::ops::Range<f64>) -> Option<Glint>;
}
