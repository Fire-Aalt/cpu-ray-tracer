use crate::color::Color;
use crate::hittable::Quad;
use crate::material::{DiffuseLight, Lambertian};
use crate::math::{Quaternion, Transform, Vec3};
use crate::scene::Scene;
use crate::utils::degrees_to_radians;

pub fn build(scene: &mut Scene) {
    let red = Lambertian::from_color(Color::new(0.65, 0.05, 0.05));
    let white = Lambertian::from_color(Color::new(0.73, 0.73, 0.73));
    let green = Lambertian::from_color(Color::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::from_color(Color::new(15.0, 15.0, 15.0));

    scene.add(Quad::from_edges(
        Vec3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light,
    ));
    scene.add(Quad::from_edges(
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    ));
    scene.add(Quad::from_edges(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    ));
    scene.add(Quad::from_edges(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    ));
    scene.add(Quad::from_edges(
        Vec3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    ));
    scene.add(Quad::from_edges(
        Vec3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    ));

    scene.add(Quad::r#box(
        Transform::from_trs(
            Vec3::new(366.041_44, 165.0, 353.336_3),
            Quaternion::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), degrees_to_radians(15.0)),
            Vec3::new(165.0, 330.0, 165.0),
        ),
        white.clone(),
    ));
    scene.add(Quad::r#box(
        Transform::from_trs(
            Vec3::new(182.968_26, 82.5, 168.956_07),
            Quaternion::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), degrees_to_radians(-18.0)),
            Vec3::new(165.0, 165.0, 165.0),
        ),
        white.clone(),
    ));

    let cam = &mut scene.camera;
    cam.aspect_ratio = 1.0;
    cam.image_width = 600;
    cam.samples_per_pixel = 100;
    cam.max_depth = 32;
    cam.background = Color::BLACK;

    cam.v_fov = 40.0;
    cam.look_from = Vec3::new(278.0, 278.0, -800.0);
    cam.look_at = Vec3::new(278.0, 278.0, 0.0);
}
