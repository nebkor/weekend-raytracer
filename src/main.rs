extern crate rand;
use rand::prelude::*;

use std::fs::File;
use std::io::Write;

mod point;
use point::Point;

mod color;
use color::Color;

mod ray;
use ray::{Glimmer, Ray, World};

#[allow(dead_code)]
mod sphere;
use sphere::Sphere;

#[allow(dead_code)]
mod camera;
use camera::Camera;

fn make_ppm_header(w: usize, h: usize, max: usize) -> String {
    format!("P3\n{} {}\n{}\n", w, h, max)
}

fn write_image<F>(f: F, name: &str) -> std::io::Result<(usize, usize)>
where
    F: Fn(File) -> (usize, usize),
{
    let file = File::create(name)?;

    Ok(f(file))
}

fn color<T: Glimmer>(r: &Ray, world: &World<T>) -> Color {
    if let Some(rec) = world.hit(r, 0.0, std::f64::MAX) {
        0.5 * Color::c3(rec.n.x() + 1., rec.n.y() + 1., rec.n.z() + 1.)
    } else {
        let unit = r.direction().unit();
        let t = 0.5 * (unit.y() + 1.);
        // interpolate between blue at the top and white at the bottom
        (1. - t) * Color::c3(1., 1., 1.) + t * Color::c3(0.5, 0.7, 1.0)
    }
}

fn main() {
    let nx = 800;
    let ny = 400;
    let ns = 100;
    let maxval = 255;
    let sf = 255.99; // scaling factor for RGB vals in PPM

    let cam = Camera::default();

    let world = World::new(vec![
        Sphere::new(Point::p3(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Point::p3(0.0, -100.5, -1.0), 100.0),
    ]);

    let bluesky = |mut f: File| {
        // this is bogus; the panic in the else branch is masking its later use
        #[allow(unused_assignments)]
        let mut count: usize = 0;
        let mut err: usize = 0;

        let mut rng = thread_rng();

        // handle header logic, bail if it fails
        let header = make_ppm_header(nx, ny, maxval);
        match f.write_all(header.as_bytes()) {
            Ok(_) => {
                count = 3;
            }
            Err(_) => {
                panic!("no hed, we r ded");
            }
        }
        // Now the real rendering work:
        for j in (0..ny).rev() {
            for i in 0..nx {
                let mut col = Color::c3(0.0, 0.0, 0.0);
                for _s in 0..ns {
                    let u = (i as f64 + rng.gen::<f64>()) / nx as f64;
                    let v = (j as f64 + rng.gen::<f64>()) / ny as f64;
                    let r = cam.ray(u, v);
                    // let p = r.pt_at_param(2.0);
                    col = col + color(&r, &world);
                }

                let c = (col / ns as f64) * sf;
                match f.write_all(format!("{}\n", c).as_bytes()) {
                    Err(_) => err += 1,
                    Ok(_) => count += 1,
                };
            }
        }
        (count, err)
    }; // end closure def

    match write_image(bluesky, "bluesky.ppm") {
        Ok(n) => println!("Wrote {} lines, and didn't write {} lines.", n.0, n.1),
        Err(e) => println!("Oh noes, something went wrong. {:?}", e),
    }
}
