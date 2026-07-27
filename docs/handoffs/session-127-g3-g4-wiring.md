# Session 127 Handoff: G3 BTSP→CAS Wiring + G4 Cross-Platform Paths

**Date**: Jul 27, 2026 | **Wave**: 155b | **Session**: 127

## Summary

Addressed NestGate's two glacial goals from Wave 155b:

- **G3: Nest Atomic Phase 0** — Wired the BTSP client handshake (shipped Session 126) into all CAS federation call sites. Cross-gate content replication now uses `JsonRpcClient` + BTSP instead of `socat`/raw TCP.
- **G4: Nest Atomic cross-platform** — Made storage path resolution, ZFS binary detection, and IPC endpoint heuristics platform-aware with Windows fallbacks.

## G3: BTSP → CAS Federation Wiring

### What changed

1. **`federation_ops.rs`** — Complete rewrite of transport layer:
   - Removed `send_jsonrpc_uds` (socat-based, Unix-only)
   - Removed `send_jsonrpc_tcp` (raw TCP, no BTSP)
   - Added `connect_federation(target)` — returns `JsonRpcClient` via `connect_with_btsp`
   - Added `parse_federation_target(target)` — converts `tcp://host:port` or socket paths to `TransportEndpoint`
   - `socat` is no longer a runtime dependency

2. **`content_federation_handlers.rs`** — Callers refactored:
   - `pull_blob_from_remote` — uses `connect_federation` + `client.call("content.get", ...)`
   - `replicate_blob_to_remote` — uses single `connect_federation` for both `content.exists` and `content.put` (connection reuse, one BTSP handshake)

3. **`jsonrpc_client.rs`** — Added `connect_btsp_aware()` method:
   - BTSP-aware entry point for all outbound connections
   - Delegates to `connect_with_btsp` which checks `is_btsp_required()`

4. **`federation_ops_tests.rs`** — Updated tests + 3 new unit tests for `parse_federation_target`

### Impact

- **BTSP coverage**: All cross-gate CAS operations now go through BTSP when the ecosystem requires it
- **socat eliminated**: No external tool dependency for UDS federation
- **Connection efficiency**: Multi-call operations reuse a single connection
- **Cross-platform ready**: Federation now works on Windows via TCP (no UDS dependency)

## G4: Cross-Platform Path Resolution

### What changed

1. **`storage_paths/resolve.rs`** — Platform-aware fallbacks:
   - Added `resolve_home()` — tries `HOME` then `USERPROFILE`
   - Added `system_*_fallback()` helpers — `%ProgramData%`/`%LOCALAPPDATA%` on Windows, FHS on Unix
   - `Path::join` used consistently instead of string concatenation with `/`

2. **`storage_paths/paths.rs`** — ZFS binary detection:
   - `zfs_binary_path()` / `zpool_binary_path()` fall back to bare `zfs`/`zpool` on Windows

3. **`btsp_client.rs`** — Runtime base resolution:
   - Added `resolve_runtime_base()` — `$XDG_RUNTIME_DIR` → `/run/user/{uid}` (Unix) → `$TEMP` (Windows)

4. **`security_primal.rs`** — IPC endpoint heuristics:
   - Added `\\.\pipe\` prefix recognition for Windows named pipes

5. **`api/transport/security.rs`** — Capability discovery:
   - Extended local IPC detection to include named pipes and `.sock` extension

### Impact

- **Windows-ready path resolution**: All storage paths resolve correctly on Windows
- **No FHS fallback on Windows**: Uses `%ProgramData%` and `%LOCALAPPDATA%` instead of `/var/lib/`
- **Named pipe awareness**: IPC discovery can find Windows-native endpoints

## Additional Debt

- Fixed pre-existing test `get_socket_path_prefers_xdg_runtime_dir_when_set` (expected wrong path)
- 9 additional primal name → capability-based language replacements in docs
- Remaining `biomeOS` protocol naming (~60 refs) is intentional ecosystem protocol layer — not primal coupling

## Scorecard

| Metric | Before | After |
|--------|--------|-------|
| BTSP in CAS federation | No (socat/raw TCP) | Yes (connect_with_btsp) |
| socat dependency | Required for UDS | Eliminated |
| Windows path fallbacks | Hardcoded FHS | Platform-aware |
| Pre-existing test failures fixed | 1 (socket path) | 0 remaining in nestgate-rpc |
| Clippy | 0 warnings | 0 warnings |
| nestgate-rpc tests | 960 pass | 960 pass |

## Files Changed

| File | Change |
|------|--------|
| `nestgate-rpc/federation_ops.rs` | socat/TCP → connect_with_btsp + parse_federation_target |
| `nestgate-rpc/content_federation_handlers.rs` | Callers → connect_federation + client.call |
| `nestgate-rpc/federation_ops_tests.rs` | Updated + 3 new parse tests |
| `nestgate-rpc/jsonrpc_client.rs` | Added connect_btsp_aware() |
| `nestgate-rpc/btsp_client.rs` | resolve_runtime_base() cross-platform |
| `nestgate-rpc/btsp_server_handshake/mod.rs` | Primal name → capability-based |
| `nestgate-rpc/isomorphic_ipc/server/server_tests.rs` | Fixed pre-existing test |
| `nestgate-config/storage_paths/resolve.rs` | Platform-aware fallbacks |
| `nestgate-config/storage_paths/paths.rs` | ZFS binary cross-platform |
| `nestgate-api/transport/security.rs` | Named pipe IPC detection |
| `nestgate-security/security_primal.rs` | Named pipe endpoint detection |
| `nestgate-types/transport/endpoint.rs` | Doc: sourDough → scaffolding tool |
| Root docs (5 files) | Primal names → capability-based |
| `sporeprint/validation-summary.md` | Primal names → capability-based |
