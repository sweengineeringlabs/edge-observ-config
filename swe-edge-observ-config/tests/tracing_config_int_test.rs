//! Integration tests for TracingConfig and ObservabilityConfig deserialization.

use swe_edge_observ_config::{ObservabilityConfig, TracingConfig, TracingFormat, TracingLevel};

/// @covers: TracingConfig::default
#[test]
fn test_tracing_config_int_default_values_are_sane() {
    let cfg = TracingConfig::default();
    assert!(cfg.enabled);
    assert_eq!(cfg.format, TracingFormat::Pretty);
    assert_eq!(cfg.level, TracingLevel::Info);
    assert!(cfg.filter.is_none());
}

/// @covers: TracingConfig
#[test]
fn test_tracing_config_int_full_toml_round_trip() {
    let toml = r#"
        enabled = true
        format  = "json"
        level   = "debug"
        filter  = "tower=warn"
    "#;
    let cfg: TracingConfig = toml::from_str(toml).unwrap();
    assert_eq!(cfg.format, TracingFormat::Json);
    assert_eq!(cfg.level,  TracingLevel::Debug);
    assert_eq!(cfg.filter.as_deref(), Some("tower=warn"));
}

/// @covers: TracingConfig
#[test]
fn test_tracing_config_int_partial_toml_uses_defaults() {
    let cfg: TracingConfig = toml::from_str(r#"level = "error""#).unwrap();
    assert!(cfg.enabled);
    assert_eq!(cfg.level, TracingLevel::Error);
    assert_eq!(cfg.format, TracingFormat::Pretty);
}

/// @covers: ObservabilityConfig
#[test]
fn test_observability_config_int_nested_tracing_section() {
    let toml = r#"
        [tracing]
        level  = "warn"
        format = "json"
    "#;
    let cfg: ObservabilityConfig = toml::from_str(toml).unwrap();
    assert_eq!(cfg.tracing.level,  TracingLevel::Warn);
    assert_eq!(cfg.tracing.format, TracingFormat::Json);
    assert!(cfg.tracing.enabled);
}

/// @covers: ObservabilityConfig
#[test]
fn test_observability_config_int_empty_toml_uses_all_defaults() {
    let cfg: ObservabilityConfig = toml::from_str("").unwrap();
    assert!(cfg.tracing.enabled);
    assert_eq!(cfg.tracing.level,  TracingLevel::Info);
    assert_eq!(cfg.tracing.format, TracingFormat::Pretty);
}

/// @covers: TracingLevel
#[test]
fn test_tracing_level_int_all_variants_deserialize() {
    for (s, expected) in [
        ("trace", TracingLevel::Trace),
        ("debug", TracingLevel::Debug),
        ("info",  TracingLevel::Info),
        ("warn",  TracingLevel::Warn),
        ("error", TracingLevel::Error),
    ] {
        #[derive(serde::Deserialize)]
        struct W { level: TracingLevel }
        let w: W = toml::from_str(&format!("level = \"{s}\"")).unwrap();
        assert_eq!(w.level, expected, "failed for: {s}");
    }
}
