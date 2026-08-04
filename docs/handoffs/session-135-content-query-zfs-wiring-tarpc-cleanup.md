# Session 135 — content.query + ZFS REST wiring + tarpc 0.37 + cleanup

**Date**: Aug 4, 2026  
**Commits**: `527c9c81`, `67387f63`, `2ace37b0`, `e192c0f9`

## Changes

### New: `content.query` JSON-RPC method
- Sidecar-scanning CAS metadata filter (`content_type`, `source`, `pipeline`, `stored_by`, `parent_hash`)
- Pagination via `limit`/`offset`; requires at least one filter field
- Wired in dispatch, semantic router, and capability registry
- 5 unit tests (filter extraction, matching logic)

### ZFS REST snapshot handlers evolved (501 → real)
- `list_snapshots` → `ZfsOperations::list_snapshots`
- `create_snapshot` → `ZfsOperations::create_snapshot`
- `get_snapshot` → filters `list_snapshots` by name
- `delete_snapshot` → `ZfsCommand::zfs(["destroy"])`
- `clone_snapshot` → `ZfsCommand::zfs(["clone"])`
- Returns 503 when ZFS CLI unavailable (not silent 501)

### Dependency evolution: tarpc 0.34→0.37
- `tokio-serde` 0.8→0.9 (only breaking change)
- Eliminates opentelemetry 0.18/0.26 chain (thiserror 1.x, hashbrown 0.12, indexmap 1.x, syn 1.x)
- thiserror 1.x reduced to tungstenite (axum upstream) + axum-test (dev)

### content.store_stream sidecar gap fixed
- Streamed CAS finalize now writes `.meta.json` with hash, size, stored_at, pipeline, stored_by, content_type
- Both normal and zero-size paths produce sidecars

### WebSocket synthetic data purge
- Fabricated log/event generators removed from production
- `/ws/logs` and `/ws/events` send honest `not_implemented` and close
- Generators moved behind `#[cfg(test)]`

### Hardcoded values evolved
- `127.0.0.1` → `NESTGATE_API_HOST` / `bind_host()` in primal_announce
- `/mnt/{name}` → `NESTGATE_SUBSTRATE_BASE` env in ZFS helpers
- f64→usize casts → `saturating_f64_to_usize` helper in introspection

### Cleanup
- Quarantined crates (`nestgate-fsmonitor`, `nestgate-middleware`) deleted from tree
- Empty dirs (`nestgate-zfs/config/`, `nestgate-zfs/data/`) removed
- Root docs updated to Session 135 (quarantined crate references removed)

## Validation

- 1,630 tests passed, 0 failed, ~80 ignored
- 0 clippy warnings (pedantic+nursery)
- 0 compiler warnings

## Remaining gaps

- `rand` 0.8+0.9: Blocked across tarpc, oxitls, jsonrpsee, axum
- BTSP local-trust `SO_PEERCRED` (G63): Architecture decision
- `bincode` 1.3→2.x migration
- `network_hardcoded` migration (640+ centralized defaults)
