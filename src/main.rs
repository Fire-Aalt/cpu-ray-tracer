mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod utils;
mod interval;
mod camera;

use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

fn main() {
    let mut world = HittableList::new();
    
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let mut camera = Camera::default();
    camera.image_width = 400;
    camera.samples_per_pixel = 20;
    camera.max_depth = 50;
    
    camera.render(&world);
}
