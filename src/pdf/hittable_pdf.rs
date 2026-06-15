use crate::hittable::Hittable;
use crate::math::Vec3;
use crate::pdf::PDF;

pub struct HittablePDF<'a> {
    objects: &'a dyn Hittable,
    origin: Vec3,
}

impl<'a> HittablePDF<'a> {
    pub fn new(objects: &'a (dyn Hittable + 'a), origin: Vec3) -> Self {
        Self { objects, origin }
    }
}

impl PDF for HittablePDF<'_> {
    fn value(&self, direction: &Vec3) -> f32 {
        self.objects.pdf_value(&self.origin, direction)
    }

    fn generate(&self) -> Vec3 {
        self.objects.random(&self.origin)
    }
}
