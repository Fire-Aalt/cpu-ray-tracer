use std::path::Path;
use std::sync::Arc;

use crate::color::Color;
use crate::hittable::{BVHNode, ConstantMedium, Hittable, Quad, Sphere};
use crate::material::{Dialectric, DiffuseLight, Lambertian, Metal};
use crate::math::{Quaternion, Transform, Vec3};
use crate::scene::Scene;
use crate::texture::{CheckerTexture, ImageTexture, SolidColorTexture};
use crate::utils::{degrees_to_radians, random_range_f32};

pub fn build(scene: &mut Scene) {
    let ground = Lambertian::from_color(Color::new(0.48, 0.83, 0.53));

    const BOXES_PER_SIDE: i32 = 20;
    const BOX_WIDTH: f32 = 100.0;
    for i in 0..BOXES_PER_SIDE {
        for j in 0..BOXES_PER_SIDE {
            let x0 = -1000.0 + i as f32 * BOX_WIDTH;
            let z0 = -1000.0 + j as f32 * BOX_WIDTH;
            let height = random_range_f32(1.0..101.0);
            scene.add(Quad::r#box(
                Transform::from_trs(
                    Vec3::new(x0 + BOX_WIDTH * 0.5, height * 0.5, z0 + BOX_WIDTH * 0.5),
                    Quaternion::IDENTITY,
                    Vec3::new(BOX_WIDTH, height, BOX_WIDTH),
                ),
                ground.clone(),
            ));
        }
    }

    let light = DiffuseLight::from_color(Color::new(7.0, 7.0, 7.0));
    scene.add_light(Quad::from_edges(
        Vec3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        light,
    ));

    let center1 = Vec3::new(400.0, 400.0, 200.0);
    scene.add(Sphere::new_dynamic(
        center1,
        center1 + Vec3::new(30.0, 0.0, 0.0),
        50.0,
        Lambertian::from_color(Color::new(0.7, 0.3, 0.1)),
    ));

    scene.add(Sphere::new(
        Vec3::new(260.0, 150.0, 45.0),
        50.0,
        Dialectric::new(1.5),
    ));
    scene.add(Sphere::new(
        Vec3::new(0.0, 150.0, 145.0),
        50.0,
        Metal::new(Color::new(0.8, 0.8, 0.9), 1.0),
    ));

    let blue_boundary: Arc<dyn Hittable> =
        Sphere::new(Vec3::new(360.0, 150.0, 145.0), 70.0, Dialectric::new(1.5));
    scene.add(blue_boundary.clone());
    scene.add(ConstantMedium::new(
        blue_boundary,
        0.2,
        SolidColorTexture::new(Color::new(0.2, 0.4, 0.9)),
    ));

    let atmosphere: Arc<dyn Hittable> = Sphere::new(Vec3::default(), 5000.0, Dialectric::new(1.5));
    scene.add(ConstantMedium::new(
        atmosphere,
        0.0001,
        SolidColorTexture::new(Color::WHITE),
    ));

    scene.add(Sphere::new(
        Vec3::new(400.0, 200.0, 400.0),
        100.0,
        Lambertian::new(ImageTexture::new(Path::new("images/earthmap.jpg"))),
    ));
    scene.add(Sphere::new(
        Vec3::new(220.0, 280.0, 300.0),
        80.0,
        Lambertian::new(CheckerTexture::from_colors(
            25.0,
            Color::BLACK,
            Color::WHITE,
        )),
    ));

    let white = Lambertian::from_color(Color::new(0.73, 0.73, 0.73));
    let cluster_transform = Transform::from_trs(
        Vec3::new(-100.0, 270.0, 395.0),
        Quaternion::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), degrees_to_radians(15.0)),
        Vec3::new(1.0, 1.0, 1.0),
    );

    for _ in 0..1000 {
        let center = cluster_transform * Vec3::random_range(0.0..165.0);
        scene.add(Sphere::new(center, 10.0, white.clone()));
    }

    let cam = &mut scene.camera;
    cam.aspect_ratio = 1.0;
    cam.image_width = 1440;
    cam.samples_per_pixel = 500;
    cam.max_depth = 40;
    cam.background = Color::BLACK;

    cam.v_fov = 40.0;
    cam.look_from = Vec3::new(478.0, 278.0, -600.0);
    cam.look_at = Vec3::new(278.0, 278.0, 0.0);
    cam.v_up = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;
}