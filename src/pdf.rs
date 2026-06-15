mod cosine_pdf;
mod hittable_pdf;
mod mixture_pdf;
mod sphere_pdf;

use crate::math::Vec3;

pub use cosine_pdf::*;
pub use hittable_pdf::*;
pub use mixture_pdf::*;
pub use sphere_pdf::*;

pub trait PDF {
    fn value(&self, direction: &Vec3) -> f32 {
        0.0
    }

    fn generate(&self) -> Vec3 {
        Vec3::default()
    }
}
