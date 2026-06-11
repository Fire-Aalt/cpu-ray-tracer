use std::ops::{Mul, MulAssign, Neg};

use crate::vec3::{cross, dot, Vec3};

/// A right-handed rotation stored as `(x, y, z, w)`.
///
/// The 16-byte layout and alignment match the SIMD-friendly layout.
/// Quaternion multiplication composes rotations so that
/// `a * b` applies `b` first and then `a`.
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {
    pub const IDENTITY: Self = Self::new(0.0, 0.0, 0.0, 1.0);

    #[inline(always)]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    #[inline(always)]
    pub fn from_axis_angle(axis: Vec3, angle_radians: f32) -> Self {
        let length_squared = axis.length_squared();
        if length_squared <= f32::EPSILON {
            return Self::IDENTITY;
        }

        let half_angle = angle_radians * 0.5;
        let scale = half_angle.sin() / length_squared.sqrt();
        Self::new(
            axis.x * scale,
            axis.y * scale,
            axis.z * scale,
            half_angle.cos(),
        )
    }

    /// Creates a rotation that applies X, then Y, then Z rotation.
    #[inline]
    pub fn from_euler_xyz(x_radians: f32, y_radians: f32, z_radians: f32) -> Self {
        let (sx, cx) = (x_radians * 0.5).sin_cos();
        let (sy, cy) = (y_radians * 0.5).sin_cos();
        let (sz, cz) = (z_radians * 0.5).sin_cos();

        Self::new(
            sx * cy * cz - cx * sy * sz,
            cx * sy * cz + sx * cy * sz,
            cx * cy * sz - sx * sy * cz,
            cx * cy * cz + sx * sy * sz,
        )
    }

    #[inline(always)]
    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    #[inline(always)]
    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    #[inline(always)]
    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn normalized(self) -> Self {
        let length_squared = self.length_squared();
        if length_squared <= f32::EPSILON {
            Self::IDENTITY
        } else {
            self * length_squared.sqrt().recip()
        }
    }

    #[inline(always)]
    pub const fn conjugate(self) -> Self {
        Self::new(-self.x, -self.y, -self.z, self.w)
    }

    #[inline]
    pub fn inverse(self) -> Self {
        let length_squared = self.length_squared();
        if length_squared <= f32::EPSILON {
            Self::IDENTITY
        } else {
            self.conjugate() * length_squared.recip()
        }
    }

    /// Rotates a vector without constructing temporary quaternions.
    #[inline(always)]
    pub fn rotate_vector(self, vector: Vec3) -> Vec3 {
        let length_squared = self.length_squared();
        if length_squared <= f32::EPSILON {
            return vector;
        }

        let imaginary = Vec3::new(self.x, self.y, self.z);
        let twice_cross = cross(&imaginary, &vector) * (2.0 / length_squared);
        vector + twice_cross * self.w + cross(&imaginary, &twice_cross)
    }

    /// Normalized linear interpolation along the shortest arc.
    #[inline]
    pub fn nlerp(self, mut rhs: Self, t: f32) -> Self {
        if self.dot(rhs) < 0.0 {
            rhs = -rhs;
        }
        (self * (1.0 - t) + rhs * t).normalized()
    }

    /// Spherical interpolation along the shortest arc.
    #[inline]
    pub fn slerp(self, mut rhs: Self, t: f32) -> Self {
        let lhs = self.normalized();
        rhs = rhs.normalized();

        let mut cosine = lhs.dot(rhs);
        if cosine < 0.0 {
            rhs = -rhs;
            cosine = -cosine;
        }

        if cosine > 0.9995 {
            return lhs.nlerp(rhs, t);
        }

        let angle = cosine.clamp(-1.0, 1.0).acos();
        let inverse_sine = angle.sin().recip();
        let lhs_weight = ((1.0 - t) * angle).sin() * inverse_sine;
        let rhs_weight = (t * angle).sin() * inverse_sine;
        (lhs * lhs_weight + rhs * rhs_weight).normalized()
    }

    /// Returns the shortest rotation that maps `from` onto `to`.
    #[inline]
    pub fn from_to_rotation(from: Vec3, to: Vec3) -> Self {
        let from_length_squared = from.length_squared();
        let to_length_squared = to.length_squared();
        if from_length_squared <= f32::EPSILON || to_length_squared <= f32::EPSILON {
            return Self::IDENTITY;
        }

        let from = from / from_length_squared.sqrt();
        let to = to / to_length_squared.sqrt();
        let cosine = dot(&from, &to);

        if cosine > 1.0 - 1.0e-6 {
            return Self::IDENTITY;
        }

        if cosine < -1.0 + 1.0e-6 {
            let basis = if from.x.abs() < from.y.abs() && from.x.abs() < from.z.abs() {
                Vec3::new(1.0, 0.0, 0.0)
            } else if from.y.abs() < from.z.abs() {
                Vec3::new(0.0, 1.0, 0.0)
            } else {
                Vec3::new(0.0, 0.0, 1.0)
            };
            return Self::from_axis_angle(cross(&from, &basis), std::f32::consts::PI);
        }

        let axis = cross(&from, &to);
        Self::new(axis.x, axis.y, axis.z, 1.0 + cosine).normalized()
    }
}

impl Default for Quaternion {
    #[inline(always)]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Neg for Quaternion {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl std::ops::Add for Quaternion {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl Mul<f32> for Quaternion {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl Mul for Quaternion {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x,
            self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w,
            self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
        )
    }
}

impl MulAssign for Quaternion {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Mul<Vec3> for Quaternion {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, rhs: Vec3) -> Self::Output {
        self.rotate_vector(rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_vec3_near(actual: Vec3, expected: Vec3) {
        let error = actual - expected;
        assert!(
            error.length() < 1.0e-5,
            "actual={actual}, expected={expected}"
        );
    }

    #[test]
    fn default_is_identity() {
        assert_eq!(Quaternion::default(), Quaternion::IDENTITY);
        assert_eq!(std::mem::size_of::<Quaternion>(), 16);
        assert_eq!(std::mem::align_of::<Quaternion>(), 16);
    }

    #[test]
    fn axis_angle_rotates_vector() {
        let rotation =
            Quaternion::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), std::f32::consts::FRAC_PI_2);
        assert_vec3_near(
            rotation * Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0),
        );
    }

    #[test]
    fn inverse_undoes_rotation() {
        let rotation = Quaternion::from_euler_xyz(0.3, -0.7, 1.2);
        let vector = Vec3::new(2.0, -4.0, 1.0);
        assert_vec3_near(rotation.inverse() * (rotation * vector), vector);
    }

    #[test]
    fn from_to_handles_opposite_vectors() {
        let rotation =
            Quaternion::from_to_rotation(Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, -1.0, 0.0));
        assert_vec3_near(
            rotation * Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
        );
    }
}
