extern crate raytracer;

use raytracer::{make_ppm_header, write_image, Color, Glimmer, Point, Ray, Sphere, World};
use std::fs::File;
use std::io::Write;

fn color<T>(r: &Ray, world: &World<T>) -> Color
where
    T: Glimmer,
{
    if let Some(rec) = world.hit(r, 0.0, std::f64::MAX) {
        0.5 * Color::new(rec.n.x() + 1., rec.n.y() + 1., rec.n.z() + 1., 1.0)
    } else {
        let unit = r.direction().unit();
        let t = 0.5 * (unit.y() + 1.);
        // interpolate between blue at the top and white at the bottom
        (1. - t) * Color::new(1., 1., 1., 1.) + t * Color::new(0.5, 0.7, 1.0, 1.0)
    }
}

fn main() {
    let nx = 800;
    let ny = 400;
    let maxval = 255;
    let sf = 255.99; // scaling factor for RGB vals in PPM

    let lower_left_corner = Point::new(-2., -1., -1., 0.);
    let horizontal = Point::new(4., 0., 0., 0.);
    let vertical = Point::new(0., 2., 0., 0.);
    let origin = Point::new(0., 0., 0., 0.);

    let world = World::new(vec![
        Sphere::new(Point::new(0.0, 0.0, -1.0, 0.0), 0.5),
        Sphere::new(Point::new(0.0, -100.5, -1.0, 0.0), 100.0),
    ]);

    let bluesky = |mut f: File| {
        // this is bogus; the panic in the else branch is masking its later use
        #[allow(unused_assignments)]
        let mut count: usize = 0;
        let mut err: usize = 0;

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
                let u = i as f64 / nx as f64;
                let v = j as f64 / ny as f64;
                let d = lower_left_corner + (u * horizontal) + (v * vertical);
                let r = Ray::new(origin, d);
                let c = color(&r, &world) * sf;

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
