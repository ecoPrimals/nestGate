# NestGate Session 137 AAR — O8 Neural API Wiring

**Date**: Aug 5, 2026
**Primal**: nestGate
**Gate**: eastGate (overwatch)
**Wave**: 156e

## Summary

Session 137 completed O8 — the reframed "canonical client crate" → "complete Nest
Atomic participant" wiring. Instead of publishing a `nestgate-client` crate (which
would violate the ecosystem's runtime-discovery principle), nestGate is now fully
discoverable and routable through the biomeOS Neural API coordinator.

Consumers call `capability.call("content", "get", { hash: "..." })` on the coordinator.
The coordinator already knows nestGate from `primal.announce`, scores it by
latency/cost/affinity, and routes the request. No consumer needs a nestgate-specific
crate — they use the ecosystem's JSON-RPC client (biomeOS/Tower).

## Changes

### 1. Announce payload expanded (`primal_announce.rs`)

- `ANNOUNCED_CAPABILITIES`: added `"dataset"` (5 domains total)
- `FEDERATION_METHODS`: added `"dataset.convergence"` (cross-gate provenance checks)
- Method filter: added `|| m.starts_with("dataset.")` so `dataset.*` methods appear
- Tests updated to assert all 5 domains and new federation method

### 2. `capability_registry.toml` `[announce]` section updated

- `capabilities` array now includes `"dataset"`
- `federation_methods` now includes `"dataset.convergence"`

### 3. `route.register` expanded (`dispatch.rs`)

- Previously hardcoded `announce_capability("storage", ...)` and
  `announce_capability("content", ...)` — now dynamically iterates all capabilities
  from the announce payload
- Response JSON reflects full announced set instead of hardcoded `["storage", "content"]`

### 4. Remote capability router wired to Neural API (`router.rs`)

**Key architectural change.** `send_universal_request()` evolved from:
```
Err(not_implemented("mesh relay transport (not yet wired)"))
```
to an async method that:
1. Discovers the coordinator socket (same env-var chain as `primal_announce`)
2. Connects via `JsonRpcClient::connect_btsp_aware`
3. Forwards the request as `capability.call` JSON-RPC
4. Returns the coordinator's response as a `CapabilityResponse`

Added `discover_coordinator_for_routing()` helper with standard env-var chain.

### 5. `MeshRelay` transport connected (`transport_stream.rs`)

- `connect_transport()` now handles `MeshRelay` endpoints by discovering the
  coordinator socket and connecting to it (coordinator proxies relay traffic)
- Added `discover_coordinator_socket_for_relay()` for transport-layer discovery
- Updated tests in both `transport_stream.rs` and `streams.rs`

## Stats

| Metric | Value |
|--------|-------|
| Tests | 1,630 pass / 0 fail / ~80 ignored |
| Clippy | 0 warnings (pedantic+nursery) |
| Files changed | 7 Rust files + 1 TOML config |
| Announce domains | 4 → 5 (added `dataset`) |
| Federation methods | 5 → 6 (added `dataset.convergence`) |
| Route register | 2 hardcoded → dynamic (all 5) |

## Architecture (post-O8)

```
Consumers (tideGlass, groundSpring, airSpring)
  │
  └─ capability.call("content", "get", {hash}) ─→ Neural API Coordinator
                                                      │
                                                      ├─ primal.announce registry
                                                      │   (nestGate: storage, content,
                                                      │    dataset, coordination, footprint)
                                                      │
                                                      └─ route to best nest provider ─→ nestGate
                                                         (or cross-gate via songBird mesh relay)
```

## Upstream impact

- **tideGlass divergences (DIV-1→6)**: consumers should migrate from direct nestGate
  connections to `capability.call` through the Neural API — nestGate is now fully
  registered and routable
- **federation**: `dataset.convergence` available cross-gate for provenance verification
- **other primals**: this session establishes the Nest Atomic wiring pattern that
  other primals can follow for their own `primal.announce` + coordinator routing

## Remaining for upstream

- Consumers need to migrate to `capability.call` routing (upstream coordination)
- `ipc.relay` handshake protocol for `MeshRelay` streams (coordinator-side)
- `bincode` 1.3→2.x blocked on tarpc compatibility
- BTSP `SO_PEERCRED` (G63) needs architecture guidance
