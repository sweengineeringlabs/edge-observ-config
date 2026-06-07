# swe-edge-observ-config

## WHAT

Typed observability configuration for swe-edge services — tracing level, format, and output settings
with opinionated defaults and TOML deserialization.

Key capabilities:

- **`TracingConfig`** — typed tracing configuration (`level`, `format`, `output`); `Default` gives production-safe settings (INFO / pretty / stdout)
- **`TracingLevel`** — `trace | debug | info | warn | error`; deserializes from lowercase TOML strings
- **`TracingFormat`** — `pretty | json | compact`; `pretty` is the default for human-readable local logs
- **`ObservabilityConfig`** — top-level config struct wrapping `TracingConfig` with TOML section alignment
- **`TracingSvc`** — SAF factory that initialises the `tracing-subscriber` global from a `TracingConfig`; idempotent
- **`ObservabilityTrait`** — single-method contract (`init()`) for custom observability back-ends
- **`Validator`** — validates string fields against a max-length constraint before initialisation

## WHY

| Problem | Solution |
|---------|----------|
| Every service bootstraps tracing with copy-pasted `tracing_subscriber::fmt()` calls | Single `TracingSvc::init(config)` — consistent subscriber setup across all edge services |
| Tracing level and format are magic strings scattered in bootstrap code | Typed enums (`TracingLevel`, `TracingFormat`) — invalid values caught at deserialisation, not at runtime |
| Config defaults hard-coded in service code diverge across the fleet | `Default` impl on `TracingConfig` / `ObservabilityConfig` supplies production-safe defaults; overriding via TOML is additive |
| Pulling in full runtime just to initialise tracing | Standalone crate — no dependency on `swe-edge-runtime`; usable in CLI tools and library integration tests |
| Diamond dep conflicts when observability config types change | One crate, one tag — all edge consumers pin the same version; kgraph detects conflicts pre-commit |
