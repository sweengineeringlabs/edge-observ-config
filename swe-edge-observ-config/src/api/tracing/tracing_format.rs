//! `TracingFormat` — output format selector for the tracing subscriber.

use serde::{Deserialize, Serialize};

/// Output format for the tracing subscriber.
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
