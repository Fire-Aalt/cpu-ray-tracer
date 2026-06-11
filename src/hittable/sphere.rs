use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{dot, Vec3};
use std::f32::consts::PI;
use std::sync::Arc;

pub struct Sphere {
    center: Ray,
    radius: f32,
    material: Arc<dyn Material>,
    bbox: AABB
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, mat: Arc<dyn Material>) -> Self {
        let r_vec = Vec3::new(radius, radius, radius);
        
        Sphere { 
            center: Ray::new(center, Vec3::default()), 
            radius: radius.max(0.001), 
            material: mat, 
            bbox: AABB::from_min_max(center - r_vec, center + r_vec)
        }
    }
    
    pub fn new_dynamic(center1: Vec3, center2: Vec3, radius: f32, mat: Arc<dyn Material>) -> Self {
        let r_vec = Vec3::new(radius, radius, radius);
        let box1 = AABB::from_min_max(center1 - r_vec, center1 + r_vec);
        let box2 = AABB::from_min_max(center2 - r_vec, center2 + r_vec);
        
        Sphere { 
            center: Ray::new(center1, center2 - center1), 
            radius: radius.max(0.001), 
            material: mat,
            bbox: AABB::from_combined(box1, box2)
        }
    }
    
    pub fn bounding_box(&self) -> &AABB {
        &self.bbox
    }

    fn get_sphere_uv(point: &Vec3, u: &mut f32, v: &mut f32) {
        // point: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
        
        let theta = (-point.y).acos();
        let phi = (-point.z).atan2(point.x) + PI;
        
        *u = phi / (2.0 * PI);
        *v = theta / PI;
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_i: Interval, record: &mut HitRecord) -> Option<&dyn Material> {
        let current_center = self.center.at(ray.time());
        let oc = current_center - *ray.origin();
        
        let a = ray.direction().length_squared();
        let h = dot(ray.direction(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None
        }
        
        let sqrt_d = discriminant.sqrt();
        let mut root = (h - sqrt_d) / a;
        if !ray_i.surrounds(root) {
            root = (h + sqrt_d) / a;
            if !ray_i.surrounds(root) {
                return None
            }
        }
        
        record.t = root;
        record.point = ray.at(root);
        let outward_normal = (record.point - current_center) / self.radius;
        record.set_face_normal(ray, &outward_normal);
        Self::get_sphere_uv(&outward_normal, &mut record.u, &mut record.v);
        
        Some(self.material.as_ref())
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}