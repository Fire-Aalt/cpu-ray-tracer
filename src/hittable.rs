use std::rc::Rc;
use crate::aabb::AABB;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{dot, Vec3};

#[derive(Clone, Default)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Option<Rc<dyn Material>>,
    pub t: f32,
    pub front_face: bool
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        //assert!(1.0 - outward_normal.length() < 0.0001);
        
        self.front_face = dot(ray.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_i: Interval, record: &mut HitRecord) -> bool;
    
    fn bounding_box(&self) -> AABB;
}

