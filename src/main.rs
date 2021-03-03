extern crate minifb;

use bevy_math::{Vec2};
use std::{collections::HashMap, error::Error, time::Instant};
use minifb::{Key, Window, WindowOptions};
use rayon::prelude::*;

// Project modules
mod color;
mod distortion;
mod utils;
mod sdf;
mod transform;

use color::*;
use distortion::*;
use utils::*;
use sdf::*;
use transform::*;

// Main
const WIDTH: usize = 600;
const HEIGHT: usize = 600;

struct Char {
    width: f32,
    height: f32,
    x: f32,
    x_advance: f32,
    x_offset: f32,
    y: f32,
    y_offset: f32,
}

struct Font {
    line_height: f32,
    base: f32,
    scale_width: f32,
    scale_height: f32,
    chars: HashMap<u8, Char>,
}

struct Layer {
    color: Color,
    shape: usize,
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
    let mut frame = 0;
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
            sdf: Box::new(operator::OpSmoothUnion {
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
            sdf: Box::new(primitive::Circle {
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
            sdf: Box::new(primitive::Square {
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
            sdf: Box::new(primitive::Square {
                size: Vec2::new(10.0, 100.0)
            })
        },
        // 4
        Object {
            transform: Transform {
                x: 0.0,
                y: -100.0,
                rotation: 0.0,
                scale: 1.0,
            },
            distortion: vec![
                Box::new(Wave {
                    x_amplitude: 20.0,
                    x_freq: 100.0,
                    y_amplitude: 1.0,
                    y_freq: 1.0,
                })
            ],
            parent_id: None,
            sdf: Box::new(primitive::Circle {
                radius: 100.0,
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
        Layer {
            color: Color(0.5, 0.1, 1.0, 1.0),
            shape: 4,
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
        let fps = ((frame as f32) / time) as u32;

        // Update first object
        objects[0] = Object {
            transform: Transform {
                rotation: time * 5.0,
                scale: 1.0 + ((time * 2.0).sin() * 0.25),
                ..objects[0].transform
            },
            parent_id: objects[0].parent_id,
            distortion: Vec::new(),
            sdf: Box::new(operator::OpSmoothUnion {
                sdf_1: 1,
                sdf_2: 2,
                fuzz: 25.0 + ((time * 2.0).sin() * 20.0),
            }),
        };

        // Animate wave distortion
        objects[4].distortion[0] = Box::new(Wave {
            x_amplitude: 11.0 + ((time * 1.5).sin() * 10.0),
            x_freq: 51.0 + ((time * 2.5).sin() * 50.0),
            y_amplitude: 11.0 + ((time * 0.5).sin() * 10.0),
            y_freq: 21.0 + ((time * 0.25).sin() * 20.0),
        });

        // Selected parents transforms tree
        let debug_transform = get_debug_transform(selected_id, &objects);

        // Render
        buffer
            .par_chunks_mut(WIDTH)
            .enumerate()
            .for_each(|(j, chunk)| {
                for i in 0..WIDTH {
                    let mut color = Color(0.0, 0.0, 0.0, 0.0); // Background color
                    let point = Vec2::new(
                        i as f32 - (WIDTH as f32 / 2.0),
                        (HEIGHT as f32 / 2.0) - j as f32
                    );

                    for layer in &layers {
                        let distance = objects[layer.shape].get_distance(&objects, point);
                        color = layer.color.mix(color, distance);

                        // Alpha check to skip below layers
                        if color.3 == 1.0 {
                            break;
                        }
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

        window.set_title(&format!("2D Signal Distance Fields - ESC to exit - {}FPS", fps));
        window.update_with_buffer(&buffer, WIDTH, HEIGHT)?;
        frame += 1;
    }

    Ok(())
}
