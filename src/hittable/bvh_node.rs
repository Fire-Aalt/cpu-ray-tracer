use std::sync::Arc;

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::math::{AABB, Interval, Ray, Vec3};

const BIN_COUNT: usize = 16;
const MAX_LEAF_SIZE: usize = 4;
const MAX_BUILD_DEPTH: usize = 48;
const TRAVERSAL_STACK_SIZE: usize = MAX_BUILD_DEPTH + 2;

pub struct BVHNode {
    nodes: Box<[FlatNode]>,
    objects: Box<[Arc<dyn Hittable>]>,
    object_bounds: Box<[AABB]>,
    bbox: AABB,
}

#[derive(Clone, Copy, Default)]
struct FlatNode {
    bbox: AABB,
    /// Right child index for internal nodes, first object index for leaves.
    first_index: u32,
    /// Zero identifies an internal node. Leaves contain at least one object.
    object_count: u32,
}

impl FlatNode {
    fn leaf(bbox: AABB, first_object: usize, object_count: usize) -> Self {
        Self {
            bbox,
            first_index: first_object as u32,
            object_count: object_count as u32,
        }
    }

    fn internal(bbox: AABB, right_child: usize) -> Self {
        Self {
            bbox,
            first_index: right_child as u32,
            object_count: 0,
        }
    }

    fn is_leaf(&self) -> bool {
        self.object_count != 0
    }
}

struct Primitive {
    object: Arc<dyn Hittable>,
    bbox: AABB,
    centroid: Vec3,
}

#[derive(Clone, Copy, Default)]
struct Bin {
    bbox: AABB,
    count: usize,
}

#[derive(Clone, Copy, Default)]
struct StackEntry {
    node_index: u32,
    entry_t: f32,
}

impl BVHNode {
    pub fn new_root(objects: &mut Vec<Arc<dyn Hittable>>) -> Self {
        Self::new(objects, 0, objects.len())
    }

    pub fn new(objects: &mut Vec<Arc<dyn Hittable>>, start: usize, end: usize) -> Self {
        assert!(
            start <= end && end <= objects.len(),
            "Invalid BVH object range"
        );

        let mut primitives: Vec<_> = objects[start..end]
            .iter()
            .cloned()
            .map(|object| {
                let bbox = object.bounding_box();
                let centroid = Vec3::new(
                    (bbox.x.min + bbox.x.max) * 0.5,
                    (bbox.y.min + bbox.y.max) * 0.5,
                    (bbox.z.min + bbox.z.max) * 0.5,
                );
                Primitive {
                    object,
                    bbox,
                    centroid,
                }
            })
            .collect();

        if primitives.is_empty() {
            return Self {
                nodes: Vec::new().into_boxed_slice(),
                objects: Vec::new().into_boxed_slice(),
                object_bounds: Vec::new().into_boxed_slice(),
                bbox: AABB::EMPTY,
            };
        }

        let mut nodes = Vec::with_capacity(primitives.len() * 2 - 1);
        Self::build(&mut primitives, 0, 0, &mut nodes);
        let bbox = nodes[0].bbox;
        let mut reordered_objects = Vec::with_capacity(primitives.len());
        let mut object_bounds = Vec::with_capacity(primitives.len());
        for primitive in primitives {
            reordered_objects.push(primitive.object);
            object_bounds.push(primitive.bbox);
        }

        Self {
            nodes: nodes.into_boxed_slice(),
            objects: reordered_objects.into_boxed_slice(),
            object_bounds: object_bounds.into_boxed_slice(),
            bbox,
        }
    }

    fn build(
        primitives: &mut [Primitive],
        first_primitive: usize,
        depth: usize,
        nodes: &mut Vec<FlatNode>,
    ) -> usize {
        let node_index = nodes.len();
        nodes.push(FlatNode::default());

        let bbox = primitives.iter().fold(AABB::EMPTY, |bounds, primitive| {
            AABB::from_combined(bounds, primitive.bbox)
        });

        if primitives.len() <= MAX_LEAF_SIZE || depth >= MAX_BUILD_DEPTH {
            nodes[node_index] = FlatNode::leaf(bbox, first_primitive, primitives.len());
            return node_index;
        }

        let (centroid_min, centroid_max) = Self::centroid_bounds(primitives);
        let split = Self::best_sah_split(primitives, bbox, centroid_min, centroid_max);
        let mid = if let Some((axis, split_bin)) = split {
            let axis_index = axis as usize;
            let extent = centroid_max[axis_index] - centroid_min[axis_index];
            let mut mid = 0;
            for index in 0..primitives.len() {
                let bin = Self::bin_index(
                    primitives[index].centroid[axis],
                    centroid_min[axis_index],
                    extent,
                );
                if bin <= split_bin {
                    primitives.swap(index, mid);
                    mid += 1;
                }
            }
            mid
        } else {
            0
        };

        let mid = if mid == 0 || mid == primitives.len() {
            let axis = Self::widest_centroid_axis(centroid_min, centroid_max);
            primitives.sort_unstable_by(|a, b| a.centroid[axis].total_cmp(&b.centroid[axis]));
            primitives.len() / 2
        } else {
            mid
        };

        let (left_primitives, right_primitives) = primitives.split_at_mut(mid);
        let left_child = Self::build(left_primitives, first_primitive, depth + 1, nodes);
        debug_assert_eq!(left_child, node_index + 1);
        let right_child = Self::build(right_primitives, first_primitive + mid, depth + 1, nodes);
        nodes[node_index] = FlatNode::internal(bbox, right_child);
        node_index
    }

    fn centroid_bounds(primitives: &[Primitive]) -> ([f32; 3], [f32; 3]) {
        let mut min = [f32::INFINITY; 3];
        let mut max = [f32::NEG_INFINITY; 3];
        for primitive in primitives {
            for axis in 0..3 {
                min[axis] = min[axis].min(primitive.centroid[axis as i32]);
                max[axis] = max[axis].max(primitive.centroid[axis as i32]);
            }
        }
        (min, max)
    }

    fn best_sah_split(
        primitives: &[Primitive],
        parent_bbox: AABB,
        centroid_min: [f32; 3],
        centroid_max: [f32; 3],
    ) -> Option<(i32, usize)> {
        let parent_area = parent_bbox.surface_area();
        if parent_area <= 0.0 || !parent_area.is_finite() {
            return None;
        }

        let mut best_cost = f32::INFINITY;
        let mut best_split = None;
        for axis in 0..3 {
            let extent = centroid_max[axis] - centroid_min[axis];
            if extent <= f32::EPSILON || !extent.is_finite() {
                continue;
            }

            let mut bins = [Bin::default(); BIN_COUNT];
            for primitive in primitives {
                let bin_index =
                    Self::bin_index(primitive.centroid[axis as i32], centroid_min[axis], extent);
                bins[bin_index].count += 1;
                bins[bin_index].bbox = AABB::from_combined(bins[bin_index].bbox, primitive.bbox);
            }

            let mut right_bounds = [AABB::EMPTY; BIN_COUNT];
            let mut right_counts = [0usize; BIN_COUNT];
            let mut bounds = AABB::EMPTY;
            let mut count = 0;
            for bin_index in (0..BIN_COUNT).rev() {
                bounds = AABB::from_combined(bounds, bins[bin_index].bbox);
                count += bins[bin_index].count;
                right_bounds[bin_index] = bounds;
                right_counts[bin_index] = count;
            }

            let mut left_bounds = AABB::EMPTY;
            let mut left_count = 0;
            for split_bin in 0..BIN_COUNT - 1 {
                left_bounds = AABB::from_combined(left_bounds, bins[split_bin].bbox);
                left_count += bins[split_bin].count;
                let right_count = right_counts[split_bin + 1];
                if left_count == 0 || right_count == 0 {
                    continue;
                }

                let cost = 1.0
                    + (left_bounds.surface_area() * left_count as f32
                        + right_bounds[split_bin + 1].surface_area() * right_count as f32)
                        / parent_area;
                if cost < best_cost {
                    best_cost = cost;
                    best_split = Some((axis as i32, split_bin));
                }
            }
        }

        best_split
    }

    fn bin_index(centroid: f32, min: f32, extent: f32) -> usize {
        ((((centroid - min) / extent) * BIN_COUNT as f32) as usize).min(BIN_COUNT - 1)
    }

    fn widest_centroid_axis(min: [f32; 3], max: [f32; 3]) -> i32 {
        let x = max[0] - min[0];
        let y = max[1] - min[1];
        let z = max[2] - min[2];
        if x > y {
            if x > z { 0 } else { 2 }
        } else if y > z {
            1
        } else {
            2
        }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, ray_i: Interval, record: &mut HitRecord) -> Option<&dyn Material> {
        if self.nodes.is_empty() {
            return None;
        }

        let direction = ray.direction();
        let inverse_direction = Vec3::new(1.0 / direction.x, 1.0 / direction.y, 1.0 / direction.z);
        let root_interval =
            self.nodes[0]
                .bbox
                .hit_interval_cached(ray.origin(), &inverse_direction, ray_i)?;

        let mut stack = [StackEntry::default(); TRAVERSAL_STACK_SIZE];
        let mut stack_size = 1;
        stack[0] = StackEntry {
            node_index: 0,
            entry_t: root_interval.min,
        };

        let mut closest = ray_i.max;
        let mut hit_material = None;
        while stack_size != 0 {
            stack_size -= 1;
            let entry = stack[stack_size];
            if entry.entry_t >= closest {
                continue;
            }

            let node_index = entry.node_index as usize;
            let node = self.nodes[node_index];
            if node.is_leaf() {
                let first = node.first_index as usize;
                let end = first + node.object_count as usize;
                for object_index in first..end {
                    let interval = Interval::new(ray_i.min, closest);
                    if self.object_bounds[object_index]
                        .hit_interval_cached(ray.origin(), &inverse_direction, interval)
                        .is_none()
                    {
                        continue;
                    }

                    if let Some(material) = self.objects[object_index].hit(ray, interval, record) {
                        closest = record.t;
                        hit_material = Some(material);
                    }
                }
                continue;
            }

            let left_child = node_index + 1;
            let right_child = node.first_index as usize;
            let interval = Interval::new(ray_i.min, closest);
            let left_hit = self.nodes[left_child].bbox.hit_interval_cached(
                ray.origin(),
                &inverse_direction,
                interval,
            );
            let right_hit = self.nodes[right_child].bbox.hit_interval_cached(
                ray.origin(),
                &inverse_direction,
                interval,
            );

            let (near, far) = match (left_hit, right_hit) {
                (Some(left), Some(right)) if left.min <= right.min => (
                    Some(StackEntry {
                        node_index: left_child as u32,
                        entry_t: left.min,
                    }),
                    Some(StackEntry {
                        node_index: right_child as u32,
                        entry_t: right.min,
                    }),
                ),
                (Some(left), Some(right)) => (
                    Some(StackEntry {
                        node_index: right_child as u32,
                        entry_t: right.min,
                    }),
                    Some(StackEntry {
                        node_index: left_child as u32,
                        entry_t: left.min,
                    }),
                ),
                (Some(left), None) => (
                    Some(StackEntry {
                        node_index: left_child as u32,
                        entry_t: left.min,
                    }),
                    None,
                ),
                (None, Some(right)) => (
                    Some(StackEntry {
                        node_index: right_child as u32,
                        entry_t: right.min,
                    }),
                    None,
                ),
                (None, None) => (None, None),
            };

            if let Some(far) = far {
                debug_assert!(stack_size < stack.len());
                stack[stack_size] = far;
                stack_size += 1;
            }
            if let Some(near) = near {
                debug_assert!(stack_size < stack.len());
                stack[stack_size] = near;
                stack_size += 1;
            }
        }

        hit_material
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
