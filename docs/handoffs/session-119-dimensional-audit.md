# Session 119: Dimensional Audit, cargo fmt, GAP-038 Socket Liveness

**Date**: Jul 18, 2026
**Wave**: 149b
**Commit**: (pending)

## Summary

Ecosystem-aligned dimensional audit of nestGate, formatting cleanup, and GAP-038
stale socket detection. Brings nestGate into full compliance with the 7-project
dimensional review standard established by Wave 149b overwatch.

## Changes

### cargo fmt (133 files)
- `cargo fmt --all` applied to resolve formatting drift across the workspace.
- `cargo fmt --all -- --check` now passes clean.

### GAP-038: PID Sidecar Liveness Check
- **New function**: `is_socket_owned_by_live_process()` in `socket_config.rs`
  - Reads `{socket}.pid` sidecar
  - Parses PID, skips own PID
  - Probes with `rustix::process::test_kill_process(pid)` (kill signal 0)
  - Handles `EPERM` as "process exists" (cross-user scenarios)
- **Wired into both socket preparation paths**:
  - `SocketConfig::prepare_socket_path()` — returns `NestGateError::configuration_error`
  - `IsomorphicIpcServer::prepare_socket_path()` — returns `anyhow::bail!`
- **5 new tests**: stale PID, missing sidecar, own PID, live PID (init/PID 1),
  non-numeric PID

### btsp_client cleanup
- `is_btsp_required()` re-export was production-dead (only used by tests in same
  file). Removed unfulfilled `#[expect(dead_code)]` and gated function with `#[cfg(test)]`.

### Wave stamps
- All root docs (9 files), sporeprint, and DOCS_QUICK_GUIDE updated from 144b to 149b.

### PROJECTS_PATH verification
- Blurb lists as "Open | footPrint | P1" but confirmed complete since Session 114
  (Wave 143b). Implementation in `footprint_base_path()` with 3 tests. Tracker
  entry is stale.

## Dimensional Scorecard (nestGate, Wave 149b)

| Dimension | Status |
|-----------|--------|
| Clippy | **0** warnings (pedantic+nursery) |
| `cargo fmt` | **PASS** (0 files need fmt) |
| Debt markers | **0** (no TODO/FIXME/HACK) |
| Unsafe | **0** (20/20 crates `#![forbid(unsafe_code)]`) |
| Files >800L | **0** (largest: 760L) |
| Tests | **1,630+ pass / 0 fail / 80 ignored** |
| Prod unwrap/expect | **10** (all documented `.expect()`) |
| Prod panic/todo | **0** |
| Production mocks | **0** in default build |
| `cargo fmt` | **PASS** |

## Verification

```
cargo fmt --all -- --check    → PASS (0 files)
cargo check                   → 0 nestgate warnings
cargo clippy --all-features   → 0 nestgate errors
cargo test                    → 1,630+ passed, 0 failed
```

## Remaining (tracked, not blocking)

- **GAP-036**: Socket naming convention — nestGate has canonical `SocketConfig` layout
  but 3+ legacy discovery/launcher paths diverge. Ecosystem-wide standard needed.
- **17 strum::Display candidates**: Deferred (proc-macro cost vs marginal benefit).
- **Dependency patch bumps**: tokio, serde_json, chrono, blake3 have safe patches available.
- **axum 0.8 migration**: Major — coordinate with tower-http + axum-test upgrades.
