use std::sync::Arc;
use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{cross, dot, Vec3};

pub struct Quad {
    q: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Arc<dyn Material>,
    bbox: AABB,
    normal: Vec3,
    d: f32
}

impl Quad {
    pub fn new(q: Vec3, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Self {
        let n = cross(&u, &v);
        let normal = n.unit_vector();
        let d = dot(&normal, &q);
        let w = n / dot(&n, &n);
        
        let bbox_diagonal1 = AABB::from_min_max(q, q + u + v);
        let bbox_diagonal2 = AABB::from_min_max(q + u, q + v);
        
        Quad { q, u, v, w, mat, bbox: AABB::from_combined(bbox_diagonal1, bbox_diagonal2), normal, d }
    }
    
    fn is_interior(a: f32, b: f32, hit_rec: &mut HitRecord) -> bool {
        const UNIT_INTERVAL: Interval = Interval::new(0.0, 1.0);
        // Given the hit point in plane coordinates, return false if it is outside the
        // primitive, otherwise set the hit record UV coordinates and return true.
        
        if !UNIT_INTERVAL.contains(a) || !UNIT_INTERVAL.contains(b) { 
            return false
        }
        
        hit_rec.u = a;
        hit_rec.v = b;
        true
    }
}


impl Hittable for Quad {
    fn hit(&self, ray: &Ray, ray_i: Interval, record: &mut HitRecord) -> bool {
        let denominator = dot(&self.normal, ray.direction());

        // No hit if the ray is parallel to the plane.
        if denominator.abs() < f32::EPSILON { 
            return false
        }

        // Return false if the hit point parameter t is outside the ray interval.
        let t = (self.d - dot(&self.normal, ray.origin())) / denominator;
        if !ray_i.contains(t) { 
            return false
        }

        // Determine if the hit point lies within the planar shape using its plane coordinates.
        let intersection = ray.at(t);
        let planar_hit_point = intersection - self.q;
        let alpha = dot(&self.w, &cross(&planar_hit_point, &self.v));
        let beta = dot(&self.w, &cross(&self.u, &planar_hit_point));
        
        if !Self::is_interior(alpha, beta, record) {
            return false;
        }
        
        // Ray hits the 2D shape; set the rest of the hit record and return true.
        record.t = t;
        record.point = intersection;
        record.set_face_normal(ray, &self.normal);
        record.material = Some(self.mat.clone());
        
        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}