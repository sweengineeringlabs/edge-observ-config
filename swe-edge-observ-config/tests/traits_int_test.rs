//! Integration tests for the Observability and Validator trait contracts.

use swe_edge_observ_config::{Observability, TracingConfig};

struct NoopObservability;

impl swe_edge_observ_config::Observability for NoopObservability {
    fn init(&self, _config: &TracingConfig) {}
}

/// @covers: Observability
#[test]
fn test_observability_trait_int_custom_impl_is_callable() {
    let obs = NoopObservability;
    obs.init(&TracingConfig::default());
}

/// @covers: Observability
#[test]
fn test_observability_trait_int_is_object_safe() {
    let obs: Box<dyn swe_edge_observ_config::Observability> = Box::new(NoopObservability);
    obs.init(&TracingConfig::default());
}
