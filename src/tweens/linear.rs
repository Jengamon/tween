declare_tween!(
    /// A Linear tween is a simple lerp from one value to another.
    pub struct Linear;

    fn tween(&mut self, value_delta: Value, percent: f64) -> Value {
        value_delta.scale(percent)
    }
);

#[cfg(test)]
mod tests {
    use super::Linear;
    use approx::assert_ulps_eq;

    #[test]
    fn linear_over_frames() {
        let mut value;
        let mut tweener = crate::Tweener::with_tween(0.0, 100.0, 10, Linear::new());

        for val in 1..=10 {
            value = tweener.run(val);
            assert_ulps_eq!(value, val as f32 * 10.0);
        }
    }

    #[test]
    fn linear_over_frames_rev() {
        let mut value;
        let mut tweener = crate::Tweener::with_tween(100.0, 0.0, 10, Linear::new());

        for val in 1..=10 {
            value = tweener.run(val);
            assert_ulps_eq!(value, 100.0 - val as f32 * 10.0);
        }
    }
}
