//! Integration tests for `TracingFormat` — tracing output format selector.

use swe_edge_observ_config::TracingFormat;

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
fn test_tracing_format_int_json_not_equal_to_pretty() {
    assert_ne!(TracingFormat::Json, TracingFormat::Pretty);
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
