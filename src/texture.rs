use crate::color::Color;
use crate::vec3::Vec3;

mod solid_color_texture;

pub trait Texture {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Color;
}

pub use solid_color_texture::SolidColorTexture;
