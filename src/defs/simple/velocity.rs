pub use measurements::AngularVelocity;
pub use measurements::Speed;

/// This trait allows a closed-loop velocity to be set for a component
pub trait VelocityOutput {
    /// Set a closed-loop velocity
    fn set_velocity(velocity: &Speed);
}

/// This trait allows velocity measurement
pub trait VelocityInput {
    /// Measure / read a velocity
    fn get_velocity() -> Speed;
}

/// This trait allows a closed-loop angular velocity to be set for a component
pub trait AngularVelocityOutput {
    /// Set a closed-loop angular velocity
    fn set_angular_velocity(av: &AngularVelocity);
}

/// This trait allows angular velocity measurement
pub trait AngularVelocityInput {
    /// Measure / read an angular velocity
    fn get_angular_velocity() -> AngularVelocity;
}
