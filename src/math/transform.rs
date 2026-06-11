use std::ops::{Index, IndexMut, Mul, MulAssign};
use crate::math::{Quaternion, Vec3};

/// A column-major affine transform stored as four `Vec3` columns.
///
/// Points are treated as column vectors. The final column stores translation,
/// and composition `parent * child` applies `child` before `parent`. The final
/// homogeneous row `(0, 0, 0, 1)` is implicit, so only 12 floats are stored.
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    pub matrix: [f32; 12],
}

impl Transform {
    pub const IDENTITY: Self = Self {
        matrix: [1.0, 0.0, 0.0, 0.0,
                1.0, 0.0, 0.0, 0.0,
                1.0, 0.0, 0.0, 0.0],
    };

    #[inline(always)]
    pub const fn from_matrix(matrix: [f32; 12]) -> Self {
        Self { matrix }
    }

    #[inline(always)]
    pub const fn from_translation(translation: Vec3) -> Self {
        Self {
            matrix: [
                1.0,
                0.0,
                0.0,
                0.0,
                1.0,
                0.0,
                0.0,
                0.0,
                1.0,
                translation.x,
                translation.y,
                translation.z,
            ],
        }
    }

    #[inline(always)]
    pub fn from_rotation(rotation: Quaternion) -> Self {
        Self::from_trs(Vec3::default(), rotation, Vec3::new(1.0, 1.0, 1.0))
    }

    #[inline(always)]
    pub const fn from_scale(scale: Vec3) -> Self {
        Self {
            matrix: [
                scale.x, 0.0, 0.0, 0.0, scale.y, 0.0, 0.0, 0.0, scale.z, 0.0, 0.0, 0.0,
            ],
        }
    }

    #[inline(always)]
    pub const fn from_uniform_scale(scale: f32) -> Self {
        Self {
            matrix: [
                scale, 0.0, 0.0, 0.0, scale, 0.0, 0.0, 0.0, scale, 0.0, 0.0, 0.0,
            ],
        }
    }

    /// Builds `translation * rotation * scale` directly, without temporary matrices.
    #[inline]
    pub fn from_trs(translation: Vec3, rotation: Quaternion, scale: Vec3) -> Self {
        let rotation = rotation.normalized();
        let x2 = rotation.x + rotation.x;
        let y2 = rotation.y + rotation.y;
        let z2 = rotation.z + rotation.z;
        let xx = rotation.x * x2;
        let yy = rotation.y * y2;
        let zz = rotation.z * z2;
        let xy = rotation.x * y2;
        let xz = rotation.x * z2;
        let yz = rotation.y * z2;
        let wx = rotation.w * x2;
        let wy = rotation.w * y2;
        let wz = rotation.w * z2;

        Self {
            matrix: [
                (1.0 - yy - zz) * scale.x,
                (xy + wz) * scale.x,
                (xz - wy) * scale.x,
                (xy - wz) * scale.y,
                (1.0 - xx - zz) * scale.y,
                (yz + wx) * scale.y,
                (xz + wy) * scale.z,
                (yz - wx) * scale.z,
                (1.0 - xx - yy) * scale.z,
                translation.x,
                translation.y,
                translation.z,
            ],
        }
    }

    #[inline(always)]
    pub const fn translation(self) -> Vec3 {
        Vec3 {
            x: self.matrix[9],
            y: self.matrix[10],
            z: self.matrix[11],
        }
    }

    #[inline(always)]
    pub fn transform_point(self, point: Vec3) -> Vec3 {
        Vec3::new(
            self.matrix[0] * point.x
                + self.matrix[3] * point.y
                + self.matrix[6] * point.z
                + self.matrix[9],
            self.matrix[1] * point.x
                + self.matrix[4] * point.y
                + self.matrix[7] * point.z
                + self.matrix[10],
            self.matrix[2] * point.x
                + self.matrix[5] * point.y
                + self.matrix[8] * point.z
                + self.matrix[11],
        )
    }

    #[inline(always)]
    pub fn transform_vector(self, vector: Vec3) -> Vec3 {
        Vec3::new(
            self.matrix[0] * vector.x + self.matrix[3] * vector.y + self.matrix[6] * vector.z,
            self.matrix[1] * vector.x + self.matrix[4] * vector.y + self.matrix[7] * vector.z,
            self.matrix[2] * vector.x + self.matrix[5] * vector.y + self.matrix[8] * vector.z,
        )
    }

    /// Transforms a normal with the inverse transpose. The result is not normalized.
    #[inline]
    pub fn transform_normal(self, normal: Vec3) -> Vec3 {
        let inverse = self.inverse();
        Vec3::new(
            inverse.matrix[0] * normal.x
                + inverse.matrix[1] * normal.y
                + inverse.matrix[2] * normal.z,
            inverse.matrix[3] * normal.x
                + inverse.matrix[4] * normal.y
                + inverse.matrix[5] * normal.z,
            inverse.matrix[6] * normal.x
                + inverse.matrix[7] * normal.y
                + inverse.matrix[8] * normal.z,
        )
    }

    #[inline(always)]
    pub fn determinant(self) -> f32 {
        let m = self.matrix;
        m[0] * (m[4] * m[8] - m[7] * m[5]) - m[3] * (m[1] * m[8] - m[7] * m[2])
            + m[6] * (m[1] * m[5] - m[4] * m[2])
    }

    /// Inverts the affine matrix. Singular transforms produce non-finite values.
    #[inline]
    pub fn inverse(self) -> Self {
        self.inverse_with_reciprocal_determinant(self.determinant().recip())
    }

    #[inline]
    pub fn try_inverse(self) -> Option<Self> {
        let determinant = self.determinant();
        if !determinant.is_finite() || determinant.abs() <= f32::EPSILON {
            None
        } else {
            Some(self.inverse_with_reciprocal_determinant(determinant.recip()))
        }
    }

    #[inline]
    fn inverse_with_reciprocal_determinant(self, inverse_determinant: f32) -> Self {
        let m = self.matrix;
        let i00 = (m[4] * m[8] - m[7] * m[5]) * inverse_determinant;
        let i01 = (m[6] * m[5] - m[3] * m[8]) * inverse_determinant;
        let i02 = (m[3] * m[7] - m[6] * m[4]) * inverse_determinant;
        let i10 = (m[7] * m[2] - m[1] * m[8]) * inverse_determinant;
        let i11 = (m[0] * m[8] - m[6] * m[2]) * inverse_determinant;
        let i12 = (m[6] * m[1] - m[0] * m[7]) * inverse_determinant;
        let i20 = (m[1] * m[5] - m[4] * m[2]) * inverse_determinant;
        let i21 = (m[3] * m[2] - m[0] * m[5]) * inverse_determinant;
        let i22 = (m[0] * m[4] - m[3] * m[1]) * inverse_determinant;

        let tx = -(i00 * m[9] + i01 * m[10] + i02 * m[11]);
        let ty = -(i10 * m[9] + i11 * m[10] + i12 * m[11]);
        let tz = -(i20 * m[9] + i21 * m[10] + i22 * m[11]);

        Self {
            matrix: [i00, i10, i20, i01, i11, i21, i02, i12, i22, tx, ty, tz],
        }
    }

    /// Multiplies two affine transforms using only their meaningful 3x4 parts.
    #[inline]
    pub fn compose(self, rhs: Self) -> Self {
        let a = self.matrix;
        let b = rhs.matrix;
        let mut result = [0.0; 12];

        for column in 0..3 {
            let offset = column * 3;
            result[offset] = a[0] * b[offset] + a[3] * b[offset + 1] + a[6] * b[offset + 2];
            result[offset + 1] = a[1] * b[offset] + a[4] * b[offset + 1] + a[7] * b[offset + 2];
            result[offset + 2] = a[2] * b[offset] + a[5] * b[offset + 1] + a[8] * b[offset + 2];
        }

        result[9] = a[0] * b[9] + a[3] * b[10] + a[6] * b[11] + a[9];
        result[10] = a[1] * b[9] + a[4] * b[10] + a[7] * b[11] + a[10];
        result[11] = a[2] * b[9] + a[5] * b[10] + a[8] * b[11] + a[11];

        Self { matrix: result }
    }
}

impl Default for Transform {
    #[inline(always)]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Index<(usize, usize)> for Transform {
    type Output = f32;

    #[inline(always)]
    fn index(&self, (row, column): (usize, usize)) -> &Self::Output {
        &self.matrix[column * 3 + row]
    }
}

impl IndexMut<(usize, usize)> for Transform {
    #[inline(always)]
    fn index_mut(&mut self, (row, column): (usize, usize)) -> &mut Self::Output {
        &mut self.matrix[column * 3 + row]
    }
}

impl Mul for Transform {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        self.compose(rhs)
    }
}

impl MulAssign for Transform {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.compose(rhs);
    }
}

impl Mul<Vec3> for Transform {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, rhs: Vec3) -> Self::Output {
        self.transform_point(rhs)
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{Quaternion, Vec3};
    use super::*;

    fn assert_vec3_near(actual: Vec3, expected: Vec3) {
        let error = actual - expected;
        assert!(
            error.length() < 1.0e-4,
            "actual={actual}, expected={expected}"
        );
    }

    #[test]
    fn transform_has_expected_layout() {
        assert_eq!(size_of::<Transform>(), 48);
        assert_eq!(align_of::<Transform>(), 16);
    }

    #[test]
    fn trs_applies_scale_rotation_translation() {
        let transform = Transform::from_trs(
            Vec3::new(5.0, 2.0, -1.0),
            Quaternion::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), std::f32::consts::FRAC_PI_2),
            Vec3::new(2.0, 3.0, 4.0),
        );
        assert_vec3_near(
            transform * Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(5.0, 2.0, -3.0),
        );
    }

    #[test]
    fn composition_matches_sequential_transforms() {
        let parent = Transform::from_trs(
            Vec3::new(3.0, 0.0, 2.0),
            Quaternion::from_euler_xyz(0.1, 0.5, -0.2),
            Vec3::new(2.0, 2.0, 2.0),
        );
        let child = Transform::from_translation(Vec3::new(1.0, 4.0, -2.0));
        let point = Vec3::new(-2.0, 1.0, 0.5);
        assert_vec3_near((parent * child) * point, parent * (child * point));
    }

    #[test]
    fn inverse_undoes_non_uniform_trs() {
        let transform = Transform::from_trs(
            Vec3::new(3.0, -4.0, 8.0),
            Quaternion::from_euler_xyz(0.3, -0.7, 1.2),
            Vec3::new(2.0, 3.0, 0.5),
        );
        let point = Vec3::new(-2.0, 1.0, 0.5);
        assert_vec3_near(transform.inverse() * (transform * point), point);
    }

    #[test]
    fn singular_transform_has_no_checked_inverse() {
        assert!(Transform::from_scale(Vec3::new(1.0, 0.0, 1.0))
            .try_inverse()
            .is_none());
    }
}
