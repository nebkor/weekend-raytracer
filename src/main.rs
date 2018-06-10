extern crate rand;
use rand::prelude::*;
use rand::FromEntropy;

use std::fs::File;
use std::io::Write;

mod point;
use point::Point;

mod color;
use color::Color;

mod ray;
use ray::{Glimmer, HitRecord, Ray};

#[allow(dead_code)]
mod sphere;
use sphere::Sphere;

#[allow(dead_code)]
mod camera;
use camera::Camera;

impl<G: Glimmer> Glimmer for Vec<G> {
    fn glimmer(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut record: Option<HitRecord> = None;
        let mut current_closest = t_max;
        for thing in self.iter() {
            if let Some(hr) = thing.glimmer(r, t_min, current_closest) {
                current_closest = hr.t;
                std::mem::swap(&mut Some(hr), &mut record);
            }
        }
        record
    }
}

fn make_ppm_header(w: usize, h: usize, max: usize) -> String {
    format!("P3\n{} {}\n{}\n", w, h, max)
}

fn random_unit_point<R: Rng>(r: &mut R) -> Point {
    let mut p: Point;
    loop {
        p = 2.0 * Point::p3(r.gen(), r.gen(), r.gen()) - Point::p3(1.0, 1.0, 1.0);
        if p.len().powi(2) < 1.0 {
            break;
        }
    }
    p
}

fn color<G: Glimmer, R: Rng>(r: &Ray, world: &Vec<G>, rng: &mut R) -> Color {
    if let Some(rec) = world.glimmer(r, 0.001, std::f64::MAX) {
        let target = rec.p + rec.n + random_unit_point(rng);
        // twiddle the factor on the RHS of the less-than for more or less
        // recursion (higher factor is less recursion, more original color)
        if rng.gen::<f64>() < 0.3 {
            Color::c3(0.5, 0.5, 0.5)
        } else {
            0.5 * color(&Ray::new(rec.p, target - rec.p), world, rng)
        }
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

    let world = vec![
        Sphere::new(Point::p3(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Point::p3(0.0, -100.5, -1.0), 100.0),
    ];

    let mut file = match File::create("chapter7.ppm") {
        Ok(f) => f,
        Err(e) => panic!(format!("got {:?} we r ded", e)),
    };

    // this is bogus; the panic in the header writing is masking its later use
    #[allow(unused_assignments)]
    let mut count: usize = 0;

    let header = make_ppm_header(nx, ny, maxval);
    match file.write_all(header.as_bytes()) {
        Ok(_) => {
            count = 3;
        }
        Err(_) => {
            panic!("no hed, we r ded");
        }
    };

    let mut err: usize = 0;
    let mut rng = SmallRng::from_entropy();

    // Now the real rendering work:
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Color::c3(0.0, 0.0, 0.0);
            for _s in 0..ns {
                let u = (i as f64 + rng.gen::<f64>()) / nx as f64;
                let v = (j as f64 + rng.gen::<f64>()) / ny as f64;
                let r = cam.ray(u, v);
                // let p = r.pt_at_param(2.0);
                col = col + color(&r, &world, &mut rng);
            }

            let c = (col / ns as f64).gamma_correct(1.0) * sf;
            match file.write_all(format!("{}\n", c).as_bytes()) {
                Err(_) => err += 1,
                Ok(_) => count += 1,
            };
        }
    }

    println!(
        "Wrote {} lines, and didn't write {} lines, to {:?}.",
        count, err, file
    );
}
