use crate::vec::Vec3;
use std::ops::{Add, Sub};

/// Represent points in 3D space
#[derive(Clone, Default)]
pub struct Point3(Vec3);

impl Point3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Point3(Vec3::new(e0, e1, e2))
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
}

impl Add<Vec3> for Point3 {
    type Output = Self;

    fn add(self, other: Vec3) -> Self::Output {
        Point3::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}
