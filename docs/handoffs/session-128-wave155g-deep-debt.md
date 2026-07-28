# Session 128 Handoff: Wave 155g Deep Debt Sweep

**Date**: Jul 28, 2026 | **Wave**: 155g | **Session**: 128 (westGate code team)

## Summary

Full deep debt sweep on westGate. P0/P1 audit items resolved, hardcoded paths centralized, CLI evolved to live daemon probes, EnvSource resolution bug fixed.

## Key Changes

1. **P0: nestgate-api test compilation** — 308 type errors fixed via `pub use` re-exports + `#[cfg(test)]` gating
2. **P1: Security fingerprint test** — Expected hash updated SHA-256 to BLAKE3
3. **P1: CLI health/status** — Evolved from static printlns to live UDS JSON-RPC daemon probes
4. **Hardcoded FHS paths** — 8+ `/var/lib/nestgate/storage` defaults replaced with `get_storage_base_path()` / `resolve_data_dir_from_env_source()`
5. **EnvSource priority bug** — Fixed resolution order: injected env > etcetera auto-detect > system fallback
6. **Repository URLs** — All 21 Cargo.toml `repository =` updated to `git.primals.eco`
7. **Doc stamps** — All root docs updated from Wave 151c to 155g with current test counts

## Verification

- `cargo check`: PASS
- `cargo clippy --all-features -- -D warnings`: zero warnings
- `cargo fmt --check`: clean
- `cargo test --workspace`: 12,973 passed, 0 failed

## Next

- westGate hardware team for ZFS pool creation (5x14TB HDD)
- Storage tiering validation once pool provisioned
- Upstream overwatch audit via golgiBody cascade
