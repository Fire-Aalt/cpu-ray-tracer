use std::sync::Arc;
use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::texture::{SolidColorTexture, Texture};
use crate::vec3::Vec3;

pub struct Lambertian {
    tex: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { tex: Arc::new(SolidColorTexture::new(albedo)) }
    }

    pub fn from_texture(texture: Arc<dyn Texture>) -> Self {
        Lambertian { tex: texture }
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
        *attenuation = self.tex.value(hit_rec.u, hit_rec.v, &hit_rec.point);
        true
    }
}