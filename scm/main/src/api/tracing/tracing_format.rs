//! `TracingFormat` — output format selector for the tracing subscriber.

use serde::{Deserialize, Serialize};

/// Output format for the tracing subscriber.
///
/// Choose `Pretty` during local development (coloured, human-readable) and
/// `Json` in production (structured lines for log aggregators like Loki,
/// Datadog, or CloudWatch).
///
/// # Examples
///
/// ```rust
/// use swe_edge_observ_config::TracingFormat;
///
/// assert_eq!(TracingFormat::default(), TracingFormat::Pretty);
/// assert_ne!(TracingFormat::Pretty, TracingFormat::Json);
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TracingFormat {
    /// Human-readable colour output — for local development.
    #[default]
    Pretty,
    /// Structured JSON lines — for prod log aggregators (Loki, Datadog, CloudWatch).
    ///
    /// Each log line is a self-contained JSON object with span fields flattened inline.
    Json,
}
