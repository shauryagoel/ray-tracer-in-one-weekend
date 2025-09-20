use crate::{HitRecord, Point3, Ray, hittable::Hittable};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Sphere {
            center,
            radius: radius.max(0.0),
        }
    }
}

impl Hittable for Sphere {
    /// Store the information regarding the intersection of sphere in a `HitRecord` if the ray
    /// hit the Sphere between `t_min` and `t_max`, and return true, else return false
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let oc = &self.center - r.origin();
        let a = r.direction().length_squared();
        let h = r.direction().dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (h - sqrtd) / a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.set_face_normal(r, (&rec.p - &self.center) / self.radius);

        true
    }
}
