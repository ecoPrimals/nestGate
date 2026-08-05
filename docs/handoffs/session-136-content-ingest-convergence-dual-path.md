# Session 136 — content.ingest, dataset.convergence, dual-path CAS

**Date**: Aug 5, 2026
**Wave**: 156e (overwatch cascade from eastGate)

## Summary

Executed on overwatch punch list items O1, O3, O4 from Wave 156e.
Fixed a gap where `content.fetch` was missing from the semantic router dispatch.

## Changes

### O1: `content.ingest` — P1 upstream gap resolved

New JSON-RPC method: bulk directory scan → BLAKE3 hash → CAS store.

- **Handler**: `content_handlers/ingest.rs` (~350 lines)
- Recursively walks a directory, hashes each file with BLAKE3, stores to CAS
- Automatic dedup (same file in two paths → one CAS object)
- Returns `{manifest: {path: hash}, count, bytes_total, bytes_stored, deduplicated}`
- Optional `collection` param → auto-publishes manifest via `content.publish`
- Files >256 MiB skipped (use `content.store_stream`)
- Provenance sidecar: `pipeline: "content.ingest"`, `stored_by: "nestgate"`
- MIME type guessing for 30+ extensions (science formats included)
- **9 unit tests**: validation, empty dir, manifest, dedup, content types
- Eliminates `revalidate_data.py` and directory-to-CAS Python glue

### O3: `dataset.convergence` — P1 upstream gap resolved

New JSON-RPC method: CAS provenance state per dataset path.

- **Handler**: `dataset_handlers.rs` (~300 lines)
- Walks a filesystem dataset path, hashes each file, checks CAS presence
- Reports state: CONVERGED | PARTIAL | PRIMORDIAL | EMPTY | MISSING
- Returns `{state, total_files, total_bytes, cas_files, cas_bytes, convergence_pct}`
- Optional `include_details` for per-file breakdown
- Optional `sample_limit` for fast sampling
- New `dataset` capability domain in registry
- **10 unit tests**: classification, validation, edge cases
- Trust gate for spring data consumption

### O4: Dual-path CAS — hot/cold tier resolution

Wired `SubstrateTiers` (already in `nestgate-config`) into CAS read/write paths.

- **Write policy**: `NESTGATE_WARM_PATHS` first path → CAS writes (NVMe hot tier)
- **Read policy**: check hot → cold → legacy `get_storage_base_path()`
- **Backward compatible**: when no warm/cold env vars set, behavior is identical
- Updated 5 CAS handlers: `content.put`, `content.get`, `content.exists`, `content.list`, `content.query`, `content_get_raw`, `content.fetch`
- Added `resolve_cas_object()` and `cas_content_dirs()` helpers
- Env vars: `NESTGATE_WARM_PATHS` (colon-separated hot paths), `NESTGATE_COLD_PATHS` (colon-separated cold paths)

### Gap fix: `content.fetch` + `content.ingest` in semantic router

- `content.fetch` was missing from semantic router dispatch — added
- `content.ingest` wired into all 6 dispatch surfaces
- Both added to `capabilities.rs` advertisement list

### Wiring (all new methods)

For each new method, wired into:
1. UDS primary dispatch (`dispatch.rs`)
2. Module exports (`content_handlers/mod.rs` / `dataset_handlers` in `mod.rs`)
3. Semantic router content/dataset domain (`semantic_router/{content,dataset}.rs`)
4. Semantic router dispatch (`semantic_router/mod.rs`)
5. Capabilities advertisement (`semantic_router/capabilities.rs`)
6. Capability registry (`config/capability_registry.toml`)

## Validation

- `cargo check --workspace --all-features` — PASS
- `cargo clippy --all-features -- -D warnings` — PASS (zero warnings)
- `cargo test --all-features -p nestgate-rpc` — 996 passed, 0 failed
- No pre-existing tests broken

## Method Count

- UDS methods: 91 → 94 (+3: `content.ingest`, `dataset.convergence`, + `content.fetch` semantic router gap)
- Capability domains: 20 → 21 (+`dataset`)

## Remaining from Wave 156e overwatch punch list

- O2 (`content.fetch`): Already shipped (streaming fix Session 133)
- O5 (TCP on westGate): Already implemented — ops config only (`NESTGATE_JSONRPC_TCP=1` or `--port 8080`)
- O7 (inter-gate E2E): Ops testing, not code
- O8 (canonical client crate): Architecture decision — requires cross-primal coordination
