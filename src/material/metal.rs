use std::sync::Arc;
use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::math::{dot, Ray, Vec3};

pub struct Metal {
    albedo: Color,
    fuzz: f32
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Arc<Self> {
        Arc::new(Metal { albedo, fuzz: if fuzz < 1.0 { fuzz } else { 1.0 } })
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit_rec: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut reflected = Vec3::reflect(r_in.direction(), &hit_rec.normal);
        reflected = reflected.unit_vector() + (Vec3::random_unit_vector() * self.fuzz);
        *scattered = Ray::new_t(hit_rec.point, reflected, r_in.time());
        *attenuation = self.albedo;
        dot(scattered.direction(), &hit_rec.normal) > 0.0
    }

    fn emitted(&self, _u: f32, _v: f32, _p: &Vec3) -> Color {
        Color::BLACK
    }
}
