extern crate raytracer;
use raytracer::*;

extern crate png;
use png::HasParameters;

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

fn main() {
    let nx = 800;
    let ny = 400;
    let ns = 100;
    let sf = 255.99; // scaling factor for RGB vals in PPM

    let cam = Camera::default();

    let world = vec![
        Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0),
    ];

    let path = Path::new(r"chapter7.png");

    let file = match File::create(path.clone()) {
        Ok(f) => f,
        Err(e) => panic!(format!("got {:?} we r ded", e)),
    };

    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, nx, ny); // Width is nx pixels and height is ny
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    let mut data: Vec<u8> = Vec::with_capacity(nx as usize * ny as usize * 4);

    let mut rng = SmallRng::from_entropy();

    // Now the real rendering work:
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Color::new(0.0, 0.0, 0.0);
            for _s in 0..ns {
                let u = (i as f64 + rng.gen::<f64>()) / nx as f64;
                let v = (j as f64 + rng.gen::<f64>()) / ny as f64;
                let r = cam.ray(u, v);
                // let p = r.pt_at_param(2.0);
                col = col + color(r, &world, &mut rng);
            }

            let c = (col / ns as f32).gamma_correct(2.0) * sf;
            let v: Coloru8 = c.cast();
            data.extend_from_slice(v.to_array().as_ref());
        }
    }

    writer.write_image_data(&data).unwrap();

    println!("Wrote to {:?}.", path);
}
