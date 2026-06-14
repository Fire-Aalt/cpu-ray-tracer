mod color;
mod hittable;
mod utils;
mod camera;
mod material;
mod texture;
mod scene;
mod image;
mod math;
mod pdf;

use crate::hittable::{BVHNode, HittableList};
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
    build_scene(SceneId::CornellBox, &mut scene);
    
    // This line enables BVH acceleration.
    let world = HittableList::from_one(Arc::new(BVHNode::new_root(&mut scene.world.objects)));
    let lights = &scene.lights;

    let cam = &mut scene.camera;
    
    span!(Level::TRACE, "render").in_scope(|| {
        cam.render(&world, lights);
    })
}
