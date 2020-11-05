mod canvas;
#[macro_use]
mod math;
mod intersection;
mod objects;

use crate::math::{Color, Mat4, Point4, Vec3, Vec4};

use crate::intersection::{hit, intersect};
use crate::objects::{Ray, SphereManager};
use canvas::Canvas;

const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;

fn main() {
    // let start = Point::new(0.0, 1.0, 0.0);
    // let v = Vec3::new(1.0, 1.8, 0.0).normalize() * 11.25;

    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    let mid_x = canvas.width() as f32 / 2.0;
    let mid_y = canvas.height() as f32 / 2.0;

    let ray_z = -5.0;
    let ray_origin = point!(0.0, 0.0, ray_z);

    let wall_z = 10.0;
    let wall_size = 7.0;
    let pixel_size = wall_size / WIDTH as f32;
    let half = wall_size as f32 / 2.0;

    let color = Color::red();
    let mut sm = SphereManager::new();
    let (_, sphere) = sm.create_sphere();

    sphere
        .transform(&scaling!(1.0, 1.0, 1.0))
        .transform(&shear!(0.5, 0.5, 0.5, 0.0, 0.0, 0.0));

    for y in 0..HEIGHT as i32 {
        let world_y = half - pixel_size * y as f32;
        for x in 0..WIDTH as i32 {
            let world_x = -half + pixel_size * x as f32;
            let position = point!(world_x as f32, world_y as f32, wall_z);
            let ray = Ray::new(ray_origin.clone(), (position - &ray_origin).normalize());
            if let Some(hit) = intersect(&ray, sphere).as_ref().and_then(hit) {
                canvas.set_pixel(x as u32, y as u32, color);
            }
        }
    }

    canvas.to_file("test_file.png");
}
