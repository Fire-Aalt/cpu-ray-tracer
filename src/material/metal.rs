use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{dot, Vec3};

pub struct Metal {
    albedo: Color,
    fuzz: f32
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        Metal { albedo, fuzz: if fuzz < 1.0 { fuzz } else { 1.0 } }
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

    fn emitted(&self, u: f32, v: f32, p: &Vec3) -> Color {
        Color::BLACK
    }
}
