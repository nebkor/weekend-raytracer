use crate::{Glint, Illumable, Point3, Ray, Vec3};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Illumable for Sphere {
    fn shine(&self, r: &Ray, t_range: std::ops::Range<f64>) -> Option<Glint> {
        let oc = *r.origin() - self.center;
        let a = r.direction().square_length();
        let half_b = oc.dot(*r.direction());
        let c = oc.square_length() - self.radius.powi(2);

        let discrim = half_b.powi(2) - a * c;
        if discrim < 0.0 {
            return None;
        }
        // discrim is always positive here, we can take the square root
        let sqrtd = discrim.sqrt();
        let nroot = (-half_b - sqrtd) / a;
        let proot = (-half_b + sqrtd) / a;

        let root = if t_range.contains(&nroot) {
            nroot
        } else if t_range.contains(&proot) {
            proot
        } else {
            return None;
        };

        let mut glint = Glint::default();

        let pnt = r.at(root);
        let normal = (pnt - self.center) / self.radius;
        glint.p = pnt;
        glint.t = root;
        glint.set_face_normal(&r, normal);

        Some(glint)
    }
}

impl Illumable for Vec<Sphere> {
    fn shine(&self, r: &Ray, t_range: std::ops::Range<f64>) -> Option<Glint> {
        let mut t_range = t_range;
        let mut glint: Option<Glint> = None;
        for sphere in self.iter() {
            if let Some(glnt) = sphere.shine(r, t_range.clone()) {
                t_range.end = glnt.t;
                glint = Some(glnt)
            }
        }
        glint
    }
}
