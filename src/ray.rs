use crate::point::Point3;
use crate::vec::Vec3;

/// Represents a ray using `origin` and `direction`
#[derive(Clone)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    /// Get value along a ray at time `t`
    pub fn at(&self, t: f64) -> Point3 {
        self.origin.clone() + t * self.direction.clone()
    }

    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }
}
