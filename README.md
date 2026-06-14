# edge-observ-config — DEPRECATED / ABSORBED

> **This repository is no longer maintained.**
>
> Its functionality has been **absorbed into
> [`sweengineeringlabs/observability`](https://github.com/sweengineeringlabs/observability)**
> as the crate **`swe_observability_config`** (workspace member
> `main/backend/features/config`), available from tag **`v0.2.5`**.
>
> Absorbed per RFC sweengineeringlabs/observability#8 (2026-06-08).
> Consumer migration is tracked in sweengineeringlabs/edge#224.

## Where it went

| Old (this repo) | New |
|---|---|
| repo `sweengineeringlabs/edge-observ-config` | repo `sweengineeringlabs/observability` |
| crate `swe-edge-observ-config` (pkg `swe-edge-observ-config`) | crate `swe_observability_config` |
| last published tag `v0.2.1` | `v0.2.5`+ |

## Migration note (API changed — not a drop-in rename)

The subscriber-init input changed type:

- **Old:** `TracingSvc::init(&TracingConfig)`  (config at `ObservabilityConfig.tracing`)
- **New:** `TracingSvc::init(&SubscriberConfig)` (config at `ObservabilityConfig.subscriber`)

`TracingFormat` / `TracingLevel` now belong to `SubscriberConfig`.

---

_This repo is kept (read-only/archived) only so existing `v0.2.1` git pins continue to resolve. Do not add new dependencies on it._
