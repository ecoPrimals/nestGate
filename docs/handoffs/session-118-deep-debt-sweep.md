# Session 118: Deep Debt Sweep — Dead Code Cleanup, Let-Chains, Clippy Zero

**Date**: Jul 16, 2026  
**Wave**: 144b  
**Commit**: (pending)

## Summary

Comprehensive dead code elimination and code hygiene session targeting the 292 dead code
warnings accumulated after Session 116's visibility tightening, plus Rust 2024 let-chain
modernization and clippy pedantic compliance.

## Changes

### Dead Code (292 → 0 warnings)
- **32 stale imports removed** via `cargo fix`: re-exports in `handlers/mod.rs` pointing
  at functions no longer visible after pub → pub(crate) tightening
- **7 stub modules gated** with `#[expect(dead_code, reason = "...")]`:
  `auth_production`, `hardware_tuning`, `workspace_management`, `zero_cost_api_handlers`,
  `rest` (deprecated), `rest::rpc` (deprecated)
- **Dead type alias removed**: `AnalysisConfigCanonical` in `performance_analytics.rs`
  (duplicate of `PerformanceAnalysisConfigCanonical` in `performance_analyzer`)
- **4 unfulfilled lint expectations fixed**: `compliance`, `metrics_collector`,
  `performance_dashboard` modules don't trigger dead_code at module level (items
  are referenced elsewhere)

### Let-Chain Modernization (8 sites)
- `nestgate-config`: `capability_based.rs` (service health check)
- `nestgate-discovery`: `cache.rs` ×3 (port/endpoint/general cache expiration)
- `nestgate-platform`: `linux_proc.rs` (CPU physical-id parsing)
- `nestgate-rpc`: `discovery.rs` (socket path exists), `socket_config.rs` (parent dir
  creation), `session.rs` (prev-data merge)

### Clippy (30 → 0 errors)
- Empty line after doc comment: `rest/handlers/mod.rs`
- Redundant `pub(crate)`: `crud.rs`, `websocket.rs`, `zfs/mod.rs`
- Struct field naming: 3 deprecated REST types (`TimeoutConfig`, `Dataset`, `CostBreakdown`)
- Unnecessary wraps: 7 deprecated REST RPC functions
- Unused async: 4 stub handlers (`credential_validation`, `session`, `secrets`)
- Needless ref mut: 3 deprecated REST RPC methods
- Unfulfilled expects: 2 removed (`pub_underscore_fields`)

### Misc
- Removed unfulfilled `#[expect(async_fn_in_trait)]` from `ZeroCostApiHandler` and
  `ZeroCostDatasetManager` (lint stabilized in Rust 2024)
- Removed unused `BTreeMap` import in `nestgate-rpc` footprint query tests
- `models.rs` auth structs: `LoginRequest`, `LoginResponse`, `AuthToken` gated with
  `#[expect(dead_code)]` (used only by `auth_production` stub chain)

## Verification

```
cargo check                          → 0 nestgate warnings (18 vendored rustls-webpki only)
cargo clippy --all-features -D warn  → 0 nestgate errors
cargo test                           → 1710 passed, 0 failed
```

## Remaining Debt (tracked)

- **17 strum::Display candidates**: `Unified*` enums in `nestgate-types` with manual
  `Display` + `as_str()` duplication. Requires adding `strum` dependency. Deferred.
- **18 `cfg(feature = "ring")` warnings**: From vendored `rustls-webpki`. Not our code.
- **~40 NotImplemented stubs**: Honest 501 returns awaiting capability wiring. Structural.
- **`rest/` module**: Deprecated but maintained for backward compatibility. 182 items
  behind module-level `#[expect(dead_code)]`.
