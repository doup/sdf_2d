use crate::gui::Gui;
use log::error;
use pixels::{Pixels, SurfaceTexture};
use winit::{dpi::LogicalSize};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod gui;

use bevy_math::{Vec2};
use rayon::prelude::*;
use std::{error::Error, sync::Arc, time::Instant};

// Project modules
mod color;
mod distortion;
mod font;
mod utils;
mod sdf;
mod transform;

use color::*;
use distortion::*;
use font::*;
use utils::*;
use sdf::{*, color::{Border, BorderPosition, Fill, LayerColor, SDFColor}};
use transform::*;

// Main
const WIDTH: usize = 600;
const HEIGHT: usize = 600;

struct Layer {
    color: LayerColor,
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

struct World<'a> {
    font: Arc<Font>,
    layers: Vec<Layer>,
    objects: Vec<Object<'a>>,
    selected_id: usize,
    is_debug: bool,
    is_initialized: bool,
    debug_transform: Transform,
}

impl<'a> World<'a> {
    fn new() -> World<'a> {
        World {
            font: Arc::new(get_comic_sans()),
            layers: vec![],
            objects: vec![],
            selected_id: 0,
            is_debug: true,
            is_initialized: false,
            debug_transform: Transform::new(),
        }
    }

    fn init(&mut self) {
        if self.is_initialized {
            return;
        }

        self.is_initialized = true;

        self.objects = vec![
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
                    y: (HEIGHT / 6) as f32,
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
                        width: WIDTH as f32,
                        height: HEIGHT as f32,
                        x_amplitude: 20.0,
                        x_freq: 100.0,
                        y_amplitude: 1.0,
                        y_freq: 1.0,
                        time: 1.0,
                    })
                ],
                parent_id: None,
                sdf: Box::new(primitive::Circle {
                    radius: 100.0,
                })
            },
            // 5
            Object {
                transform: Transform {
                    x: -150.0,
                    y: -65.0,
                    rotation: 0.0,
                    scale: 1.2,
                },
                distortion: vec![
                    Box::new(Wave {
                        width: WIDTH as f32,
                        height: HEIGHT as f32,
                        x_amplitude: 20.0,
                        x_freq: 100.0,
                        y_amplitude: 1.0,
                        y_freq: 1.0,
                        time: 1.0,
                    })
                ],
                parent_id: None,
                sdf: Box::new(primitive::Text::new(String::from("Hello world! :-)"), 32.0, Arc::clone(&self.font)))
                // sdf: Box::new(primitive::Circle { radius: 10.0 })
            },
            // 6
            Object {
                transform: Transform {
                    x: -100.0,
                    y: 100.0,
                    rotation: 0.0,
                    scale: 1.0,
                },
                distortion: Vec::new(),
                parent_id: None,
                sdf: Box::new(primitive::Circle {
                    radius: 3.0,
                })
            },
        ];
    
        self.layers = vec![
            Layer {
                shape: 6,
                color: LayerColor {
                    inside: Some(Fill::Solid(Color::new(1.0, 0.0, 0.0, 1.0))),
                    border: None,
                    outside: None,
                },
            },
            Layer {
                shape: 5, // Text
                color: LayerColor {
                    inside: Some(Fill::Solid(Color::new(1.0, 1.0, 1.0, 1.0))),
                    border: None,
                    outside: None,
                },
            },
            Layer {
                shape: 3,
                color: LayerColor {
                    inside: Some(Fill::Solid(Color::new(0.0, 1.0, 1.0, 1.0))),
                    border: Some(Border {
                        position: BorderPosition::Outside,
                        size: 5.0,
                        color: Color::new(1.0, 0.75, 0.1, 1.0),
                    }),
                    outside: None,// Some(Fill::Solid(Color(1.0, 0.0, 0.0, 1.0))),
                },
            },
            Layer {
                shape: 0,
                color: LayerColor {
                    inside: Some(Fill::Solid(Color::new(1.0, 0.0, 0.0, 1.0))),
                    border: None,
                    outside: None,
                },
            },
            Layer {
                shape: 4,
                color: LayerColor {
                    inside: None,
                    border: Some(Border {
                        position: BorderPosition::Outside,
                        size: 10.0,
                        color: Color::new(1.0, 0.0, 0.0, 1.0),
                    }),
                    outside: None,
                },
            },
        ];
    }

    fn render(&self, frame: &mut [u8]) {
        // Render
        frame
            .par_chunks_mut((WIDTH * 4) as usize)
            .enumerate()
            .for_each(|(j, row)| {
                for (i, pixel) in row.chunks_exact_mut(4).enumerate() {
                    let mut color = Color::new(0.0, 0.0, 0.0, 0.0); // First invisible layer at the top
                    let point = Vec2::new(
                        i as f32 - (WIDTH as f32 / 2.0),
                        (HEIGHT as f32 / 2.0) - j as f32
                    );

                    // Draw layer top to bottom
                    for layer in &self.layers {
                        let distance = self.objects[layer.shape].get_distance(&self.objects, point);

                        // Mix front color with layer color
                        let back_color = layer.color.get_color(distance);
                        color = back_color.mix(&color);

                        // Alpha check to skip below layers
                        if color.a >= 1.0 {
                            break;
                        }
                    }

                    // Add black background
                    color = (Color::new(0.0, 0.0, 0.0, 1.0)).mix(&color);

                    // Draw debug elements
                    if self.is_debug {
                        let point = self.debug_transform.map(point.clone());
                        let distance = self.objects[self.selected_id].get_distance(&self.objects, point);
                        let border_width = 2.0;
                        let alpha = smoothstep(0.0, border_width, distance) - smoothstep(border_width, border_width * 2.0, distance);
                        let debug_color = Color::new(1.0, 1.0, 0.0, alpha);

                        color = color.mix(&debug_color);
                    }

                    pixel.copy_from_slice(&color.to_array());
                }
            });
    }

    fn update(&mut self, time: f32) {
        // if input.key_pressed(VirtualKeyCode::D) {
        //     is_debug = !is_debug;
        // }
        //
        // if input.key_pressed(VirtualKeyCode::Up) {
        //     selected_id = (selected_id + 1) % objects.len();
        // }
        //
        // if input.key_pressed(VirtualKeyCode::Down) {
        //     selected_id = (selected_id - 1 + objects.len()) % objects.len();
        // }
        //
        // input.get_keys_pressed().map(|keys| {
        //     for t in keys {
        //         match t {
        //             Key::NumPad8 => objects[selected_id].transform.y += 5.0,
        //             Key::NumPad5 => objects[selected_id].transform.y -= 5.0,
        //             Key::NumPad4 => objects[selected_id].transform.x -= 5.0,
        //             Key::NumPad6 => objects[selected_id].transform.x += 5.0,
        //             Key::NumPad7 => objects[selected_id].transform.rotation -= 5.0,
        //             Key::NumPad9 => objects[selected_id].transform.rotation += 5.0,
        //             Key::NumPad1 => objects[selected_id].transform.scale -= 0.2,
        //             Key::NumPad3 => objects[selected_id].transform.scale += 0.2,
        //             Key::NumPad2 => println!("{:?}", objects[selected_id].transform),
        //             _ => (),
        //         }
        //     }
        // });

        // Selected parents transforms tree
        self.debug_transform = get_debug_transform(self.selected_id, &self.objects);

        // Update first object
        self.objects[0] = Object {
            transform: Transform {
                rotation: time * 5.0,
                scale: 1.0 + ((time * 2.0).sin() * 0.25),
                ..self.objects[0].transform
            },
            parent_id: self.objects[0].parent_id,
            distortion: Vec::new(),
            sdf: Box::new(operator::OpSmoothUnion {
                sdf_1: 1,
                sdf_2: 2,
                fuzz: 25.0 + ((time * 2.0).sin() * 20.0),
            }),
        };

        // Animate purple-circle wave distortion
        self.objects[4].distortion[0] = Box::new(Wave {
            width: WIDTH as f32,
            height: HEIGHT as f32,
            x_amplitude: 11.0 + ((time * 1.5).sin() * 10.0),
            x_freq: 51.0 + ((time * 2.5).sin() * 50.0),
            y_amplitude: 11.0 + ((time * 0.5).sin() * 10.0),
            y_freq: 21.0 + ((time * 0.25).sin() * 20.0),
            time: 1.0,
        });

        // Animate text wave distortion
        self.objects[5].distortion[0] = Box::new(Wave {
            width: WIDTH as f32,
            height: HEIGHT as f32,
            x_amplitude: 1.0,
            x_freq: 1.0,
            y_amplitude: 10.0,
            y_freq: 20.0,
            time: time,
        });
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    let mut world = World::new();
    let mut frame = 0;

    world.init();
    env_logger::init();

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        WindowBuilder::new()
            .with_title("2D Signal Distance Fields - ESC to exit")
            .with_maximized(true)
            .with_min_inner_size(LogicalSize::new(WIDTH as f64, HEIGHT as f64))
            .build(&event_loop)
            .unwrap()
    };

    let (mut pixels, mut gui) = {
        let window_size = window.inner_size();
        let scale_factor = window.scale_factor();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let pixels = Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?;
        let gui = Gui::new(
            window_size.width,
            window_size.height,
            scale_factor,
            pixels.context(),
        );

        (pixels, gui)
    };

    event_loop.run(move |event, _, control_flow| {
        // Update egui inputs
        gui.handle_event(&event);

        let time = start_time.elapsed().as_millis() as f32 / 1000.0;
        let fps = ((frame as f32) / time) as u32;

        world.update(time);

        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            // Draw the world
            let frame = pixels.get_frame();

            world.render(frame);

            // Prepare egui
            gui.prepare();

            // Render everything together
            let render_result = pixels.render_with(|encoder, render_target, context| {
                context.scaling_renderer.render(encoder, render_target); // Render the world texture
                gui.render(encoder, render_target, context); // Render egui
            });

            // Basic error handling
            if render_result.map_err(|e| error!("pixels.render() failed: {}", e)).is_err() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Update the scale factor
            if let Some(scale_factor) = input.scale_factor() {
                gui.scale_factor(scale_factor);
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
                gui.resize(size.width, size.height);
            }

            // Update internal state and request a redraw
            window.set_title(&format!("2D Signal Distance Fields - ESC to exit - {}FPS", fps));
            window.request_redraw();
            frame += 1;
        }
    });
}
