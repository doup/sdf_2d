use crate::utils::*;

#[derive(Debug)]
pub struct Color(pub f32, pub f32, pub f32, pub f32);

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color(r, g, b, a)
    }

    pub fn blend(&self, front: &Color, t: f32) -> Color {
        let gamma = 2.2;
        let t = t.clamp(0.0, 1.0);

        let front_alpha = front.3 * (1.0 - t);
        let alpha = 1.0 - (1.0 - front_alpha) * (1.0 - self.3);
        let s = self.3 * (1.0 - front_alpha) / alpha;
    
        Color(
            f32::powf(f32::powf((1.0 - s) * front.0, gamma) + f32::powf(s * self.0, gamma), 1.0 / gamma),
            f32::powf(f32::powf((1.0 - s) * front.0, gamma) + f32::powf(s * self.0, gamma), 1.0 / gamma),
            f32::powf(f32::powf((1.0 - s) * front.0, gamma) + f32::powf(s * self.0, gamma), 1.0 / gamma),
            alpha,
        )
    }

    pub fn mix(&self, front: &Color) -> Color {
        let alpha = 1.0 - (1.0 - self.3) * (1.0 - front.3);

        Color (
            self.0 * (1.0 - front.3) + front.0 * front.3,
            self.1 * (1.0 - front.3) + front.1 * front.3,
            self.2 * (1.0 - front.3) + front.2 * front.3,
            alpha,
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
