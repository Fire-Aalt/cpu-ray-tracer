use std::sync::Arc;
use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::texture::{SolidColorTexture, Texture};
use crate::vec3::Vec3;

pub struct DiffuseLight {
    tex: Arc<dyn Texture>
}

impl DiffuseLight {
    pub fn new(tex: Arc<dyn Texture>) -> Arc<Self> {
        Arc::new(DiffuseLight { tex })
    }

    pub fn from_color(emit: Color) -> Arc<Self> {
        Arc::new(DiffuseLight { tex: SolidColorTexture::new(emit) })
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, r_in: &Ray, hit_rec: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        false
    }

    fn emitted(&self, u: f32, v: f32, p: &Vec3) -> Color {
        self.tex.value(u, v, p)
    }
}