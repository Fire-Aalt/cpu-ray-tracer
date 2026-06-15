use crate::math::Vec3;
use crate::pdf::PDF;
use crate::utils::random_f32;

pub struct MixturePDF<'a> {
    pdf: [&'a dyn PDF; 2],
}

impl<'a> MixturePDF<'a> {
    pub fn new(p0: &'a dyn PDF, p1: &'a dyn PDF) -> Self {
        Self { pdf: [p0, p1] }
    }
}

impl PDF for MixturePDF<'_> {
    fn value(&self, direction: &Vec3) -> f32 {
        0.5 * self.pdf[0].value(direction) + 0.5 * self.pdf[1].value(direction)
    }

    fn generate(&self) -> Vec3 {
        if random_f32() < 0.5 {
            self.pdf[0].generate()
        } else {
            self.pdf[1].generate()
        }
    }
}
