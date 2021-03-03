use bevy_math::{Vec2};

pub trait Distorsion {
    fn map(&self, point: Vec2) -> Vec2;
}

pub struct Wave {
    pub x_amplitude: f32,
    pub x_freq: f32,
    pub y_amplitude: f32,
    pub y_freq: f32,
}

impl Distorsion for Wave {
    fn map(&self, point: Vec2) -> Vec2 {
        Vec2::new(
            point.x + ((point.y /*/ HEIGHT as f32*/) * self.x_freq).sin() * self.x_amplitude,
            point.y + ((point.x /*/ WIDTH as f32*/) * self.y_freq).sin() * self.y_amplitude,
        )
    }
}
