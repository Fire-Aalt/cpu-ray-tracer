use crate::math::{ONB, Vec3, dot};
use crate::pdf::PDF;
use std::f32::consts::PI;

pub struct CosinePDF {
    uvw: ONB,
}

impl CosinePDF {
    pub fn new(w: Vec3) -> Self {
        CosinePDF { uvw: ONB::new(w) }
    }
}

impl PDF for CosinePDF {
    fn value(&self, direction: &Vec3) -> f32 {
        let cosine_theta = dot(&direction.unit_vector(), &self.uvw.w);
        (cosine_theta / PI).max(0.0)
    }

    fn generate(&self) -> Vec3 {
        self.uvw.transform(&Vec3::random_cosine_direction())
    }
}
