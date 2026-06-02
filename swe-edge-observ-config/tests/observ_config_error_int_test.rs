//! Integration tests for the `ObservConfigError` domain error type.

use swe_edge_observ_config::ObservConfigError;

/// @covers: ObservConfigError
#[test]
fn test_observ_config_error_int_io_variant_displays_correctly() {
    let err = ObservConfigError::Io(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "missing file",
    ));
    assert!(err.to_string().contains("I/O error"), "display: {err}");
}

/// @covers: ObservConfigError
#[test]
fn test_observ_config_error_int_config_variant_displays_message() {
    let err = ObservConfigError::Config {
        message: "bad level".to_string(),
    };
    assert!(err.to_string().contains("bad level"), "display: {err}");
}

/// @covers: ObservConfigError
#[test]
fn test_observ_config_error_int_io_from_std_io_error() {
    let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "denied");
    let err = ObservConfigError::Io(io_err);
    assert!(err.to_string().starts_with("I/O error"));
}
