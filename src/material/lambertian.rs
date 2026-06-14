use std::f32::consts::PI;
use std::sync::Arc;
use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::math::{dot, Ray, Vec3, ONB};
use crate::texture::{SolidColorTexture, Texture};

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
    fn scatter(&self, r_in: &Ray, hit_rec: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray, pdf: &mut f32) -> bool {
        let uvw = ONB::new(hit_rec.normal);
        let scatter_direction = uvw.transform(&Vec3::random_cosine_direction());
        
        *scattered = Ray::new_t(hit_rec.point, scatter_direction.unit_vector(), r_in.time());
        *attenuation = self.texture.value(hit_rec.u, hit_rec.v, &hit_rec.point);
        *pdf = dot(&uvw.w, scattered.direction()) / PI;
        true
    }

    fn scattering_pdf(&self, r_in: &Ray, hit_rec: &HitRecord, scattered: &Ray) -> f32 {
        //let cos_theta = dot(&hit_rec.normal, &scattered.direction().unit_vector());
        //if cos_theta < 0.0 { 0.0 } else { cos_theta / PI }
        1.0 / (2.0 * PI)
    }
}