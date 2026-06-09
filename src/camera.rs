use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{cross, Vec3};
use std::fs::File;
use std::io::Write;
use tracing::{span, Level};
use crate::utils::{degrees_to_radians, random_f32};

pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub v_fov: f32,
    pub look_from: Vec3,
    pub look_at: Vec3,
    pub v_up: Vec3,
    pub defocus_angle: f32,
    pub focus_distance: f32,
    image_height: i32,
    pixel_samples_scale: f32,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Camera { 
            aspect_ratio: 16.0 / 9.0, // Ratio of image width over height
            image_width: 320, // Rendered image width in pixel count
            samples_per_pixel: 10, // Count of random samples for each pixel
            max_depth: 10, // Maximum number of ray bounces into scene
            
            v_fov: 90.0, // Vertical view angle (field of view)
            look_from: Vec3::new(0.0, 0.0, 0.0), // Point camera is looking from
            look_at: Vec3::new(0.0, 0.0, -1.0), // Point camera is looking at
            v_up: Vec3::new(0.0, 1.0, 0.0), // Camera-relative "up" direction

            defocus_angle: 0.0, // Variation angle of rays through each pixel
            focus_distance: 10.0, // Distance from camera lookfrom point to plane of perfect focus
            
            image_height: Default::default(),
            pixel_samples_scale: Default::default(),
            center: Default::default(),
            pixel00_loc: Default::default(),
            pixel_delta_u: Default::default(),
            pixel_delta_v: Default::default(),
            u: Default::default(),
            v: Default::default(),
            w: Default::default(),
            defocus_disk_u: Default::default(),
            defocus_disk_v: Default::default(),
        }
    }
}

impl Camera {
    fn initialize(&mut self) {
        self.image_height = (self.image_width as f32 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 { 1 } else { self.image_height };

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f32;
        
        self.center = self.look_from;

        // Determine viewport dimensions.
        let theta = degrees_to_radians(self.v_fov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_distance;
        let viewport_width = viewport_height * (self.image_width as f32 / self.image_height as f32);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        self.w = (self.look_from - self.look_at).unit_vector();
        self.u = cross(&self.v_up, &self.w).unit_vector();
        self.v = cross(&self.w, &self.u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = self.u * viewport_width; // Vector across viewport horizontal edge
        let viewport_v = -self.v * viewport_height; // Vector down viewport vertical edge

        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = self.center
            - self.w * self.focus_distance - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = self.focus_distance * degrees_to_radians(self.defocus_angle / 2.0).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius; 
    }
    
    pub fn render(&mut self, world: &dyn Hittable) {
        span!(Level::TRACE, "initialize camera").in_scope(|| {
            self.initialize();
        });
        
        let mut file = File::create("image.ppm").unwrap();
        writeln!(file, "P3\n{} {}\n255", self.image_width, self.image_height).unwrap();

        for j in 0..self.image_height {
            println!("Scanlines remaining: {}", self.image_height - j);

            span!(Level::TRACE, "scanline").in_scope(|| {
                for i in 0..self.image_width {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                    for _ in 0..self.samples_per_pixel {
                        let r = self.get_ray(i, j);
                        pixel_color += self.ray_color(&r, self.max_depth, world);
                    }
                    writeln!(file, "{}", pixel_color * self.pixel_samples_scale).unwrap();
                }
            })
        }

        println!("Done.");
    }
    
    fn get_ray(&self, i: i32, j: i32) -> Ray {
        // Construct a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j.
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + self.pixel_delta_u * (i as f32 + offset.x)
            + self.pixel_delta_v * (j as f32 + offset.y);
        
        let ray_origin = if self.defocus_angle <= 0.0 { self.center } else { self.defocus_disk_sample() };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random_f32();
        
        Ray::new_t(ray_origin, ray_direction, ray_time)
    }
    
    fn sample_square() -> Vec3 {
        Vec3::new(random_f32() - 0.5, random_f32() - 0.5, 0.0)
    }
    
    fn defocus_disk_sample(&self) -> Vec3 {
        let p = Vec3::random_in_unit_disk();
        self.center + (self.defocus_disk_u * p.x) + (self.defocus_disk_v * p.y)
    }
    
    fn ray_color(&self, ray: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        
        let mut record = HitRecord::default();
        
        if world.hit(ray, Interval::new(0.001, f32::INFINITY), &mut record) {
            let mut scattered = Ray::default();
            let mut attenuation = Color::default();
            let mat = record.material.clone().unwrap();

            if mat.scatter(&ray, &mut record, &mut attenuation, &mut scattered) { 
                return self.ray_color(&scattered, depth - 1, world) * attenuation;
            }
            return Color::new(0.0, 0.0, 0.0);
        }

        let unit_direction = ray.direction().unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }
}