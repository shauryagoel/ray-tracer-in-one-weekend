use crate::{Color, HitRecord, Hittable, HittableList, Interval, Point3, Ray, Vec3};
use std::io::Write;

#[derive(Default)]
pub struct Camera {
    /// Ratio of image width to image height
    pub aspect_ratio: f64,
    /// Rendered image width in pixel count
    pub image_width: usize,
    /// Stream to write the rendered image
    // pub output_stream: Box<dyn Write>,

    // Rendered image height
    image_height: usize,
    // Camera center
    camera_center: Point3,
    // Location of pixel (0, 0)
    pixel00_loc: Vec3,
    // Offset of the pixel in the +x direction
    pixel_delta_u: Vec3,
    // Offset of the pixel in the -y direction
    pixel_delta_v: Vec3,
}

impl Camera {
    /// Renders the world to the `output_stream`
    pub fn render<T: Write>(&mut self, world: &HittableList, output_stream: &mut T) {
        self.initialise();

        writeln!(
            output_stream,
            "P3\n{} {}\n255",
            self.image_width, self.image_height
        )
        .unwrap();

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let pixel_location = self.pixel00_loc.clone()
                    + (i as f64 * self.pixel_delta_u.clone())
                    + (j as f64 * self.pixel_delta_v.clone());
                let ray_direction = pixel_location - self.camera_center.clone();
                let r = Ray::new(self.camera_center.clone(), ray_direction);
                let pixel_color = self.ray_color(&r, world);
                Color::write_color(output_stream, pixel_color);
            }
        }
    }

    fn initialise(&mut self) {
        // Calculate the image height
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as usize;
        let image_height: usize = usize::max(self.image_height, 1);
        let image_height_f64 = image_height as f64;
        let image_width_f64 = self.image_width as f64;

        // Camera center
        self.camera_center = Point3::default();

        // Viewport
        let focal_length: f64 = 1.0;
        let vieport_height: f64 = 2.0;
        let vieport_width: f64 = vieport_height * (image_width_f64 / image_height_f64);

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = Vec3::new(vieport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -vieport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u.clone() / image_width_f64;
        self.pixel_delta_v = viewport_v.clone() / image_height_f64;

        // Calculate the location of the upper left pixel
        let viewport_upper_left = Point3::new(0.0, 0.0, -focal_length)
            - self.camera_center.clone()
            - viewport_u / 2.0
            - viewport_v / 2.0;
        self.pixel00_loc =
            viewport_upper_left + 0.5 * (self.pixel_delta_u.clone() + self.pixel_delta_v.clone());
    }

    fn ray_color(&self, r: &Ray, world: &HittableList) -> Color {
        let mut rec: HitRecord = Default::default();
        if world.hit(r, Interval::new(0.0, f64::MAX), &mut rec) {
            let new_r = 0.5 * (rec.normal.x() + 1.0);
            let new_g = 0.5 * (rec.normal.y() + 1.0);
            let new_b = 0.5 * (rec.normal.z() + 1.0);
            return Color::new(new_r, new_g, new_b);
        }

        let unit_direction = r.direction().unit_vector();
        let alpha = 0.5 * (unit_direction.y() + 1.0);
        let white_color = Color::new(1.0, 1.0, 1.0);
        let blue_color = Color::new(0.5, 0.7, 1.0);
        (1.0 - alpha) * white_color + alpha * blue_color
    }
}
