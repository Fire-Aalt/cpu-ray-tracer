use std::path::Path;

pub struct Image {
    pixels: Vec<u8>,
    image_width: u32,
    image_height: u32,
}

impl Image {
    pub fn new(path: &Path) -> Self {
        let result = image::open(path);

        if result.is_err() {
            return Image {
                pixels: vec![255, 0, 255],
                image_width: 1,
                image_height: 1,
            };
        }

        let img = result.unwrap();
        let rgb = img.to_rgb8();

        let width = rgb.width();
        let height = rgb.height();
        let raw: Vec<u8> = rgb.into_raw();

        Image {
            pixels: raw,
            image_width: width,
            image_height: height,
        }
    }

    pub fn width(&self) -> u32 {
        self.image_width
    }

    pub fn height(&self) -> u32 {
        self.image_height
    }

    pub fn sample(&self, x: u32, y: u32) -> (u8, u8, u8) {
        let x = x.clamp(0, self.image_width - 1);
        let y = y.clamp(0, self.image_height - 1);

        let index = ((y * self.image_width + x) * Self::BYTES_PER_PIXEL) as usize;
        (
            self.pixels[index],
            self.pixels[index + 1],
            self.pixels[index + 2],
        )
    }

    const BYTES_PER_PIXEL: u32 = 3;
}
