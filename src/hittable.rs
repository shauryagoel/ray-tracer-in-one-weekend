use crate::{Interval, Point3, Ray, Vec3};

#[derive(Clone, Default)]
pub struct HitRecord {
    pub p: Point3,        // The point of intersection
    pub normal: Vec3,     // The normal to p (can be inwards or outwards to the object)
    pub t: f64,           // The time of intersection, the `t` variable in ray equation
    pub front_face: bool, // Whether the ray hit the object from outside the surface on inside
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

impl HitRecord {
    /// Set the `front_face` and the normal vector of `HitRecord`
    /// `outward_normal` should be a unit vector
    ///
    /// The ray and the surface normal are opposite to each other
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.direction().dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}
