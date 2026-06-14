use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::material::Material;
use std::sync::Arc;
use crate::math::{cross, dot, Interval, Quaternion, Ray, Transform, Vec3, AABB};
use crate::utils::random_f32;

pub struct Quad {
    q: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Arc<dyn Material>,
    bbox: AABB,
    normal: Vec3,
    d: f32,
    area: f32
}

impl Quad {
    /// Creates a centered unit XY quad and applies `transform` to it.
    pub fn new(transform: Transform, mat: Arc<dyn Material>) -> Arc<Self> {
        let q = transform.transform_point(Vec3::new(-0.5, -0.5, 0.0));
        let u = transform.transform_vector(Vec3::new(1.0, 0.0, 0.0));
        let v = transform.transform_vector(Vec3::new(0.0, 1.0, 0.0));
        Self::from_world_edges(q, u, v, mat)
    }

    /// Creates a quad from one corner and two full world-space edge vectors.
    pub fn from_edges(q: Vec3, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Arc<Self> {
        Self::from_world_edges(q, u, v, mat)
    }

    fn from_world_edges(q: Vec3, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Arc<Self> {
        let n = cross(&u, &v);
        let normal = n.unit_vector();
        let d = dot(&normal, &q);
        let w = n / dot(&n, &n);

        let bbox_diagonal1 = AABB::from_min_max(q, q + u + v);
        let bbox_diagonal2 = AABB::from_min_max(q + u, q + v);

        Arc::new(Quad {
            q,
            u,
            v,
            w,
            mat,
            bbox: AABB::from_combined(bbox_diagonal1, bbox_diagonal2),
            normal,
            d,
            area: n.length()
        })
    }

    fn is_interior(a: f32, b: f32, hit_rec: &mut HitRecord) -> bool {
        const UNIT_INTERVAL: Interval = Interval::new(0.0, 1.0);
        // Given the hit point in plane coordinates, return false if it is outside the
        // primitive, otherwise set the hit record UV coordinates and return true.

        if !UNIT_INTERVAL.contains(a) || !UNIT_INTERVAL.contains(b) {
            return false;
        }

        hit_rec.u = a;
        hit_rec.v = b;
        true
    }

    pub fn r#box(transform: Transform, mat: Arc<dyn Material>) -> Arc<HittableList> {
        let mut sides = HittableList::new();

        let face = |translation, rotation| {
            transform * Transform::from_trs(translation, rotation, Vec3::new(1.0, 1.0, 1.0))
        };

        sides.add(Quad::new(
            face(Vec3::new(0.0, 0.0, 0.5), Quaternion::IDENTITY),
            mat.clone(),
        ));
        sides.add(Quad::new(
            face(
                Vec3::new(0.0, 0.0, -0.5),
                Quaternion::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), std::f32::consts::PI),
            ),
            mat.clone(),
        ));
        sides.add(Quad::new(
            face(
                Vec3::new(0.5, 0.0, 0.0),
                Quaternion::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), std::f32::consts::FRAC_PI_2),
            ),
            mat.clone(),
        ));
        sides.add(Quad::new(
            face(
                Vec3::new(-0.5, 0.0, 0.0),
                Quaternion::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), -std::f32::consts::FRAC_PI_2),
            ),
            mat.clone(),
        ));
        sides.add(Quad::new(
            face(
                Vec3::new(0.0, 0.5, 0.0),
                Quaternion::from_axis_angle(Vec3::new(1.0, 0.0, 0.0), -std::f32::consts::FRAC_PI_2),
            ),
            mat.clone(),
        ));
        sides.add(Quad::new(
            face(
                Vec3::new(0.0, -0.5, 0.0),
                Quaternion::from_axis_angle(Vec3::new(1.0, 0.0, 0.0), std::f32::consts::FRAC_PI_2),
            ),
            mat,
        ));

        Arc::new(sides)
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, ray_i: Interval, record: &mut HitRecord) -> Option<&dyn Material> {
        let denominator = dot(&self.normal, ray.direction());

        // No hit if the ray is parallel to the plane.
        if denominator.abs() < f32::EPSILON {
            return None;
        }

        // Return false if the hit point parameter t is outside the ray interval.
        let t = (self.d - dot(&self.normal, ray.origin())) / denominator;
        if !ray_i.contains(t) {
            return None;
        }

        // Determine if the hit point lies within the planar shape using its plane coordinates.
        let intersection = ray.at(t);
        let planar_hit_point = intersection - self.q;
        let alpha = dot(&self.w, &cross(&planar_hit_point, &self.v));
        let beta = dot(&self.w, &cross(&self.u, &planar_hit_point));

        if !Self::is_interior(alpha, beta, record) {
            return None;
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

    fn pdf_value(&self, origin: &Vec3, direction: &Vec3) -> f32 {
        let mut rec = HitRecord::default();
        if self.hit(&Ray::new(*origin, *direction), Interval::new(0.001, f32::INFINITY), &mut rec).is_none() {
            return 0.0
        }
        
        let distance_squared = rec.t * rec.t * direction.length_squared();
        let cosine = (dot(direction, &rec.normal) / direction.length()).abs();
        
        distance_squared / (cosine * self.area)
    }
    
    fn random(&self, origin: &Vec3) -> Vec3 {
        let p = self.q + self.u * random_f32() + self.v * random_f32();
        p - *origin
    }
}