use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

/// Our point is, at heart, an array of four 64-bit floats.
#[derive(Clone, Copy, Debug, PartialEq)]
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

    pub fn p3(x: f64, y: f64, z: f64) -> Self {
        Point {
            elements: [x, y, z, 0.0],
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

    pub fn dot(&self, rhs: &Point) -> f64 {
        (self.x() * rhs.x()) + (self.y() * rhs.y()) + (self.z() * rhs.z())
    }

    pub fn self_dot(&self) -> f64 {
        self.x().powi(2) + self.y().powi(2) + self.z().powi(2)
    }

    pub fn len_sq(&self) -> f64 {
        self.self_dot()
    }

    pub fn cross(&self, rhs: &Point) -> Point {
        // for 3-vectors A and B, A cross B =
        // Vec3(AyBz - AzBy, AzBx - AxBz, AxBy - AyBx)
        Point::new(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
            self.w(),
        )
    }
}

/// Trait impls here.
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}

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

/// Point * f64
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

/// f64 * Point
impl Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, point: Point) -> Point {
        point * self
    }
}

/// Point * Point
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
