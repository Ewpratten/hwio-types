pub use measurements::voltage::Voltage;

/// For components that can read a voltage
pub trait AnalogInput {
    /// Read voltage
    fn read_voltage() -> Voltage;
}

/// For components that can write a voltage
pub trait AnalogOutput {
    /// Write voltage
    fn write_voltage(val: &Voltage);
}
