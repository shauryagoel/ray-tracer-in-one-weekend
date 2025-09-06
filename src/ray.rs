use crate::point::Point3;
use crate::vec::Vec3;

/// Represents a ray using `origin` and `direction`
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray {
            orig: origin,
            dir: direction,
        }
    }

    /// Get value along a ray at time `t`
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir // `Copy` happens here
    }

    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }
}
