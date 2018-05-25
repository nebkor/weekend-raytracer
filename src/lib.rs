use std::fmt;

use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Point {
    elements: [f64; 4],
}

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Color {
    elements: [f64; 4],
}

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Ray {
    a: Point,
    b: Point,
}

impl Point {
    pub fn new() -> Self {
        Point {
            elements: [0., 0., 0., 1.],
        }
    }

    pub fn init(x: f64, y: f64, z: f64, w: f64) -> Self {
        Point {
            elements: [x, y, z, w],
        }
    }

    pub fn x(&self) -> f64 {
        self.elements[0]
    }
    pub fn y(&self) -> f64 {
        self.elements[1]
    }
    pub fn z(&self) -> f64 {
        self.elements[2]
    }
    pub fn w(&self) -> f64 {
        self.elements[3]
    }

    pub fn len(&self) -> f64 {
        (self.x().powi(2) + self.y().powi(2) + self.z().powi(2)).sqrt()
    }
}

impl Ray {
    pub fn new() -> Self {
        Ray {
            a: Default::default(),
            b: Default::default(),
        }
    }

    pub fn init(a: Point, b: Point) -> Self {
        Ray { a, b }
    }

    pub fn direction(&self) -> Point {
        self.b
    }

    pub fn origin(&self) -> Point {
        self.a
    }

    pub fn pt_at_param(&self, t: f64) -> Point {
        self.a + (self.b * t)
    }
}

impl Color {
    pub fn new() -> Self {
        Color { elements: [1.0; 4] }
    }

    pub fn init(r: f64, g: f64, b: f64, a: f64) -> Self {
        Color {
            elements: [r, g, b, a],
        }
    }

    pub fn r(&self) -> f64 {
        self.elements[0]
    }
    pub fn g(&self) -> f64 {
        self.elements[1]
    }
    pub fn b(&self) -> f64 {
        self.elements[2]
    }
    pub fn a(&self) -> f64 {
        self.elements[3]
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.r() as i64,
            self.g() as i64,
            self.b() as i64
        )
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Point::init(
            self.x() + rhs.x(),
            self.y() + rhs.y(),
            self.z() + rhs.z(),
            1.0,
        )
    }
}

impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, scale: f64) -> Self {
        Point::init(
            self.x() * scale,
            self.y() * scale,
            self.z() * scale,
            self.w(),
        )
    }
}
