# Session 116 — Typed JSON-RPC Errors, Visibility Tightening, Hardcoded Path Elimination

**Date**: Jul 16, 2026  
**Wave**: 144a  
**Commit**: `978dd7ec`  
**Primal**: nestGate

## Changes

### P1: Canonical JSON-RPC Error Types

- `nestgate_types::transport::jsonrpc::JsonRpcErrorCode` enum — single source of truth for
  `-32700` through `-32604` with `code()`, `default_message()`, `Display`.
- `nestgate_types::transport::jsonrpc::JsonRpcError` struct — replaces **6 independent
  definitions** across `nestgate-api` (3) and `nestgate-rpc` (3).
- **~97 stringly-typed error construction sites** converted to typed enum constructors.
- Zero raw `-326xx` integer literals remain in production code.
- 11 new tests (serde roundtrip, code values, factory methods, Display).

### P2: Visibility Tightening

- 10 handler modules in `nestgate-api` narrowed `pub mod` → `pub(crate) mod`:
  `linux_proc`, `stub_helpers`, `auth_production`, `content_serve`, `coordination`,
  `health`, `performance_analytics`, `workspace_management`, `zero_cost_api_handlers`.
- `models.rs` and deprecated `rest` module also tightened.
- Revealed **283 dead code items** for future cleanup pass.

### Hardcoded Path Elimination

- Removed `/opt/ecoPrimals/depot` hardcoded fallback (2 sites) — now requires
  `ECOPRIMALS_DEPOT_PATH` env var.
- Security socket tier-6 fallback evolved from hardcoded `/run/capability/security.sock`
  to XDG-based construction (`$XDG_RUNTIME_DIR/{ecosystem}/security.sock`).

### Audits

- **Files >800L**: CLEAN — max 760L, zero violations.
- **Dependencies**: CLEAN — pure Rust sovereign stack confirmed (zero C/native crypto).
- **Hardcoding**: 12 HIGH-severity findings documented, 3 fixed this session.

## Remaining Deep Debt Queue

- **Dead code cleanup**: 283 newly-visible dead items from visibility tightening
- **P3**: `strum::Display` on 17 label enums
- **footPrint domain extraction**: in-tree `footprint.*` handlers are composition wiring
  (P2 per Wave 143b), not a violation per se, but flagged for upstream review
- **Coordination provenance fields**: loamSpine/sweetGrass-specific schema in coord types

## Build Status

- `cargo check --workspace` — PASS (zero production code warnings)
- `cargo test --workspace` — PASS (all tests green)
