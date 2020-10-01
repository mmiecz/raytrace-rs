mod vec;

use image::{EncodableLayout, ImageBuffer, RgbImage};
use std::io::Write;

const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;
fn main() {
    let image = ImageBuffer::from_fn(WIDTH, HEIGHT, |x, y| {
        let y = HEIGHT - y; // bottom up;
        let r: f32 = x as f32 / (WIDTH as f32 - 1.0);
        let g: f32 = y as f32 / (HEIGHT as f32 - 1.0);
        let b: f32 = 0.25;
        let mut stdout = std::io::stdout();
        stdout.lock();
        stdout.write(format!("\rScanlines remaining: {}", y).as_bytes());
        image::Rgb([
            (255.999 * r) as u8,
            (255.999 * g) as u8,
            (255.999 * b) as u8,
        ])
    });
    image.save("output.png");
}
