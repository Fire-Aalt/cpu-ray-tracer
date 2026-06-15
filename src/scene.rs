mod bouncing_balls;
mod checker_spheres;
mod cornell_box;
mod cornell_smoke;
mod earth;
mod final_scene;
mod quads;
mod simple_light;

use crate::camera::Camera;
use crate::hittable::{Hittable, HittableList};
use std::sync::Arc;

pub struct Scene {
    pub camera: Camera,
    pub world: HittableList,
    pub lights: HittableList,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            camera: Camera::default(),
            world: HittableList::new(),
            lights: HittableList::new(),
        }
    }

    pub fn add(&mut self, hittable: Arc<dyn Hittable>) {
        self.world.add(hittable);
    }
    pub fn add_light(&mut self, light: Arc<dyn Hittable>) {
        self.lights.add(light);
    }
}

pub enum SceneId {
    BouncingBalls,
    CheckerSpheres,
    Earth,
    Quads,
    SimpleLight,
    CornellBox,
    CornellSmoke,
    FinalScene,
}

pub fn build_scene(id: SceneId, scene: &mut Scene) {
    match id {
        SceneId::BouncingBalls => bouncing_balls::build(scene),
        SceneId::CheckerSpheres => checker_spheres::build(scene),
        SceneId::Earth => earth::build(scene),
        SceneId::Quads => quads::build(scene),
        SceneId::SimpleLight => simple_light::build(scene),
        SceneId::CornellBox => cornell_box::build(scene),
        SceneId::CornellSmoke => cornell_smoke::build(scene),
        SceneId::FinalScene => final_scene::build(scene),
    }
}
