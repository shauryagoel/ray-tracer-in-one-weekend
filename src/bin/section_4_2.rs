use ray_tracer_in_one_weekend::{Color, Point3, Ray, Vec3};
use std::io::{self, Write};

fn ray_color(r: Ray) -> Color {
    let unit_direction = r.direction().unit_vector();
    let alpha = 0.5 * (unit_direction.y() + 1.0);
    let white_color = Color::new(1.0, 1.0, 1.0);
    let blue_color = Color::new(0.5, 0.7, 1.0);
    (1.0 - alpha) * white_color + alpha * blue_color
}

fn generate_ppm<T: Write>(output_stream: &mut T) {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_width_f64 = image_width as f64;

    // Calculate the image height
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let image_height: u32 = u32::max(image_height, 1);
    let image_height_f64 = image_height as f64;

    // Viewport and Camera
    let focal_length: f64 = 1.0;
    let vieport_height: f64 = 2.0;
    let vieport_width: f64 = vieport_height * (image_width_f64 / image_height_f64);
    let camera_center = Point3::default();

    // Calculate the vectors across the horizontal and down the vertical viewport edges
    let viewport_u = Vec3::new(vieport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -vieport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u.clone() / image_width_f64;
    let pixel_delta_v = viewport_v.clone() / image_height_f64;

    // Calculate the location of the upper left pixel
    let viewport_upper_left = Point3::new(0.0, 0.0, -focal_length)
        - camera_center.clone()
        - viewport_u / 2.0
        - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u.clone() + pixel_delta_v.clone());

    // Render
    writeln!(output_stream, "P3\n{image_width} {image_height}\n255").unwrap();

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center = pixel00_loc.clone()
                + (i as f64 * pixel_delta_u.clone())
                + (j as f64 * pixel_delta_v.clone());
            let ray_direction = pixel_center - camera_center.clone();
            let r = Ray::new(camera_center.clone(), ray_direction);
            let pixel_color = ray_color(r);
            Color::write_color(output_stream, pixel_color);
        }
    }
}

fn main() {
    generate_ppm(&mut io::stdout())
}
