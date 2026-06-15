use crate::math::Vec3;
use crate::pdf::PDF;
use std::f32::consts::PI;

pub struct SpherePDF {}

impl SpherePDF {
    pub fn new() -> Self {
        Self {}
    }
}

impl PDF for SpherePDF {
    fn value(&self, direction: &Vec3) -> f32 {
        1.0 / (4.0 * PI)
    }

    fn generate(&self) -> Vec3 {
        Vec3::random_unit_vector()
    }
}
