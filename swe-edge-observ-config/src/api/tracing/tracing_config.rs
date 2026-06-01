//! `TracingConfig` — TOML-driven tracing subscriber configuration.

use serde::{Deserialize, Serialize};

use crate::api::tracing::tracing_format::TracingFormat;
use crate::api::tracing::tracing_level::TracingLevel;

/// Configuration for the tracing subscriber.
///
/// Maps from the `[observability.tracing]` TOML section.
/// `RUST_LOG` always takes precedence over `level` and `filter`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TracingConfig {
    /// Install the tracing subscriber on startup.  Default: `true`.
    pub enabled: bool,
    /// Output format: `"pretty"` (dev) or `"json"` (prod).  Default: `"pretty"`.
    pub format: TracingFormat,
    /// Minimum log level.  Ignored when `RUST_LOG` is set.  Default: `"info"`.
    pub level: TracingLevel,
    /// Optional module-level filter (e.g. `"my_crate=debug,tower=warn"`).
    /// Appended after `level` when both are present.  Ignored when `RUST_LOG` is set.
    pub filter: Option<String>,
}

impl Default for TracingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            format: TracingFormat::Pretty,
            level: TracingLevel::Info,
            filter: None,
        }
    }
}
