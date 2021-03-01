pub use measurements::Angle;

/// For components that can read an angle
pub trait AngleInput {
    /// Read an angle
    fn read_angle() -> Angle;
}

/// For components that can write an angle
pub trait AngleOutput {
    /// Write an angle
    fn write_angle(val: &Angle);
}
