use std::ops::{Range, RangeInclusive};

const PI: f32 = std::f32::consts::PI;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn random_f32() -> f32 {
    rand::random_range(0.0..1.0)
}

pub fn random_range_f32(range: Range<f32>) -> f32 {
    rand::random_range(range)
}

pub fn random_range_i32(range: RangeInclusive<i32>) -> i32 {
    rand::random_range(range)
}