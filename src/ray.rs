use crate::vec3::Vec3;

#[derive(Clone, Copy, Default)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, dir: direction }
    }
    
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.dir * t
    }
}