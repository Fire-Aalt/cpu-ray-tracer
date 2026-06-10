use crate::color::Color;
use crate::vec3::Vec3;

mod solid_color_texture;
mod checker_texture;
mod image_texture;

pub trait Texture: Send + Sync {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Color;
}

pub use solid_color_texture::SolidColorTexture;
pub use checker_texture::CheckerTexture;
pub use image_texture::ImageTexture;