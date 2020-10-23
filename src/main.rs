mod canvas;
mod types;

use crate::types::{Color, Point3, Vec3};
use canvas::Canvas;

const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;

#[derive(Debug)]
struct Projectile {
    position: Point3,
    velocity: Vec3,
}

struct Environment {
    gravity: Vec3,
    wind: Vec3,
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;
    Projectile { position, velocity }
}

fn main() {
    // let start = Point::new(0.0, 1.0, 0.0);
    // let v = Vec3::new(1.0, 1.8, 0.0).normalize() * 11.25;

    let mut canvas = Canvas::new(900, 550);
    let mut projectile = Projectile {
        position: Point3::new(0.0, 10.0, 0.0),
        velocity: Vec3::new(1.0, 1.8, 0.0).normalize() * 11.25,
    };
    let env = Environment {
        gravity: Vec3::new(0.0, -0.1, 0.0),
        wind: Vec3::new(-0.00, 0.0, 0.0),
    };

    while projectile.position.x < canvas.width() as f32
        && projectile.position.y < canvas.height() as f32
        && projectile.position.x >= 0 as f32
        && projectile.position.y >= 0 as f32
    {
        canvas.set_pixel(
            projectile.position.x as u32,
            canvas.height() - projectile.position.y as u32,
            Color::red(),
        );
        projectile = tick(&env, &projectile);
        println!("pos: {:?}", projectile);
    }
    canvas.to_file("test_file.png");
}
