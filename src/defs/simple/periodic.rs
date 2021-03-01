pub use core::time::Duration;

/// A component that relies on periodic "pokes" to keep its data up to date
pub trait Periodic {
    /// Update. Specify how long since the last time this was called
    fn update(dt: &Duration);
}

/// AutomaticPeriodic can be implemented to hide the requirement to pass `dt` to `Periodic::update()` from the user
pub trait AutomaticPeriodic: Periodic {
    /// Update. Automatically calculate `dt`
    fn update();
}
