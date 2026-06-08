use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::utils::random_f32;
use crate::vec3::{dot, unit_vector, Vec3};

pub struct Dialectric {
    // Refractive index in vacuum or air, or the ratio of the material's refractive index over
    // the refractive index of the enclosing media
    refraction_index: f32,
}

impl Dialectric {
    pub fn new(refraction_index: f32) -> Self {
        Dialectric { refraction_index }
    }

    // Use Schlick's approximation for reflectance.
    fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dialectric {
    fn scatter(&self, r_in: &Ray, hit_rec: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if hit_rec.front_face {1.0 / self.refraction_index} else { self.refraction_index };
        
        let unit_direction = unit_vector(&r_in.dir);
        let cos_theta = dot(&(-unit_direction), &hit_rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        
        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract || Dialectric::reflectance(cos_theta, ri) > random_f32() {
            Vec3::reflect(&unit_direction, &hit_rec.normal)
        } else {
            Vec3::refract(&unit_direction, &hit_rec.normal, ri)
        };
        
        *scattered = Ray::new(hit_rec.point, direction);
        true
    }
}