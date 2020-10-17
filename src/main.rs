mod types;
mod canvas;

use canvas::Canvas;
use crate::types::Color;

const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;

fn main() {
    let mut canvas = Canvas::new(512, 512);
    canvas.set_pixel(128, 128, Color::red());
    canvas.set_pixel(256, 256, Color::green());
    canvas.set_pixel(511, 511, Color::blue());

    canvas.to_file("test_file.png");
}
