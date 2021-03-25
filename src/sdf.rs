use bevy_math::Vec2;

use crate::distortion::*;
use crate::transform::*;
use crate::utils::*;

pub trait SDF {
    fn get_distance(&self, arena: &Vec<Object>, point: Vec2) -> f32;
}

pub struct Object<'a> {
    pub transform: Transform,
    // Meaning:
    // - `Box<T>`: Boxed value, stored in heap
    // - `dyn Distortion`: Implements Distorsion Trait, but we don't know which implementation
    // - `Sync + Send`: Mark with Sync/Send to tell the compiler that it's OK for concurrency
    // - `'a`: Specify lifetime, boxed value will live as long as `Object`, otherwise `Box` defaults to `'static`
    pub distortion: Vec<Box<dyn Distorsion + Sync + Send + 'a>>,
    pub parent_id: Option<usize>,
    pub sdf: Box<dyn SDF + Sync + Send + 'a>,
}

impl<'a> SDF for Object<'a> {
    fn get_distance(&self, arena: &Vec<Object>, point: Vec2) -> f32 {
        // Transform point
        let mut point = self.transform.map(point);

        // Apply distortion
        for dist in &self.distortion {
            point = dist.map(point);
        }

        self.sdf.get_distance(arena, point) * self.transform.scale

        // Matrix
        // let point = self.transform.to_matrix().inverse() * point.extend(1.0);
        // self.sdf.get_distance(arena, point.xy() / self.transform.scale) * self.transform.scale
    }
}

pub mod primitive {
    use crate::font::{Char, Font};

    use super::*;

    pub struct Circle {
        pub radius: f32,
    }

    impl SDF for Circle {
        fn get_distance(&self, _arena: &Vec<Object>, point: Vec2) -> f32 {
            point.length() - self.radius
        }
    }

    pub struct Square {
        pub size: Vec2,
    }

    impl SDF for Square {
        fn get_distance(&self, _arena: &Vec<Object>, point: Vec2) -> f32 {
            let d = point.abs() - self.size;
            let a = Vec2::new(d.x.max(0.0), d.y.max(0.0));

            a.length() + d.x.max(d.y).min(0.0)
        }
    }

    pub struct Text<'a> {
        bboxes: Vec<BBox<'a>>,
        font: &'a Font,
        size: f32,
        text: String,
    }

    #[derive(Debug)]
    struct BBox<'a> {
        char: &'a Char,
        pos: Vec2,
        size: Vec2,
    }

    impl<'a> BBox<'a> {
        pub fn contains(&self, point: Vec2) -> bool {
            (point.x >= self.pos.x && point.x <= (self.pos.x + self.size.x)) &&
            (point.y >= self.pos.y && point.y <= (self.pos.y + self.size.y))
        }
    }

    impl<'a> Text<'a> {
        pub fn new(text: String, size: f32, font: &Font) -> Text {
            let mut text = Text { bboxes: vec![], font, size, text };
            text.generate_bboxes();
            text
        }

        fn get_bboxes(&self, point: Vec2) -> Vec<&BBox> {
            self.bboxes.iter().filter(|bbox| bbox.contains(point)).collect()
        }

        fn get_char_distance(&self, point: &Vec2, bbox: &BBox) -> f32 {
            let bbox_point: Vec2 = Vec2::new(point.x, point.y) - bbox.pos; // BBox Space
            let img_point: Vec2 = bbox_point + Vec2::new(bbox.char.x, bbox.char.y); // Texture Space

            // Top-left sample position on the buffer
            let x = img_point.x.floor() as usize;
            let y = img_point.y.floor() as usize;
            let width = self.font.scale_width as usize;

            // Get the value for the 4 distance samples
            let a = self.font.image[(y       * width) + x] as f32;     // Top-left
            let b = self.font.image[(y       * width) + x + 1] as f32; // Top-right
            let c = self.font.image[((y + 1) * width) + x] as f32;     // Bot-left
            let d = self.font.image[((y + 1) * width) + x + 1] as f32; // Bot-right

            // Calculate the weights for each sample
            let a_weight = ((x + 1) as f32 - img_point.x) * ((y + 1) as f32 - img_point.y);
            let b_weight = (img_point.x - (x as f32)) * ((y + 1) as f32 - img_point.y);
            let c_weight = ((x + 1) as f32 - img_point.x) * (img_point.y - y as f32);
            let d_weight = (img_point.x - (x as f32)) * (img_point.y - y as f32);

            let value = (a * a_weight) + (b * b_weight) + (c * c_weight) + (d * d_weight); // bilinear-interpolation
            let value = (255.0 - value) / 255.0; // Map [0.0, 1.0]

            // Map to [-8.0, 8.0] range, font are generated with 8px padding (16px SDF gradient)
            // 0.0 is the font curve boundary
            (value * 16.0) - 8.0
        }

        fn get_initial_cursor_position(&self) -> f32 {
            let first_char = self.text.chars().next();

            match first_char {
                Some(char) => -self.font.get_char(char).x_offset,
                None => 0.0,
            }
        }

        fn generate_bboxes(&mut self) {
            let mut cursor = self.get_initial_cursor_position();

            for letter in self.text.chars() {
                let char = self.font.get_char(letter);
                let bbox = BBox {
                    char: char,
                    pos: Vec2::new(cursor + char.x_offset, char.y_offset),
                    size: Vec2::new(char.width, char.height),
                };

                self.bboxes.push(bbox);
                cursor += char.x_advance;
            }
        }
    }

    impl<'a> SDF for Text<'a> {
        fn get_distance(&self, _arena: &Vec<Object>, point: Vec2) -> f32 {
            let point = Vec2::new(point.x, -point.y);
            let bboxes = self.get_bboxes(point);
            let mut distances = bboxes.iter()
                .map(|bbox| self.get_char_distance(&point, bbox))
                .collect::<Vec<f32>>();

            if distances.len() > 0 {
                // Use the lowest distance
                distances.sort_by(|a: &f32, b: &f32| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                distances[0]
            } else {
                99999.0
            }
        }
    }
}

pub mod operator {
    use super::*;

    pub struct OpSmoothUnion {
        pub sdf_1: usize,
        pub sdf_2: usize,
        pub fuzz: f32,
    }

    impl SDF for OpSmoothUnion {
        fn get_distance(&self, arena: &Vec<Object>, point: Vec2) -> f32 {
            let distance_1 = arena[self.sdf_1].get_distance(arena, point);
            let distance_2 = arena[self.sdf_2].get_distance(arena, point);

            let h = (0.5 + 0.5 * (distance_2 - distance_1) / self.fuzz).clamp(0.0, 1.0);
            return lerp(distance_2, distance_1, h) - self.fuzz * h * (1.0 - h);
        }
    }
}

pub mod color {
    use crate::color::Color;

    pub trait SDFColor {
        fn get_color(&self, distance: f32) -> Color;
    }

    pub enum Fill {
        Solid(Color)
    //     - solid
    //         - color
    //     - linear gradient
    //     - radial gradient
    //     - repeating gradient
    //         - is_repeating
    //         - size
    //         - profile
    }

    impl SDFColor for Fill {
        fn get_color(&self, distance: f32) -> Color {
            match self {
                Fill::Solid(color) => color.clone(),
            }
        }
    }

    pub enum BorderPosition {
        Inside,
        Center,
        Outside,
    }

    pub struct Border {
        pub position: BorderPosition,
        pub size: f32,
        pub color: Color,
    }

    pub struct LayerColor {
        pub inside: Option<Fill>,
        pub border: Option<Border>,
        pub outside: Option<Fill>,
    }

    impl LayerColor {
        fn get_outside_threshold(&self) -> f32 {
            match &self.border {
                Some(border) => match &border.position {
                    BorderPosition::Inside => 0.0,
                    BorderPosition::Center => border.size.abs() / 2.0,
                    BorderPosition::Outside => border.size.abs(),
                },
                None => 0.0,
            }
        }
    }

    impl SDFColor for LayerColor {
        fn get_color(&self, distance: f32) -> Color {
            let fuzz = 2.0;
            let has_border = self.border.is_some() && self.border.as_ref().unwrap().size > 0.0;
            let transparent = Color::new(0.0, 0.0, 0.0, 0.0);
            let outside_threshold = self.get_outside_threshold();
            let inside_threshold = match &self.border {
                Some(border) => outside_threshold - border.size.abs(),
                None => 0.0,
            };

            if has_border {
                //    inside       border        outside
                //         v          v          v
                // ···-----------|x|-----|x|-----------···
                //                 ^     ^                   x = fuzz, transition between boundaries
                //  inside-threshold     outside-threshold
                if distance > (outside_threshold + fuzz) {
                    // Outside
                    match &self.outside {
                        Some(fill) => fill.get_color(distance),
                        None => transparent,
                    }
                } else if distance > outside_threshold {
                    // Inside/Border => Outside transition
                    let t = (distance - outside_threshold) / fuzz;
                    let outside_color = match &self.outside {
                        Some(fill) => fill.get_color(distance),
                        None => transparent.clone()
                    };
    
                    match &self.border {
                        Some(border) => border.color.blend(&outside_color, t),
                        None => {
                            let inside_color = match &self.inside {
                                Some(fill) => fill.get_color(distance),
                                None => transparent
                            };
    
                            inside_color.blend(&outside_color, t)
                        }
                    }
                } else if distance > inside_threshold && self.border.is_some() {
                    // Border
                    self.border.as_ref().unwrap().color.clone()
                } else if distance > (inside_threshold - fuzz) {
                    // Inside => Border/Outside transition
                    let t = 1.0 - (distance + inside_threshold.abs()).abs() / fuzz;
                    let inside_color = match &self.inside {
                        Some(fill) => fill.get_color(distance),
                        None => transparent.clone()
                    };
    
                    match &self.border {
                        Some(border) => inside_color.blend(&border.color, t),
                        None => {
                            let outside_color = match &self.outside {
                                Some(fill) => fill.get_color(distance),
                                None => transparent
                            };
    
                            inside_color.blend(&outside_color, t)
                        }
                    }
                } else {
                    // Inside
                    match &self.inside {
                        Some(fill) => fill.get_color(distance),
                        None => transparent,
                    }
                }
            } else {
                //    inside                 outside
                //         v                 v
                // ···------------|xxx|------------···
                //                  ^                     x = fuzz, transition between boundaries
                //  inside-threshold/outside-threshold
                if distance > (outside_threshold + fuzz / 2.0) {
                    // Outside
                    match &self.outside {
                        Some(fill) => fill.get_color(distance),
                        None => transparent,
                    }
                } else if distance > (outside_threshold - fuzz / 2.0) {
                    let t = 1.0 - ((outside_threshold + fuzz / 2.0) - distance) / fuzz;
                    let outside_color = match &self.outside {
                        Some(fill) => fill.get_color(distance),
                        None => transparent.clone()
                    };

                    // Mix Inside & Outside
                    match &self.inside {
                        Some(fill) => fill.get_color(distance).blend(&outside_color, t),
                        None => transparent,
                    }
                } else {
                    // Inside
                    match &self.inside {
                        Some(fill) => fill.get_color(distance),
                        None => transparent,
                    }
                }
            }
        }
    }
}
