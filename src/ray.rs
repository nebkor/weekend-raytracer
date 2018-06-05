use point::Point;
use std;

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

impl HitRecord {
    pub fn new(t: f64, p: Point, n: Point) -> Self {
        HitRecord { t, p, n }
    }
}

pub trait Glimmer {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct World<T>
where
    T: Glimmer,
{
    w: Vec<T>,
}

impl<T: Glimmer> World<T> {
    pub fn new(w: Vec<T>) -> Self {
        World { w: w }
    }
}

impl<T: Glimmer> Glimmer for World<T> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut record: Option<HitRecord> = None;
        let mut current_closest = t_max;
        for thing in self.w.iter() {
            if let Some(hr) = thing.hit(r, t_min, current_closest) {
                current_closest = hr.t;
                std::mem::swap(&mut Some(hr), &mut record);
            }
        }
        record
    }
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
