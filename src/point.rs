use std::ops::{Add, Div, Mul, Sub};

/// Our point is, at heart, an array of four 64-bit floats.
#[derive(Clone, Copy, Default, PartialEq)]
pub struct Point {
    elements: [f64; 4],
}

/// No mutating; init a new Point if you need to change
/// values.
impl Point {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
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

    pub fn unit(&self) -> Self {
        self.clone() / self.len()
    }
}

/// Trait impls here.
impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Point::new(
            self.x() + rhs.x(),
            self.y() + rhs.y(),
            self.z() + rhs.z(),
            1.0,
        )
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Point::new(
            self.x() - rhs.x(),
            self.y() - rhs.y(),
            self.z() - rhs.z(),
            1.0,
        )
    }
}

impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, scale: f64) -> Self {
        Point::new(
            self.x() * scale,
            self.y() * scale,
            self.z() * scale,
            self.w(),
        )
    }
}

impl Mul<Point> for Point {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Point::new(
            self.x() * rhs.x(),
            self.y() * rhs.y(),
            self.z() * rhs.z(),
            self.w(),
        )
    }
}

impl Div<f64> for Point {
    type Output = Self;

    fn div(self, scale: f64) -> Self {
        Point::new(
            self.x() / scale,
            self.y() / scale,
            self.z() / scale,
            self.w(),
        )
    }
}

impl Div<Point> for Point {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Point::new(
            self.x() / rhs.x(),
            self.y() / rhs.y(),
            self.z() / rhs.z(),
            self.w(),
        )
    }
}
