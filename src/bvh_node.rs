use std::cmp::Ordering;
use std::rc::Rc;
use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::utils::random_range_i32;

pub struct BVHNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: AABB
}

impl BVHNode {
    pub fn new_root(objects: &mut Vec<Rc<dyn Hittable>>) -> Self {
        BVHNode::new(objects, 0, objects.len())
    }
    
    pub fn new(objects: &mut Vec<Rc<dyn Hittable>>, start: usize, end: usize) -> Self {
        let left: Option<Rc<dyn Hittable>>;
        let right: Option<Rc<dyn Hittable>>;
        let mut bbox = AABB::EMPTY;

        for object_i in start..end {
            bbox = AABB::from_combined(bbox, objects[object_i].bounding_box());
        }
        let axis = bbox.longest_axis();
        
        let comparator = match axis { 
            0 => BVHNode::box_x_compare,
            1 => BVHNode::box_y_compare,
            _ => BVHNode::box_z_compare
        };
        
        let object_span = end - start;
        
        if object_span == 1 {
            right = Some(objects[start].clone());
            left = right.clone();
        } else if object_span == 2 {
            left = Some(objects[start].clone());
            right = Some(objects[start + 1].clone());
        } else { 
            objects[start..end].sort_by(comparator);
            
            let mid = start + (object_span / 2);
            left = Some(Rc::new(BVHNode::new(objects, start, mid)));
            right = Some(Rc::new(BVHNode::new(objects, mid, end)));
        } 
        
        let left = left.expect("Left node is None");
        let right = right.expect("Right node is None");
        BVHNode { left, right, bbox }
    }
    
    fn box_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis_index: i32) -> Ordering {
        let a_axis_interval = a.bounding_box().axis_interval(axis_index);
        let b_axis_interval = b.bounding_box().axis_interval(axis_index);
        a_axis_interval.min.partial_cmp(&b_axis_interval.min).expect("A NaN value was encountered in the comparison")
    }
    
    fn box_x_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
        BVHNode::box_compare(a, b, 0)
    }
    fn box_y_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
        BVHNode::box_compare(a, b, 1)
    }
    fn box_z_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
        BVHNode::box_compare(a, b, 2)
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, ray_i: Interval, record: &mut HitRecord) -> bool {
        if !self.bbox.hit(ray, ray_i) { 
            return false;
        }
        
        if self.left.hit(ray, ray_i, record) {
            return true;
        }
        self.right.hit(ray, ray_i, record)
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}