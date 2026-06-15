use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::math::{AABB, Interval, Ray, Vec3};
use crate::utils::random_range_i32;
use std::sync::Arc;

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    bbox: AABB,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: vec![],
            bbox: AABB::default(),
        }
    }

    pub fn from_one(object: Arc<dyn Hittable>) -> Self {
        HittableList {
            bbox: object.bounding_box(),
            objects: vec![object],
        }
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
            if let Some(material) =
                object.hit(ray, Interval::new(ray_i.min, closest_so_far), &mut temp_rec)
            {
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

    fn pdf_value(&self, origin: &Vec3, direction: &Vec3) -> f32 {
        if self.objects.is_empty() {
            return 0.0;
        }

        let weight = 1.0 / self.objects.len() as f32;
        self.objects
            .iter()
            .map(|object| weight * object.pdf_value(origin, direction))
            .sum()
    }

    fn random(&self, origin: &Vec3) -> Vec3 {
        if self.objects.is_empty() {
            return Vec3::new(1.0, 0.0, 0.0);
        }

        let index = random_range_i32(0..=(self.objects.len() as i32 - 1)) as usize;
        self.objects[index].random(origin)
    }
}
