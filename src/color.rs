use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Color {
    elements: [f64; 4],
}

/// naked impls
impl Color {
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Color {
            elements: [r, g, b, a],
        }
    }

    pub fn c3(r: f64, g: f64, b: f64) -> Self {
        Color {
            elements: [r, g, b, 1.0],
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

/// impl traits
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

/// Trait impls here.
impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Color::new(
            self.r() + rhs.r(),
            self.g() + rhs.g(),
            self.b() + rhs.b(),
            1.0,
        )
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Color::new(
            self.r() - rhs.r(),
            self.g() - rhs.g(),
            self.b() - rhs.b(),
            1.0,
        )
    }
}

/// Color * f64
impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, scale: f64) -> Self {
        Color::new(
            self.r() * scale,
            self.g() * scale,
            self.b() * scale,
            self.a(),
        )
    }
}

/// f64 * Color
impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, color: Color) -> Color {
        color * self
    }
}

/// Color * Color
impl Mul<Color> for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Color::new(
            self.r() * rhs.r(),
            self.g() * rhs.g(),
            self.b() * rhs.b(),
            self.a(),
        )
    }
}

impl Div<f64> for Color {
    type Output = Self;

    fn div(self, scale: f64) -> Self {
        Color::new(
            self.r() / scale,
            self.g() / scale,
            self.b() / scale,
            self.a(),
        )
    }
}

impl Div<Color> for Color {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Color::new(
            self.r() / rhs.r(),
            self.g() / rhs.g(),
            self.b() / rhs.b(),
            self.a(),
        )
    }
}
