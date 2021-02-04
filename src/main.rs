extern crate minifb;

use bevy_math::Vec2;
use std::{error::Error};
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

trait SDF {
    fn get_distance(&self, point: Vec2) -> f32;
}

struct Circle {
    radius: f32,
}

struct Primitive {
    x: f32,
    y: f32,
    color: u32,
    sdf: Box<dyn SDF>,
}

impl SDF for Circle {
    fn get_distance(&self, point: Vec2) -> f32 {
        point.length() - self.radius
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new(
        "2D Signal Distance Fields - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default()
    )?;

    // Limit to max 60fpx update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16_600)));

    let circle_sdf = Circle { radius: 50.0 };
    let thing = Primitive {
        x: (WIDTH / 2) as f32,
        y: (HEIGHT / 2) as f32,
        color: from_u8_rgb(255, 0, 0),
        sdf: Box::new(circle_sdf),
    };

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let point = Vec2::new(x as f32, y as f32);
                let transform = Vec2::new(thing.x as f32, thing.y as f32);
                let distance = thing.sdf.get_distance(point - transform);

                if distance > 0.0 {
                    buffer[x + y * WIDTH] = from_u8_rgb(0, 0, 0);
                } else {
                    buffer[x + y * WIDTH] = thing.color;
                }
            }
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT)?;
    }

    Ok(())
}
