use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{unit_vector, Vec3};
use std::fs::File;
use std::io::Write;
use crate::utils::random_f32;

pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    image_height: i32,
    pixel_samples_scale: f32,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Camera { aspect_ratio: 16.0 / 9.0,
            image_width: 320,
            samples_per_pixel: 10,
            max_depth: 10,
            image_height: Default::default(),
            pixel_samples_scale: Default::default(),
            center: Default::default(),
            pixel00_loc: Default::default(),
            pixel_delta_u: Default::default(),
            pixel_delta_v: Default::default()
        }
    }
}

impl Camera {
    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();
        
        let mut file = File::create("image.ppm").unwrap();
        writeln!(file, "P3\n{} {}\n255", self.image_width, self.image_height).unwrap();

        for j in 0..self.image_height {
            println!("Scanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, self.max_depth, world);
                }
                writeln!(file, "{}", pixel_color * self.pixel_samples_scale).unwrap();
            }
        }

        println!("Done.");
    }
    
    fn initialize(&mut self) {
        self.image_height = (self.image_width as f32 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 { 1 } else { self.image_height };

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f32;
        
        self.center = Vec3::new(0.0, 0.0, 0.0);
        
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f32 / self.image_height as f32);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

        let viewport_upper_left = self.center
            - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;
    }
    
    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + self.pixel_delta_u * (i as f32 + offset.x)
            + self.pixel_delta_v * (j as f32 + offset.y);
        
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        
        Ray::new(ray_origin, ray_direction)
    }
    
    fn sample_square() -> Vec3 {
        Vec3::new(random_f32() - 0.5, random_f32() - 0.5, 0.0)
    }
    
    fn ray_color(&self, ray: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        
        let mut record = HitRecord::default();
        
        if world.hit(ray, Interval::new(0.001, f32::INFINITY), &mut record) {
            let direction = record.normal + Vec3::random_unit_vector();
            return self.ray_color(&Ray::new(record.point, direction), depth - 1, world) * 0.5 // <- reflectance
        }

        let unit_direction = unit_vector(&ray.dir);
        let a = 0.5 * (unit_direction.y + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }
}