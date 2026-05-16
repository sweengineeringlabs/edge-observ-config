//! SAF — public facade.

pub use crate::api::observability_config::ObservabilityConfig;
pub use crate::api::tracing_config::TracingConfig;
pub use crate::api::tracing_format::TracingFormat;
pub use crate::api::tracing_level::TracingLevel;
pub use crate::api::traits::{Observability, Validator};

/// Install a `tracing-subscriber` driven by `config`.
///
/// Requires the `observability` feature. Idempotent — safe to call multiple
/// times. Does nothing when `config.enabled` is `false`. `RUST_LOG` overrides
/// `config.level` and `config.filter`.
#[cfg(feature = "observability")]
pub fn init_tracing(config: &TracingConfig) {
    crate::core::observability::DefaultObservability.init(config);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_saf_types_are_publicly_accessible() {
        let _: TracingConfig = TracingConfig::default();
        let _: ObservabilityConfig = ObservabilityConfig::default();
        let _: TracingLevel = TracingLevel::default();
        let _: TracingFormat = TracingFormat::default();
    }

    #[cfg(feature = "observability")]
    #[test]
    fn test_saf_init_tracing_does_not_panic() {
        init_tracing(&TracingConfig::default());
    }
}
