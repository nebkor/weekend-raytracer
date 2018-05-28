use point::Point;

#[derive(Clone, Copy, PartialEq)]
pub struct Ray {
    a: Point,
    b: Point,
}

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub t: f64,
    pub p: Point,
    pub n: Point,
}

pub trait Glimmer<T> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool;
}

impl Ray {
    pub fn new(a: Point, b: Point) -> Self {
        Ray { a, b }
    }

    pub fn direction(&self) -> Point {
        self.b - self.a
    }

    pub fn origin(&self) -> Point {
        self.a
    }

    pub fn pt_at_param(&self, t: f64) -> Point {
        self.a + (self.b * t)
    }
}
