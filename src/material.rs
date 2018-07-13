use point::Point;
use ray::{HitRecord, Ray};

pub trait Material {
    fn scatter(&self, record: &HitRecord) -> Option<(Point, Ray)>;
}
