use std::f32::MAX as FMAX;

pub use rand::prelude::*;

use clap::{App, Arg};

use euclid::*;
pub type Color = Vector3D<f32>;
pub type Coloru8 = Vector3D<u8>;
pub type Point = Vector3D<f32>;

const SEED: [u32; 4] = [0x193a_6754, 0xa8a7_d469, 0x9783_0e05, 0x113b_a7bb];
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn get_rng() -> SmallRng {
    let mut s: [u8; 16] = [0; 16];
    unsafe { s.copy_from_slice(SEED.align_to::<u8>().1) };
    SmallRng::from_seed(s)
}

pub trait MulAss {
    fn mul_ass(&mut self, rhs: Self) -> Self;
}

impl MulAss for Color {
    fn mul_ass(&mut self, rhs: Self) -> Self {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
        *self
    }
}

//--------------------------------------------------------------------
pub fn get_outfile() -> String {
    let args = App::new("Weekend Raytracer")
        .version(VERSION)
        .arg(
            Arg::with_name("OUTPUT")
                .help("Sets the basename of the PNG output file.")
                .required(true)
                .index(1),
        )
        .get_matches();

    args.value_of("OUTPUT").unwrap().into()
}

mod sphere;
pub use crate::sphere::Sphere;

mod material;
pub use crate::material::*;

mod camera;
pub use crate::camera::Camera;

mod ray;
pub use crate::ray::*;

pub type World<'w> = &'w [&'w dyn Visible];
pub type ImageBuf = Vec<u8>;

pub trait Gamma {
    fn gamma_correct(&self, factor: f32) -> Self;
}

impl Gamma for Color {
    fn gamma_correct(&self, factor: f32) -> Self {
        let pow = 1.0 / factor;
        Color::new(self.x.powf(pow), self.y.powf(pow), self.z.powf(pow))
    }
}

impl Visible for World<'_> {
    fn bounce(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Bounce> {
        let mut ret: Option<Bounce> = None;
        for thing in self.iter() {
            if let Some(bounce) = thing.bounce(r, t_min, t_max) {
                match ret {
                    None => ret = Some(bounce),
                    Some(ref prev) => {
                        if bounce.t < prev.t {
                            ret = Some(bounce)
                        }
                    }
                }
            }
        }
        ret
    }
}

pub fn random_unit_point<R: Rng>(r: &mut R) -> Point {
    let mut p: Point;
    let one = Point::new(1.0, 1.0, 1.0);
    loop {
        p = (Point::new(r.gen(), r.gen(), r.gen()) * 2.0) - one;
        if p.square_length() < 1.0 {
            break;
        }
    }
    p
}

pub fn color(r: &Ray, world: &World<'_>, depth: usize) -> Color {
    match world.bounce(&r, 0.001, FMAX) {
        Some(bounce) => {
            if depth > 0 {
                if let Some(scatrec) = (bounce.mat).scatter(r, &bounce) {
                    return color(&(scatrec.scattered), world, depth - 1)
                        .mul_ass(scatrec.attenuation);
                } else {
                    return Color::new(0.0, 0.0, 0.0);
                }
            } else {
                Color::new(0.0, 0.0, 0.0)
            }
        }
        None => {
            let unit = r.direction().normalize();
            let t = 0.5 * (unit.y + 1.) as f32;
            // interpolate between blue at the top and white at the bottom
            (Color::new(1., 1., 1.) * (1.0 - t) + Color::new(0.5, 0.7, 1.0)) * t
        }
    }
}
