//! `DefaultSweEdgeObservConfig` — default service implementation.

use crate::api::error::ObservConfigError;
use crate::api::types::config::Config;
use crate::api::types::swe_edge_observ_config::SweEdgeObservConfig;

/// Default implementation of [`SweEdgeObservConfig`].
#[derive(Debug, Default)]
pub(crate) struct DefaultSweEdgeObservConfig;

impl DefaultSweEdgeObservConfig {
    /// Create a new default instance.
    pub(crate) fn new() -> Self {
        Self
    }
}

impl SweEdgeObservConfig for DefaultSweEdgeObservConfig {
    fn execute(&self, config: &Config) -> Result<(), ObservConfigError> {
        if config.verbose {
            #[cfg(feature = "observability")]
            tracing::info!("[swe-edge-observ-config] executing with verbose=true");
            #[cfg(not(feature = "observability"))]
            let _ = config;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_creates_default_swe_edge_observ_config() {
        let _svc = DefaultSweEdgeObservConfig::new();
    }

    /// @covers: execute
    #[test]
    fn test_execute_succeeds_with_default_config() {
        let svc = DefaultSweEdgeObservConfig::new();
        let config = Config::default();
        assert!(svc.execute(&config).is_ok());
    }

    /// @covers: execute
    #[test]
    fn test_execute_succeeds_in_verbose_mode() {
        let svc = DefaultSweEdgeObservConfig::new();
        let config = Config { verbose: true };
        assert!(svc.execute(&config).is_ok());
    }
}
