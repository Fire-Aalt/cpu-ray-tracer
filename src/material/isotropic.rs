use std::f32::consts::PI;
use std::sync::Arc;
use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::{Material, ScatterRecord};
use crate::math::{Ray, Vec3};
use crate::pdf::{CosinePDF, SpherePDF};
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
    fn scatter(&self, r_in: &Ray, hit_rec: &HitRecord, scatter_record: &mut ScatterRecord) -> bool {
        scatter_record.attenuation = self.texture.value(hit_rec.u, hit_rec.v, &hit_rec.point);
        scatter_record.pdf = Some(Arc::new(SpherePDF::new()));
        true
    }

    fn scattering_pdf(&self, r_in: &Ray, hit_rec: &HitRecord, scattered: &Ray) -> f32 {
        1.0 / (4.0 * PI)
    }
}
