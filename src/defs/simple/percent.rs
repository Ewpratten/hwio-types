/// For components that take a percent output
pub trait PercentOutput {
    /// Write a percent output
    fn write_percent(val: &f64);
}

/// For components that can read a percent input
pub trait PercentInput {
    /// Read a percent input
    fn read_percent(val: &f64);
}
