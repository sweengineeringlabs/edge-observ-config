//! Default SweEdgeObservConfig implementation.

use crate::api::config::Config;
use crate::api::error::Error;
use crate::api::swe_edge_observ_config::SweEdgeObservConfig;

/// Default implementation of the SweEdgeObservConfig trait.
#[derive(Debug, Default)]
pub(crate) struct DefaultSweEdgeObservConfig;

impl DefaultSweEdgeObservConfig {
    /// Create a new default instance.
    pub(crate) fn new() -> Self {
        Self
    }
}

impl SweEdgeObservConfig for DefaultSweEdgeObservConfig {
    fn execute(&self, config: &Config) -> Result<(), Error> {
        if config.verbose {
            tracing::info!("[swe-edge-observ-config] executing with verbose=true");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_default_swe_edge_observ_config() {
        let _svc = DefaultSweEdgeObservConfig::new();
    }

    #[test]
    fn test_execute_succeeds_with_default_config() {
        let svc = DefaultSweEdgeObservConfig::new();
        let config = Config::default();
        assert!(svc.execute(&config).is_ok());
    }

    #[test]
    fn test_execute_succeeds_in_verbose_mode() {
        let svc = DefaultSweEdgeObservConfig::new();
        let config = Config::default().with_verbose(true);
        assert!(svc.execute(&config).is_ok());
    }
}
