//! `DefaultObservability` — default tracing subscriber implementation.

use crate::api::tracing::tracing_config::TracingConfig;
use crate::api::traits::observability::Observability;

/// Default implementation of [`Observability`].
///
/// Installs a `tracing-subscriber` driven by a [`TracingConfig`].
/// Requires the `observability` feature for actual subscriber installation.
pub(crate) struct DefaultObservability;

impl DefaultObservability {
    /// Install a `tracing-subscriber` driven by `config`.
    ///
    /// Requires the `observability` feature. Idempotent — safe to call multiple
    /// times; subsequent calls are silent no-ops. Does nothing when
    /// `config.enabled` is `false`.
    #[cfg(feature = "observability")]
    pub(crate) fn init_tracing(config: &TracingConfig) {
        use crate::api::tracing::tracing_format::TracingFormat;
        use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

        if !config.enabled {
            return;
        }

        let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            let base = config.level.as_str();
            let directive = match &config.filter {
                Some(f) if !f.is_empty() => format!("{base},{f}"),
                _ => base.to_owned(),
            };
            EnvFilter::new(directive)
        });

        match config.format {
            TracingFormat::Json => {
                let _ = tracing_subscriber::registry()
                    .with(filter)
                    .with(
                        fmt::layer()
                            .json()
                            .flatten_event(true)
                            .with_current_span(true)
                            .with_span_list(false),
                    )
                    .try_init();
            }
            TracingFormat::Pretty => {
                let _ = tracing_subscriber::registry()
                    .with(filter)
                    .with(fmt::layer().pretty())
                    .try_init();
            }
        }
    }
}

impl Observability for DefaultObservability {
    fn init(&self, config: &TracingConfig) {
        #[cfg(feature = "observability")]
        Self::init_tracing(config);
        #[cfg(not(feature = "observability"))]
        let _ = config;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: init
    #[test]
    fn test_init_without_feature_is_noop() {
        let obs = DefaultObservability;
        obs.init(&TracingConfig::default()); // must not panic
    }

    #[cfg(feature = "observability")]
    /// @covers: init_tracing
    #[test]
    fn test_init_tracing_does_not_panic_for_json_format() {
        use crate::api::tracing::tracing_format::TracingFormat;
        let cfg = TracingConfig {
            format: TracingFormat::Json,
            ..TracingConfig::default()
        };
        DefaultObservability::init_tracing(&cfg);
    }

    #[cfg(feature = "observability")]
    /// @covers: init_tracing
    #[test]
    fn test_init_tracing_does_not_panic_for_pretty_format() {
        DefaultObservability::init_tracing(&TracingConfig::default());
    }

    #[cfg(feature = "observability")]
    /// @covers: init_tracing
    #[test]
    fn test_init_tracing_disabled_is_noop() {
        let cfg = TracingConfig {
            enabled: false,
            ..TracingConfig::default()
        };
        DefaultObservability::init_tracing(&cfg);
    }

    #[cfg(feature = "observability")]
    /// @covers: init_tracing
    #[test]
    fn test_init_tracing_with_custom_level_does_not_panic() {
        use crate::api::tracing::tracing_level::TracingLevel;
        let cfg = TracingConfig {
            level: TracingLevel::Warn,
            ..TracingConfig::default()
        };
        DefaultObservability::init_tracing(&cfg);
    }

    #[cfg(feature = "observability")]
    /// @covers: init_tracing
    #[test]
    fn test_init_tracing_idempotent_called_twice_does_not_panic() {
        DefaultObservability::init_tracing(&TracingConfig::default());
        DefaultObservability::init_tracing(&TracingConfig::default());
    }
}
