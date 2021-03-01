pub use measurements::Angle;

pub trait AngleInput {
    fn read_angle() -> Angle;
}

pub trait AngleOutput {
    fn write_angle(val: &Angle);
}
