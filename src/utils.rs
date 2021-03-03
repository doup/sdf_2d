pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    a * (1.0 - t) + b * t
}

/// Given a `a`, `b` & `t`. Where `t` is between `a` & `b`
/// it returns a value between `0.0` and `1.0`. When `t=a` then
/// the value is `0.0`, when `t=b` the value is `1.0`. 
pub fn smoothstep(a: f32, b: f32, t: f32) -> f32 {
    if a == b {
        a
    } else {
        let t = ((t - a) / (b - a)).clamp(0.0, 1.0);
        t * t * (3.0 - 2.0 * t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lerp() {
        // Test clamping
        assert_eq!(lerp(0.0, 1.0, -1.0), 0.0);
        assert_eq!(lerp(0.0, 1.0, 2.0), 1.0);

        // 0.0 to 1.0
        assert_eq!(lerp(0.0, 1.0, 0.0), 0.0);
        assert_eq!(lerp(0.0, 1.0, 0.25), 0.25);
        assert_eq!(lerp(0.0, 1.0, 0.5), 0.5);
        assert_eq!(lerp(0.0, 1.0, 0.75), 0.75);
        assert_eq!(lerp(0.0, 1.0, 1.0), 1.0);

        // 1.0 to 0.0
        assert_eq!(lerp(1.0, 0.0, 0.25), 0.75);
        assert_eq!(lerp(1.0, 0.0, 0.5), 0.5);
        assert_eq!(lerp(1.0, 0.0, 0.75), 0.25);
    }

    #[test]
    fn test_smoothstep() {
        assert_eq!(smoothstep(0.0, 10.0, -5.0), 0.0);
        assert_eq!(smoothstep(0.0, 10.0,  0.0), 0.0);
        assert_eq!(smoothstep(0.0, 10.0,  5.0), 0.5);
        assert_eq!(smoothstep(0.0, 10.0, 10.0), 1.0);
        assert_eq!(smoothstep(0.0, 10.0, 15.0), 1.0);

        assert_eq!(smoothstep(10.0, 0.0, 15.0), 0.0);
        assert_eq!(smoothstep(10.0, 0.0, 10.0), 0.0);
        assert_eq!(smoothstep(10.0, 0.0,  5.0), 0.5);
        assert_eq!(smoothstep(10.0, 0.0,  0.0), 1.0);
        assert_eq!(smoothstep(10.0, 0.0, -5.0), 1.0);

        assert_eq!(smoothstep(-10.0, -20.0,  -5.0), 0.0);
        assert_eq!(smoothstep(-10.0, -20.0, -10.0), 0.0);
        assert_eq!(smoothstep(-10.0, -20.0, -15.0), 0.5);
        assert_eq!(smoothstep(-10.0, -20.0, -20.0), 1.0);
        assert_eq!(smoothstep(-10.0, -20.0, -25.0), 1.0);
    }
}
