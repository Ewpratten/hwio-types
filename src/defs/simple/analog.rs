pub use measurements::voltage::Voltage;

pub trait AnalogInput {
    fn read_voltage() -> Voltage;
}

pub trait AnalogOutput {
    fn write_voltage(val: &Voltage);
}