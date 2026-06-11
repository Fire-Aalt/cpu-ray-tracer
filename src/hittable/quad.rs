use std::sync::Arc;
use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable, HittableList};
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
    pub fn new(q: Vec3, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Arc<Self> {
        let n = cross(&u, &v);
        let normal = n.unit_vector();
        let d = dot(&normal, &q);
        let w = n / dot(&n, &n);
        
        let bbox_diagonal1 = AABB::from_min_max(q, q + u + v);
        let bbox_diagonal2 = AABB::from_min_max(q + u, q + v);
        
        Arc::new(Quad { q, u, v, w, mat, bbox: AABB::from_combined(bbox_diagonal1, bbox_diagonal2), normal, d })
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
    
    pub fn r#box(a: Vec3, b: Vec3, mat: Arc<dyn Material>) -> Arc<HittableList> {
        let mut sides = HittableList::new();
        
        let min = Vec3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
        let max = Vec3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));
        
        let dx = Vec3::new(max.x - min.x, 0.0, 0.0);
        let dy = Vec3::new(0.0, max.y - min.y, 0.0);
        let dz = Vec3::new(0.0, 0.0, max.z - min.z);
        
        sides.add(Quad::new(Vec3::new(min.x, min.y, max.z), dx, dy, mat.clone()));
        sides.add(Quad::new(Vec3::new(max.x, min.y, max.z), -dz, dy, mat.clone()));
        sides.add(Quad::new(Vec3::new(max.x, min.y, min.z), -dx, dy, mat.clone()));

        sides.add(Quad::new(Vec3::new(min.x, min.y, min.z), dz, dy, mat.clone()));
        sides.add(Quad::new(Vec3::new(min.x, max.y, max.z), dx, -dz, mat.clone()));
        sides.add(Quad::new(Vec3::new(min.x, min.y, min.z), dx, dz, mat.clone()));

        Arc::new(sides)
    }
}


impl Hittable for Quad {
    fn hit(&self, ray: &Ray, ray_i: Interval, record: &mut HitRecord) -> Option<&dyn Material> {
        let denominator = dot(&self.normal, ray.direction());

        // No hit if the ray is parallel to the plane.
        if denominator.abs() < f32::EPSILON { 
            return None
        }

        // Return false if the hit point parameter t is outside the ray interval.
        let t = (self.d - dot(&self.normal, ray.origin())) / denominator;
        if !ray_i.contains(t) { 
            return None
        }

        // Determine if the hit point lies within the planar shape using its plane coordinates.
        let intersection = ray.at(t);
        let planar_hit_point = intersection - self.q;
        let alpha = dot(&self.w, &cross(&planar_hit_point, &self.v));
        let beta = dot(&self.w, &cross(&self.u, &planar_hit_point));
        
        if !Self::is_interior(alpha, beta, record) {
            return None
        }
        
        // Ray hits the 2D shape; set the rest of the hit record and return true.
        record.t = t;
        record.point = intersection;
        record.set_face_normal(ray, &self.normal);

        Some(self.mat.as_ref())
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

