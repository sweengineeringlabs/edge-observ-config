//! Integration tests for `TracingSvc` and `TracingFormat`/`TracingLevel` types.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_observ_config::{TracingFormat, TracingLevel};

/// @covers: TracingFormat
#[test]
fn test_tracing_format_int_default_is_pretty() {
    assert_eq!(TracingFormat::default(), TracingFormat::Pretty);
}

/// @covers: TracingFormat
#[test]
fn test_tracing_format_int_variants_are_constructible() {
    let _json = TracingFormat::Json;
    let _pretty = TracingFormat::Pretty;
}

/// @covers: TracingFormat
#[test]
fn test_tracing_format_int_deserializes_from_toml() {
    #[derive(serde::Deserialize)]
    struct W {
        format: TracingFormat,
    }
    let json: W = toml::from_str(r#"format = "json""#).expect("deserialize json");
    let pretty: W = toml::from_str(r#"format = "pretty""#).expect("deserialize pretty");
    assert_eq!(json.format, TracingFormat::Json);
    assert_eq!(pretty.format, TracingFormat::Pretty);
}

/// @covers: TracingLevel
#[test]
fn test_tracing_level_int_default_is_info() {
    assert_eq!(TracingLevel::default(), TracingLevel::Info);
}

/// @covers: as_str
#[test]
fn test_tracing_level_int_as_str_round_trips() {
    assert_eq!(TracingLevel::Trace.as_str(), "trace");
    assert_eq!(TracingLevel::Debug.as_str(), "debug");
    assert_eq!(TracingLevel::Info.as_str(), "info");
    assert_eq!(TracingLevel::Warn.as_str(), "warn");
    assert_eq!(TracingLevel::Error.as_str(), "error");
}

/// @covers: TracingSvc
#[test]
fn test_tracing_svc_int_init_does_not_panic() {
    use swe_edge_observ_config::{TracingConfig, TracingSvc};
    TracingSvc::init(&TracingConfig::default());
}
