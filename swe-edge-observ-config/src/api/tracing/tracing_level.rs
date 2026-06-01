//! `TracingLevel` — log level selector for the tracing subscriber.

use serde::{Deserialize, Serialize};

/// Minimum log level for the tracing subscriber.
///
/// Overridden at runtime by the `RUST_LOG` environment variable.
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
