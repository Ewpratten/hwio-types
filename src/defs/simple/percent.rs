pub trait PercentOutput {
    fn write_percent(val: &f64);
}

pub trait PercentInput {
    fn read_percent(val: &f64);
}
