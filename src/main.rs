extern crate raytracer;

use raytracer::{Color, Point, Ray};

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
