use std::ops::{Add, Div, Mul, Neg, Sub};

/// Represent a collection of 3 values
/// It can represent a point in 3D space or RGB values of a color via the "New Type Pattern"
#[derive(Clone)]
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
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3 { e: [0.0, 0.0, 0.0] }
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

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, t: f64) -> Self::Output {
        Self::new(self.x() / t, self.y() / t, self.z() / t)
    }
}
