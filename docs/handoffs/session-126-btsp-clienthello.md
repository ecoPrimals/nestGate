# Session 126 — BTSP ClientHello (Wave 151c)

**Date**: Jul 26, 2026
**Wave**: 151c
**Focus**: P1 Nest Atomic blocker — BTSP client-side wire handshake

---

## What Shipped

### BTSP ClientHello — `btsp_client_handshake.rs`

NestGate had a complete BTSP **server** (Phases 1-3) but zero outbound wire
handshake. Every outbound IPC connection sent plain JSON-RPC, which fails when
a peer enforces BTSP (e.g. bearDog with `BEARDOG_UDS_REQUIRE_BTSP=1`).

New module: `nestgate-rpc/src/rpc/btsp_client_handshake.rs`

**Full 7-step outbound handshake:**

1. `btsp.session.create` → security provider → get `client_ephemeral_pub`, `session_token`
2. Connect to target peer via `connect_transport`
3. Write `ClientHello` (NDJSON): `{protocol, version, client_ephemeral_pub}`
4. Read `ServerHello`: extract `server_ephemeral_pub`, `challenge`
5. `btsp.session.verify` → security provider → get `response`
6. Write `ChallengeResponse`: `{response, preferred_cipher}`
7. Read `HandshakeComplete`: extract `status`, `cipher`, `session_id`

**Integration hook:**

- `connect_with_btsp(endpoint)` → returns `JsonRpcClient` (BTSP or plain based on `is_btsp_required()`)
- `JsonRpcClient::from_btsp_stream(buf_reader)` → construct client from authenticated stream

**Wire format:** JSON-line (NDJSON) — ecosystem-converged format. Servers auto-detect
from the first `{` byte.

### Dead Code Cleanup — `btsp_client.rs`

The old `BtspClient` struct/impl was a dead-code stub with wrong API parameters
(used `family_id` instead of `family_seed`, wrong `btsp.session.verify` params).
Superseded by `btsp_client_handshake::perform_client_handshake`.

- Removed: `BtspClient`, `BtspHandshakeResult`, `BtspSessionStatus`, all parse
  helpers, 3 dead stub tests
- Retained: `resolve_security_socket_path()`, `default_security_socket_path()`,
  `discover_security_socket_xdg()`, 8 resolver tests
- Module renamed in purpose: "BTSP client" → "security provider socket resolution"

### 8 Dependency Patch Bumps

cc 1.3→1.4, clap 4.6.3→4.6.4, either 1.16→1.17, libc 0.2.188→0.2.189,
rustls-pki-types 1.15.0→1.15.1, syn 3.0.2→3.0.3, tokio-stream 0.1.18→0.1.19

---

## Scorecard

| Metric | Value |
|--------|-------|
| Tests | 1,630 passed, 80 ignored |
| Clippy | 0 warnings |
| Fmt | CLEAN |
| Unsafe | 0 (`#![forbid(unsafe_code)]` on all 20 crates) |
| >800L | 0 |
| BTSP Status | **DONE** — server + client handshake shipped |

---

## Next Steps

1. Wire `connect_with_btsp` into priority outbound call sites (capability_discovery,
   primal_announce, storage_encryption, crypto/delegate, atomic health checks)
2. Optional Phase 3 client negotiate (encrypted frame loop for outbound)
3. Integration testing with bearDog BTSP-enforced endpoint
