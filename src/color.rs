use crate::utils::*;

#[derive(Clone, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
    // Secret field, this makes mandatory the use of `::new` to instanciate a `Color`
    _s: (),
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        let alpha = a.clamp(0.0, 1.0);

        Color {
            // Alpha pre-multiply colors
            r: r.clamp(0.0, 1.0) * alpha,
            g: g.clamp(0.0, 1.0) * alpha,
            b: b.clamp(0.0, 1.0) * alpha,
            a: alpha,
            _s: ()
        }
    }

    pub fn blend(&self, front: &Color, t: f32) -> Color {
        self.mix(
            &Color {
                r:  front.r * t,
                g:  front.g * t,
                b:  front.b * t,
                a:  front.a * t,
                _s: (),
            }
        )

        // Color {
        //     r:  lerp(self.r, front.r, t),
        //     g:  lerp(self.g, front.g, t),
        //     b:  lerp(self.b, front.b, t),
        //     a:  lerp(self.a, front.a, t),
        //     _s: ()
        // }

        // self.mix(
        //     &Color::new(
        //         front.0 * t,
        //         front.1 * t,
        //         front.2 * t,
        //         t,
        //     )
        // )
    }

    // pub fn blend(&self, front: &Color, t: f32) -> Color {
    //     let gamma = 2.2;
    //     let t = t.clamp(0.0, 1.0);

    //     let front_alpha = front.3 * (1.0 - t);
    //     let alpha = 1.0 - (1.0 - front_alpha) * (1.0 - self.3);
    //     let s = self.3 * (1.0 - front_alpha) / alpha;
    
    //     Color(
    //         f32::powf(f32::powf((1.0 - s) * front.0, gamma) + f32::powf(s * self.0, gamma), 1.0 / gamma),
    //         f32::powf(f32::powf((1.0 - s) * front.0, gamma) + f32::powf(s * self.0, gamma), 1.0 / gamma),
    //         f32::powf(f32::powf((1.0 - s) * front.0, gamma) + f32::powf(s * self.0, gamma), 1.0 / gamma),
    //         alpha,
    //     )
    // }

    pub fn mix(&self, front: &Color) -> Color {
        Color {
            r:  self.r * (1.0 - front.a) + front.r,
            g:  self.g * (1.0 - front.a) + front.g,
            b:  self.b * (1.0 - front.a) + front.b,
            a:  (self.a + front.a).clamp(0.0, 1.0),
            _s: (),
        }
    }

    pub fn mix_smooth(&self, color: Color, t: f32) -> Color {
        Color {
            r:  smoothstep(self.r, color.r, t),
            g:  smoothstep(self.g, color.g, t),
            b:  smoothstep(self.b, color.b, t),
            a:  smoothstep(self.a, color.a, t),
            _s: (),
        }
    }

    pub fn to_array(&self) -> [u8; 4] {
        let r = (self.r * 255.0) as u8;
        let g = (self.g * 255.0) as u8;
        let b = (self.b * 255.0) as u8;

        [r, g, b, 0xFF]
    }
}

impl From<Color> for u32 {
    fn from(item: Color) -> Self {
        let r = (item.r * 255.0) as u32;
        let g = (item.g * 255.0) as u32;
        let b = (item.b * 255.0) as u32;

        ((r & 0xFF) << 16) | ((g & 0xFF) << 8) | (b & 0xFF)
    }
}
