use ray_tracer_in_one_weekend::{Camera, HittableList, Point3, Sphere};
use std::io::{self, Write};
use std::rc::Rc;

fn generate_ppm<T: Write>(output_stream: &mut T) {
    let mut world: HittableList = Default::default();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
    // world.add(Rc::new(Sphere::new(Point3::new(0.0, 100.5, -1.0), 100.0)));

    let mut cam: Camera = Default::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;

    cam.render(&world, output_stream);
}

fn main() {
    generate_ppm(&mut io::stdout())
}
