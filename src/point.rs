use crate::vec::Vec3;
use std::ops::Add;

/// Represent points in 3D space
pub type Point3 = Vec3;

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
