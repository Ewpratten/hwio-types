use crate::defs::simple::angle::{Angle, AngleInput};
use crate::defs::complex::rotary_sensor::RotarySensor;

/// Describes a potentiometer
pub trait Potentiometer: RotarySensor + AngleInput {

    /// Set the angle corresponding to "zero rotations"
    fn set_zero_angle(angle: Angle);
}
