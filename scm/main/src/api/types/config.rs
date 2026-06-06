//! `Config` — runtime configuration for swe_edge_observ_config.

/// Configuration for swe_edge_observ_config.
#[derive(Debug, Clone, Default)]
pub struct Config {
    /// Enable verbose output.
    pub verbose: bool,
}
