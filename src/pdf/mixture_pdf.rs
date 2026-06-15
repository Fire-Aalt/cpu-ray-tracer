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
        let p0 = self.pdf[0].value(direction);
        let p1 = self.pdf[1].value(direction);
        let p0 = if p0.is_finite() && p0 >= 0.0 { p0 } else { 0.0 };
        let p1 = if p1.is_finite() && p1 >= 0.0 { p1 } else { 0.0 };
        0.5 * p0 + 0.5 * p1
    }

    fn generate(&self) -> Vec3 {
        if random_f32() < 0.5 {
            self.pdf[0].generate()
        } else {
            self.pdf[1].generate()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FixedPDF(f32);

    impl PDF for FixedPDF {
        fn value(&self, _direction: &Vec3) -> f32 {
            self.0
        }

        fn generate(&self) -> Vec3 {
            Vec3::new(0.0, 0.0, 1.0)
        }
    }

    #[test]
    fn invalid_component_does_not_poison_mixture_value() {
        let invalid = FixedPDF(f32::NAN);
        let valid = FixedPDF(0.5);
        let mixture = MixturePDF::new(&invalid, &valid);

        assert_eq!(mixture.value(&Vec3::new(0.0, 0.0, 1.0)), 0.25);
    }
}
