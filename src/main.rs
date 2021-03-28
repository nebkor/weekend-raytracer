use raytracer::*;

use chrono::Local;

const NX: u32 = 800;
const NY: u32 = 400;
const NS: u32 = 60;
const SF: f64 = 256.0; // scaling factor for converting color64 to u8

const CHAPTER: &str = "chapter8";

fn main() {
    let now = format!("{}", Local::now().format("%Y%m%d_%H:%M:%S"));
    let default_file = format!("{}/{}", CHAPTER, now);
    let args = get_args(&default_file);
    let outfile = args.value_of("OUTPUT").unwrap();

    let capacity = (NX * NY * 3) as usize;
    let mut data: Vec<u8> = Vec::with_capacity(capacity);

    let img_width = NX as f64;
    let img_height = NY as f64;
    let ratio = img_width / img_height;

    // set up our world
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

    // we'll need some random numbers
    let mut big_rng = thread_rng();
    let mut smol_rng = SmallRng::from_rng(&mut big_rng).unwrap();

    // Now the real rendering work:
    let unsat = 1.0 / NS as f64;
    for j in (0..NY).rev() {
        for i in 0..NX {
            let mut col = Color64::default();
            for _ in 0..NS {
                let u: f64 = (i as f64 + smol_rng.gen::<f64>()) / img_width;
                let v: f64 = (j as f64 + smol_rng.gen::<f64>()) / img_height;
                let r = Ray::new(
                    origin,
                    lower_left_corner + horizontal * u + vertical * v - origin,
                );
                col += color(&r, &world, &mut smol_rng, MAX_BOUNCES);
            }
            let col = col
                .to_array()
                .iter()
                .map(|elem| ((elem * unsat).sqrt().clamp(0.0, CLAMP_MAX) * SF) as u8)
                .collect::<Vec<u8>>();
            let col = Color8::new(col[0], col[1], col[2]);
            data.extend_from_slice(col.to_array().as_ref());
        }
    }

    write_png(outfile, data.as_ref(), NX, NY);
}
