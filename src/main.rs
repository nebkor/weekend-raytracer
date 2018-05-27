extern crate raytracer;

use raytracer::{make_ppm_header, write_image, Color, File, Point, Ray, Write};

fn hit_sphere(center: &Point, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin() - *center;

    let rd = ray.direction();
    // a, b, c correspond to quadratic equation terms
    let a = rd.self_dot();
    let b = 2.0 * oc.dot(&rd);
    let c = oc.self_dot() - radius.powi(2);
    let disc = b * b - 4.0 * a * c; // b^2 -4ac
    if disc < 0.0 {
        -1.0
    } else {
        (-b - disc.sqrt()) / (2.0 * a)
    }
}

fn color(r: &Ray) -> Color {
    let center = Point::new(0.0, 0.0, -1., 0.0);

    let t = hit_sphere(&center, 0.5, r);

    if t > 0.0 {
        let n: Point = (r.pt_at_param(t) - center).unit();
        0.5 * Color::new(n.x() + 1., n.y() + 1., n.z() + 1., 1.0)
    } else {
        let unit = r.direction().unit();
        let t = 0.5 * (unit.y() + 1.);
        // interpolate between blue at the top and white at the bottom
        (1. - t) * Color::new(1., 1., 1., 1.) + t * Color::new(0.5, 0.7, 1.0, 1.0)
        //Color::new(0.0, 0.0, 0.0, 1.0)
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
                let c = color(&r) * sf;

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
