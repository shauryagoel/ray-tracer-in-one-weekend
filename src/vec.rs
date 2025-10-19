use crate::utils;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Represent a collection of 3 values
/// It can represent a point in 3D space or RGB values of a color via the "New Type Pattern"
#[derive(Clone, Default)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    /// Find dot product between vectors
    pub fn dot(&self, other: &Self) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    /// Find cross product between vectors
    pub fn cross(&self, other: &Self) -> Self {
        Self::new(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(&self) -> Self {
        self.clone() / self.length()
    }

    /// Generate a random vector with value of all components between `min` and `max`
    pub fn random(min: f64, max: f64) -> Self {
        Self::new(
            utils::random_f64(min, max),
            utils::random_f64(min, max),
            utils::random_f64(min, max),
        )
    }

    /// Generate a random unit vector on a sphere
    pub fn random_unit_vector() -> Self {
        loop {
            let p = Vec3::random(-1.0, 1.0);
            let len_squared = p.length_squared();
            if len_squared > 1e-160 && len_squared <= 1.0 {
                return p / len_squared.sqrt();
            }
        }
    }

    /// Generate a random vector on same hemisphere as the sphere normal
    pub fn random_on_hemisphere(normal: &Vec3) -> Self {
        let on_unit_sphere = Self::random_unit_vector();

        // In the same hemisphere as the normal
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    /// Returns true if the vector is near-zero in all dimensions
    pub fn near_zero(&self) -> bool {
        let epsilon = 1e-8;
        (self.x().abs() < epsilon) && (self.y().abs() < epsilon) && (self.z().abs() < epsilon)
    }

    /// Get reflected vector given the incident ray (self) and the normal vector
    pub fn reflect(&self, n: &Vec3) -> Vec3 {
        self - 2.0 * self.dot(n) * n
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x(), -self.y(), -self.z())
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, other: Vec3) -> Self::Output {
        Self::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, other: Vec3) -> Self::Output {
        Self::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Self::Output {
        Vec3::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, t: f64) -> Self::Output {
        Self::new(self.x() * t, self.y() * t, self.z() * t)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        Vec3::new(self * other.x(), self * other.y(), self * other.z())
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Self::Output {
        Vec3::new(self * other.x(), self * other.y(), self * other.z())
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, t: f64) -> Self::Output {
        Self::new(self.x() / t, self.y() / t, self.z() / t)
    }
}
