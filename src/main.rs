extern crate raytracer;

use raytracer::{make_ppm_header, write_image, Color, File, Point, Ray, Write};

fn color(r: Ray) -> Color {
    let unit = r.direction().unit();
    let t = 0.5 * (unit.y() + 1.);
    // interpolate between blue at the top and white at the bottom
    (1. - t) * Color::new(1., 1., 1., 1.) + t * Color::new(0.5, 0.7, 1.0, 1.0)
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

    let testppm = |mut f: File| {
        // this is bogus; the panic in the else branch is masking its later use
        #[allow(unused_assignments)]
        let mut count: usize = 0;
        let mut err: usize = 0;

        let header = make_ppm_header(nx, ny, maxval);

        match f.write_all(header.as_bytes()) {
            Ok(_) => {
                count = 3;
            }
            Err(_) => {
                panic!("no hed, we r ded");
            }
        }

        for j in (0..ny).rev() {
            for i in 0..nx {
                let r = i as f64 / nx as f64;
                let g = j as f64 / ny as f64;
                let b = 0.5 * sf;

                match f.write_all(format!("{}\n", Color::new(r * sf, g * sf, b, 1.)).as_bytes()) {
                    Err(_) => err += 1,
                    Ok(_) => count += 1,
                };
            }
        }
        (count, err)
    };

    match write_image(testppm, "test.ppm") {
        Ok(n) => println!("Wrote {} lines, and didn't write {} lines.", n.0, n.1),
        Err(e) => println!("Oh noes, something went wrong. {:?}", e),
    }
}
