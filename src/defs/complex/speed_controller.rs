use crate::defs::simple::analog::AnalogOutput;
use crate::defs::simple::percent::PercentOutput;
use crate::defs::zeroing::Zeroable;
pub use measurements::AngularVelocity;

pub trait SpeedController: AnalogOutput + Zeroable + PercentOutput {
    fn set_open_loop_velocity(speed: &AngularVelocity);
}
