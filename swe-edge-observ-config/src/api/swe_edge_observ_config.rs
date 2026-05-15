//! SweEdgeObservConfig trait definition.
//!
//! Implement this trait in core/ to define swe_edge_observ_config's primary behavior.

use super::config::Config;
use super::error::Error;

/// Primary service trait for swe_edge_observ_config.
pub trait SweEdgeObservConfig: Send + Sync {
    /// Execute the primary operation with the given configuration.
    fn execute(&self, config: &Config) -> Result<(), Error>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swe_edge_observ_config_trait_is_object_safe() {
        fn _accept(_s: &dyn SweEdgeObservConfig) {}
    }
}
