# Session 138: C2 Dual-Socket Cephalization + Deep Debt Sweep

**Date**: Aug 6, 2026  
**Wave**: 156j  
**Posture**: Cephalization Advancing

## Summary

Wired the C2 cephalization dual-socket pattern into nestGate's NUCLEUS startup
path — tarpc UDS alongside JSON-RPC UDS — and swept remaining deep debt items
(string literals, doc drift, lint blanket audit).

## C2 Dual-Socket

- **tarpc `unix` feature**: Enabled in workspace `Cargo.toml` and `nestgate-rpc/Cargo.toml`
- **`serve_tarpc_uds()`**: New function in `tarpc_server/mod.rs` (+63 lines) — mirrors
  `serve_tarpc()` but binds via `tarpc::serde_transport::unix::listen` on a UDS path
- **NUCLEUS wiring**: `start_socket_server()` in `service.rs` derives `{name}.tarpc.sock`
  from JSON-RPC socket path and spawns tarpc UDS server alongside IsomorphicIpcServer
- **Cleanup**: `SocketCleanupGuard` + `write_pid_file` made public for reuse; tarpc socket
  gets RAII cleanup + PID sidecar identical to JSON-RPC socket
- **`primal.announce`**: `endpoints.tarpc_uds` advertised in the payload, derived from
  the JSON-RPC socket path
- **`capability_registry.toml`**: Transport list updated to `["uds", "tarpc_uds", "tcp", "http"]`,
  transport_evolution bumped to `phase3-c2`
- **Re-exports**: `serve_tarpc_uds`, `SocketCleanupGuard`, `write_pid_file` re-exported
  through `rpc/mod.rs` → `nestgate-core::rpc`

## Deep Debt Sweep

- **`DEFAULT_SERVICE_NAME`**: Replaced `"nestgate"` string literals with
  `nestgate_config::constants::system::DEFAULT_SERVICE_NAME` in 7 production files:
  `primal_announce.rs`, `content_handlers/ingest.rs`, `content_handlers/fetch.rs`,
  `content_stream.rs`, `footprint_handlers/ingest.rs`, `method_gate.rs`, `launcher.rs`
- **Doc drift**: Fixed `discovery_port` doc comment from 8500 to 8083 in
  `nestgate-config/src/config/runtime/services.rs`
- **Lint blanket audit**: Verified `#[expect(dead_code)]` / `#[expect(missing_docs)]`
  in `nestgate-zfs` and `nestgate-performance` remain justified (staged/incremental
  wiring) — all use `#[expect]` with `reason`, no `#[allow]` in production

## Verification

- `cargo clippy --all-features -- -D warnings` — zero warnings
- `cargo test --all-features` — 1,630+ pass, 0 failures
- 16 files changed, +135/-25 lines

## Files Changed

| File | Change |
|------|--------|
| `Cargo.toml` | tarpc `unix` feature |
| `nestgate-rpc/Cargo.toml` | tarpc `unix` feature |
| `tarpc_server/mod.rs` | `serve_tarpc_uds()` (+63 lines) |
| `service.rs` | Wire tarpc UDS into NUCLEUS startup (+31 lines) |
| `isomorphic_ipc/server/mod.rs` | `SocketCleanupGuard` + `write_pid_file` → pub |
| `isomorphic_ipc/mod.rs` | Re-export `SocketCleanupGuard` + `write_pid_file` |
| `rpc/mod.rs` | Re-export `serve_tarpc_uds` + cleanup utilities |
| `primal_announce.rs` | `endpoints.tarpc_uds` + `DEFAULT_SERVICE_NAME` |
| `content_stream.rs` | `DEFAULT_SERVICE_NAME` |
| `content_handlers/fetch.rs` | `DEFAULT_SERVICE_NAME` |
| `content_handlers/ingest.rs` | `DEFAULT_SERVICE_NAME` |
| `footprint_handlers/ingest.rs` | `DEFAULT_SERVICE_NAME` |
| `method_gate.rs` | `DEFAULT_SERVICE_NAME` |
| `isomorphic_ipc/launcher.rs` | `DEFAULT_SERVICE_NAME` (alias from canonical) |
| `services.rs` | Doc drift fix (8500→8083) |
| `capability_registry.toml` | Transport + transport_evolution updated |

## Upstream Impact

- **C2 pattern adopted**: nestGate joins songBird (C1a) and petalTongue (C1b+C2) as
  primals with the dual-socket C2 cephalization pattern. Other primals can follow
  the same pattern: bind `.tarpc.sock` alongside `.sock`, advertise in `primal.announce`.
- **`SocketCleanupGuard` now reusable**: Made public so any socket-spawning code
  can use RAII cleanup for arbitrary socket paths.
