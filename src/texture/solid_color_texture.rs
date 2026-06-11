use std::sync::Arc;
use crate::color::Color;
use crate::vec3::Vec3;

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
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Color {
        self.albedo
    }
}
