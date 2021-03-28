use raytracer::*;

use chrono::Local;
use clap::{App, Arg};

const NX: u32 = 800;
const NY: u32 = 400;
const NS: u32 = 100;
const SF: f64 = 255.99; // scaling factor for converting color64 to u8

const VERSION: &str = env!("CARGO_PKG_VERSION");

const CHAPTER: &str = "chapter7";

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

    let mut data: Vec<u8> = Vec::with_capacity(NX as usize * NY as usize * 4);

    let WIDTH = NX as f64;
    let HEIGHT = NY as f64;
    let ratio = WIDTH / HEIGHT;

    // set up our world
    let mut big_rng = thread_rng();
    let mut smol_rng = SmallRng::from_rng(&mut big_rng).unwrap();

    let world = vec![
        Sphere {
            center: Point3::new(0.0, 0.0, -1.0),
            radius: 0.5,
        },
        Sphere {
            center: Point3::new(0.0, -100.5, -1.0),
            radius: 100.0,
        },
    ];

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
    let unsat = 1.0 / NS as f64;
    for j in (0..NY).rev() {
        for i in 0..NX {
            let mut col = Color64::default();
            for _ in 0..NS {
                let u: f64 = (i as f64 + smol_rng.gen::<f64>()) / WIDTH;
                let v: f64 = (j as f64 + smol_rng.gen::<f64>()) / HEIGHT;
                let r = Ray::new(
                    origin,
                    lower_left_corner + horizontal * u + vertical * v - origin,
                );
                col += color(&r, &world);
            }
            col *= unsat;
            let col = col
                .to_array()
                .iter()
                .map(|elem| elem.clamp(0.0, 0.99999))
                .collect::<Vec<f64>>();
            let col = Color64::new(col[0], col[1], col[2]);
            let c = col * SF;
            let v: Color8 = c.cast();
            data.extend_from_slice(v.to_array().as_ref());
        }
    }

    write_png(outfile, data.as_ref(), NX, NY);
}
