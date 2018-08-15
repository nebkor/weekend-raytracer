extern crate raytracer;
use raytracer::*;

extern crate png;
use png::HasParameters;

extern crate clap;
use clap::{App, Arg};

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

const NX: u32 = 800;
const NY: u32 = 400;
const NS: u32 = 100;
const SF: f32 = 255.99; // scaling factor for converting colorf32 to u8

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn write_png(out: &str, framebuffer: &[u8]) {
    let pngfile = format!("{}.png", out);
    let path = Path::new(&pngfile);

    let file = match File::create(path.clone()) {
        Ok(f) => f,
        Err(e) => panic!(format!("got {:?} we r ded", e)),
    };

    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, NX, NY); // Width is nx pixels and height is ny
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(framebuffer).unwrap();

    println!("Wrote to {:?}.", path);
}

fn main() {
    let args = App::new("Weekend Raytracer")
        .version(VERSION)
        .arg(
            Arg::with_name("OUTPUT")
                .help("Sets the basename of the PNG output file.")
                .required(true)
                .index(1),
        ).get_matches();

    let outfile = args.value_of("OUTPUT").unwrap();

    let cam = Camera::default();

    let world = vec![
        Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0),
    ];

    let mut data: Vec<u8> = Vec::with_capacity(NX as usize * NY as usize * 4);

    let mut rng = SmallRng::from_entropy();

    // Now the real rendering work:
    for j in (0..NY).rev() {
        for i in 0..NX {
            let mut col = Color::new(0.0, 0.0, 0.0);
            for _s in 0..NS {
                let u = (i as f64 + rng.gen::<f64>()) / NX as f64;
                let v = (j as f64 + rng.gen::<f64>()) / NY as f64;
                let r = cam.ray(u, v);
                // let p = r.pt_at_param(2.0);
                col = col + color(r, &world, &mut rng);
            }

            let c = (col / NS as f32).gamma_correct(2.0) * SF;
            let v: Coloru8 = c.cast();
            data.extend_from_slice(v.to_array().as_ref());
        }
    }

    write_png(outfile, data.as_ref());
}
