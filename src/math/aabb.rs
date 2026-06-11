use crate::math::{Interval, Ray, Vec3};

#[derive(Clone, Copy, Default)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval
}

impl AABB {
    pub const EMPTY: AABB = AABB { x: Interval::EMPTY, y: Interval::EMPTY, z: Interval::EMPTY };
    pub const UNIVERSE: AABB = AABB { x: Interval::UNIVERSE, y: Interval::UNIVERSE, z: Interval::UNIVERSE };

    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        (AABB { x, y, z }).with_min_padding()
    }
    
    pub fn from_min_max(a: Vec3, b: Vec3) -> Self {
        (AABB { // TODO: simplify
            x: if a.x <= b.x { Interval::new(a.x, b.x) } else { Interval::new(b.x, a.x) },
            y: if a.y <= b.y { Interval::new(a.y, b.y) } else { Interval::new(b.y, a.y) },
            z: if a.z <= b.z { Interval::new(a.z, b.z) } else { Interval::new(b.z, a.z) }
        }).with_min_padding()
    }
    
    fn with_min_padding(&self) -> Self {
        const DELTA: f32 = 0.0001;
        
        let mut aabb = *self;
        if aabb.x.size() < DELTA { aabb.x = aabb.x.expand(DELTA) }
        if aabb.y.size() < DELTA { aabb.y = aabb.y.expand(DELTA) }
        if aabb.z.size() < DELTA { aabb.z = aabb.z.expand(DELTA) }
        aabb
    }
    
    pub fn from_combined(a: AABB, b: AABB) -> Self {
        AABB {
            x: Interval::from_combined(a.x, b.x),
            y: Interval::from_combined(a.y, b.y),
            z: Interval::from_combined(a.z, b.z)
        }
    }
    
    pub fn axis_interval(&self, n: i32) -> Interval {
        match n { 
            0 => self.x,
            1 => self.y,
            _ => self.z
        }
    }
    
    pub fn longest_axis(&self) -> i32 {
        if self.x.size() > self.y.size() { 
            if self.x.size() > self.z.size() { 0 } else { 2 }
        } else { 
            if self.y.size() > self.z.size() { 1 } else { 2 }
        }
    }
    
    pub fn hit(&self, r: &Ray, ray_i: Interval) -> bool {
        let ray_orig = r.origin();
        let ray_dir = r.direction();
        let mut ray_i = ray_i;
        
        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir[axis];
            
            let t0 = (ax.min - ray_orig[axis]) * adinv;
            let t1 = (ax.max - ray_orig[axis]) * adinv;

            if t0 < t1 { 
                if t0 > ray_i.min { ray_i.min = t0; }
                if t1 < ray_i.max { ray_i.max = t1; }
            } else { 
                if t1 > ray_i.min { ray_i.min = t1; }
                if t0 < ray_i.max { ray_i.max = t0; }
            }
            
            if ray_i.max <= ray_i.min {
                return false
            }
        }
        true
    }
}