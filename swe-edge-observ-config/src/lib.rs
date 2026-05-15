//! Standalone observability configuration for swe-edge services.
//!
//! Provides TOML-driven tracing config types and subscriber initialisation
//! with no dependency on edge-runtime. Consumers that do not use the runtime
//! can depend on this crate directly.
//!
//! # Feature flags
//! - `observability` — enable tracing-subscriber initialisation via [`init_tracing`].

mod api;
mod core;
mod saf;

pub use saf::*;
