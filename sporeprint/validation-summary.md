+++
title = "NestGate Validation Summary"
description = "Content-addressed storage primal v0.5.0 — 3,790 tests, 20 crates, 20 capability domains, 4 transport surfaces, Wave 142b (visibility tightening, unwrap_or, infallible nonce), CI-DIV-03, NESTGATE-ANDROID-01, STARTUP-NG-01, riboCipher, BTSP auth"
date = 2026-07-15

[taxonomies]
primals = ["nestgate"]
springs = ["airspring", "neuralspring", "wetspring", "groundspring"]
+++

## Status

- **3,790 tests** (3,790 passing, 73 ignored), **0 failures** (serial and parallel), 0 clippy warnings
- **Session 112 Deep debt sweep** (Wave 142b): `btsp_client`, `btsp_phase3`, `primal_announce` → `pub(crate)` (3 internal-only modules); `generate_server_nonce()` simplified from `Result<[u8;32]>` → `[u8;32]` (infallible); 31 `unwrap_or_else(|| String::from(...))` → `.into()` across 18 files
- **Session 111 Deep debt sweep** (Wave 141b): Streaming hot-path clone elimination — 4 function signatures `Value` → `&Value`, removes 4 `.clone()` of potentially-MB base64 chunks in `content_ops.rs`; cast safety fix (`u64 as u8` → `u8::try_from`); 55 `String::from` → `.into()` across 6 files; cross-arch 14/14 reported to overwatch
- **Session 110 Deep debt sweep** (Wave 141a): 11 ZFS production mocks → honest `not_implemented`; `/proc/meminfo` for real memory (was hardcoded 16GB); ARC fallback 0.85→0.0; 7 `String::from` → `.into()`; full codebase audit confirmed clean (no files >800L, no primal coupling, pure Rust deps)
- **Session 109 Cross-architecture adoption** (Wave 141a): `cargo check --target x86_64-pc-windows-gnu` PASS; Category 3 (rustix::fs::statvfs gated) + Category 1 (UDS transport gated across 9 files); TCP fallback always available
- **Session 108 Deep debt sweep** (Wave 140a): Test fixture gating (`create_test_certificate` → `#[cfg(test)]`); platform FS audit (all `PermissionsExt` already `#[cfg(unix)]` — zero Phase 3 blockers); 63 more `String::from` → `.into()` (pools 31, knowledge 13, system_config 9, cert/utils 10)
- **Session 107 Deep debt sweep** (Wave 139a): ~425 `String::from("literal")` → `.into()` across 36 files / 9 crates; 8 `Result<_, String>` → `&'static str` (3 → `const fn`); `ZfsError` → `thiserror`; 3 enum `#[default]` migrations; `/opt/nestgate` → `NESTGATE_INSTALL_PATH` env override; unused type + redundant docs removed; production mock surfaces verified feature-gated
- **Session 106 COORD-ACTIVATE + deep debt sweep**: Coordination domain wired to all 4 RPC surfaces (UDS dispatch was 14/14 — now HTTP JSON-RPC, transport handler, capability advertisement all 14/14); `coord_ops.rs` bridge module; `primal_announce` + `capability_registry.toml` updated with coordination capability; 3 error types → `thiserror`; 6 other-primal runtime strings → capability-type references; health.rs fake metrics eliminated; `DEFAULT_SECURITY_SOCKET_PATH` → env-overridable function; 8 `Result<_, String>` → `&'static str`; 3 `const fn` promotions; 30+ `String::from` → `.into()`
- **Session 105 NESTGATE-ANDROID-01**: UDS fatal on grapheneGate fixed — `PRIMAL_BIND_MODE=tcp_only` authoritative across service.rs + IsomorphicIpcServer; empty env-var filtering; 6 new tests
- **Session 104 CI-DIV-03**: `.cargo/config.toml` musl linker converged from `ld.lld` to `aarch64-linux-gnu-gcc` (ecosystem standard)
- **Session 103 riboCipher signal acceptance**: `[0xEC, 0x01]` prefix stripped via `fill_buf()`/`consume(2)` on production UDS, legacy UDS, and TCP fallback connection handlers — no-op for plain JSON-RPC clients; 7 new tests in `protocol.rs`
- **Session 102 STARTUP-NG-01**: `nestgate server` defaults to HTTP (guideStone Stream 1 primal startup contract); `--enable-http` removed, `--socket-only` is the opt-out; `PRIMAL_BIND_MODE` env respected (`tcp_only`/`tcp` → HTTP, `uds_only`/`uds` → socket-only); legacy `nestgate-server` symlink defaults HTTP; 17 new tests (7 `resolve_enable_http`, 10 `commands/env.rs`)
- **Session 101b NG-DOWNCAST-01 fix**: `is_platform_constraint()` chain-walking evolution — `find_io_error()` walks `source()` chain to find nested `io::Error` through `.context()` wrappers; `UnixListener::bind` error path preserved via `.context()` instead of `anyhow::anyhow!()`; 7 new tests; root docs refreshed to honest test counts; registry synced from canonical `config/`; gitignore cleaned
- **Session 101 Deep Debt Sweep Pass 3**: 43 new tests (TLS validation 11, safe migration 10, config migrator 14, dispatch handlers 7, protocol 1); `CAPABILITY_ANNOUNCE_TTL` dedup (3 announce call sites → shared constant in `protocol.rs`); stale `#[expect(clippy::option_if_let_else)]` removed; `doc_markdown` lint fix for `SELinux`
- **Session 100 Deep Debt Sweep Pass 2**: BLAKE3 hash centralization (`content_hash_hex()` + `content_cas_path()` as canonical CAS helpers — 4 production files consolidated); `CONNECTION_IDLE_LIMIT` dedup (3 identical definitions → shared constant in `protocol.rs`); 45 new tests (template storage 19, storage paths 13, validation runner 6, protocol 1, plus inline test updates); `storage_paths` module visibility evolved to `pub(crate)` for cross-module CAS access
- **Session 99 Deep Debt Sweep**: Security auth stubs → real HMAC-SHA256 (4 new tests); legacy `BEARDOG_*` env vars emit deprecation warnings; 3 unused deps removed (`getrandom`, `etcetera` x2); migration validators → real field checks; TLS `validate()` → real cert path checks; `create_stub_*` → `snapshot_*` naming; `#![expect(clippy::unnecessary_wraps)]` removed
- **Session 98 Transport Evolution Phase 2**: Outbound IPC migration complete — all production `connect_unix()` and raw `UnixStream::connect` calls (11 sites across 9 files) migrated to `connect_transport()` via `TransportEndpoint` abstraction. Only transport layer implementations and test code retain direct socket calls. `tokio::net::UnixStream` import removed from `transport/security.rs`. `security_primal.rs` evolved from UDS-specific `writable()`/`try_write()` to idiomatic `AsyncReadExt`/`AsyncWriteExt`.
- **Session 97 Transport Evolution Phase 1**: 21 new tests — `TransportEndpoint` type (13), `connect_transport()` (3), `transport_to_ipc_endpoint()` (3), `resolve_outbound_endpoint()` (3); `JsonRpcClient` evolved to `BufReader<IpcStream>` with `connect_transport()` method; binary accepts `TRANSPORT_ENDPOINT` env var; HTTP mode documented as Tier 5 fallback; `capability_registry.toml` updated with `transport_evolution = "phase1"`
- **Session 96 coverage Tier 2**: 31 new tests — route handlers (5), ZFS helpers parsers (18), StorageConfig (5), content_ops facade (3)
- **Session 95b coverage sprint**: 22 new tests — encrypted content roundtrip (7), JSON-RPC handler content dispatch (7), content stream edge branches (8)
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
- **aarch64-musl fix (validated)**: `link-self-contained=yes` + `relocation-model=static` prevents musl ≤1.2.2 segfault; linker converged to `aarch64-linux-gnu-gcc` (ecosystem standard, Session 104)
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
