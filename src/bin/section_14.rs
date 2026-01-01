use ray_tracer_in_one_weekend::{
    Camera, Color, Dielectric, HittableList, Lambertian, Material, Metal, Point3, Sphere, Vec3,
    utils,
};
use std::io::{self, Write};
use std::rc::Rc;

fn generate_ppm<T: Write>(output_stream: &mut T) {
    let mut world: HittableList = Default::default();

    // Add the ground ball
    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    // Add the small balls
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = utils::random_f64(0.0, 1.0);
            let center = Point3::new(
                (a as f64) + 0.9 * utils::random_f64(0.0, 1.0),
                0.2,
                (b as f64) + 0.9 * utils::random_f64(0.0, 1.0),
            );

            if (center.clone() - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material> = if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Color::random(0.0, 1.0) * Color::random(0.0, 1.0);
                    Rc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Color::random(0.5, 1.0);
                    let fuzz = utils::random_f64(0.0, 0.5);
                    Rc::new(Metal::new(albedo, fuzz))
                } else {
                    // Glass
                    Rc::new(Dielectric::new(1.5))
                };

                world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    // Add the 3 big balls
    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    // Set the camera
    let mut cam: Camera = Default::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 500;
    // cam.image_width = 400;
    // cam.samples_per_pixel = 10;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.render(&world, output_stream);
}

fn main() {
    generate_ppm(&mut io::stdout())
}
