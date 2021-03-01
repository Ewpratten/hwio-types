use crate::defs::simple::analog::AnalogOutput;
use crate::defs::simple::percent::PercentOutput;
use crate::defs::zeroing::Zeroable;
pub use measurements::AngularVelocity;

/// Describes the simplest form of speed controller
pub trait SpeedController: AnalogOutput + Zeroable + PercentOutput {

    /// Set an open-loop velocity.
    /// This will not be the real output velocity. For more precision, 
    /// `AngularVelocityOutput` should be implemented along with a closed-loop controller.
    fn set_open_loop_velocity(speed: &AngularVelocity);
}
