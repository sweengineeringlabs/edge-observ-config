//! Integration tests for the SAF init_tracing function.

use swe_edge_observ_config::{TracingConfig, TracingFormat, TracingLevel};

/// @covers: init_tracing
#[cfg(feature = "observability")]
#[test]
fn test_init_tracing_int_json_format_does_not_panic() {
    let cfg = TracingConfig {
        format: TracingFormat::Json,
        ..TracingConfig::default()
    };
    swe_edge_observ_config::init_tracing(&cfg);
}

/// @covers: init_tracing
#[cfg(feature = "observability")]
#[test]
fn test_init_tracing_int_pretty_format_does_not_panic() {
    swe_edge_observ_config::init_tracing(&TracingConfig::default());
}

/// @covers: init_tracing
#[cfg(feature = "observability")]
#[test]
fn test_init_tracing_int_idempotent_called_twice_does_not_panic() {
    swe_edge_observ_config::init_tracing(&TracingConfig::default());
    swe_edge_observ_config::init_tracing(&TracingConfig::default());
}

/// @covers: init_tracing
#[cfg(feature = "observability")]
#[test]
fn test_init_tracing_int_disabled_is_noop() {
    let cfg = TracingConfig {
        enabled: false,
        ..TracingConfig::default()
    };
    swe_edge_observ_config::init_tracing(&cfg);
}

/// @covers: init_tracing
#[cfg(feature = "observability")]
#[test]
fn test_init_tracing_int_warn_level_does_not_panic() {
    let cfg = TracingConfig {
        level: TracingLevel::Warn,
        ..TracingConfig::default()
    };
    swe_edge_observ_config::init_tracing(&cfg);
}

/// @covers: init_tracing
#[cfg(feature = "observability")]
#[test]
fn test_init_tracing_int_with_filter_does_not_panic() {
    let cfg = TracingConfig {
        filter: Some("tower=warn,hyper=error".into()),
        ..TracingConfig::default()
    };
    swe_edge_observ_config::init_tracing(&cfg);
}
