use raytracer::*;

use chrono::Local;

const NX: u32 = 1800;
const NY: u32 = 1200;
const NS: u32 = 800;
const SF: f64 = 256.0; // scaling factor for converting color64 to u8

const CHAPTER: &str = "chapter13";

fn main() {
    let now = format!("{}", Local::now().format("%Y%m%d_%H:%M:%S"));
    let default_file = format!("{}-{}", CHAPTER, now);
    let args = get_args(&default_file);
    let outfile = args.value_of("OUTPUT").unwrap();

    let capacity = (NX * NY * 3) as usize;
    let mut data: Vec<u8> = Vec::with_capacity(capacity);

    let img_width = NX as f64;
    let img_height = NY as f64;
    let ratio = img_width / img_height;

    // set up our world
    let mut big_rng = thread_rng();
    let world = random_scene(&mut big_rng);

    // camera
    let cam_origin = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let focus_dist = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(
        cam_origin,
        look_at,
        Vec3::new(0.0, 1.0, 0.0), // up
        20.0,                     // vfov in degrees
        ratio,
        aperture,
        focus_dist,
    );

    // cheap rng for anti-alias calls
    let mut smol_rng = SmallRng::from_rng(&mut big_rng).unwrap();

    // Now the real rendering work:
    let unsat = 1.0 / NS as f64;
    for j in (0..NY).rev() {
        for i in 0..NX {
            let mut col = Color64::default();
            for _ in 0..NS {
                let u: f64 = (i as f64 + smol_rng.gen::<f64>()) / img_width;
                let v: f64 = (j as f64 + smol_rng.gen::<f64>()) / img_height;
                let r = camera.get_ray(u, v, &mut smol_rng);
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

fn random_scene(rng: &mut impl Rng) -> Vec<Sphere> {
    let mut world = Vec::new();

    let ground_material = Lambertian::new(Color64::new(0.5, 0.5, 0.5)).mat_ptr();
    world.push(Sphere {
        center: Point3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: ground_material,
    });

    for a in -11..11 {
        for b in -11..11 {
            let a = a as f64;
            let b = b as f64;
            let choose_mat: f64 = rng.gen();
            let center = Point3::new(a + 0.9 * rng.gen::<f64>(), 0.2, b + 0.9 * rng.gen::<f64>());
            let focus = Point3::new(4.0, 0.2, 0.0);
            if (center - focus).length() > 0.9 {
                let sphere_material = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color64::new(
                        rng.gen::<f64>() * rng.gen::<f64>(),
                        rng.gen::<f64>() * rng.gen::<f64>(),
                        rng.gen::<f64>() * rng.gen::<f64>(),
                    );
                    Lambertian::new(albedo).mat_ptr()
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color64::new(
                        rng.gen_range(0.0..0.5),
                        rng.gen_range(0.0..0.5),
                        rng.gen_range(0.0..0.5),
                    );
                    let fuzz = rng.gen_range(0.0..0.5);
                    Metal { albedo, fuzz }.mat_ptr()
                } else {
                    // glass
                    Dialectric { i_o_r: 1.5 }.mat_ptr()
                };
                world.push(Sphere {
                    center,
                    radius: 0.2,
                    material: sphere_material,
                });
            }
        }
    }

    let material1 = Dialectric { i_o_r: 1.5 }.mat_ptr();
    world.push(Sphere {
        center: Point3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: material1,
    });

    let material2 = Lambertian::new(Color64::new(0.4, 0.2, 0.1)).mat_ptr();
    world.push(Sphere {
        center: Point3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: material2,
    });

    let material3 = Metal {
        albedo: Color64::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    }
    .mat_ptr();
    world.push(Sphere {
        center: Point3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: material3,
    });

    world
}
