pub use std::time::Duration;

pub trait Periodic {
    fn update(dt: &Duration);
}

pub trait AutomaticPeriodic: Periodic {
    fn update();
}
