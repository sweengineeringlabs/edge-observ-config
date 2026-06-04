//! `SweEdgeObservConfig` trait — primary service contract.
//!
//! Implement this trait in core/ to define swe_edge_observ_config's primary behaviour.

use crate::api::error::ObservConfigError;
use crate::api::types::config::Config;

/// Primary service trait for swe_edge_observ_config.
pub trait SweEdgeObservConfig: Send + Sync {
    /// Execute the primary operation with the given configuration.
    fn execute(&self, config: &Config) -> Result<(), ObservConfigError>;
}
