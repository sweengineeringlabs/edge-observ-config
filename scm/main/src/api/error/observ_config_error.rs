//! `ObservConfigError` — domain error variants for swe_edge_observ_config.

/// Domain errors for in swe_edge_observ_config.
#[derive(Debug, thiserror::Error)]
pub enum ObservConfigError {
    /// An I/O error occurred.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    /// A configuration error occurred.
    #[error("Configuration error: {message}")]
    Config {
        /// Human-readable description of the configuration problem.
        message: String,
    },
}
