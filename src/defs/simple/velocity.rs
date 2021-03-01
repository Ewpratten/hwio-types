pub use measurements::AngularVelocity;
pub use measurements::Speed;

pub trait VelocityOutput {
    fn set_velocity(velocity: &Speed);
}

pub trait VelocityInput {
    fn get_velocity() -> Speed;
}

pub trait AngularVelocityOutput {
    fn set_angular_velocity(av: &AngularVelocity);
}

pub trait AngularVelocityInput {
    fn get_angular_velocity() -> AngularVelocity;
}
