use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;

mod dialectric;
mod lambertian;
mod metal;

pub use dialectric::Dialectric;
pub use lambertian::Lambertian;
pub use metal::Metal;

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, hit_rec: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}
