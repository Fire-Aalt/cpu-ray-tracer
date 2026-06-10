use std::sync::Arc;
use crate::color::Color;
use crate::texture::{SolidColorTexture, Texture};
use crate::vec3::Vec3;

pub struct CheckerTexture {
    inv_scale: f32,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>
}

impl CheckerTexture {
    pub fn new(scale: f32, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        CheckerTexture {
            inv_scale: 1.0 / scale,
            even,
            odd
        }
    }
    
    pub fn from_colors(scale: f32, even: Color, odd: Color) -> Self {
        CheckerTexture::new(scale, Arc::new(SolidColorTexture::new(even)), Arc::new(SolidColorTexture::new(odd)))
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Color {
        let x_int = (self.inv_scale * p.x).floor() as i32;
        let y_int = (self.inv_scale * p.y).floor() as i32;
        let z_int = (self.inv_scale * p.z).floor() as i32;
        
        let is_even = (x_int + y_int + z_int) % 2 == 0;
        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}