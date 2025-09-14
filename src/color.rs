use crate::vec::Vec3;
use std::io::Write;

/// Represent RGB color using Vec3
///
/// Vec3 components should be between 0 and 1
#[derive(Clone)]
pub struct Color(Vec3);

impl Color {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Color(Vec3::new(e0, e1, e2))
    }

    pub fn x(&self) -> f64 {
        self.0.x()
    }

    pub fn y(&self) -> f64 {
        self.0.y()
    }

    pub fn z(&self) -> f64 {
        self.0.z()
    }

    /// Write the color to the output stream
    pub fn write_color<T: Write>(output_stream: &mut T, color: Color) {
        let r = color.x();
        let g = color.y();
        let b = color.z();

        // Translate [0,1] component values to the byte range [0, 255]
        // This is done because when r = 0.99999, (255 * r) as u8 -> 254, which is incorrect
        let rbyte = (255.999 * r) as u8;
        let gbyte = (255.999 * g) as u8;
        let bbyte = (255.999 * b) as u8;

        // output_stream
        //     .write_all(&[rbyte, b' ', gbyte, b' ', bbyte, b'\n'])
        //     .unwrap();
        writeln!(output_stream, "{rbyte} {gbyte} {bbyte}").unwrap();
    }
}
