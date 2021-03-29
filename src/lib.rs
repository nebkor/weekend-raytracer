pub use std::f64::consts::PI;
pub use std::f64::MAX as FMAX;

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use euclid::{Point3D, UnknownUnit, Vector3D};
pub use rand::prelude::*;

pub type Color64 = Vector3D<f64, UnknownUnit>;
pub type Color8 = Vector3D<u8, UnknownUnit>;
pub type Point3 = Point3D<f64, UnknownUnit>;
pub type Vec3 = Vector3D<f64, UnknownUnit>;

mod ray;
pub use ray::*;
mod sphere;
pub use sphere::*;
mod material;
pub use material::*;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const MAX_BOUNCES: i8 = 50;
pub const CLAMP_MAX: f64 = 1.0 - std::f64::EPSILON;

pub fn d2r(d: f64) -> f64 {
    (d * PI) / 180.0
}

pub fn random_unit_point(r: &mut impl Rng) -> Point3 {
    let mut p: Vec3;
    let one = Vec3::one();
    loop {
        p = (Vec3::new(r.gen(), r.gen(), r.gen()) * 2.0) - one;
        if p.square_length() < 1.0 {
            break;
        }
    }
    p.to_point()
}

pub fn color(r: &Ray, world: &[Sphere], rng: &mut SmallRng, depth: i8) -> Color64 {
    if depth < 1 {
        return Color64::zero();
    }

    if let Some(glint) = world.shine(r, 0.01..FMAX) {
        if let Some(scatter) = glint.material.scatter(r, &glint, rng) {
            scatter
                .attenuation
                .component_mul(color(&scatter.ray, world, rng, depth - 1))
        } else {
            Color64::zero()
        }
    } else {
        let unit = r.direction().normalize();
        let t = 0.5 * (unit.y + 1.);
        // interpolate between blue at the top and white at the bottom
        (Color64::new(1., 1., 1.) * (1.0 - t)) + (Color64::new(0.5, 0.7, 1.0) * t)
    }
}

pub fn write_png(out: &str, framebuffer: &[u8], width: u32, height: u32) {
    let pngfile = format!("{}.png", out);
    let path = Path::new(&pngfile);

    let file = match File::create(path) {
        Ok(f) => f,
        Err(e) => panic!("got {:?} we r ded", e),
    };

    let w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(framebuffer).unwrap();

    println!("Wrote to {:?}.", path);
}

use clap::{App, Arg, ArgMatches};
pub fn get_args(default_file: &str) -> ArgMatches<'_> {
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
    args
}
