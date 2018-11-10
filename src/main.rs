use raytracer::*;

use png::HasParameters;

use clap::{App, Arg};

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

const NX: u32 = 800;
const NY: u32 = 400;
const NS: u32 = 100;
const SF: f32 = 255.99; // scaling factor for converting colorf32 to u8
const GAMMA: f32 = 2.0;

const SEED: [u32; 4] = [0x193a6754, 0xa8a7d469, 0x97830e05, 0x113ba7bb];

const VERSION: &str = env!("CARGO_PKG_VERSION");

//--------------------------------------------------------------------
fn main() {
    let cam = Camera::default();

    let mut seed: [u8; 16] = [0; 16];
    unsafe { seed.copy_from_slice(SEED.align_to::<u8>().1) };
    let rng = SmallRng::from_seed(seed);

    let world: World<'_> = &[
        &Sphere::new(
            Point::new(0.0, 0.0, -1.0),
            0.5,
            Box::new(Lambertian::new(Color::new(0.8, 0.3, 0.3), rng.clone())),
        ),
        &Sphere::new(
            Point::new(0.0, -100.5, -1.0),
            100.0,
            Box::new(Lambertian::new(Color::new(0.8, 0.8, 0.8), rng.clone())),
        ),
        &Sphere::new(
            Point::new(1.0, 0.0, -1.0),
            0.5,
            Box::new(Metal::new(Color::new(0.8, 0.6, 0.2))),
        ),
        &Sphere::new(
            Point::new(-1.0, 0.0, -1.0),
            0.5,
            Box::new(Metal::new(Color::new(0.8, 0.8, 0.8))),
        ),
    ];

    let outfile = get_outfile();

    render_to_file(&cam, &world, &outfile);
}

//--------------------------------------------------------------------
fn get_outfile() -> String {
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

//--------------------------------------------------------------------
fn render_to_file(cam: &Camera, world: &World<'_>, filename: &str) {
    let mut imgbuf = ImageBuf::with_capacity(NX as usize * NY as usize * 4);
    let mut rng = SmallRng::from_entropy();

    let pngfile = format!("{}.png", filename);
    let path = Path::new(&pngfile);

    let file =
        File::create(&path).unwrap_or_else(|_| panic!("Couldn't open {} for writing.", pngfile));

    let w = &mut BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, NX, NY); // Width is nx pixels and height is ny
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    for j in (0..NY).rev() {
        for i in 0..NX {
            let mut col = Color::new(0.0, 0.0, 0.0);
            for _sub_sample in 0..NS {
                let u = (f64::from(i) + rng.gen::<f64>()) / f64::from(NX);
                let v = (f64::from(j) + rng.gen::<f64>()) / f64::from(NY);
                let r = cam.ray(u, v);
                col += color(&r, &world, 51);
            }

            //let col = (col / NS as f32) * SF;
            let col = (col / NS as f32).gamma_correct(GAMMA) * SF;
            let v: Coloru8 = col.cast();
            imgbuf.extend_from_slice(&(v.to_array()));
        }
    }

    writer.write_image_data(&imgbuf).unwrap();

    println!("Wrote to {:?}.", path);
}
