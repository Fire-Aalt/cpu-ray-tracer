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
    fn scatter(&self, r_in: &Ray, hit_rec: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;

    fn emitted(&self, u: f32, v: f32, p: &Vec3) -> Color;
}
