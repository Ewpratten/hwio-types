use crate::defs::zeroing::Zeroable;

pub trait RotarySensor: Zeroable {
    fn get_rotations() -> f64;
    fn get_abs_rotations() -> f64;
    fn get_ticks() -> i64;
    fn set_rotations(val: &f64);
}
