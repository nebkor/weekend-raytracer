use crate::{HitRecord, Point, Ray, Vec3D};

pub trait Material {
    fn scatter(
        &self,
        record: &HitRecord,
        attenuation: &Vec3D,
        scattered: &Ray,
    ) -> Option<(Point, Ray)>;
}
