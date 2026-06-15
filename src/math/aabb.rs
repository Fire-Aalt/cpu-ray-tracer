use crate::math::{Interval, Ray, Vec3};

#[derive(Clone, Copy, Default)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub const EMPTY: AABB = AABB {
        x: Interval::EMPTY,
        y: Interval::EMPTY,
        z: Interval::EMPTY,
    };
    pub const UNIVERSE: AABB = AABB {
        x: Interval::UNIVERSE,
        y: Interval::UNIVERSE,
        z: Interval::UNIVERSE,
    };

    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        (AABB { x, y, z }).with_min_padding()
    }

    pub fn from_min_max(a: Vec3, b: Vec3) -> Self {
        (AABB {
            // TODO: simplify
            x: if a.x <= b.x {
                Interval::new(a.x, b.x)
            } else {
                Interval::new(b.x, a.x)
            },
            y: if a.y <= b.y {
                Interval::new(a.y, b.y)
            } else {
                Interval::new(b.y, a.y)
            },
            z: if a.z <= b.z {
                Interval::new(a.z, b.z)
            } else {
                Interval::new(b.z, a.z)
            },
        })
        .with_min_padding()
    }

    fn with_min_padding(&self) -> Self {
        const DELTA: f32 = 0.0001;

        let mut aabb = *self;
        if aabb.x.size() < DELTA {
            aabb.x = aabb.x.expand(DELTA)
        }
        if aabb.y.size() < DELTA {
            aabb.y = aabb.y.expand(DELTA)
        }
        if aabb.z.size() < DELTA {
            aabb.z = aabb.z.expand(DELTA)
        }
        aabb
    }

    pub fn from_combined(a: AABB, b: AABB) -> Self {
        AABB {
            x: Interval::from_combined(a.x, b.x),
            y: Interval::from_combined(a.y, b.y),
            z: Interval::from_combined(a.z, b.z),
        }
    }

    pub fn axis_interval(&self, n: i32) -> Interval {
        match n {
            0 => self.x,
            1 => self.y,
            _ => self.z,
        }
    }

    pub fn longest_axis(&self) -> i32 {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() { 0 } else { 2 }
        } else {
            if self.y.size() > self.z.size() { 1 } else { 2 }
        }
    }

    pub fn surface_area(&self) -> f32 {
        let x = self.x.size().max(0.0);
        let y = self.y.size().max(0.0);
        let z = self.z.size().max(0.0);
        2.0 * (x * y + y * z + z * x)
    }

    pub fn hit_interval(&self, ray: &Ray, ray_i: Interval) -> Option<Interval> {
        let direction = ray.direction();
        let inverse_direction = Vec3::new(1.0 / direction.x, 1.0 / direction.y, 1.0 / direction.z);
        self.hit_interval_cached(ray.origin(), &inverse_direction, ray_i)
    }

    pub(crate) fn hit_interval_cached(
        &self,
        ray_origin: &Vec3,
        inverse_direction: &Vec3,
        mut ray_i: Interval,
    ) -> Option<Interval> {
        for axis in 0..3 {
            let bounds = self.axis_interval(axis);
            let origin = ray_origin[axis];
            let inverse = inverse_direction[axis];

            if inverse.is_nan() {
                return None;
            }

            if inverse.is_infinite() {
                if origin < bounds.min || origin > bounds.max {
                    return None;
                }
                continue;
            }

            let mut t0 = (bounds.min - origin) * inverse;
            let mut t1 = (bounds.max - origin) * inverse;
            if t0 > t1 {
                std::mem::swap(&mut t0, &mut t1);
            }

            ray_i.min = ray_i.min.max(t0);
            ray_i.max = ray_i.max.min(t1);
            if ray_i.max <= ray_i.min {
                return None;
            }
        }

        Some(ray_i)
    }

    pub fn hit(&self, r: &Ray, ray_i: Interval) -> bool {
        self.hit_interval(r, ray_i).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parallel_ray_outside_a_slab_is_rejected() {
        let bbox = AABB::from_min_max(Vec3::default(), Vec3::new(1.0, 1.0, 1.0));
        let ray = Ray::new(Vec3::new(2.0, 0.5, -1.0), Vec3::new(0.0, 0.0, 1.0));

        assert!(!bbox.hit(&ray, Interval::new(0.0, f32::INFINITY)));
    }

    #[test]
    fn parallel_ray_inside_the_slabs_keeps_the_hit_interval() {
        let bbox = AABB::from_min_max(Vec3::default(), Vec3::new(1.0, 1.0, 1.0));
        let ray = Ray::new(Vec3::new(0.5, 0.5, -1.0), Vec3::new(0.0, 0.0, 1.0));
        let interval = bbox
            .hit_interval(&ray, Interval::new(0.0, f32::INFINITY))
            .expect("ray should intersect the box");

        assert_eq!(interval.min, 1.0);
        assert_eq!(interval.max, 2.0);
    }
}
