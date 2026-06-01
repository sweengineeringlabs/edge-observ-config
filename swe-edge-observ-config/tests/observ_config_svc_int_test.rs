//! Integration tests for `ObservConfigSvc` — SAF facade.

use swe_edge_observ_config::{Config, ObservConfigSvc};

/// @covers: ObservConfigSvc::execute
#[test]
fn test_execute_int_with_default_config_succeeds() {
    let cfg = Config::default();
    assert!(ObservConfigSvc::execute(&cfg).is_ok());
}

/// @covers: ObservConfigSvc::execute
#[test]
fn test_execute_int_with_verbose_config_succeeds() {
    let cfg = Config { verbose: true };
    assert!(ObservConfigSvc::execute(&cfg).is_ok());
}

/// @covers: ObservConfigSvc::run
#[test]
fn test_run_int_succeeds() {
    assert!(ObservConfigSvc::run().is_ok());
}

/// @covers: ObservConfigSvc::run
#[test]
fn test_run_int_is_idempotent() {
    assert!(ObservConfigSvc::run().is_ok());
    assert!(ObservConfigSvc::run().is_ok());
}
