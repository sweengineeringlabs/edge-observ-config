//! Integration tests for `ObservConfigError` — domain error variants.

use swe_edge_observ_config::ObservConfigError;

/// @covers: ObservConfigError
#[test]
fn test_error_io_variant_displays_correctly() {
    let err = ObservConfigError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "missing file"));
    assert!(err.to_string().contains("I/O error"), "display: {err}");
}

/// @covers: ObservConfigError
#[test]
fn test_error_config_variant_displays_message() {
    let err = ObservConfigError::Config { message: "bad value".to_string() };
    assert!(err.to_string().contains("bad value"), "display: {err}");
}

/// @covers: ObservConfigError
#[test]
fn test_error_config_variant_includes_configuration_error_prefix() {
    let err = ObservConfigError::Config { message: "invalid level".to_string() };
    assert!(err.to_string().contains("Configuration error"));
}
