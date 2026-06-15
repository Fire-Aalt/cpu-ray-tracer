use crate::math::{Vec3, cross};

/// Orthonormal Basis
pub struct ONB {
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl ONB {
    pub fn new(n: Vec3) -> Self {
        let w = n.unit_vector();
        let a = if w.x.abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let v = cross(&w, &a).unit_vector();
        ONB {
            u: cross(&w, &v),
            v,
            w,
        }
    }

    pub fn transform(&self, v: &Vec3) -> Vec3 {
        self.u * v.x + self.v * v.y + self.w * v.z
    }
}
