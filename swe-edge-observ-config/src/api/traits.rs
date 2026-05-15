//! Primary service trait and validator contract.

use crate::api::tracing_config::TracingConfig;

/// Initialises an observability subscriber from a [`TracingConfig`].
///
/// Implementations must be idempotent — calling `init` after a subscriber is
/// already installed must be a silent no-op.
pub trait Observability: Send + Sync {
    /// Install (or skip if already installed) the tracing subscriber.
    fn init(&self, config: &TracingConfig);
}

/// Validates a value before it is used.
pub trait Validator: Send + Sync {
    /// The value being validated.
    type Target: ?Sized;
    /// The error returned when validation fails.
    type Error;
    /// Returns `Ok(())` when `target` is valid, `Err(Self::Error)` otherwise.
    fn validate(&self, target: &Self::Target) -> Result<(), Self::Error>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_observability_trait_is_object_safe() {
        fn _accept(_: &dyn Observability) {}
    }
}
