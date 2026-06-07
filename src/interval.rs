pub struct Interval {
    pub min: f32,
    pub max: f32
}

impl Interval {
    pub const fn new(min: f32, max: f32) -> Self {
        Interval { min, max }
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

    const EMPTY: Interval = Interval::new(f32::INFINITY, -f32::INFINITY);
    const UNIVERSE: Interval = Interval::new(-f32::INFINITY, f32::INFINITY);
}

impl Default for Interval {
    fn default() -> Self {
        Interval::EMPTY
    }
}