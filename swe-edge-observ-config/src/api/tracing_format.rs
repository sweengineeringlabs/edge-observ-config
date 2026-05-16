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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracing_format_variants_are_constructible() {
        let _json = TracingFormat::Json;
        let _pretty = TracingFormat::Pretty;
    }

    #[test]
    fn test_tracing_format_default_is_pretty() {
        assert_eq!(TracingFormat::default(), TracingFormat::Pretty);
    }

    #[test]
    fn test_tracing_format_deserializes_from_lowercase_toml() {
        #[derive(serde::Deserialize)]
        struct W {
            format: TracingFormat,
        }
        let json: W = toml::from_str(r#"format = "json""#).unwrap();
        let pretty: W = toml::from_str(r#"format = "pretty""#).unwrap();
        assert_eq!(json.format, TracingFormat::Json);
        assert_eq!(pretty.format, TracingFormat::Pretty);
    }
}
