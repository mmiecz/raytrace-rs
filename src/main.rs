mod canvas;
#[macro_use]
mod math;
mod intersection;
mod objects;

use crate::math::{Color, Mat4, Point4, Vec3, Vec4};

use canvas::Canvas;

const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;

fn main() {
    // let start = Point::new(0.0, 1.0, 0.0);
    // let v = Vec3::new(1.0, 1.8, 0.0).normalize() * 11.25;

    let mut canvas = Canvas::new(900, 550);

    let mid_x = canvas.width() as f32 / 2.0;
    let mid_y = canvas.height() as f32 / 2.0;
    for t in 0..12 {
        let point = point!(0.0, mid_y / 1.2, 0.0);
        let rotation = rotation!(0.0, 0.0, t as f32 * std::f32::consts::PI / 6.0);
        let point = translation!(mid_x, mid_y, 0.0) * rotation * point;
        let point = canvas.set_pixel(point.x as u32, point.y as u32, Color::red());
    }

    canvas.to_file("test_file.png");
}
