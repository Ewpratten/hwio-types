/// Describes a stateful component that can be reset to a "zero state".
/// Weather this action involves physical movement is up to the implementation.
/// Do not assume calling zero() is physically safe
pub trait Zeroable {
    /// Reset the component to its "zero state"
    /// This may cause systems to physically move, based on the implementation
    fn zero();
}
