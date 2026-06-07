mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod utils;
mod interval;

use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{unit_vector, Vec3};
use std::fs::File;
use std::io::Write;
use crate::interval::Interval;

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
    let mut record = HitRecord::default();
    if world.hit(ray, Interval::new(0.0, f32::INFINITY), &mut record) {
        return (record.normal.as_color() + Color::new(1.0, 1.0, 1.0)) * 0.5
    }
    
    let unit_direction = unit_vector(&ray.dir);
    let a = 0.5 * (unit_direction.y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}

fn main() {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    let image_width = 400;
    
    let mut image_height = (image_width as f32 / ASPECT_RATIO) as i32;
    image_height = if image_height < 1 { 1 } else { image_height };

    // World
    
    let mut world = HittableList::new();
    
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
    let camera_center = Vec3::new(0.0, 0.0, 0.0);
    
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
    
    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;
    
    let viewport_upper_left = camera_center 
        - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;
    
    let mut file = File::create("image.ppm").unwrap();
    
    writeln!(file, "P3\n{} {}\n255", image_width, image_height).unwrap();

    for j in 0..image_height {
        println!("Scanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_center = pixel00_loc + (pixel_delta_u * i as f32) + (pixel_delta_v * j as f32);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);
            
            let pixel_color = ray_color(&r, &world);
            writeln!(file, "{}", pixel_color).unwrap();
        }
    }

    println!("Done.");
}
