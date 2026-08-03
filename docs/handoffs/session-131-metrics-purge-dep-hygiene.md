# Session 131 — Fabricated Metrics Purge + Dep Hygiene

**Date**: Aug 3, 2026

## Changes

- **Dashboard metrics evolved to live data**: `get_overview()` now reads real disk usage via
  `statvfs_space("/")`, network throughput from live collector, and honest zeroes for
  active connections / response time / error rate (previously hardcoded fabricated values).
- **Metrics cache write-path wired**: `start_collection()` persists samples to `metrics_cache`.
- **Hardware tuning deltas computed**: `estimated_power_increase` and `performance_improvement`
  derive from actual before/after metric deltas (3 sites).
- **ZFS AI field sunset**: `AiIntegrationStatus` → `None` (was fabricating `prediction_accuracy: 0.0`).
- **`tokio-util` dep pruned**: Removed direct dep from `nestgate-rpc`, dropped `codec` feature
  from workspace (zero `tokio_util::` imports in production code).
- **Stale comments purged**: "demo purposes", "placeholder", "simplified for demo" cleaned
  across 10+ production files.
- **Tests updated**: 4 dashboard tests evolved to assert honest values.

## Validation

- `cargo check --all-features` PASS
- `cargo clippy --all-features -D warnings` zero warnings
- 327+ tests pass across health/dashboard/hw-tuning modules

## Files Modified

- `code/crates/nestgate-api/src/handlers/performance_dashboard/handlers.rs`
- `code/crates/nestgate-api/src/handlers/performance_dashboard/metrics/mod.rs`
- `code/crates/nestgate-api/src/handlers/performance_dashboard/handlers_tests.rs`
- `code/crates/nestgate-api/src/handlers/hardware_tuning/handlers.rs`
- `code/crates/nestgate-api/src/handlers/hardware_tuning/native_handlers.rs`
- `code/crates/nestgate-api/src/handlers/hardware_tuning/procfs_helpers.rs`
- `code/crates/nestgate-zfs/src/manager/health.rs`
- `code/crates/nestgate-rpc/Cargo.toml`
- `Cargo.toml` (workspace)
- Multiple stale-comment cleanups across `nestgate-zfs`, `nestgate-api`, `nestgate-security`, `nestgate-installer`
