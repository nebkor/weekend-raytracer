pub use std::fs::File;
pub use std::io::prelude::*;

mod point;
pub use point::Point;

mod color;
pub use color::Color;

mod ray;
pub use ray::Ray;

pub fn make_ppm_header(w: usize, h: usize, max: usize) -> String {
    format!("P3\n{} {}\n{}\n", w, h, max)
}

pub fn write_image<F>(f: F, name: &str) -> std::io::Result<(usize, usize)>
where
    F: Fn(File) -> (usize, usize),
{
    let file = File::create(name)?;

    Ok(f(file))
}
