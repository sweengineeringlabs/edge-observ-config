//! `DefaultObservability` — default tracing subscriber implementation.

use crate::api::tracing_config::TracingConfig;
use crate::api::traits::Observability;

/// Default implementation of [`Observability`].
///
/// Installs a `tracing-subscriber` driven by a [`TracingConfig`].
/// Requires the `observability` feature for actual subscriber installation.
#[cfg_attr(not(feature = "observability"), allow(dead_code))]
pub(crate) struct DefaultObservability;

impl Observability for DefaultObservability {
    fn init(&self, config: &TracingConfig) {
        #[cfg(feature = "observability")]
        init_tracing(config);
        #[cfg(not(feature = "observability"))]
        let _ = config;
    }
}

/// Install a `tracing-subscriber` driven by `config`.
///
/// Respects `RUST_LOG` (takes precedence over `config.level` and `config.filter`).
/// Idempotent — safe to call multiple times; subsequent calls are silent no-ops.
/// Does nothing when `config.enabled` is `false`.
#[cfg(feature = "observability")]
pub(crate) fn init_tracing(config: &TracingConfig) {
    use crate::api::tracing_format::TracingFormat;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::tracing_level::TracingLevel;

    #[test]
    fn test_default_observability_init_without_feature_is_noop() {
        let obs = DefaultObservability;
        obs.init(&TracingConfig::default()); // must not panic
    }

    #[cfg(feature = "observability")]
    #[test]
    fn test_init_tracing_does_not_panic_for_json_format() {
        use crate::api::tracing_format::TracingFormat;
        let cfg = TracingConfig {
            format: TracingFormat::Json,
            ..TracingConfig::default()
        };
        init_tracing(&cfg);
    }

    #[cfg(feature = "observability")]
    #[test]
    fn test_init_tracing_does_not_panic_for_pretty_format() {
        init_tracing(&TracingConfig::default());
    }

    #[cfg(feature = "observability")]
    #[test]
    fn test_init_tracing_disabled_is_noop() {
        let cfg = TracingConfig {
            enabled: false,
            ..TracingConfig::default()
        };
        init_tracing(&cfg);
    }

    #[cfg(feature = "observability")]
    #[test]
    fn test_init_tracing_with_custom_level_does_not_panic() {
        let cfg = TracingConfig {
            level: TracingLevel::Warn,
            ..TracingConfig::default()
        };
        init_tracing(&cfg);
    }

    #[cfg(feature = "observability")]
    #[test]
    fn test_init_tracing_with_filter_does_not_panic() {
        let cfg = TracingConfig {
            filter: Some("tower=warn".into()),
            ..TracingConfig::default()
        };
        init_tracing(&cfg);
    }

    #[cfg(feature = "observability")]
    #[test]
    fn test_init_tracing_idempotent_called_twice_does_not_panic() {
        init_tracing(&TracingConfig::default());
        init_tracing(&TracingConfig::default());
    }
}
