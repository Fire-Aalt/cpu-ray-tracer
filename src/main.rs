mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod utils;
mod interval;
mod camera;
mod material;
mod lambertian;
mod metal;
mod dialectric;

use std::rc::Rc;
use crate::camera::Camera;
use crate::color::Color;
use crate::dialectric::Dialectric;
use crate::hittable_list::HittableList;
use crate::lambertian::Lambertian;
use crate::metal::Metal;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

fn main() {
    let mut world = HittableList::new();
    
    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dialectric::new(1.5));
    let material_bubble = Rc::new(Dialectric::new(1.0 / 1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Box::new(Sphere::new(Vec3::new( 0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Box::new(Sphere::new(Vec3::new( 0.0,    0.0, -1.2),   0.5, material_center)));
    world.add(Box::new(Sphere::new(Vec3::new(-1.0,    0.0, -1.0),   0.5, material_left)));
    world.add(Box::new(Sphere::new(Vec3::new(-1.0,    0.0, -1.0),   0.4, material_bubble)));
    world.add(Box::new(Sphere::new(Vec3::new( 1.0,    0.0, -1.0),   0.5, material_right)));

    let mut camera = Camera::default();
    camera.image_width = 320;
    camera.samples_per_pixel = 20;
    camera.max_depth = 50;
    
    camera.render(&world);
}
