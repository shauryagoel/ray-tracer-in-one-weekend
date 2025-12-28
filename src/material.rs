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

/// Material which scatters and attenuates light accoording to it reflectance
#[derive(Default)]
pub struct Lambertian {
    albedo: Color,
}

/// Material which completely reflects incident ray and attenuate light accoording to it reflectance
#[derive(Default)]
pub struct Metal {
    albedo: Color,
    fuzz: f64, // A value between 0 and 1, indicating fuzzing magnitude
}

/// Material which refracts the light
#[derive(Default)]
pub struct Dielectric {
    refraction_index: f64, // Refractive index in vacuum or air, or the ratio of the material's refractive index over the refractive index of the enclosing medium
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

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}
impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        // Refraction formula takes refraction index of incident medium in the numerator
        // So, if the incident ray comes from another medium, take inverse of the self.refraction_index
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = r_in.direction().unit_vector();

        // Handle total internal reflection
        let cos_theta = f64::min((-&unit_direction).dot(&rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction = {
            if ri * sin_theta > 1.0 {
                unit_direction.reflect(&rec.normal)
            } else {
                unit_direction.refract(&rec.normal, ri)
            }
        };

        *scattered = Ray::new(rec.p.clone(), direction);
        true
    }
}
