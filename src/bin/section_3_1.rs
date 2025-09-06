use ray_tracer_in_one_weekend::Color;
use std::io::{self, Write};

fn generate_ppm<T: Write>(output_stream: &mut T) {
    let image_width: i32 = 256;
    let image_height: i32 = 256;

    writeln!(output_stream, "P3\n{image_width} {image_height}\n255").unwrap();

    for j in 0..image_height {
        for i in 0..image_width {
            let r = (i as f64) / (image_width as f64 - 1.0);
            let g = (j as f64) / (image_height as f64 - 1.0);
            let b = 0.0;

            let pixel_color = Color::new(r, g, b);
            Color::write_color(output_stream, pixel_color);
        }
    }
}

fn main() {
    generate_ppm(&mut io::stdout())
}

#[cfg(test)]
mod ppm_tests {
    use super::*;

    fn generate_ppm_original() -> String {
        let image_width: i32 = 256;
        let image_height: i32 = 256;
        let mut output_string = String::new();

        output_string += &format!("P3\n{image_width} {image_height}\n255\n");

        for j in 0..image_height {
            for i in 0..image_width {
                let r = (i as f64) / (image_width as f64 - 1.0);
                let g = (j as f64) / (image_height as f64 - 1.0);
                let b = 0.0;

                let ir = (255.999 * r) as i32;
                let ig = (255.999 * g) as i32;
                let ib = (255.999 * b) as i32;

                output_string += &format!("{ir} {ig} {ib}\n");
            }
        }
        output_string
    }

    #[test]
    fn test_hello_world_ppm() {
        let original_ppm = generate_ppm_original();
        let mut output_stream = Vec::new();
        generate_ppm(&mut output_stream);
        assert_eq!(original_ppm.as_bytes(), output_stream);
    }
}
