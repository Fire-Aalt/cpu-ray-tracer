use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::{Material, ScatterRecord};
use crate::math::{Ray, Vec3, dot};
use std::sync::Arc;

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Arc<Self> {
        Arc::new(Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        })
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit_rec: &HitRecord, scatter_record: &mut ScatterRecord) -> bool {
        let Some(unit_direction) = r_in.direction().try_unit_vector() else {
            return false;
        };
        let reflected = Vec3::reflect(&unit_direction, &hit_rec.normal)
            + (Vec3::random_unit_vector() * self.fuzz);

        if !reflected.is_finite() || reflected.near_zero() {
            return false;
        }

        scatter_record.attenuation = self.albedo;
        scatter_record.pdf = None;
        scatter_record.skip_pdf_ray = Ray::new_t(hit_rec.point, reflected, r_in.time());

        true
    }
}
