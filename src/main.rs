mod vec3;
mod color;
mod ray;
mod hittable;
mod utils;
mod interval;
mod camera;
mod material;
mod aabb;
mod texture;
mod scene;

use crate::hittable::{BVHNode, HittableList, Sphere};
use crate::material::{Dialectric, Lambertian, Metal};
use crate::utils::{random_f32, random_range_f32};
use crate::vec3::Vec3;
use std::sync::Arc;
use tracing::{span, Level};

use tracing_subscriber::layer::SubscriberExt;
use crate::scene::{build_scene, Scene, SceneId};

fn main() {
    tracing::subscriber::set_global_default(
        tracing_subscriber::registry().with(tracing_tracy::TracyLayer::default())
    ).expect("setup tracy layer");
    let _client = tracy_client::Client::start();
    
    let mut scene = Scene::new();
    build_scene(SceneId::CheckerSpheres, &mut scene);
    
    // This line enables BVH acceleration.
    let world = HittableList::from_one(Arc::new(BVHNode::new_root(&mut scene.world.objects)));
    
    let cam = &mut scene.camera;
    cam.aspect_ratio      = 16.0 / 9.0;
    cam.image_width       = 720;
    cam.samples_per_pixel = 100;
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
