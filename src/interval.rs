#[derive(Copy, Clone, Debug)]
pub struct Interval {
    pub min: f32,
    pub max: f32
}

impl Interval {
    pub const EMPTY: Interval = Interval::new(f32::INFINITY, -f32::INFINITY);
    pub const UNIVERSE: Interval = Interval::new(-f32::INFINITY, f32::INFINITY);
    
    pub const fn new(min: f32, max: f32) -> Self {
        Interval { min, max }
    }
    
    pub const fn from_combined(a: Interval, b: Interval) -> Self {
        Interval { 
            min: a.min.min(b.min),
            max: a.max.max(b.max)
        }
    }   
    
    pub fn size(&self) -> f32 {
        self.max - self.min
    }
    
    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }
    
    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }
    
    pub fn clamp(&self, x: f32) -> f32 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }   
    
    pub fn expand(&self, delta: f32) -> Interval {
        let padding = delta / 2.0;
        Interval::new(self.min - padding, self.max + padding)
    }
}

impl Default for Interval {
    fn default() -> Self {
        Interval::EMPTY
    }
}