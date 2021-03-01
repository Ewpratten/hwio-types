use crate::defs::simple::angle::{Angle, AngleInput};
use crate::defs::complex::rotary_sensor::RotarySensor;

pub trait Potentiometer: RotarySensor + AngleInput {
    fn set_zero_angle(angle: Angle);
}
