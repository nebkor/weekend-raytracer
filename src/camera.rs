use crate::{d2r, random_unit_disk, Point3, Ray, Vec3};

use rand::rngs::SmallRng;

pub struct Camera {
    origin: Point3,
    lower_left: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        origin: Point3,
        look_at: Point3,
        up: Vec3,
        vfov: f64, // vertical field of view in degrees
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = d2r(vfov); // vfov is vertical field of view in degrees
        let h = (theta / 2.0).tan(); // TODO: is this really "height"? (probably)
        let viewport_h = 2.0 * h;
        let viewport_w = aspect_ratio * viewport_h;

        // basis vector construction
        let w = (origin - look_at).normalize();
        let u = (up.cross(w)).normalize();
        let v = w.cross(u).normalize();

        let horizontal = u * viewport_w * focus_dist;
        let vertical = v * viewport_h * focus_dist;
        let lower_left = origin - (horizontal / 2.0) - (vertical / 2.0) - w * focus_dist;

        Camera {
            origin,
            lower_left,
            horizontal,
            vertical,
            u,
            v,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64, rng: &mut SmallRng) -> Ray {
        let rd = random_unit_disk(rng).to_vector() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left.to_vector() + self.horizontal * s + self.vertical * t
                - self.origin.to_vector()
                - offset,
        )
    }
}
