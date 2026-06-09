mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod utils;
mod interval;
mod camera;
mod material;
mod lambertian;
mod metal;
mod dialectric;
mod aabb;
mod bvh_node;

use crate::camera::Camera;
use crate::color::Color;
use crate::dialectric::Dialectric;
use crate::hittable_list::HittableList;
use crate::lambertian::Lambertian;
use crate::metal::Metal;
use crate::sphere::Sphere;
use crate::utils::{random_f32, random_range_f32};
use crate::vec3::Vec3;
use std::rc::Rc;
use tracing::{debug_span, span, Level};
use crate::bvh_node::BVHNode;

use tracing_subscriber::layer::SubscriberExt;

fn main() {
    tracing::subscriber::set_global_default(
        tracing_subscriber::registry().with(tracing_tracy::TracyLayer::default())
    ).expect("setup tracy layer");
    let _client = tracy_client::Client::start();
    
    let mut world = HittableList::new();
    
    // let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    // let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    // let material_left = Rc::new(Dialectric::new(1.5));
    // let material_bubble = Rc::new(Dialectric::new(1.0 / 1.5));
    // let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));
    // 
    // world.add(Box::new(Sphere::new(Vec3::new( 0.0, -100.5, -1.0), 100.0, material_ground)));
    // world.add(Box::new(Sphere::new(Vec3::new( 0.0,    0.0, -1.2),   0.5, material_center)));
    // world.add(Box::new(Sphere::new(Vec3::new(-1.0,    0.0, -1.0),   0.5, material_left)));
    // world.add(Box::new(Sphere::new(Vec3::new(-1.0,    0.0, -1.0),   0.4, material_bubble)));
    // world.add(Box::new(Sphere::new(Vec3::new( 1.0,    0.0, -1.0),   0.5, material_right)));


    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(Vec3::new(0.0,-1000.0,0.0), 1000.0, ground_material)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f32();
            let center = Vec3::new(a as f32 + 0.9*random_f32(), 0.2, b as f32 + 0.9*random_f32());

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::random().as_color() * Vec3::random().as_color();
                    let sphere_material = Rc::new(Lambertian::new(albedo));
                    let center2 = center + Vec3::new(0.0, random_range_f32(0.0..0.5), 0.0);
                    world.add(Rc::new(Sphere::new_dynamic(center, center2, 0.2, sphere_material)));
                } else if (choose_mat < 0.95) {
                    // metal
                    let albedo = Vec3::random_range(0.5..1.0).as_color();
                    let fuzz = random_range_f32(0.0..0.5);
                    let sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Rc::new(Dialectric::new(1.5));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Rc::new(Dialectric::new(1.5));
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3)));

    // This line enables BVH acceleration.
    //let world = HittableList::from_one(Rc::new(BVHNode::new_root(&mut world.objects)));
    
    let mut cam = Camera::default();
    cam.aspect_ratio      = 16.0 / 9.0;
    cam.image_width       = 400;
    cam.samples_per_pixel = 20;
    cam.max_depth         = 50;

    cam.v_fov     = 20.0;
    cam.look_from = Vec3::new(13.0,2.0,3.0);
    cam.look_at   = Vec3::new(0.0,0.0,0.0);
    cam.v_up      = Vec3::new(0.0,1.0,0.0);

    cam.defocus_angle = 0.6;
    cam.focus_distance    = 10.0;
    
    span!(Level::TRACE, "render").in_scope(|| {
    cam.render(&world);
    })
}
