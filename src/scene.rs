mod bouncing_balls;
mod checker_spheres;
mod earth;
mod quads;
mod simple_light;
mod cornell_box;

use std::sync::Arc;
use crate::camera::Camera;
use crate::hittable::{Hittable, HittableList};

pub struct Scene {
    pub camera: Camera,
    pub world: HittableList,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            camera: Camera::default(),
            world: HittableList::new(),
        }
    }
    
    pub fn add(&mut self, hittable: Arc<dyn Hittable>) {
        self.world.add(hittable);
    }
}

pub enum SceneId {
    BouncingBalls,
    CheckerSpheres,
    Earth,
    Quads,
    SimpleLight,
    CornellBox,
}

pub fn build_scene(id: SceneId, scene: &mut Scene) {
    match id {
        SceneId::BouncingBalls => bouncing_balls::build(scene),
        SceneId::CheckerSpheres => checker_spheres::build(scene),
        SceneId::Earth => earth::build(scene),
        SceneId::Quads => quads::build(scene),
        SceneId::SimpleLight => simple_light::build(scene),
        SceneId::CornellBox => cornell_box::build(scene),
    }
}
