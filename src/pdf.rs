mod sphere_pdf;
mod CosinePDF;

use crate::math::Vec3;

pub use sphere_pdf::*;

pub trait PDF {
    fn value(&self, direction: &Vec3) -> f32 {
        0.0
    }
    
    fn generate(&self) -> Vec3 {
        Vec3::default()
    }
}

