use std::sync::Arc;
use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::texture::{SolidColorTexture, Texture};
use crate::vec3::Vec3;

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
    fn scatter(&self, r_in: &Ray, hit_rec: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = hit_rec.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() { 
            scatter_direction = hit_rec.normal;
        }
        
        *scattered = Ray::new_t(hit_rec.point, scatter_direction, r_in.time());
        *attenuation = self.texture.value(hit_rec.u, hit_rec.v, &hit_rec.point);
        true
    }
    
    fn emitted(&self, _u: f32, _v: f32, _p: &Vec3) -> Color {
        Color::BLACK
    }
}