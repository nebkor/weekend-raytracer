use std::fmt;

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
