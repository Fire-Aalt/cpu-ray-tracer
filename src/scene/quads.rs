use std::sync::Arc;
use crate::color::Color;
use crate::hittable::Quad;
use crate::material::Lambertian;
use crate::scene::Scene;
use crate::vec3::Vec3;

pub fn build(scene: &mut Scene) {
    // Materials
    let left_red     = Lambertian::from_color(Color::new(1.0, 0.2, 0.2));
    let back_green   = Lambertian::from_color(Color::new(0.2, 1.0, 0.2));
    let right_blue   = Lambertian::from_color(Color::new(0.2, 0.2, 1.0));
    let upper_orange = Lambertian::from_color(Color::new(1.0, 0.5, 0.0));
    let lower_teal   = Lambertian::from_color(Color::new(0.2, 0.8, 0.8));

    // Quads
    scene.add(Quad::new(Vec3::new(-3.0,-2.0, 5.0), Vec3::new(0.0, 0.0,-4.0), Vec3::new(0.0, 4.0, 0.0), left_red));
    scene.add(Quad::new(Vec3::new(-2.0,-2.0, 0.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 4.0, 0.0), back_green));
    scene.add(Quad::new(Vec3::new( 3.0,-2.0, 1.0), Vec3::new(0.0, 0.0, 4.0), Vec3::new(0.0, 4.0, 0.0), right_blue));
    scene.add(Quad::new(Vec3::new(-2.0, 3.0, 1.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 4.0), upper_orange));
    scene.add(Quad::new(Vec3::new(-2.0,-3.0, 5.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 0.0,-4.0), lower_teal));

    scene.camera.aspect_ratio      = 1.0;
    scene.camera.image_width       = 400;
    scene.camera.samples_per_pixel = 100;
    scene.camera.max_depth         = 50;

    scene.camera.v_fov     = 80.0;
    scene.camera.look_from = Vec3::new(0.0,0.0,9.0);
    scene.camera.look_at   = Vec3::new(0.0,0.0,0.0);
    scene.camera.v_up      = Vec3::new(0.0,1.0,0.0);

    scene.camera.defocus_angle = 0.0;
}