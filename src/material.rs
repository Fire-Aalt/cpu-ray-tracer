use crate::color::Color;
use crate::hittable::HitRecord;
use std::sync::Arc;

mod dialectric;
mod diffuse_light;
mod isotropic;
mod lambertian;
mod metal;

use crate::math::{Ray, Vec3};
use crate::pdf::PDF;
pub use dialectric::*;
pub use diffuse_light::*;
pub use isotropic::*;
pub use lambertian::*;
pub use metal::*;

#[derive(Default)]
pub struct ScatterRecord {
    pub attenuation: Color,
    pub pdf: Option<Arc<dyn PDF>>,
    pub skip_pdf_ray: Ray,
}

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, hit_rec: &HitRecord, scatter_rec: &mut ScatterRecord) -> bool {
        false
    }

    fn scattering_pdf(&self, r_in: &Ray, hit_rec: &HitRecord, scattered: &Ray) -> f32 {
        0.0
    }

    fn emitted(&self, r_in: &Ray, hit_rec: &HitRecord, u: f32, v: f32, p: &Vec3) -> Color {
        Color::BLACK
    }
}
