use crate::material::Material;

mod bvh_node;
mod list;
mod sphere;
mod quad;

pub use bvh_node::BVHNode;
pub use list::HittableList;
pub use quad::Quad;
pub use sphere::Sphere;
use crate::math::{dot, Interval, Ray, Vec3, AABB};

#[derive(Clone, Default)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub u: f32,
    pub v: f32,
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

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, ray_i: Interval, record: &mut HitRecord) -> Option<&dyn Material>;
    
    fn bounding_box(&self) -> AABB;
}
