//! `TracingConfig` — TOML-driven tracing subscriber configuration.

use serde::{Deserialize, Serialize};

use crate::api::tracing::tracing_format::TracingFormat;
use crate::api::tracing::tracing_level::TracingLevel;

/// Configuration for the tracing subscriber.
///
/// Maps from the `[observability.tracing]` TOML section.
/// `RUST_LOG` always takes precedence over `level` and `filter`.
///
/// # Examples
///
/// ```rust
/// use swe_edge_observ_config::{TracingConfig, TracingFormat, TracingLevel};
///
/// // SWE baseline: enabled, pretty format, info level.
/// let cfg = TracingConfig::default();
/// assert!(cfg.enabled);
/// assert_eq!(cfg.format, TracingFormat::Pretty);
/// assert_eq!(cfg.level, TracingLevel::Info);
/// assert!(cfg.filter.is_none());
///
/// // Production: JSON format, warn level, module filter.
/// let cfg = TracingConfig {
///     enabled: true,
///     format: TracingFormat::Json,
///     level: TracingLevel::Warn,
///     filter: Some("hyper=error,tower=warn".to_string()),
/// };
/// assert_eq!(cfg.level, TracingLevel::Warn);
/// assert_eq!(cfg.filter.as_deref(), Some("hyper=error,tower=warn"));
/// ```
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
