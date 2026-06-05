+++
title = "NestGate Validation Summary"
description = "Content-addressed storage primal v0.5.0 — 13,039 tests (9,216 lib), 22 crates, 16 capability domains, 4 transport surfaces, BLAKE3 dedup, chunked CAS streaming, cross-gate federation with HTTP parity, direct content serving, BTSP auth, native UDS compliance"
date = 2026-06-05

[taxonomies]
primals = ["nestgate"]
springs = ["airspring", "neuralspring", "wetspring", "groundspring"]
+++

## Status

- **13,039 tests** passing (9,216 lib), **0 failures** (serial and parallel), 0 clippy warnings
- **Session 95 binary UDS compliance**: `service start --socket PATH` natively supported; `IsomorphicIpcServer` fallback now honors `NESTGATE_SOCKET`; unblocks VPS binary refresh for port-free deployment
- **Session 94 Wave 78 parity + deep debt sweep**: `config/capability_registry.toml` at ecosystem convention path; 46 new content pipeline tests; `transport/handlers.rs` refactored (833→384L); primal coupling decoupled (`announce_to_coordinator`); ZFS placeholders evolved to real parsing; production stubs → honest errors; `std::sync::Mutex` → `tokio::sync::Mutex` in async handler; `.to_string()` → `String::from()` migration (32 sites)
- **Session 93 HTTP parity + content serving**: `GET /content/:hash` direct content serving endpoint (raw bytes, correct MIME, immutable caching, `ETag`); 5 UDS-only methods surfaced on HTTP; `content_ops::get_raw` for binary content retrieval; 24 new tests total; westGate ZFS readiness verified
- **Session 92 deep debt evolution**: Load testing fake data → 501 NOT_IMPLEMENTED (3 handlers, 9 tests); `/etc`+`/tmp` hardcoded config defaults → XDG/env-based (TLS certs, ZFS keys, workflows, cache, SSL discovery); `String::from()` batch migration; env-var race fix in `nestgate-config` (20 tests serialized)
- **Session 91 zero failures (Wave 76)**: 16 stale test assertions fixed (auth, migration, discovery, security); env-var race eliminated with `#[serial]` on 55 filesystem tests
- **Session 90 content trust (Wave 75)**: BLAKE3 integrity verification in `content.replicate.pull` — content is self-certifying; `/tmp` hardcoding eliminated (3 sites → `std::env::temp_dir()`); 2 new integrity tests
- **Session 88 evolution sweep**: 9 fake-success paths eliminated (cert manager, auth token minting, credential validation, storage detector, migration framework, network discovery synthesis), unused deps removed (walkdir, async-stream), 3 new auth manager tests
- **Session 87 deep debt sweep**: `storage_stream.rs` split (1,101→676+455), CapabilityRouter fake successes → explicit errors, `String::from()` migration (454 files), dispatch.rs dedup, fsmonitor XDG security defaults, 2 new tests
- **Wave 74 ZFS integration + streaming + snapshots (Session 86)**: Cross-gate integration tests, `content.store_stream`/`content.retrieve_stream` for chunked CAS (4 MiB chunks, BLAKE3 on finalize), `zfs.snapshot.create`/`destroy` RPC, 16 new tests
- **Wave 73 ZFS + federation + mesh (Session 85)**: `NESTGATE_STORAGE_BASE_PATH` for ZFS CAS mounts, `content.replicate.pull` (cold-from-hot), `route.register` for mesh, extended `primal.announce` with gate identity, 15 new tests
- **Deep debt /tmp centralization (Session 84)**: 12 production sites evolved to `std::env::temp_dir()`, idiomatic `String::from()` migration, 10 new tests
- **Wave 67 audit response (Session 82)**: version regression fix, /tmp hardcoding removal, 27 new tests (federation_ops, fsmonitor config)
- **Deep debt sweep (Session 81)**: module split (937L → 525+430), placeholder metrics → honest zeroes, 501 endpoint evolution, 21 new tests
- **Content federation (Wave 60)**: `content.fetch_heads`, `content.push`, `content.replicate`, `content.sync` — 4 new methods enabling waterFall / rootPulse signal graphs to graduate from bash to Neural API
- **v0.5.0**: Unified version across all 22 workspace crates (was `4.7.0-dev` internal / `0.1.0` workspace / `2.1.0` binary)
- **22 workspace packages** (nestgate-rpc, nestgate-api, nestgate-core, nestgate-config, nestgate-types, nestgate-storage, nestgate-security, nestgate-zfs, nestgate-cache, nestgate-discovery, nestgate-bin, and 11 more)
- **16 capability domains** registered in `capability_registry.toml` — storage, content, model, templates, session, audit, nat, beacon, bonding, zfs, health, identity, discovery, lifecycle, auth, btsp
- **4 transport surfaces** with full parity: SemanticRouter, isomorphic IPC (UDS), primary UDS dispatch, HTTP JSON-RPC
- **Content-addressed storage** (NG-1): BLAKE3 hash-as-key, automatic dedup, optional encrypt-at-rest, provenance metadata sidecars
- **Content manifests** (NG-2): versioned path→hash manifests, atomic deploy via `content.promote` aliases, index.html path normalization
- **MethodGate** adopted: Public/Protected method classification, BTSP auth gating
- **`primal.announce`**: JSON-RPC self-registration with biomeOS Neural API on startup (Wave 43)
- **Wave 47 deployment convergence**: `--socket PATH` CLI flag, `health.liveness` normalized to `{"status":"alive","primal":"nestgate"}` across all transports
- **Wave 49 ecosystem tightening**: `plasmidBin` sole binary channel documented, `genomeBin` terminology evolved, 3 dead fuzz targets removed, `notify-plasmidbin.yml` active
- **aarch64-musl segfault fix (validated)**: Replaced `aarch64-linux-gnu-gcc` linker with `ld.lld` + `link-self-contained=yes`; binary built, inspected (static ELF, no dynamic deps), and run under QEMU — no segfault. `nucleus-aarch64-mixed-tcp` cell unblocked
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
