use std::sync::Arc;
use crate::color::Color;
use crate::math::Vec3;

use super::Texture;

#[derive(Clone)]
pub struct SolidColorTexture {
    albedo: Color,
}

impl SolidColorTexture {
    pub fn new(albedo: Color) -> Arc<Self> {
        Arc::new(SolidColorTexture { albedo })
    }
}

impl Texture for SolidColorTexture {
    fn value(&self, _u: f32, _v: f32, _p: &Vec3) -> Color {
        self.albedo
    }
}
