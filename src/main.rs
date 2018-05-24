use std::fmt;

struct Point {
    elements: [f64; 4],
}

struct Color {
    elements: [f64; 4],
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
        write!(f, "{} {} {}", self.r(), self.g(), self.b())
    }
}

fn main() {
    let nx = 800;
    let ny = 400;

    println!("P3\n{} {}\n255", nx, ny);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let r = i as f64 / nx as f64;
            let g = j as f64 / ny as f64;
            let b = 0.5;

            let ir = (255.99 * r) as i64;
            let ig = (255.99 * g) as i64;
            let ib = (255.99 * b) as i64;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
