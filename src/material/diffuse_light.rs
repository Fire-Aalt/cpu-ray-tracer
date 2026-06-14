use std::sync::Arc;
use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::math::{Ray, Vec3};
use crate::texture::{SolidColorTexture, Texture};

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
    fn emitted(&self, r_in: &Ray, hit_rec: &HitRecord, u: f32, v: f32, p: &Vec3) -> Color {
        if !hit_rec.front_face { 
            return Color::BLACK;
        }
        self.tex.value(u, v, p)
    }
}