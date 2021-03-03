use bevy_math::{Vec2};

use crate::distortion::*;
use crate::transform::*;
use crate::utils::*;

pub trait SDF {
    fn get_distance(&self, arena: &Vec<Object>, point: Vec2) -> f32;
}

pub struct Object {
    pub transform: Transform,
    pub distortion: Vec<Box<dyn Distorsion + Sync + Send>>,
    pub parent_id: Option<usize>,
    pub sdf: Box<dyn SDF + Sync + Send>,
}

impl SDF for Object {
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
    
            let h = (0.5 + 0.5 * (distance_2 - distance_1) / self.fuzz).clamp(0.0, 1.0 );
            return lerp(distance_2, distance_1, h) - self.fuzz * h * (1.0 - h);
        }
    }
}
