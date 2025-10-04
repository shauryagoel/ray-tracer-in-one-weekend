use crate::{Color, HitRecord, Hittable, HittableList, Interval, Point3, Ray, Vec3, utils};
use std::io::Write;

pub struct Camera {
    /// Ratio of image width to image height
    pub aspect_ratio: f64,
    /// Rendered image width in pixel count
    pub image_width: usize,
    /// Count of random samples for each pixel
    pub samples_per_pixel: usize,
    /// Max number of ray bounces into the scene
    pub max_depth: usize,
    /// Stream to write the rendered image
    // pub output_stream: Box<dyn Write>,

    // Rendered image height
    image_height: usize,
    // Color scale factor for a sum of pixel samples (is inverse of samples_per_pixel)
    pixel_samples_scale: f64,
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
    fn initialise(&mut self) {
        // Calculate the image height
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as usize;
        let image_height: usize = usize::max(self.image_height, 1);
        let image_height_f64 = image_height as f64;
        let image_width_f64 = self.image_width as f64;

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

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
                // Sample points around pixels
                let mut pixel_color = Color::default();
                for _ in 0..self.samples_per_pixel {
                    let r: Ray = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, self.max_depth, world);
                }

                Color::write_color(output_stream, self.pixel_samples_scale * pixel_color);
            }
        }
    }

    // Get a ray originating from the camera center (origin) and directed at a
    // randomly sampled point around the pixel location (i, j)
    fn get_ray(&self, i: usize, j: usize) -> Ray {
        let offset = Camera::sample_square();
        let pixel_sample = self.pixel00_loc.clone()
            + ((i as f64 + offset.x()) * self.pixel_delta_u.clone())
            + ((j as f64 + offset.y()) * self.pixel_delta_v.clone());

        let ray_origin = self.camera_center.clone();
        let ray_direction = pixel_sample - ray_origin.clone();
        Ray::new(ray_origin, ray_direction)
    }

    // Returns a random point in the unit square from [-0.5, -0.5] to [0.5, 0.5]
    fn sample_square() -> Vec3 {
        Vec3::new(
            utils::random_f64(0.0, 1.0) - 0.5,
            utils::random_f64(0.0, 1.0) - 0.5,
            0.0,
        )
    }

    // Get the color of the closest object in the `world` when passing `ray` through the world
    fn ray_color(&self, r: &Ray, depth: usize, world: &HittableList) -> Color {
        // If ray bounce limit is exceeded, no more light is gathered
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        let mut rec: HitRecord = Default::default();
        // Solve the "Acne Problem"
        if world.hit(r, Interval::new(0.001, f64::MAX), &mut rec) {
            let direction = Vec3::random_on_hemisphere(&rec.normal);
            return 0.5 * self.ray_color(&Ray::new(rec.p, direction), depth - 1, world);
        }

        let unit_direction = r.direction().unit_vector();
        let alpha = 0.5 * (unit_direction.y() + 1.0);
        let white_color = Color::new(1.0, 1.0, 1.0);
        let blue_color = Color::new(0.5, 0.7, 1.0);
        (1.0 - alpha) * white_color + alpha * blue_color
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            image_height: Default::default(),
            pixel_samples_scale: Default::default(),
            camera_center: Default::default(),
            pixel00_loc: Default::default(),
            pixel_delta_u: Default::default(),
            pixel_delta_v: Default::default(),
        }
    }
}
