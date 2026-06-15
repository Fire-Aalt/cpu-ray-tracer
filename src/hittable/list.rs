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
        let sampleable_count = self
            .objects
            .iter()
            .filter(|object| object.can_sample())
            .count();
        if sampleable_count == 0 {
            return 0.0;
        }

        let weight = 1.0 / sampleable_count as f32;
        self.objects
            .iter()
            .filter(|object| object.can_sample())
            .map(|object| object.pdf_value(origin, direction))
            .filter(|value| value.is_finite() && *value >= 0.0)
            .map(|value| weight * value)
            .sum()
    }

    fn random(&self, origin: &Vec3) -> Vec3 {
        let sampleable_count = self
            .objects
            .iter()
            .filter(|object| object.can_sample())
            .count();
        if sampleable_count == 0 {
            return Vec3::new(1.0, 0.0, 0.0);
        }

        let index = random_range_i32(0..=(sampleable_count as i32 - 1)) as usize;
        self.objects
            .iter()
            .filter(|object| object.can_sample())
            .nth(index)
            .expect("sampleable object count changed while sampling")
            .random(origin)
    }

    fn can_sample(&self) -> bool {
        self.objects.iter().any(|object| object.can_sample())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::Material;

    struct SamplingStub {
        pdf: f32,
        direction: Vec3,
    }

    impl Hittable for SamplingStub {
        fn hit(
            &self,
            _ray: &Ray,
            _ray_i: Interval,
            _record: &mut HitRecord,
        ) -> Option<&dyn Material> {
            None
        }

        fn bounding_box(&self) -> AABB {
            AABB::EMPTY
        }

        fn pdf_value(&self, _origin: &Vec3, _direction: &Vec3) -> f32 {
            self.pdf
        }

        fn random(&self, _origin: &Vec3) -> Vec3 {
            self.direction
        }

        fn can_sample(&self) -> bool {
            true
        }
    }

    #[test]
    fn empty_list_is_not_sampleable() {
        assert!(!HittableList::new().can_sample());
    }

    #[test]
    fn invalid_child_pdf_does_not_poison_the_list_pdf() {
        let mut list = HittableList::new();
        list.add(Arc::new(SamplingStub {
            pdf: f32::NAN,
            direction: Vec3::new(1.0, 0.0, 0.0),
        }));
        list.add(Arc::new(SamplingStub {
            pdf: 0.25,
            direction: Vec3::new(0.0, 1.0, 0.0),
        }));

        let pdf = list.pdf_value(&Vec3::default(), &Vec3::new(0.0, 1.0, 0.0));
        assert_eq!(pdf, 0.125);
    }
}
