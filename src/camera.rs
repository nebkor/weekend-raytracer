use crate::ray::Ray;
use crate::Point;

pub struct Camera {
    lower_left_: Point,
    horizontal_: Point,
    vertical_: Point,
    origin_: Point,
}

impl Camera {
    pub fn default() -> Self {
        Camera {
            lower_left_: Point::new(-2.0, -1.0, -1.0),
            horizontal_: Point::new(4.0, 0.0, 0.0),
            vertical_: Point::new(0.0, 2.0, 0.0),
            origin_: Point::new(0.0, 0.0, 0.0),
        }
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        let uv = self.lower_left_ + (self.horizontal_ * u) + (self.vertical_ * v) - self.origin_;
        Ray::new(self.origin_, uv)
    }

    pub fn origin(&self) -> Point {
        self.origin_
    }

    pub fn horizontal(&self) -> Point {
        self.horizontal_
    }

    pub fn vertical(&self) -> Point {
        self.vertical_
    }

    pub fn lower_left(&self) -> Point {
        self.lower_left_
    }
}
