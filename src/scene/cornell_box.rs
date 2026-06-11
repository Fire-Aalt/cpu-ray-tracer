use crate::color::Color;
use crate::hittable::Quad;
use crate::material::{DiffuseLight, Lambertian};
use crate::scene::Scene;
use crate::vec3::Vec3;

pub fn build(scene: &mut Scene) {
    let red   = Lambertian::from_color(Color::new(0.65, 0.05, 0.05));
    let white = Lambertian::from_color(Color::new(0.73, 0.73, 0.73));
    let green = Lambertian::from_color(Color::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::from_color(Color::new(15.0, 15.0, 15.0));

    scene.add(Quad::new(Vec3::new(555.0,0.0,0.0), Vec3::new(0.0,555.0,0.0), Vec3::new(0.0,0.0,555.0), green));
    scene.add(Quad::new(Vec3::new(0.0,0.0,0.0), Vec3::new(0.0,555.0,0.0), Vec3::new(0.0,0.0,555.0), red));
    scene.add(Quad::new(Vec3::new(343.0, 554.0, 332.0), Vec3::new(-130.0,0.0,0.0), Vec3::new(0.0,0.0,-105.0), light));
    scene.add(Quad::new(Vec3::new(0.0,0.0,0.0), Vec3::new(555.0,0.0,0.0), Vec3::new(0.0,0.0,555.0), white.clone()));
    scene.add(Quad::new(Vec3::new(555.0,555.0,555.0), Vec3::new(-555.0,0.0,0.0), Vec3::new(0.0,0.0,-555.0), white.clone()));
    scene.add(Quad::new(Vec3::new(0.0,0.0,555.0), Vec3::new(555.0,0.0,0.0), Vec3::new(0.0,555.0,0.0), white));

    let cam = &mut scene.camera;
    cam.aspect_ratio      = 1.0;
    cam.image_width       = 600;
    cam.samples_per_pixel = 200;
    cam.max_depth         = 50;
    cam.background        = Color::BLACK;

    cam.v_fov     = 40.0;
    cam.look_from = Vec3::new(278.0, 278.0, -800.0);
    cam.look_at   = Vec3::new(278.0, 278.0, 0.0);
}