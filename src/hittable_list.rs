use crate::{HitRecord, hittable::Hittable, ray::Ray};
use std::rc::Rc;

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object.clone());
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    /// Find closest intersection from the list of hittable objects
    /// and update the information in the given `HitRecord` variable
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = Default::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;

        for object in self.objects.iter() {
            if object.hit(r, ray_tmin, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}
