use std::rc::Rc;
use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
    bbox: AABB
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { objects: vec![], bbox: AABB::default() }
    }

    pub fn from_one(object: Rc<dyn Hittable>) -> Self {
        HittableList { bbox: object.bounding_box(), objects: vec![object] }
    }
    
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.bbox = AABB::from_combined(self.bbox, object.bounding_box());
        self.objects.push(object);
    }
    
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_i: Interval, record: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_i.max;

        for object in &self.objects {
            if object.hit(ray, Interval::new(ray_i.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *record = temp_rec.clone()
            }
        }
        hit_anything
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
