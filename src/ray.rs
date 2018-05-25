use point::Point;

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Ray {
    a: Point,
    b: Point,
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
