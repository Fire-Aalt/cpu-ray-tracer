use std::cmp::max;
use std::rc::Rc;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{dot, Vec3};

pub struct Sphere {
    center: Ray,
    radius: f32,
    material: Rc<dyn Material>
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, mat: Rc<dyn Material>) -> Self {
        Sphere { center: Ray::new(center, Vec3::default()), radius: radius.max(0.001), material: mat }
    }
    
    pub fn new_dynamic(center1: Vec3, center2: Vec3, radius: f32, mat: Rc<dyn Material>) -> Self {
        Sphere { center: Ray::new(center1, center2 - center1), radius: radius.max(0.001), material: mat }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_i: Interval, record: &mut HitRecord) -> bool {
        let current_center = self.center.at(ray.time());
        let oc = current_center - *ray.origin();
        
        let a = ray.direction().length_squared();
        let h = dot(ray.direction(), &oc);
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
        let outward_normal = (record.point - current_center) / self.radius;
        record.set_face_normal(ray, &outward_normal);
        record.material = Some(self.material.clone());
        
        true
    }
}