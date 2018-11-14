use raytracer::*;

const NX: u32 = 600;
const NY: u32 = 300;
const NS: u32 = 100;
const SF: f32 = 255.99; // scaling factor for converting colorf32 to u8
const GAMMA: f32 = 2.0;

//--------------------------------------------------------------------
fn main() {
    let cam = Camera::default();

    let world: World<'_> = &[
        &Sphere::new(
            Point::new(0.0, 0.0, -1.0),
            0.5,
            MatSpec::Lambertian(Color::new(0.1, 0.2, 0.5)),
        ),
        &Sphere::new(
            Point::new(0.0, -100.5, -1.0),
            100.0,
            MatSpec::Lambertian(Color::new(0.8, 0.8, 0.0)),
        ),
        &Sphere::new(
            Point::new(1.0, 0.0, -1.0),
            0.5,
            MatSpec::Metal(Color::new(0.8, 0.6, 0.2), 0.0),
        ),
        &Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.5, MatSpec::Dialectric(1.5)),
        &Sphere::new(Point::new(-1.0, 0.0, -1.0), -0.45, MatSpec::Dialectric(1.5)),
    ];

    let outfile = get_outfile();

    render_to_file(&cam, &world, &outfile);
}

//--------------------------------------------------------------------
fn render_to_file(cam: &Camera, world: &World<'_>, filename: &str) {
    use png::HasParameters;
    use std::fs::File;
    use std::io::BufWriter;
    use std::path::Path;

    let mut imgbuf = ImageBuf::with_capacity(NX as usize * NY as usize * 4);
    let mut rng = get_rng();

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
            let col = (col / NS as f32).gamma_correct(GAMMA) * SF;
            let v: Coloru8 = col.cast();
            imgbuf.extend_from_slice(&(v.to_array()));
        }
    }

    writer.write_image_data(&imgbuf).unwrap();

    println!("Wrote to {:?}.", path);
}
