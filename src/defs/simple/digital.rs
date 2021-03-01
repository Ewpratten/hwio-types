/// For components that can write a digital output
pub trait DigitalInput {
    /// Read digital input
    fn read_digital() -> bool;
}

/// For components that can read a digital input
pub trait DigitalOutput {
    /// Write digital output
    fn write_digital(val: &bool);
}
