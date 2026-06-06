//! Integration tests for the `ObservConfigError` domain type (from `api/error/domain.rs`).

use swe_edge_observ_config::ObservConfigError;

/// @covers: ObservConfigError
#[test]
fn test_domain_error_int_io_displays_correctly() {
    let err = ObservConfigError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "missing"));
    assert!(err.to_string().contains("I/O error"), "display: {err}");
}

/// @covers: ObservConfigError
#[test]
fn test_domain_error_int_config_displays_message() {
    let err = ObservConfigError::Config {
        message: "test error".to_string(),
    };
    assert!(err.to_string().contains("test error"), "display: {err}");
}

/// @covers: ObservConfigError
#[test]
fn test_domain_error_int_config_prefix_is_configuration_error() {
    let err = ObservConfigError::Config {
        message: "bad".to_string(),
    };
    assert!(err.to_string().starts_with("Configuration error"));
}
