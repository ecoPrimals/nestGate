# Session 120: Dependency Bumps, Socket Ecosystem Segment, Scorecard Audit

**Wave**: 150b | **Date**: Jul 18, 2026 | **Primal**: nestGate

## Summary

Three categories of work: supply-chain freshness, GAP-036 socket path convergence,
and dimensional scorecard confirmation for ecosystem reporting.

## Changes

### 1. 99 Dependency Patch Bumps

All semver-compatible updates applied via `cargo update`. Key updates:
- blake3 1.8.4 → 1.8.5
- bytes 1.11.1 → 1.12.1
- chrono 0.4.44 → 0.4.45
- clap 4.6.1 → 4.6.2
- dashmap 6.1.0 → 6.2.1
- futures 0.3.32 → 0.3.33
- tokio-util 0.7.15 → 0.7.16
- zeroize 1.8.2 → 1.9.0

Full test suite passes: 1,630 tests, 0 failures.

### 2. Socket Path Ecosystem Segment (GAP-036 Alignment)

Legacy socket paths used flat layout (`$XDG_RUNTIME_DIR/nestgate.sock`) which
diverged from the canonical `SocketConfig` layout that includes an ecosystem
path segment (e.g., `biomeos/`).

**Fixed in 3 locations:**

- **`discovery.rs`** `discover_unix_socket_from_env()`: Now resolves
  `$XDG_RUNTIME_DIR/<ecosystem>/{service}.sock`
- **`launcher.rs`** `get_nestgate_socket_path_from_env_source()`: Same fix;
  doc comment updated; test updated to verify ecosystem subdirectory
- **`server/mod.rs`** `get_socket_path()` fallback: Same fix; doc comment clarified
  as fallback-only path

This converges all socket discovery paths toward the canonical layout established
by `SocketConfig::resolve()`.

### 3. Dimensional Scorecard Audit

Confirmed values for ecosystem reporting:

| Dimension | Value | Notes |
|-----------|-------|-------|
| Tests | 1,710 (1,630 pass + 80 ignored) | `cargo test` |
| Clippy | 0 | `cargo clippy -- -D warnings` |
| Fmt | 0 | `cargo fmt --all -- --check` |
| Debt | 0 | No markers in committed code |
| Unsafe | 0 | `#![forbid(unsafe_code)]` on all 20 crate roots |
| >800L | 0 | No production files exceed 800 lines |
| Prod unwrap | audit | 965 `unwrap()` + 1,422 `expect()` in non-test code (deep audit pending) |

### 4. Wave Stamps

All root docs updated: 149b → 150b (9 files).

## Ecosystem Status

- **GAP-036**: Socket naming convention — NestGate now uses ecosystem segment in all
  socket discovery paths. Legacy flat paths eliminated.
- **GAP-038**: Closed (Session 119 — PID sidecar liveness).
- **PROJECTS_PATH CAS**: Confirmed complete (Session 114 — `footprint_base_path()`).

## Open Items

- **Prod unwrap deep audit**: 965 + 1,422 non-test unwrap/expect calls need triage
  (many are in deprecated REST layer, stub handlers, and config defaults — candidates
  for proper error propagation).
- **`primal-transport` crate** (P2): Phase 2 `TransportStream`/`TransportListener`
  types are candidates for ecosystem extraction.
