+++
title = "NestGate Validation Summary"
description = "Content-addressed storage primal — 12,399+ tests, 22 crates, 16 capability domains, 4 transport surfaces, BLAKE3 dedup, primal.announce, BTSP auth, Wave 47 deployment convergence"
date = 2026-05-25

[taxonomies]
primals = ["nestgate"]
springs = ["airspring", "neuralspring", "wetspring", "groundspring"]
+++

## Status

- **12,399+ tests** passing (682 RPC, 11,720+ across 22 workspace packages), 0 failed, 0 clippy warnings
- **22 workspace packages** (nestgate-rpc, nestgate-api, nestgate-core, nestgate-config, nestgate-types, nestgate-storage, nestgate-security, nestgate-zfs, nestgate-cache, nestgate-discovery, nestgate-bin, and 11 more)
- **16 capability domains** registered in `capability_registry.toml` — storage, content, model, templates, session, audit, nat, beacon, bonding, zfs, health, identity, discovery, lifecycle, auth, btsp
- **4 transport surfaces** with full parity: SemanticRouter, isomorphic IPC (UDS), primary UDS dispatch, HTTP JSON-RPC
- **Content-addressed storage** (NG-1): BLAKE3 hash-as-key, automatic dedup, optional encrypt-at-rest, provenance metadata sidecars
- **Content manifests** (NG-2): versioned path→hash manifests, atomic deploy via `content.promote` aliases, index.html path normalization
- **MethodGate** adopted: Public/Protected method classification, BTSP auth gating
- **`primal.announce`**: JSON-RPC self-registration with biomeOS Neural API on startup (Wave 43)
- **Wave 47 deployment convergence**: `--socket PATH` CLI flag, `health.liveness` normalized to `{"status":"alive","primal":"nestgate"}` across all transports
- **Wave 49 ecosystem tightening**: `plasmidBin` sole binary channel documented, `genomeBin` terminology evolved, 3 dead fuzz targets removed, `notify-plasmidbin.yml` active
- **Stale socket cleanup**: `SocketCleanupGuard` (RAII), `ctrl_c` graceful shutdown, PID sidecars
- **Rust 2024 edition**, `#![forbid(unsafe_code)]`, `clippy::pedantic` + `clippy::nursery` clean
- **`cargo deny check bans`** passing, pure-Rust crypto (no ring, no OpenSSL)
- **Zero** unsafe code, bare `#[allow]` without reason, TODO/FIXME in committed code

## Key Capabilities

| Domain | Methods | Transport Parity | Stability |
|--------|---------|:----------------:|-----------|
| content | `put`, `get`, `exists`, `list`, `publish`, `resolve`, `promote`, `collections` | All 4 | stable |
| storage | `store`, `retrieve`, `list`, `delete`, `retrieve_stream`, `retrieve_range` | All 4 | stable |
| lifecycle | `status` | All 4 | stable |
| capabilities | `list` | All 4 | stable |
| auth | `check`, `mode`, `peer_info` | All 4 | stable |
| identity | `get` | All 4 | stable |
| btsp | `capabilities` | All 4 | stable |
| model | `register`, `exists`, `locate`, `metadata` | All 4 | provisional |
| zfs | `pool.list`, `pool.get`, `pool.health`, `dataset.list`, `dataset.get`, `snapshot.list`, `health` | All 4 | provisional |

## Shadow Run Readiness (Wave 24 S3)

NestGate is the storage backend for the S3 Content Hosting Shadow (vs GitHub Pages).
petalTongue is the HTTP-facing edge.

- **8 `content.*` methods** on all 4 transports (Session 60)
- **Path normalization** in `content.resolve`: `/` → `/index.html`, `/about` → `/about/index.html` (Session 66)
- **Timing metadata**: `resolved_in_ms` / `retrieved_in_ms` for TTFB measurement (Session 66)
- **Provenance**: `content.put` accepts `source`, `pipeline`, `stored_by`; `content.get` returns all metadata (Session 62)
- **Atomic deploy**: `content.publish` + `content.promote` for blue-green content deployment

## Architecture

```
Browser → petalTongue :8080 (HTTP edge)
       → nestGate content.resolve (content-addressed storage)
       → BLAKE3 hash verification + optional decrypt
       → inline base64 response with content_type + timing
```

## Consuming Springs

| Spring | Consumption |
|--------|-------------|
| neuralSpring | Weight persistence via `storage.*` IPC |
| airSpring | NestGate + Squirrel IPC wired |
| wetSpring | Content storage for pipeline outputs |
| groundSpring | NestGate IPC module in `src/ipc/` tree |

## See Also

- [Primal Catalog](https://primals.eco/architecture/primal-catalog/) on primals.eco
- `capability_registry.toml` — machine-readable capability surface
- `CHANGELOG.md` — full session history
