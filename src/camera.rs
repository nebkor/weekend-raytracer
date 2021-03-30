use crate::{d2r, Point3, Ray, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(
        origin: Point3,
        look_at: Point3,
        up: Vec3,
        vfov: f64, // vertical field of view in degrees
        aspect_ratio: f64,
    ) -> Self {
        let theta = d2r(vfov); // vfov is vertical field of view in degrees
        let h = (theta / 2.0).tan(); // TODO: is this really "height"? (probably)
        let viewport_h = 2.0 * h;
        let viewport_w = aspect_ratio * viewport_h;

        // basis vector construction
        let w = (origin - look_at).normalize();
        let u = (up.cross(w)).normalize();
        let v = w.cross(u);

        let horizontal = u * viewport_w;
        let vertical = v * viewport_h;
        let lower_left = origin - (horizontal / 2.0) - (vertical / 2.0) - w;

        Camera {
            origin,
            lower_left,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left.to_vector() + self.horizontal * u + self.vertical * v
                - self.origin.to_vector(),
        )
    }
}
