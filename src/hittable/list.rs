use std::sync::Arc;
use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    bbox: AABB
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { objects: vec![], bbox: AABB::default() }
    }

    pub fn from_one(object: Arc<dyn Hittable>) -> Self {
        HittableList { bbox: object.bounding_box(), objects: vec![object] }
    }
    
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.bbox = AABB::from_combined(self.bbox, object.bounding_box());
        self.objects.push(object);
    }
    
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_i: Interval, record: &mut HitRecord) -> Option<&dyn Material> {
        let mut temp_rec = HitRecord::default();
        let mut hit_material = None;
        let mut closest_so_far = ray_i.max;

        for object in &self.objects {
            if let Some(material) = object.hit(
                ray,
                Interval::new(ray_i.min, closest_so_far),
                &mut temp_rec,
            ) {
                closest_so_far = temp_rec.t;
                *record = temp_rec.clone();
                hit_material = Some(material);
            }
        }
        hit_material
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}