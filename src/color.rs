use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub};
use crate::math::Interval;

#[derive(Clone, Copy, Default)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

impl Color {
    pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0 };
    pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0 };
    
    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Color { r, g, b }
    }
    
    pub fn fmt(&self) -> (u8, u8, u8) {
        let r = linear_to_gamma(self.r);
        let g = linear_to_gamma(self.g);
        let b = linear_to_gamma(self.b);

        const INTENSITY: Interval = Interval::new(0.000, 0.999);
        let ir = (255.0 * INTENSITY.clamp(r)) as u8;
        let ig = (255.0 * INTENSITY.clamp(g)) as u8;
        let ib = (255.0 * INTENSITY.clamp(b)) as u8;
        
        (ir, ig, ib)
    }
}

impl Neg for Color {
    type Output = Color;

    fn neg(self) -> Self::Output {
        Color {r: -self.r, g: -self.g, b: -self.b }
    }
}

impl Index<i32> for Color {
    type Output = f32;

    fn index(&self, inder: i32) -> &Self::Output {
        match inder {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            _ => unimplemented!("Impossible inder {}", inder),
        }
    }
}

impl AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl MulAssign<f32> for Color {
    fn mul_assign(&mut self, t: f32) {
        self.r *= t;
        self.g *= t;
        self.b *= t;
    }
}

impl DivAssign<f32> for Color {
    fn div_assign(&mut self, t: f32) {
        *self *= 1.0 / t;
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = linear_to_gamma(self.r);
        let g = linear_to_gamma(self.g);
        let b = linear_to_gamma(self.b);

        const INTENSITY: Interval = Interval::new(0.000, 0.999);
        let ir = (255.0 * INTENSITY.clamp(r)) as i32;
        let ig = (255.0 * INTENSITY.clamp(g)) as i32;
        let ib = (255.0 * INTENSITY.clamp(b)) as i32;

        write!(f, "{} {} {}", ir, ig, ib)
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color {r: self.r + rhs.r, g: self.g + rhs.g, b: self.b + rhs.b}
    }
}

impl Sub<Color> for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Self::Output {
        Color {r: self.r - rhs.r, g: self.g - rhs.g, b: self.b - rhs.b}
    }
}


impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color {r: self.r * rhs.r, g: self.g * rhs.g, b: self.b * rhs.b}
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, t: f32) -> Self::Output {
        Color {r: self.r * t, g: self.g * t, b: self.b * t }
    }
}

impl Div<f32> for Color {
    type Output = Color;

    fn div(self, t: f32) -> Self::Output {
        self * (1.0 / t)
    }
}

fn linear_to_gamma(linear_component: f32) -> f32 {
    if linear_component > 0.0 {
        return linear_component.sqrt()
    }
    0.0
}