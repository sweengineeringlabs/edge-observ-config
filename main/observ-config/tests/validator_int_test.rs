//! Integration tests for the `Validator` trait contract.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_observ_config::Validator;

struct LengthValidator {
    max: usize,
}

impl Validator for LengthValidator {
    type Target = str;
    type Error = String;

    fn validate(&self, target: &str) -> Result<(), String> {
        if target.len() <= self.max {
            Ok(())
        } else {
            Err(format!("input too long: {} > {}", target.len(), self.max))
        }
    }
}

/// @covers: Validator
#[test]
fn test_validator_int_accepts_value_within_limit() {
    let v = LengthValidator { max: 10 };
    assert!(v.validate("hello").is_ok());
}

/// @covers: Validator
#[test]
fn test_validator_int_rejects_value_exceeding_limit() {
    let v = LengthValidator { max: 3 };
    let result = v.validate("toolong");
    assert!(result.is_err());
    let msg = result.unwrap_err();
    assert!(msg.contains("too long"), "unexpected message: {msg}");
}

/// @covers: Validator
#[test]
fn test_validator_int_accepts_empty_string() {
    let v = LengthValidator { max: 0 };
    assert!(v.validate("").is_ok());
}

/// @covers: Validator
#[test]
fn test_validator_int_is_object_safe() {
    let _: Box<dyn Validator<Target = str, Error = String>> = Box::new(LengthValidator { max: 5 });
}
