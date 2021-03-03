use bevy_math::{Mat3, Vec2};

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub scale: f32,
}

impl Transform {
    pub fn apply(&mut self, transform: Transform) {
        self.x += transform.x;
        self.y += transform.y;
        self.rotation += transform.rotation;
        self.scale *= transform.scale;
    }

    pub fn new() -> Transform {
        Transform {
            x: 0.0,
            y: 0.0,
            rotation: 0.0,
            scale: 1.0,
        }
    }

    pub fn to_matrix(&self) -> Mat3 {
        Mat3::from_scale_angle_translation(
            Vec2::new(1.0, 1.0),
            -self.rotation.to_radians(),
            Vec2::new(self.x, self.y),
        )
    }

    pub fn map(&self, point: Vec2) -> Vec2 {
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
