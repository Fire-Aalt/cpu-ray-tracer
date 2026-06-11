use std::path::Path;
use std::sync::Arc;
use crate::color::Color;
use crate::image::Image;
use crate::math::Vec3;
use crate::texture::Texture;

pub struct ImageTexture {
    image: Image
}

impl ImageTexture {
    pub fn new(path: &Path) -> Arc<Self> {
        Arc::new(ImageTexture {
            image: Image::new(path)
        })
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, _p: &Vec3) -> Color {
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);
        
        let i = (u * self.image.width() as f32) as u32;
        let j = (v * self.image.height() as f32) as u32;
        let pixel = self.image.sample(i, j);
        
        const COLOR_SCALE: f32 = 1.0 / 255.0;
        Color::new(COLOR_SCALE * pixel.0 as f32,
                   COLOR_SCALE * pixel.1 as f32,
                   COLOR_SCALE * pixel.2 as f32 )
    }
}