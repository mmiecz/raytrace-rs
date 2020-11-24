use crate::math::Color;
use std::error::Error;
use std::path::Path;

pub(crate) struct Canvas {
    width: u32,
    height: u32,
    pixels: Vec<[u8; 3]>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        Canvas {
            width,
            height,
            pixels: vec![[0, 0, 0]; (width * height) as usize],
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        //TODO: This conversion is bad; fix this;
        let r = (color.r() * 255.0) as u8;
        let g = (color.g() * 255.0) as u8;
        let b = (color.b() * 255.0) as u8;
        assert!(x < self.width);
        assert!(y < self.height);
        self.pixels[(y * self.width + x) as usize] = [r, g, b];
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> image::error::ImageResult<()> {
        let mut imgbuf = image::ImageBuffer::new(self.width, self.height);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            //TODO: Check if there is a better way to convert float to byte.
            *pixel = image::Rgb(self.pixels[(y * self.width + x) as usize])
        }
        imgbuf.save(path)
    }
}
