//! SAF wiring — `TracingSvc` and `ObservConfigSvc` method implementations.
//!
//! This module wires the public API types in `api/types/` to their
//! `core/` implementations. All public types are declared in `api/types/`;
//! the concrete behaviour is injected here.

use crate::api::traits::observability::Observability as _;
use crate::api::types::observ_config_svc::ObservConfigSvc;
use crate::api::types::swe_edge_observ_config::SweEdgeObservConfig as _;
use crate::api::types::tracing_svc::TracingSvc;
use crate::core::observability::DefaultObservability;

impl TracingSvc {
    /// Install a `tracing-subscriber` driven by `config`.
    ///
    /// Idempotent — safe to call multiple times. Does nothing when
    /// `config.enabled` is `false` or when the `observability` feature is
    /// disabled. `RUST_LOG` overrides `config.level` and `config.filter`.
    pub fn init(config: &crate::api::tracing::tracing_config::TracingConfig) {
        DefaultObservability.init(config);
    }
}

impl ObservConfigSvc {
    /// Execute the primary operation with the given configuration.
    pub fn execute(
        config: &crate::api::types::config::Config,
    ) -> Result<(), crate::api::error::ObservConfigError> {
        let svc = crate::core::DefaultSweEdgeObservConfig::new();
        svc.execute(config)
    }

    /// Execute with default configuration.
    pub fn run() -> Result<(), crate::api::error::ObservConfigError> {
        Self::execute(&crate::api::types::config::Config::default())
    }
}
