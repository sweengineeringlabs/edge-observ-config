# swe-edge-observ-config

> **TLDR:** Typed observability configuration for swe-edge — tracing level, format, and subscriber initialisation with production-safe defaults. No runtime dependency. See [Overview](docs/README.md) for details.

Standalone typed observability configuration for swe-edge services.

## Quick Start

```rust
use swe_edge_observ_config::{ObservabilityConfig, TracingSvc};

// Load from TOML or use defaults
let config = ObservabilityConfig::default();
TracingSvc::init(&config.tracing)?;
```

Override via `application.toml`:

```toml
[tracing]
level  = "debug"
format = "json"
```

## Documentation

| Document | Description |
|----------|-------------|
| [Overview](docs/README.md) | WHAT + WHY — capabilities and design rationale |
