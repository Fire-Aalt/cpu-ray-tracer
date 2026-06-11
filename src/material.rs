use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;

mod dialectric;
mod lambertian;
mod metal;
mod diffuse_light;

pub use dialectric::Dialectric;
pub use lambertian::Lambertian;
pub use metal::Metal;
use crate::vec3::Vec3;

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, hit_rec: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;

    fn emitted(&self, u: f32, v: f32, p: &Vec3) -> Color;
}
