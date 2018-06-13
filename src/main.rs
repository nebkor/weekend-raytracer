extern crate raytracer;
use raytracer::*;

use std::fs::File;
use std::io::Write;

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
