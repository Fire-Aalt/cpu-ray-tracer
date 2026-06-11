use std::sync::Arc;
use crate::color::Color;
use crate::hittable::{Quad, Sphere};
use crate::material::{DiffuseLight, Lambertian};
use crate::scene::Scene;
use crate::texture::CheckerTexture;
use crate::vec3::Vec3;

pub fn build(scene: &mut Scene) {
    let pertext = Arc::new(CheckerTexture::from_colors(1.0, Color::WHITE, Color::BLACK));
    scene.add(Arc::new(Sphere::new(Vec3::new(0.0,-1000.0,0.0), 1000.0, Arc::new(Lambertian::from_texture(pertext.clone())))));
    scene.add(Arc::new(Sphere::new(Vec3::new(0.0,2.0,0.0), 2.0, Arc::new(Lambertian::from_texture(pertext.clone())))));

    let difflight = Arc::new(DiffuseLight::from_color(Color::new(4.0,4.0,4.0)));
    scene.add(Arc::new(Sphere::new(Vec3::new(0.0,7.0,0.0), 2.0, difflight.clone())));
    scene.add(Arc::new(Quad::new(Vec3::new(3.0,1.0,-2.0), Vec3::new(2.0,0.0,0.0), Vec3::new(0.0,2.0,0.0), difflight)));

    let cam = &mut scene.camera;
    cam.aspect_ratio      = 16.0 / 9.0;
    cam.image_width       = 720;
    cam.samples_per_pixel = 516;
    cam.max_depth         = 50;
    cam.background        = Color::BLACK;

    cam.v_fov     = 20.0;
    cam.look_from = Vec3::new(26.0,3.0,6.0);
    cam.look_at   = Vec3::new(0.0,2.0,0.0);
    cam.v_up      = Vec3::new(0.0,1.0,0.0);
}