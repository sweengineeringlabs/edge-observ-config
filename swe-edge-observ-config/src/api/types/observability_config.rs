//! `ObservabilityConfig` — top-level observability configuration.

use serde::{Deserialize, Serialize};

use crate::api::tracing::tracing_config::TracingConfig;

/// Top-level observability configuration.
///
/// Maps from the `[observability]` TOML section.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ObservabilityConfig {
    /// Tracing subscriber configuration (`[observability.tracing]`).
    pub tracing: TracingConfig,
}
