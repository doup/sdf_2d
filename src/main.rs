extern crate minifb;

use bevy_math::Vec2;
use std::{error::Error, time::Instant};
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 600;
const HEIGHT: usize = 600;

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    ((r & 0xFF) << 16) | ((g & 0xFF) << 8) | (b & 0xFF)
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    a * (1.0 - t) + b * t
}

/// Given a `a`, `b` & `t`. Where `t` is between `a` & `b`
/// it returns a value between `0.0` and `1.0`. When `t=a` then
/// the value is `0.0`, when `t=b` the value is `1.0`. 
fn smoothstep(a: f32, b: f32, t: f32) -> f32 {
    if a == b {
        a
    } else {
        let t = ((t - a) / (b - a)).clamp(0.0, 1.0);
        t * t * (3.0 - 2.0 * t)
    }
}

trait SDF {
    fn get_distance(&self, point: Vec2) -> f32;
}

#[derive(Debug)]
struct Color(f32, f32, f32, f32);

impl Color {
    fn mix(&self, color: Color, t: f32) -> Color {
        Color(
            lerp(self.0, color.0, t),
            lerp(self.1, color.1, t),
            lerp(self.2, color.2, t),
            lerp(self.3, color.3, t),
        )
    }

    fn mix_smooth(&self, color: Color, t: f32) -> Color {
        Color(
            smoothstep(self.0, color.0, t),
            smoothstep(self.1, color.1, t),
            smoothstep(self.2, color.2, t),
            smoothstep(self.3, color.3, t),
        )
    }
}

impl From<Color> for u32 {
    fn from(item: Color) -> Self {
        from_u8_rgb(
            (item.0 * 255.0) as u8,
            (item.1 * 255.0) as u8,
            (item.2 * 255.0) as u8,
        )
    }
}

struct Layer {
    color: Color,
    shape: Object,
}

struct Object {
    x: f32,
    y: f32,
    rotation: f32,
    scale: f32,
    distortion: Vec<Box<dyn SDF>>,
    sdf: Box<dyn SDF>,
}

impl SDF for Object {
    fn get_distance(&self, point: Vec2) -> f32 {
        // Translate
        let translate = Vec2::new(self.x as f32, self.y as f32);
        let point = point - translate;

        // Rotate
        let radians = self.rotation.to_radians();
        let sin = radians.sin();
        let cos = radians.cos();
        let point = Vec2::new(
            cos * point.x + sin * point.y,
            cos * point.y - sin * point.x,
        );
        
        // TODO: Apply distortion

        // Get scaled distance
        self.sdf.get_distance(point / self.scale) * self.scale
    }
}

struct Circle {
    radius: f32,
}

impl SDF for Circle {
    fn get_distance(&self, point: Vec2) -> f32 {
        point.length() - self.radius
    }
}

struct Square {
    size: Vec2,
}

impl SDF for Square {
    fn get_distance(&self, point: Vec2) -> f32 {
        let d = point.abs() - self.size;
        let a = Vec2::new(d.x.max(0.0), d.y.max(0.0));

        a.length() + d.x.max(d.y).min(0.0)
    }
}

struct OpSmoothUnion {
    sdf_1: Box<dyn SDF>,
    sdf_2: Box<dyn SDF>,
    fuzz: f32,
}

impl SDF for OpSmoothUnion {
    fn get_distance(&self, point: Vec2) -> f32 {
        let distance_1 = self.sdf_1.get_distance(point);
        let distance_2 = self.sdf_2.get_distance(point);

        let h = (0.5 + 0.5 * (distance_2 - distance_1) / self.fuzz).clamp(0.0, 1.0 );
        return lerp(distance_2, distance_1, h) - self.fuzz * h * (1.0 - h);
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

    let start_time = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let time = start_time.elapsed().as_millis() as f32 / 1000.0;
        let circle_sdf = Circle { radius: 50.0 };
        let square_sdf = Square { size: Vec2::new(100.0, 10.0) };
        let layers = vec![
            Layer {
                color: Color(1.0, 0.0, 0.0, 1.0),
                shape: Object {
                    x: (WIDTH / 2) as f32,
                    y: (HEIGHT / 2) as f32,
                    rotation: time * 5.0,
                    scale: 1.0 + ((time * 2.0).sin() * 0.25),
                    distortion: Vec::new(),
                    sdf: Box::new(
                        OpSmoothUnion {
                            sdf_1: Box::new(circle_sdf),
                            sdf_2: Box::new(square_sdf),
                            fuzz: 25.0 + ((time * 2.0).sin() * 20.0),
                        }
                    ),
                }
            }
        ];

        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let point = Vec2::new(x as f32, y as f32);

                for layer in &layers {
                    let distance = layer.shape.get_distance(point);
                    buffer[x + y * WIDTH] = layer.color.mix_smooth(Color(0.0, 0.0, 0.0, 1.0), distance).into();
                }
            }
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lerp() {
        // Test clamping
        assert_eq!(lerp(0.0, 1.0, -1.0), 0.0);
        assert_eq!(lerp(0.0, 1.0, 2.0), 1.0);

        // 0.0 to 1.0
        assert_eq!(lerp(0.0, 1.0, 0.0), 0.0);
        assert_eq!(lerp(0.0, 1.0, 0.25), 0.25);
        assert_eq!(lerp(0.0, 1.0, 0.5), 0.5);
        assert_eq!(lerp(0.0, 1.0, 0.75), 0.75);
        assert_eq!(lerp(0.0, 1.0, 1.0), 1.0);

        // 1.0 to 0.0
        assert_eq!(lerp(1.0, 0.0, 0.25), 0.75);
        assert_eq!(lerp(1.0, 0.0, 0.5), 0.5);
        assert_eq!(lerp(1.0, 0.0, 0.75), 0.25);
    }

    #[test]
    fn test_smoothstep() {
        assert_eq!(smoothstep(0.0, 10.0, -5.0), 0.0);
        assert_eq!(smoothstep(0.0, 10.0,  0.0), 0.0);
        assert_eq!(smoothstep(0.0, 10.0,  5.0), 0.5);
        assert_eq!(smoothstep(0.0, 10.0, 10.0), 1.0);
        assert_eq!(smoothstep(0.0, 10.0, 15.0), 1.0);

        assert_eq!(smoothstep(10.0, 0.0, 15.0), 0.0);
        assert_eq!(smoothstep(10.0, 0.0, 10.0), 0.0);
        assert_eq!(smoothstep(10.0, 0.0,  5.0), 0.5);
        assert_eq!(smoothstep(10.0, 0.0,  0.0), 1.0);
        assert_eq!(smoothstep(10.0, 0.0, -5.0), 1.0);

        assert_eq!(smoothstep(-10.0, -20.0,  -5.0), 0.0);
        assert_eq!(smoothstep(-10.0, -20.0, -10.0), 0.0);
        assert_eq!(smoothstep(-10.0, -20.0, -15.0), 0.5);
        assert_eq!(smoothstep(-10.0, -20.0, -20.0), 1.0);
        assert_eq!(smoothstep(-10.0, -20.0, -25.0), 1.0);
    }
}
