# Session 117: Phase 2 Transport — TransportStream + TransportListener

**Date**: Jul 16, 2026 | **Wave**: 144b | **From**: eastGate overwatch

## Summary

Shipped Phase 2 transport for nestGate — completing server-side abstraction
(`TransportListener`, `TransportStream`) that was the last gap identified in
the Phase 2 transport audit. nestGate joins the 13/14 "shipped" count.

## Changes

### New Types (`nestgate-rpc::isomorphic_ipc::transport_stream`)

- **`TransportStream`** — enum (Unix | Tcp) with `AsyncRead + AsyncWrite` impl
- **`TransportListener`** — enum (Unix | Tcp) with `bind_unix`, `bind_tcp`,
  `from_tcp`, `accept() -> (TransportStream, String)`, `display_address()`,
  `unix_path()`, and `Display` impl
- **`connect_transport()`** — canonical outbound connect function returning
  `TransportStream` (moved from `streams.rs` implementation)

### Server Refactoring

- `IsomorphicIpcServer::try_unix_server` → `TransportListener::bind_unix` +
  shared `serve_listener()` accept loop
- `TcpFallbackServer::start/start_bound` → `TransportListener::bind_tcp` +
  shared `accept_loop_transport()` + `handle_transport_connection(TransportStream)`
- Both handlers accept `TransportStream` via `tokio::io::split()`

### Client Consolidation

- `JsonRpcClient::connect_unix` → delegates to `connect_transport(Uds)`
- `JsonRpcClient::connect_tcp` → delegates to `connect_transport(Tcp)`
- No more raw `UnixStream::connect`/`TcpStream::connect` in client code

### Backward Compatibility

- `IpcStream` re-exported as `type IpcStream = TransportStream`
- `handle_unix_connection()` retained as thin wrapper over `handle_connection(TransportStream)`
- `streams::connect_transport()` delegates to canonical function

### Removed Dead Code

- `AsyncStream` marker trait (unused; enum dispatch supersedes)
- Raw `tokio::net::UnixStream` import from `jsonrpc_client.rs`

## Test Impact

- 7 new tests (UDS roundtrip, TCP roundtrip, listener accept, mesh relay error,
  UDS nonexistent, display format, transport_type)
- All existing tests pass (952 in nestgate-rpc + full workspace green)

## Files Changed

| File | Change |
|------|--------|
| `nestgate-rpc/src/rpc/isomorphic_ipc/transport_stream.rs` | NEW — TransportStream + TransportListener + connect_transport |
| `nestgate-rpc/src/rpc/isomorphic_ipc/mod.rs` | Register module, update re-exports, IpcStream alias |
| `nestgate-rpc/src/rpc/isomorphic_ipc/streams.rs` | IpcStream → type alias, delegate connect_transport, remove AsyncStream |
| `nestgate-rpc/src/rpc/isomorphic_ipc/server/mod.rs` | TransportListener bind, serve_listener, handle_connection(TransportStream) |
| `nestgate-rpc/src/rpc/isomorphic_ipc/tcp_fallback.rs` | TransportListener bind, handle_transport_connection |
| `nestgate-rpc/src/rpc/jsonrpc_client.rs` | Delegate connect_unix/tcp to connect_transport |
| `config/capability_registry.toml` | Updated transport_evolution comment |
| Root docs (11 files) | Wave 144a → 144b |
| CHANGELOG.md | Session 117 entry |
| STATUS.md | Session range 43–117 |

## Remaining

- Window NamedPipe variant on TransportStream (post-MVP, platform-specific)
- MeshRelay connect via songBird relay negotiation
- Deprecated `nestgate-api/transport/` server stack removal
- 283 dead code items from Session 116 visibility tightening
