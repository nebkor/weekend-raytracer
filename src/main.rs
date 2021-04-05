#![allow(clippy::many_single_char_names)]
use raytracer::*;

use chrono::Local;
use crossbeam_utils::thread::scope;

//const NX: u32 = 1800;
//const NY: u32 = 1200;
const NX: u32 = (1600.0 / 2.5) as u32;
const NY: u32 = (900.0 / 2.5) as u32;
const NS: u32 = 100;
const SF: f64 = 256.0; // scaling factor for converting color64 to u8
const NC: u32 = 3; // number of color channels

const CHAPTER: &str = "chapter14";

fn main() {
    let now = format!("{}", Local::now().format("%Y%m%d_%H:%M:%S"));
    let default_file = format!("{}-{}", CHAPTER, now);
    let args = get_args(&default_file);
    let outfile = args.value_of("OUTPUT").unwrap();

    let capacity = (NX * NY * 3) as usize;
    let mut data: Vec<u8> = vec![0; capacity];

    let chunk_size = NX * NC;
    let num_threads = num_cpus::get_physical();

    let img_width = NX as f64;
    let img_height = NY as f64;
    let ratio = img_width / img_height;

    // set up our world
    let mut rng = SmallRng::from_entropy();
    let world = random_scene(&mut rng);

    // camera
    let cam_origin = Point3::new(13.0, 2.5, 4.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let focus_dist = cam_origin.to_vector().length();
    let aperture = 0.01;
    let camera = Camera::new(
        cam_origin,
        look_at,
        Vec3::new(0.0, 1.0, 0.0), // up
        20.0,                     // vfov in degrees
        ratio,
        aperture,
        focus_dist,
        0.0,
        1.0,
    );

    // make a new scope to ensure everything is tidied up, borrow-wise, by the time we need to write the outfile
    {
        let (sender, receiver) = crossbeam_channel::unbounded();

        for (j, chunk) in data.chunks_mut(chunk_size as usize).enumerate() {
            // our first bytes are in the upper left, and positive y goes down in the rendered image
            let j = (NY - 1) as usize - j;
            sender.send((j, chunk)).unwrap();
        }

        let unsat = 1.0 / NS as f64;
        scope(|s| {
            for _ in 0..num_threads {
                let recv = receiver.clone();
                let mut rng = rng.clone();
                let camera = &camera;
                let world = &world;
                s.spawn(move |_| {
                    while let Ok((j, buf)) = recv.try_recv() {
                        for (i, pixel) in buf.chunks_mut(NC as usize).enumerate() {
                            let i = i as f64;
                            let j = j as f64;
                            let mut col = Color64::zero();
                            for _ in 0..NS {
                                let u: f64 = (i + rng.gen::<f64>()) / img_width;
                                let v: f64 = (j + rng.gen::<f64>()) / img_height;
                                let r = camera.get_ray(u, v, &mut rng);
                                col += color(&r, world, &mut rng, MAX_BOUNCES);
                            }
                            let col = col
                                .to_array()
                                .iter()
                                .map(|elem| {
                                    ((elem * unsat).sqrt().clamp(0.0, CLAMP_MAX) * SF) as u8
                                })
                                .collect::<Vec<u8>>();
                            pixel[0] = col[0];
                            pixel[1] = col[1];
                            pixel[2] = col[2];
                        }
                    }
                });
            }
        })
        .unwrap();
        drop(sender);
    }

    write_png(outfile, data.as_ref(), NX, NY);
}

fn random_scene(rng: &mut impl Rng) -> Vec<Sphere> {
    let mut world = Vec::new();

    let ground_material = Lambertian::new(Color64::new(0.5, 0.5, 0.5)).mat_ptr();
    world.push(Sphere {
        center_start: Point3::new(0.0, -1000.0, 0.0),
        center_end: Point3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: ground_material,
        t_start: 0.0,
        t_end: 1.0,
    });

    let spawn_point = Point3::new(4.0, 0.2, 0.0);
    for a in -11..11 {
        for b in -11..11 {
            let a = a as f64;
            let b = b as f64;
            let choose_mat: f64 = rng.gen();
            let center_start =
                Point3::new(a + 0.9 * rng.gen::<f64>(), 0.2, b + 0.9 * rng.gen::<f64>());
            if (center_start - spawn_point).length() > 0.9 {
                let (sphere_material, center_end) = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color64::new(
                        rng.gen::<f64>() * rng.gen::<f64>(),
                        rng.gen::<f64>() * rng.gen::<f64>(),
                        rng.gen::<f64>() * rng.gen::<f64>(),
                    );
                    (
                        Lambertian::new(albedo).mat_ptr(),
                        center_start + Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0),
                    )
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color64::new(
                        rng.gen_range(0.0..0.5),
                        rng.gen_range(0.0..0.5),
                        rng.gen_range(0.0..0.5),
                    );
                    let fuzz = rng.gen_range(0.0..0.5);
                    (Metal { albedo, fuzz }.mat_ptr(), center_start)
                } else {
                    // glass
                    (Dialectric { i_o_r: 1.5 }.mat_ptr(), center_start)
                };
                world.push(Sphere {
                    center_start,
                    center_end,
                    radius: 0.2,
                    material: sphere_material,
                    t_start: 0.0,
                    t_end: 1.0,
                });
            }
        }
    }

    let material1 = Dialectric { i_o_r: 1.5 }.mat_ptr();
    world.push(Sphere {
        center_start: Point3::new(0.0, 1.0, 0.0),
        center_end: Point3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: material1,
        t_start: 0.0,
        t_end: 1.0,
    });

    let material2 = Lambertian::new(Color64::new(0.4, 0.2, 0.1)).mat_ptr();
    world.push(Sphere {
        center_start: Point3::new(-4.0, 1.0, 0.0),
        center_end: Point3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: material2,
        t_start: 0.0,
        t_end: 1.0,
    });

    let material3 = Metal {
        albedo: Color64::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    }
    .mat_ptr();
    world.push(Sphere {
        center_start: Point3::new(4.0, 1.0, 0.0),
        center_end: Point3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: material3,
        t_start: 0.0,
        t_end: 1.0,
    });

    world
}
