//! Integration tests for `TracingLevel` — log level selector.

use swe_edge_observ_config::TracingLevel;

/// @covers: TracingLevel
#[test]
fn test_tracing_level_int_default_is_info() {
    assert_eq!(TracingLevel::default(), TracingLevel::Info);
}

/// @covers: as_str
#[test]
fn test_tracing_level_int_as_str_all_variants() {
    assert_eq!(TracingLevel::Trace.as_str(), "trace");
    assert_eq!(TracingLevel::Debug.as_str(), "debug");
    assert_eq!(TracingLevel::Info.as_str(), "info");
    assert_eq!(TracingLevel::Warn.as_str(), "warn");
    assert_eq!(TracingLevel::Error.as_str(), "error");
}

/// @covers: TracingLevel
#[test]
fn test_tracing_level_int_all_variants_deserialize_from_lowercase() {
    for (s, expected) in [
        ("trace", TracingLevel::Trace),
        ("debug", TracingLevel::Debug),
        ("info", TracingLevel::Info),
        ("warn", TracingLevel::Warn),
        ("error", TracingLevel::Error),
    ] {
        #[derive(serde::Deserialize)]
        struct W {
            level: TracingLevel,
        }
        let w: W = toml::from_str(&format!("level = \"{s}\"")).expect("deserialize");
        assert_eq!(w.level, expected, "failed for: {s}");
    }
}

/// @covers: TracingLevel
#[test]
fn test_tracing_level_int_is_not_equal_across_variants() {
    assert_ne!(TracingLevel::Trace, TracingLevel::Error);
    assert_ne!(TracingLevel::Info, TracingLevel::Warn);
}
