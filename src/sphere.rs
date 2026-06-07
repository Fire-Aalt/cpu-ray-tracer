use std::cmp::max;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{dot, Vec3};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32 
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Sphere { center, radius: radius.max(0.001) }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_i: Interval, record: &mut HitRecord) -> bool {
        let oc = self.center - ray.origin;
        let a = ray.dir.length_squared();
        let h = dot(&ray.dir, &oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false
        }
        
        let sqrt_d = discriminant.sqrt();
        let mut root = (h - sqrt_d) / a;
        if !ray_i.surrounds(root) {
            root = (h + sqrt_d) / a;
            if !ray_i.surrounds(root) {
                return false
            }
        }
        
        record.t = root;
        record.point = ray.at(root);
        let outward_normal = (record.point - self.center) / self.radius;
        record.set_face_normal(ray, &outward_normal);
        true
    }
}