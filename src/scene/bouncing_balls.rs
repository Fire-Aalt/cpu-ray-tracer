use crate::color::Color;
use crate::hittable::Sphere;
use crate::material::{Dialectric, Lambertian, Metal};
use crate::scene::Scene;
use crate::texture::CheckerTexture;
use crate::utils::{random_f32, random_range_f32};
use crate::vec3::Vec3;
use std::sync::Arc;

pub fn build(scene: &mut Scene) {

    let checker = Arc::new(Lambertian::from_texture(Arc::new(CheckerTexture::from_colors(0.32, Color::new(0.1, 0.1, 0.1), Color::new(0.9, 0.9, 0.9)))));
    scene.add(Arc::new(Sphere::new(Vec3::new(0.0,-1000.0,0.0), 1000.0, checker)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f32();
            let center = Vec3::new(a as f32 + 0.9*random_f32(), 0.2, b as f32 + 0.9*random_f32());

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::random().as_color() * Vec3::random().as_color();
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    let center2 = center + Vec3::new(0.0, random_range_f32(0.0..0.5), 0.0);
                    scene.add(Arc::new(Sphere::new_dynamic(center, center2, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_range(0.5..1.0).as_color();
                    let fuzz = random_range_f32(0.0..0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    scene.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Arc::new(Dialectric::new(1.5));
                    scene.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dialectric::new(1.5));
    scene.add(Arc::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    scene.add(Arc::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    scene.add(Arc::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3)));

    let cam = &mut scene.camera;
    cam.aspect_ratio      = 16.0 / 9.0;
    cam.image_width       = 720;
    cam.samples_per_pixel = 100;
    cam.max_depth         = 32;

    cam.v_fov     = 20.0;
    cam.look_from = Vec3::new(13.0,2.0,3.0);
    cam.look_at   = Vec3::new(0.0,0.0,0.0);
    cam.v_up      = Vec3::new(0.0,1.0,0.0);

    cam.defocus_angle = 0.6;
    cam.focus_distance    = 10.0;
}
