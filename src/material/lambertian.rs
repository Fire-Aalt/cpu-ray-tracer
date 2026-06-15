use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::{Material, ScatterRecord};
use crate::math::{dot, Ray};
use crate::pdf::CosinePDF;
use crate::texture::{SolidColorTexture, Texture};
use std::f32::consts::PI;
use std::sync::Arc;

pub struct Lambertian {
    texture: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(texture: Arc<dyn Texture>) -> Arc<Self> {
        Arc::new(Lambertian { texture })
    }
    
    pub fn from_color(albedo: Color) -> Arc<Self> {
        Arc::new(Lambertian { texture: SolidColorTexture::new(albedo) })
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, hit_rec: &HitRecord, scatter_record: &mut ScatterRecord) -> bool {
        scatter_record.attenuation = self.texture.value(hit_rec.u, hit_rec.v, &hit_rec.point);
        scatter_record.pdf = Some(Arc::new(CosinePDF::new(hit_rec.normal)));
        true
    }

    fn scattering_pdf(&self, r_in: &Ray, hit_rec: &HitRecord, scattered: &Ray) -> f32 {
        let cos_theta = dot(&hit_rec.normal, &scattered.direction().unit_vector());
        if cos_theta < 0.0 { 0.0 } else { cos_theta / PI }
    }
}
