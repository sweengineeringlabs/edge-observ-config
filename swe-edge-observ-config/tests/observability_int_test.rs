//! Integration tests for the SAF tracing subscriber initialisation.

#[cfg(feature = "observability")]
use swe_edge_observ_config::{TracingConfig, TracingFormat, TracingLevel, TracingSvc};

/// @covers: TracingSvc::init
#[cfg(feature = "observability")]
#[test]
fn test_tracing_svc_int_json_format_does_not_panic() {
    let cfg = TracingConfig {
        format: TracingFormat::Json,
        ..TracingConfig::default()
    };
    TracingSvc::init(&cfg);
}

/// @covers: TracingSvc::init
#[cfg(feature = "observability")]
#[test]
fn test_tracing_svc_int_pretty_format_does_not_panic() {
    TracingSvc::init(&TracingConfig::default());
}

/// @covers: TracingSvc::init
#[cfg(feature = "observability")]
#[test]
fn test_tracing_svc_int_idempotent_called_twice_does_not_panic() {
    TracingSvc::init(&TracingConfig::default());
    TracingSvc::init(&TracingConfig::default());
}

/// @covers: TracingSvc::init
#[cfg(feature = "observability")]
#[test]
fn test_tracing_svc_int_disabled_is_noop() {
    let cfg = TracingConfig {
        enabled: false,
        ..TracingConfig::default()
    };
    TracingSvc::init(&cfg);
}

/// @covers: TracingSvc::init
#[cfg(feature = "observability")]
#[test]
fn test_tracing_svc_int_warn_level_does_not_panic() {
    let cfg = TracingConfig {
        level: TracingLevel::Warn,
        ..TracingConfig::default()
    };
    TracingSvc::init(&cfg);
}

/// @covers: TracingSvc::init
#[cfg(feature = "observability")]
#[test]
fn test_tracing_svc_int_with_filter_does_not_panic() {
    let cfg = TracingConfig {
        filter: Some("tower=warn,hyper=error".into()),
        ..TracingConfig::default()
    };
    TracingSvc::init(&cfg);
}
