use crate::material::*;
use crate::Point;

#[derive(Clone, Copy, PartialEq)]
pub struct Ray {
    a: Point,
    b: Point,
}

#[derive(Clone)]
pub struct Bounce<'m> {
    pub t: f32,
    pub p: Point,
    pub n: Point,
    pub mat: &'m BoxMat,
}

impl<'m> Bounce<'m> {
    pub fn new(t: f32, p: Point, n: Point, mat: &'m BoxMat) -> Self {
        Bounce { t, p, n, mat }
    }
}

pub trait Visible {
    fn bounce(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Bounce>;
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

    pub fn pt_at_param(&self, t: f32) -> Point {
        self.a + (self.b * t)
    }
}
