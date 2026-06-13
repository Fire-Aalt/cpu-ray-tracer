use std::sync::Arc;
use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::math::{dot, Ray, Vec3};
use crate::utils::random_f32;

pub struct Dialectric {
    // Refractive index in vacuum or air, or the ratio of the material's refractive index over
    // the refractive index of the enclosing media
    refraction_index: f32,
}

impl Dialectric {
    pub fn new(refraction_index: f32) -> Arc<Self> {
        Arc::new(Dialectric { refraction_index })
    }

    // Use Schlick's approximation for reflectance.
    fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dialectric {
    fn scatter(&self, r_in: &Ray, hit_rec: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray, pdf: &mut f32) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if hit_rec.front_face {1.0 / self.refraction_index} else { self.refraction_index };
        
        let unit_direction = r_in.direction().unit_vector();
        let cos_theta = dot(&(-unit_direction), &hit_rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        
        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract || Dialectric::reflectance(cos_theta, ri) > random_f32() {
            Vec3::reflect(&unit_direction, &hit_rec.normal)
        } else {
            Vec3::refract(&unit_direction, &hit_rec.normal, ri)
        };
        
        *scattered = Ray::new_t(hit_rec.point, direction, r_in.time());
        true
    }
}