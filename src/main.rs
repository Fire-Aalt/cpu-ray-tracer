mod vec3;
mod color;

use std::fs::File;
use std::io::Write;
use crate::color::Color;

fn main() {
    let image_height = 256;
    let image_width = 256;

    let mut file = File::create("image.ppm").unwrap();
    
    writeln!(file, "P3\n{} {}\n255", image_width, image_height).unwrap();

    for j in 0..image_height {
        println!("Scanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let r = i as f32 / (image_width - 1) as f32;
            let g = j as f32 / (image_height - 1) as f32;
            
            let pixel_color = Color::new(r, g, 0.0);
            writeln!(file, "{}", pixel_color).unwrap();
        }
    }

    println!("Done.");
}
