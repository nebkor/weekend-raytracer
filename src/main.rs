use raytracer::*;

use chrono::Local;
use png;

extern crate clap;
use clap::{App, Arg};

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

const NX: u32 = 800;
const NY: u32 = 400;
const NS: u32 = 100;
const SF: f32 = 255.99; // scaling factor for converting colorf32 to u8

const VERSION: &str = env!("CARGO_PKG_VERSION");

const CHAPTER: &str = "chapter4";

fn write_png(out: &str, framebuffer: &[u8]) {
    let pngfile = format!("{}.png", out);
    let path = Path::new(&pngfile);

    let file = match File::create(path) {
        Ok(f) => f,
        Err(e) => panic!("got {:?} we r ded", e),
    };

    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, NX, NY); // Width is nx pixels and height is ny
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(framebuffer).unwrap();

    println!("Wrote to {:?}.", path);
}

fn main() {
    let now = format!("{}", Local::now().format("%Y%m%d_%H:%M:%S"));
    let default_file = format!("{}/{}", CHAPTER, now);
    let args = App::new("Weekend Raytracer")
        .version(VERSION)
        .arg(
            Arg::with_name("OUTPUT")
                .help("Sets the basename of the PNG output file.")
                .required(false)
                .default_value(&default_file)
                .index(1),
        )
        .get_matches();

    let outfile = args.value_of("OUTPUT").unwrap();

    dbg!(outfile);

    let mut data: Vec<u8> = Vec::with_capacity(NX as usize * NY as usize * 4);

    let WIDTH = NX as f64;
    let HEIGHT = NY as f64;
    let ratio = WIDTH / HEIGHT;

    // fake out a camera
    let viewport_h = 2.0;
    let viewport_w = ratio * viewport_h;
    let focal_len = 1.0;

    let origin = Point3::default();
    let horizontal = Vec3::new(viewport_w, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_h, 0.0);
    let lower_left_corner =
        origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_len);

    // Now the real rendering work:
    for j in (0..NY).rev() {
        for i in 0..NX {
            let u = i as f64 / WIDTH;
            let v = j as f64 / HEIGHT;
            let r = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            let col = color(&r);

            let c = col * SF;
            let v: Coloru8 = c.cast();
            data.extend_from_slice(v.to_array().as_ref());
        }
    }

    write_png(outfile, data.as_ref());
}
