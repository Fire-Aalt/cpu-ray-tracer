use crate::math::Vec3;

#[derive(Clone, Copy, Default)]
pub struct Ray {
    origin: Vec3,
    dir: Vec3,
    time: f32,
}

impl Ray {
    pub fn new_t(origin: Vec3, direction: Vec3, time: f32) -> Self {
        Ray { origin, dir: direction, time }
    }

    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, dir: direction, time: 0.0 }
    }
    
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.dir * t
    }
    
    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }
    
    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }
    
    pub fn time(&self) -> f32 {
        self.time
    }
}