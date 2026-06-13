use crate::color::Color;
use crate::hittable::HitRecord;

mod dialectric;
mod lambertian;
mod metal;
mod diffuse_light;
mod isotropic;

pub use dialectric::*;
pub use lambertian::*;
pub use metal::*;
pub use diffuse_light::*;
pub use isotropic::*;
use crate::math::{Ray, Vec3};

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, hit_rec: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray, pdf: &mut f32) -> bool {
        false
    }

    fn scattering_pdf(&self, r_in: &Ray, hit_rec: &HitRecord, scattered: &Ray) -> f32 {
        0.0
    }
    
    fn emitted(&self, u: f32, v: f32, p: &Vec3) -> Color {
        Color::BLACK
    }
}
