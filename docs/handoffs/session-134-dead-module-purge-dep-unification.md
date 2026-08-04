# Session 134 — Dead Module Purge + Dep Unification

**Date**: Aug 4, 2026 (Wave 155u/156b)  
**Commit**: `d26fee28`

## Changes

### Dead Module Purge (3,573 LOC)
- **`auth_production`** (8 files, 1,130 LOC): Production auth handlers using `nestgate-core` `AuthManager`. All handlers returned `UNAUTHORIZED` / `NOT_IMPLEMENTED`. Marked `#[expect(dead_code)]`, never wired to routes. Only referenced from tests.
- **`zero_cost_api_handlers`** (7 files, 784 LOC): High-performance pool/dataset/migration handlers. All returned `NOT_IMPLEMENTED`. Same `dead_code` pattern.
- **4 associated test files** (1,659 LOC): `auth_production_tests.rs`, `zero_cost_api_handlers_tests.rs`, `zero_cost_api_handlers_additional_tests.rs`, `zero_cost_tests.rs` — tested dead code only.
- **`models.rs`** (104 LOC): `User`, `LoginRequest`, `LoginResponse`, `AuthToken` types — sole consumers were the deleted modules. Zero external references. `mod models` declaration removed from `lib.rs`.
- **`mod_tests.rs`** updated: removed dead module accessibility test, updated module list assertions.

### rustix 0.38 → 1.1
- Zero code changes required — all APIs (`process::getuid`, `process::Pid::from_raw`, `fs::statvfs`, `system::uname`) compatible
- Eliminated `rustix` + `linux-raw-sys` duplicate from dependency tree (unified with `tempfile`'s rustix 1.x)

### Workspace Quarantine
- **`nestgate-fsmonitor`**: No reverse dependencies; only depends on `notify`. Removed from workspace members.
- **`nestgate-middleware`**: Zero inbound dependencies. Removed from workspace members.
- Both crates remain on disk but excluded from `cargo check/test/clippy/build`.

### Commented-Out Code Removed
- `discovery.rs`: Removed `// discovered.extend(Self::discover_from_config()?);` and `// discovered.extend(Self::discover_from_services()?);` (future stubs — track in handoffs, not code)
- `production.rs`: Two `_service` patterns with `// service.health = status.clone()` and `// service.metadata.extend(metadata)` — replaced with read-only contains_key check + trace logging

## Validation
- `cargo check --all-features` — PASS
- `cargo clippy --all-features -- -D warnings` — PASS (0 warnings)
- `cargo test --all-features` — 1,630 passed, 0 failed, 80 ignored
- `cargo tree -d` — rustix/linux-raw-sys no longer duplicated

## Remaining Dep Duplicates (blocked on tarpc upgrade)
- `thiserror` 1.x + 2.x (via `tarpc` 0.34 → `opentelemetry` 0.18)
- `rand` 0.8 + 0.9 (same transitive path)
- Fix: `tarpc` 0.34 → 0.37 (larger migration)

## Stats
- **27 files changed**: 19 insertions, 3,923 deletions
- **Net**: -3,904 lines
