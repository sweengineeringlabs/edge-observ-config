//! `Observability` trait — tracing subscriber initialisation contract.

use crate::api::tracing::tracing_config::TracingConfig;

/// Initialises an observability subscriber from a [`TracingConfig`].
///
/// Implementations must be idempotent — calling `init` after a subscriber is
/// already installed must be a silent no-op.
pub trait Observability: Send + Sync {
    /// Install (or skip if already installed) the tracing subscriber.
    fn init(&self, config: &TracingConfig);
}
