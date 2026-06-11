use std::sync::Arc;
use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::math::{Ray, Vec3};
use crate::texture::{SolidColorTexture, Texture};

pub struct Isotropic {
    texture: Arc<dyn Texture>
}

impl Isotropic {
    pub fn new(texture: Arc<dyn Texture>) -> Arc<Self> {
        Arc::new(Isotropic { texture })
    }
    
    pub fn from_color(color: Color) -> Arc<Self> {
        Self::new(SolidColorTexture::new(color))
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, hit_rec: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *scattered = Ray::new_t(hit_rec.point, Vec3::random_unit_vector(), r_in.time());
        *attenuation = self.texture.value(hit_rec.u, hit_rec.v, &hit_rec.point);
        true
    }

    fn emitted(&self, _u: f32, _v: f32, _p: &Vec3) -> Color {
        Color::BLACK
    }
}
