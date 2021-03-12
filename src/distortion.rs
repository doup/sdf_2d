use bevy_math::{Vec2};

pub trait Distorsion {
    fn map(&self, point: Vec2) -> Vec2;
}

pub struct Wave {
    pub width: f32,
    pub height: f32,
    pub x_amplitude: f32,
    pub x_freq: f32,
    pub y_amplitude: f32,
    pub y_freq: f32,
    pub time: f32,
}

impl Distorsion for Wave {
    fn map(&self, point: Vec2) -> Vec2 {
        Vec2::new(
            point.x + (self.time + (point.y / self.height) * self.x_freq).sin() * self.x_amplitude,
            point.y + (self.time + (point.x / self.width) * self.y_freq).sin() * self.y_amplitude,
        )
    }
}
