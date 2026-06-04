//! `TracingLevel` — log level selector for the tracing subscriber.

use serde::{Deserialize, Serialize};

/// Minimum log level for the tracing subscriber.
///
/// Overridden at runtime by the `RUST_LOG` environment variable.
/// Defaults to `Info`. Use `as_str()` to get the directive string for
/// `EnvFilter::new()` or log macros.
///
/// # Examples
///
/// ```rust
/// use swe_edge_observ_config::TracingLevel;
///
/// assert_eq!(TracingLevel::default(), TracingLevel::Info);
/// assert_eq!(TracingLevel::Info.as_str(), "info");
/// assert_eq!(TracingLevel::Debug.as_str(), "debug");
/// assert_eq!(TracingLevel::Error.as_str(), "error");
/// assert_ne!(TracingLevel::Warn, TracingLevel::Error);
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TracingLevel {
    /// Every span and event.
    Trace,
    /// Debug-level diagnostics.
    Debug,
    /// Normal operational messages.
    #[default]
    Info,
    /// Recoverable anomalies.
    Warn,
    /// Errors that require attention.
    Error,
}

impl TracingLevel {
    /// Returns the directive string used in `EnvFilter`.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Trace => "trace",
            Self::Debug => "debug",
            Self::Info => "info",
            Self::Warn => "warn",
            Self::Error => "error",
        }
    }
}
