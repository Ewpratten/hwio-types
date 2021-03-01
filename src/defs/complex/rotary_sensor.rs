use crate::defs::zeroing::Zeroable;

/// Describes the simplest form of rotary sensor
pub trait RotarySensor: Zeroable {
    /// Get the relative number of rotations measured since the last call to `zero()`
    fn get_rotations() -> f64;

    /// Get the absolute number of rotations. Not affected by `zero()`
    fn get_abs_rotations() -> f64;

    /// Get the number of raw ticks observed by the sensor
    fn get_ticks() -> i64;

    /// Set the absolute rotation count to a specific value
    fn set_rotations(val: &f64);
}
