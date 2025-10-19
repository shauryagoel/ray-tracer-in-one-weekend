use crate::{Color, HitRecord, Ray, Vec3};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        false
    }
}

/// Material to scatter and attenuate light accoording to it reflectance
#[derive(Default)]
pub struct Lambertian {
    albedo: Color,
}

/// Material to completely reflect incident ray and attenuate light accoording to it reflectance
#[derive(Default)]
pub struct Metal {
    albedo: Color,
    fuzz: f64, // A value between 0 and 1, indicating fuzzing magnitude
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal.clone() + Vec3::random_unit_vector();
        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal.clone();
        }
        *scattered = Ray::new(rec.p.clone(), scatter_direction);
        *attenuation = self.albedo.clone();
        true
    }
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut reflected = r_in.direction().reflect(&rec.normal);
        // Fuzz the reflected ray in a random direction
        reflected = reflected.unit_vector() + self.fuzz * Vec3::random_unit_vector();
        *scattered = Ray::new(rec.p.clone(), reflected);
        *attenuation = self.albedo.clone();
        // If the ray is below the surface then, absorb the ray in the surface
        scattered.direction().dot(&rec.normal) > 0.0
    }
}
