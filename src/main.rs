extern crate raytracer;

use raytracer::{make_ppm_header, write_image, Color, File, Write};

fn main() {
    let nx = 800;
    let ny = 400;
    let maxval = 255;
    let sf = 255.99; // scaling factor for RGB vals in PPM

    let testppm = |mut f: File| {
        let header = make_ppm_header(nx, ny, maxval);
        let _ = f.write_all(header.as_bytes());
        let mut count: usize = 3;
        for j in (0..ny).rev() {
            for i in 0..nx {
                let r = i as f64 / nx as f64;
                let g = j as f64 / ny as f64;
                let b = 0.5 * sf;

                match f.write_all(format!("{}\n", Color::init(r * sf, g * sf, b, 1.)).as_bytes()) {
                    Err(_) => panic!(),
                    Ok(_) => count += 1,
                };
            }
        }
        count
    };

    match write_image(testppm, "test.ppm") {
        Ok(n) => println!("Wrote {} lines", n),
        Err(e) => println!("Oh noes, something went wrong. {:?}", e),
    }
}
