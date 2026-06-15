use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::{Material, ScatterRecord};
use crate::math::{Ray, Vec3, dot};
use crate::utils::random_f32;
use std::sync::Arc;

pub struct Dialectric {
    // Refractive index in vacuum or air, or the ratio of the material's refractive index over
    // the refractive index of the enclosing media
    refraction_index: f32,
}

impl Dialectric {
    pub fn new(refraction_index: f32) -> Arc<Self> {
        assert!(
            refraction_index.is_finite() && refraction_index > 0.0,
            "refraction index must be finite and positive"
        );
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
    fn scatter(&self, r_in: &Ray, hit_rec: &HitRecord, scatter_record: &mut ScatterRecord) -> bool {
        scatter_record.attenuation = Color::WHITE;
        scatter_record.pdf = None;
        let ri = if hit_rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let Some(unit_direction) = r_in.direction().try_unit_vector() else {
            return false;
        };
        let cos_theta = dot(&(-unit_direction), &hit_rec.normal).clamp(0.0, 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).max(0.0).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract || Dialectric::reflectance(cos_theta, ri) > random_f32() {
            Vec3::reflect(&unit_direction, &hit_rec.normal)
        } else {
            Vec3::refract(&unit_direction, &hit_rec.normal, ri)
        };

        scatter_record.skip_pdf_ray = Ray::new_t(hit_rec.point, direction, r_in.time());
        true
    }
}
