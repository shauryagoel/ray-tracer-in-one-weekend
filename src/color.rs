use crate::Interval;
use crate::vec::Vec3;
use std::io::Write;
use std::ops::{Add, AddAssign, Mul};

/// Represent RGB color using Vec3
///
/// Vec3 components should be between 0 and 1
#[derive(Clone, Default)]
pub struct Color(Vec3);

impl Color {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Color(Vec3::new(e0, e1, e2))
    }

    pub fn r(&self) -> f64 {
        self.0.x()
    }

    pub fn g(&self) -> f64 {
        self.0.y()
    }

    pub fn b(&self) -> f64 {
        self.0.z()
    }

    /// Write the normalize color (between 0 and 1) to the output stream with un-normalized values (between 0 and 255)
    pub fn write_color<T: Write>(output_stream: &mut T, color: Color) {
        let r = color.r();
        let g = color.g();
        let b = color.b();

        // Apply a linear to gamma transform for "gamma 2"
        let r = Color::linear_to_gamma(r);
        let g = Color::linear_to_gamma(g);
        let b = Color::linear_to_gamma(b);

        // Translate [0,1] component values to the byte range [0, 255]
        // // This is done because when r = 0.99999, `(255 * r) as u8` = 254, which is incorrect
        // let rbyte = (255.999 * r) as u8;
        // let gbyte = (255.999 * g) as u8;
        // let bbyte = (255.999 * b) as u8;

        // Clamp the RGB values before un-normalizing them
        let intensity = Interval::new(0.0, 0.9999);
        let rbyte = (256.0 * intensity.clamp(r)) as u8;
        let gbyte = (256.0 * intensity.clamp(g)) as u8;
        let bbyte = (256.0 * intensity.clamp(b)) as u8;

        // output_stream
        //     .write_all(&[rbyte, b' ', gbyte, b' ', bbyte, b'\n'])
        //     .unwrap();
        writeln!(output_stream, "{rbyte} {gbyte} {bbyte}").unwrap();
    }

    // Linear to "gamma 2" transformation
    fn linear_to_gamma(linear_component: f64) -> f64 {
        if linear_component > 0.0 {
            return linear_component.sqrt();
        }
        0.0
    }
}

impl Add<Color> for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Color::new(
            self.r() + other.r(),
            self.g() + other.g(),
            self.b() + other.b(),
        )
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other; // TODO: improve this
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, other: Color) -> Self::Output {
        Color::new(self * other.r(), self * other.g(), self * other.b())
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Self::Output {
        Color::new(
            self.r() * other.r(),
            self.g() * other.g(),
            self.b() * other.b(),
        )
    }
}
