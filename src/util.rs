use std::ops::RangeInclusive;

/// Additional methods for the [`RangeInclusive`] type
pub trait RangeInclusiveExt<T> {
    /// Perform a linear interpolation
    ///
    /// # Arguments
    ///
    /// * `at` - a normalized value describing the interpolation factor
    fn lerp(&self, at: f32) -> T;
}

impl RangeInclusiveExt<f32> for RangeInclusive<f32> {
    fn lerp(&self, at: f32) -> f32 {
        let delta = self.end() - self.start();
        self.start() + (at * delta)
    }
}
