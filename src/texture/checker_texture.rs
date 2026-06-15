use crate::color::Color;
use crate::math::Vec3;
use crate::texture::{SolidColorTexture, Texture};
use std::sync::Arc;

pub struct CheckerTexture {
    inv_scale: f32,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f32, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Arc<Self> {
        Arc::new(CheckerTexture {
            inv_scale: 1.0 / scale,
            even,
            odd,
        })
    }

    pub fn from_colors(scale: f32, even: Color, odd: Color) -> Arc<Self> {
        CheckerTexture::new(
            scale,
            SolidColorTexture::new(even),
            SolidColorTexture::new(odd),
        )
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
