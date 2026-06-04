//! `TracingSvc` — public facade type for tracing subscriber initialisation.

/// Public facade for tracing subscriber operations.
///
/// Consumers use [`TracingSvc::init`] to install a `tracing-subscriber`
/// driven by a [`TracingConfig`](crate::api::tracing::tracing_config::TracingConfig).
pub struct TracingSvc;
