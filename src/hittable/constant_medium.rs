use crate::hittable::{HitRecord, Hittable};
use crate::material::{Isotropic, Material};
use crate::math::{AABB, Interval, Ray, Vec3};
use crate::texture::Texture;
use crate::utils::random_f32;
use std::sync::Arc;

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f32,
    phase_function: Arc<dyn Material>,
}

/// Only works for convex boundaries.
impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, density: f32, texture: Arc<dyn Texture>) -> Arc<Self> {
        Arc::new(ConstantMedium {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Isotropic::new(texture),
        })
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, ray_i: Interval, record: &mut HitRecord) -> Option<&dyn Material> {
        let mut rec1 = HitRecord::default();
        let mut rec2 = HitRecord::default();

        if self
            .boundary
            .hit(ray, Interval::UNIVERSE, &mut rec1)
            .is_none()
        {
            return None;
        }

        if self
            .boundary
            .hit(
                ray,
                Interval::new(rec1.t + 0.0001, f32::INFINITY),
                &mut rec2,
            )
            .is_none()
        {
            return None;
        }

        if rec1.t < ray_i.min {
            rec1.t = ray_i.min
        }
        if rec2.t > ray_i.max {
            rec2.t = ray_i.max
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = ray.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_f32().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        record.t = rec1.t + hit_distance / ray_length;
        record.point = ray.at(record.t);

        record.normal = Vec3::new(1.0, 0.0, 0.0); // arbitrary
        record.front_face = true; // arbitrary

        Some(self.phase_function.as_ref())
    }

    fn bounding_box(&self) -> AABB {
        self.boundary.bounding_box()
    }
}
