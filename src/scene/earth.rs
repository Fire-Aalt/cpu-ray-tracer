use std::path::Path;
use std::sync::Arc;
use crate::hittable::Sphere;
use crate::material::Lambertian;
use crate::scene::Scene;
use crate::texture::ImageTexture;
use crate::vec3::Vec3;

pub fn build(scene: &mut Scene) {
    let earth_texture = Arc::new(ImageTexture::new(Path::new("images/earthmap.jpg")));
    let earth_surface = Arc::new(Lambertian::from_texture(earth_texture));
  
    scene.add(Sphere::new(Vec3::default(), 2.0, earth_surface));
    
    scene.camera.aspect_ratio      = 16.0 / 9.0;
    scene.camera.image_width       = 400;
    scene.camera.samples_per_pixel = 100;
    scene.camera.max_depth         = 50;

    scene.camera.v_fov     = 20.0;
    scene.camera.look_from = Vec3::new(0.0,12.0,0.0);
    scene.camera.look_at   = Vec3::new(0.0,0.0,0.0);
    scene.camera.v_up      = Vec3::new(0.0,0.0,-1.0);

    scene.camera.defocus_angle = 0.0;
}
