extern crate minifb;

use std::{convert::TryFrom, error::Error};
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
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

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                buffer[x + y * WIDTH] = from_u8_rgb(u8::try_from(x % 256)?, u8::try_from(y % 256)?, 0);
            }
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT)?;
    }

    Ok(())
}
