//! `Validator` trait — validation contract for typed values.

/// Validates a value before it is used.
pub trait Validator: Send + Sync {
    /// The value being validated.
    type Target: ?Sized;
    /// The error returned when validation fails.
    type Error;
    /// Returns `Ok(())` when `target` is valid, `Err(Self::Error)` otherwise.
    fn validate(&self, target: &Self::Target) -> Result<(), Self::Error>;
}
