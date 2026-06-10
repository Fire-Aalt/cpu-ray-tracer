mod bouncing_balls;
mod checker_spheres;

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
    CheckerSpheres
}

pub fn build_scene(id: SceneId, scene: &mut Scene) {
    match id {
        SceneId::BouncingBalls => bouncing_balls::build(scene),
        SceneId::CheckerSpheres => checker_spheres::build(scene),
    }
}
