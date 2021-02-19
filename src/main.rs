extern crate minifb;

use bevy_math::{Mat3, Vec2};
use std::{error::Error, time::Instant};
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 600;
const HEIGHT: usize = 600;

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
    fn get_distance(&self, arena: &Vec<Object>, point: Vec2) -> f32;
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
        let r = (item.0 * 255.0) as u32;
        let g = (item.1 * 255.0) as u32;
        let b = (item.2 * 255.0) as u32;

        ((r & 0xFF) << 16) | ((g & 0xFF) << 8) | (b & 0xFF)
    }
}

struct Layer {
    color: Color,
    shape: usize,
}

#[derive(Debug, Clone, Copy)]
struct Transform {
    x: f32,
    y: f32,
    rotation: f32,
    scale: f32,
}

impl Transform {
    fn apply(&mut self, transform: Transform) {
        self.x += transform.x;
        self.y += transform.y;
        self.rotation += transform.rotation;
        self.scale *= transform.scale;
    }

    fn new() -> Transform {
        Transform {
            x: 0.0,
            y: 0.0,
            rotation: 0.0,
            scale: 1.0,
        }
    }

    fn to_matrix(&self) -> Mat3 {
        Mat3::from_scale_angle_translation(
            Vec2::new(1.0, 1.0),
            -self.rotation.to_radians(),
            Vec2::new(self.x, self.y),
        )
    }

    fn map(&self, point: Vec2) -> Vec2 {
        // Translate
        let translate = Vec2::new(self.x as f32, self.y as f32);
        let point = point - translate;

        // Rotate
        let radians = self.rotation.to_radians();
        let sin = radians.sin();
        let cos = radians.cos();
        let point = Vec2::new(
            cos * point.x - sin * point.y,
            sin * point.x + cos * point.y,
        );

        // Get scaled distance
        point / self.scale
    }
}

struct Object {
    transform: Transform,
    distortion: Vec<Box<dyn SDF>>,
    parent_id: Option<usize>,
    sdf: Box<dyn SDF>,
}

impl SDF for Object {
    fn get_distance(&self, arena: &Vec<Object>, point: Vec2) -> f32 {
        // Direct
        let point = self.transform.map(point);
        // TODO: Apply distortion
        self.sdf.get_distance(arena, point) * self.transform.scale

        // Matrix
        // let point = self.transform.to_matrix().inverse() * point.extend(1.0);
        // self.sdf.get_distance(arena, point.xy() / self.transform.scale) * self.transform.scale
    }
}

struct Circle {
    radius: f32,
}

impl SDF for Circle {
    fn get_distance(&self, _arena: &Vec<Object>, point: Vec2) -> f32 {
        point.length() - self.radius
    }
}

struct Square {
    size: Vec2,
}

impl SDF for Square {
    fn get_distance(&self, _arena: &Vec<Object>, point: Vec2) -> f32 {
        let d = point.abs() - self.size;
        let a = Vec2::new(d.x.max(0.0), d.y.max(0.0));

        a.length() + d.x.max(d.y).min(0.0)
    }
}

struct OpSmoothUnion {
    sdf_1: usize,
    sdf_2: usize,
    fuzz: f32,
}

impl SDF for OpSmoothUnion {
    fn get_distance(&self, arena: &Vec<Object>, point: Vec2) -> f32 {
        let distance_1 = arena[self.sdf_1].get_distance(arena, point);
        let distance_2 = arena[self.sdf_2].get_distance(arena, point);

        let h = (0.5 + 0.5 * (distance_2 - distance_1) / self.fuzz).clamp(0.0, 1.0 );
        return lerp(distance_2, distance_1, h) - self.fuzz * h * (1.0 - h);
    }
}

fn get_debug_transform(mut parent_id: usize, arena: &Vec<Object>) -> Transform {
    let mut transforms: Vec<Transform> = vec![];
    let mut debug_transform = Transform::new();

    loop {
        transforms.push(arena[parent_id].transform);

        match arena[parent_id].parent_id {
            Some(id) => parent_id = id,
            None => break,
        }
    }

    transforms = transforms.into_iter().rev().collect(); // Reverse transforms
    transforms.pop(); // Remove transform corresponding to the selected object

    for transform in transforms {
        debug_transform.apply(transform);
    }

    debug_transform
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut is_debug = true;
    let mut selected_id = 0;
    let mut window = Window::new(
        "2D Signal Distance Fields - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default()
    )?;

    // Limit to max 60fpx update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16_600)));

    let start_time = Instant::now();
    let mut objects = vec![
        // 0
        Object {
            transform: Transform {
                x: 0.0,
                y: 0.0,
                rotation: 0.0,
                scale: 1.0,
            },
            distortion: Vec::new(),
            parent_id: None,
            sdf: Box::new(OpSmoothUnion {
                sdf_1: 1,
                sdf_2: 2,
                fuzz: 25.0,
            })
        },
        // 1
        Object {
            transform: Transform {
                x: 0.0,
                y: 0.0,
                rotation: 0.0,
                scale: 1.0,
            },
            distortion: Vec::new(),
            parent_id: Some(0),
            sdf: Box::new(Circle {
                radius: 50.0,
            })
        },
        // 2
        Object {
            transform: Transform {
                x: 0.0,
                y: 0.0,
                rotation: 0.0,
                scale: 1.0,
            },
            distortion: Vec::new(),
            parent_id: Some(0),
            sdf: Box::new(Square {
                size: Vec2::new(100.0, 10.0)
            })
        },
        // 3
        Object {
            transform: Transform {
                x: 0.0,
                y: (HEIGHT / 4) as f32,
                rotation: 15.0,
                scale: 1.0,
            },
            distortion: Vec::new(),
            parent_id: None,
            sdf: Box::new(Square {
                size: Vec2::new(10.0, 100.0)
            })
        }
    ];

    let layers = vec![
        Layer {
            color: Color(0.0, 1.0, 1.0, 1.0),
            shape: 3,
        },
        Layer {
            color: Color(1.0, 0.0, 0.0, 1.0),
            shape: 0,
        },
    ];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.get_keys_pressed(minifb::KeyRepeat::Yes).map(|keys| {
            for t in keys {
                match t {
                    Key::D => is_debug = !is_debug,
                    Key::Up => selected_id = (selected_id + 1) % objects.len(),
                    Key::Down => selected_id = (selected_id - 1 + objects.len()) % objects.len(),
                    Key::NumPad8 => objects[selected_id].transform.y += 5.0,
                    Key::NumPad5 => objects[selected_id].transform.y -= 5.0,
                    Key::NumPad4 => objects[selected_id].transform.x -= 5.0,
                    Key::NumPad6 => objects[selected_id].transform.x += 5.0,
                    Key::NumPad7 => objects[selected_id].transform.rotation -= 5.0,
                    Key::NumPad9 => objects[selected_id].transform.rotation += 5.0,
                    Key::NumPad1 => objects[selected_id].transform.scale -= 0.2,
                    Key::NumPad3 => objects[selected_id].transform.scale += 0.2,
                    Key::NumPad2 => println!("{:?}", objects[selected_id].transform),
                    _ => (),
                }
            }
        });

        let time = start_time.elapsed().as_millis() as f32 / 1000.0;

        // Update first object
        objects[0] = Object {
            transform: Transform {
                rotation: time * 5.0,
                scale: 1.0 + ((time * 2.0).sin() * 0.25),
                ..objects[0].transform
            },
            parent_id: objects[0].parent_id,
            distortion: Vec::new(),
            sdf: Box::new(OpSmoothUnion {
                sdf_1: 1,
                sdf_2: 2,
                fuzz: 25.0 + ((time * 2.0).sin() * 20.0),
            }),
        };

        // Selected parents transforms tree
        let debug_transform = get_debug_transform(selected_id, &objects);

        // Render
        buffer
            .chunks_mut(WIDTH)
            .enumerate()
            .for_each(|(j, chunk)| {
                for i in 0..WIDTH {
                    let mut color = Color(0.0, 0.0, 0.0, 1.0);
                    let point = Vec2::new(
                        i as f32 - (WIDTH as f32 / 2.0),
                        (HEIGHT as f32 / 2.0) - j as f32
                    );

                    for layer in (&layers).into_iter().rev() {
                        let distance = objects[layer.shape].get_distance(&objects, point);
                        color = layer.color.mix(color, distance);
                    }

                    // Draw debug elements
                    if is_debug {
                        let point = debug_transform.map(point.clone());
                        let distance = objects[selected_id].get_distance(&objects, point);
                        let border_width = 2.0;

                        color = color.mix(
                            Color(1.0, 1.0, 0.0, 1.0), 
                            smoothstep(0.0, border_width, distance) - smoothstep(border_width, border_width * 2.0, distance),
                        );
                    }

                    chunk[i] = color.into();
                }
            });

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
