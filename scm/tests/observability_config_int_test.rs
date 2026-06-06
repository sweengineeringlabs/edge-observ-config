//! Integration tests for `ObservabilityConfig` deserialization and defaults.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_observ_config::{ObservabilityConfig, TracingFormat, TracingLevel};

/// @covers: ObservabilityConfig
#[test]
fn test_observability_config_int_default_has_tracing_enabled() {
    assert!(ObservabilityConfig::default().tracing.enabled);
}

/// @covers: ObservabilityConfig
#[test]
fn test_observability_config_int_default_level_is_info() {
    assert_eq!(
        ObservabilityConfig::default().tracing.level,
        TracingLevel::Info
    );
}

/// @covers: ObservabilityConfig
#[test]
fn test_observability_config_int_default_format_is_pretty() {
    assert_eq!(
        ObservabilityConfig::default().tracing.format,
        TracingFormat::Pretty
    );
}

/// @covers: ObservabilityConfig
#[test]
fn test_observability_config_int_deserializes_nested_toml() {
    let toml = r#"
        [tracing]
        level  = "debug"
        format = "json"
    "#;
    let cfg: ObservabilityConfig = toml::from_str(toml).expect("deserialize");
    assert_eq!(cfg.tracing.level, TracingLevel::Debug);
    assert_eq!(cfg.tracing.format, TracingFormat::Json);
    assert!(cfg.tracing.enabled);
}

/// @covers: ObservabilityConfig
#[test]
fn test_observability_config_int_empty_toml_uses_all_defaults() {
    let cfg: ObservabilityConfig = toml::from_str("").expect("deserialize empty");
    assert!(cfg.tracing.enabled);
    assert_eq!(cfg.tracing.level, TracingLevel::Info);
    assert_eq!(cfg.tracing.format, TracingFormat::Pretty);
}
