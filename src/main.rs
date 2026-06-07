mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;

use std::fs::File;
use std::io::Write;
use crate::color::Color;
use crate::ray::Ray;
use crate::vec3::{dot, unit_vector, Vec3};

fn hit_sphere(center: &Vec3, radius: f32, ray: &Ray) -> f32 {
    let oc = *center - ray.origin;
    let a = ray.dir.length_squared();
    let h = dot(&ray.dir, &oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;
    
    if discriminant < 0.0 {
        -1.0
    } else {
        (h - discriminant.sqrt()) / a
    }
}

fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let N = unit_vector(&(r.at(t) - Vec3::new(0.0, 0.0, -1.0)));
        return Color::new(N.x + 1.0, N.y + 1.0, N.z + 1.0) * 0.5;
    }
    
    let unit_direction = unit_vector(&r.dir);
    let a = 0.5 * (unit_direction.y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}

fn main() {
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    let image_width = 400;
    
    let mut image_height = (image_width as f32 / ASPECT_RATIO) as i32;
    image_height = if image_height < 1 { 1 } else { image_height };
    
    
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
            
            let pixel_color = ray_color(&r);
            writeln!(file, "{}", pixel_color).unwrap();
        }
    }

    println!("Done.");
}
