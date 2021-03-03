use crate::utils::*;

#[derive(Debug)]
pub struct Color(pub f32, pub f32, pub f32, pub f32);

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color(r, g, b, a)
    }

    pub fn mix(&self, color: Color, t: f32) -> Color {
        Color(
            lerp(self.0, color.0, t),
            lerp(self.1, color.1, t),
            lerp(self.2, color.2, t),
            lerp(self.3, color.3, t),
        )
    }

    pub fn mix_smooth(&self, color: Color, t: f32) -> Color {
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
