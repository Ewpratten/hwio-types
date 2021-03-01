pub trait DigitalInput {
    fn read_digital() -> bool;
}

pub trait DigitalOutput {
    fn write_digital(val: &bool);
}