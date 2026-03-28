# NestGate - Current Status

**Last Updated**: March 28, 2026  
**Version**: 4.5.0-dev

---

## Quick Metrics

```
Build:              13/13 crates compiling (0 errors, including --all-features)
Musl static:        WORKING (4.7MB static binary, x86_64-unknown-linux-musl)
Clippy:             ZERO production warnings (lib target, --all-features)
Format:             CLEAN (cargo fmt --check passes)
Docs:               CLEAN (cargo doc --no-deps, 0 warnings)
Tests:              12,383 passing, 0 failures (469 ignored - ZFS/infra-dependent)
Coverage:           ~72% line (target: 90%, +78 new tests this session)
Files > 1000 lines: 0 (largest: 927 lines)
Unwrap/Expect:      ZERO in production code (test-only, gated by workspace lint)
TODO/FIXME:         ZERO in production code (per wateringHole §13)
Unsafe blocks:      1 production (AVX2 SIMD, feature-gated, well-documented SAFETY)
#[allow] crate-level: 26 in nestgate-api (reduced from 30→26), 1 in nestgate-core
Platforms:          6+ (Linux, FreeBSD, macOS, WSL2, illumos, Android)
```

---

## primalSpring Composition Fixes (Mar 28, 2026)

### Session — primalSpring Phase 17 Integration Debt (exp066/068)

**Fix #1: `family_id` no longer required on every RPC call**
- `StorageState` now carries the server's `family_id` (derived from socket name)
- New `resolve_family_id()` helper: params override > socket-scoped default > error
- All 8 storage handlers (store, retrieve, exists, delete, list, stats, store_blob, retrieve_blob)
  now default to the server's family when callers omit `family_id`
- Cross-family access still works by explicitly passing `family_id`

**Fix #2: Nested key paths now work**
- `store_object()` in `operations/objects.rs` now calls `create_dir_all(object_path.parent())`
- Keys like `test/primalspring/hello` create intermediate directories automatically
- Flat keys continue to work unchanged

**Fix #3: `storage.list` now returns stored keys**
- Root cause: list handler scanned `.../datasets/{family}/objects/` but store writes to
  `.../datasets/{family}/{key}` (no `objects/` segment)
- Removed `.join("objects")` from `storage_list` and `storage_stats` handlers
- List now walks the dataset directory recursively to handle nested key paths
- `storage.stats` uses the same aligned logic

**Fix #4: Musl static build no longer segfaults**
- Root cause: Rust generates static-PIE binaries by default; musl ≤1.2.2 (Ubuntu 22.04)
  crashes in `_start_c/dlstart.c` when processing PIE relocations in a static binary
- Fix: Added `-C relocation-model=static` to musl target rustflags in `.cargo/config.toml`
- Applied to both x86_64 and aarch64 musl targets
- Removed duplicate `[profile.release]` from config.toml (belongs only in Cargo.toml)
- Binary: 4.7MB statically linked, runs correctly

**Bonus: `health.check` ecosystem alignment**
- Unix socket server now responds to `health.check`, `health.liveness`, `health.readiness`
  (in addition to legacy `health`) per primalSpring handoff §1
- Response includes `"primal": "nestgate"` field for composition identification

---

## Deep Debt Evolution (Mar 28, 2026)

### Session 3 — Security Hardening, Stub Elimination & Coverage Push

**Security hardening (authentication):**
- Removed hardcoded `admin/admin` credentials from `HybridAuthenticationManager`
- Local password auth now requires `NESTGATE_LOCAL_AUTH_HASH` env var (argon2 hash)
- `call_security_primal()`: evolved from 50ms sleep + fake token to real Unix socket
  IPC with JSON-RPC `auth.authenticate` protocol, proper error handling
- `validate_token_signature()`: evolved from `return true` stub to real HMAC-SHA256
  verification (payload.signature format)
- `create_workspace_secret()`: evolved from `secret_{id}_{uuid}` to HMAC-SHA256 key
  derivation with OS entropy via `getrandom`
- `AuthTokenManager.create_token()`: now produces HMAC-signed tokens

**Production stub evolution:**
- Monitoring `get_metrics()`: replaced all placeholder system metrics (45+time, 65+cpu*0.3,
  100MB reads, etc.) with real `linux_proc` data (CPU from `/proc/stat`, memory from
  `/proc/meminfo`, network from `/proc/net/dev`, uptime from `/proc/uptime`, load avg
  from `/proc/loadavg`)
- MCP `ProtocolHandler`: health check now returns real uptime via `Instant::now().elapsed()`
  instead of `Duration::from_secs(0)` placeholder
- Connection pool factory: health check validates client handle instead of no-op `Ok(())`
- `sysinfo` fallback disk IOPS: evolved from `disks.count * 1.0` to estimated baseline

**Hardcoding & code quality fixes:**
- Fixed misplaced `#[deprecated]` attribute inside `Debug::fmt` chain in
  `universal_security_client/client.rs` (was syntactically invalid)
- Fixed ZFS dev stub tests expecting hardcoded pool names (`tank`, `backup`) — now
  uses auto-detected system state
- Fixed `parse_bandwidth_unit()` manual strip → idiomatic `strip_suffix()`
- Fixed `circuit_breaker.rs` significant drop in scrutinee (RwLock guard lifetime)
- Fixed `metrics_collector.rs` dead branches sharing identical `(0, 0)` result

**`#[allow]` reduction (nestgate-api 30→26):**
- Removed `significant_drop_in_scrutinee`: fixed 1 circuit breaker instance
- Removed `manual_strip`: fixed 2 bandwidth parser instances
- Removed `or_fun_call`: no instances remained
- Removed `branches_sharing_code`: fixed 3 instances (workspace crud, metrics collector)

**Test coverage push (+78 tests, 12305→12383 passing):**
- Added 13 unit tests for `storage_handlers.rs` (was 432 lines, 0 tests — highest ROI):
  - `resolve_family_id`: param override, state fallback, missing error, override priority
  - Handler validation: store/retrieve/exists/delete/list require params, store requires key
  - Integration: round-trip store+retrieve, list after store, nested key paths
- Updated auth tests to use argon2 password hashing (env-var-based)
- Fixed fragile ZFS config tests (no more hardcoded pool expectations)
- Fixed monitoring test (real metrics may report zero traffic)

---

### Session 2 — Deep Debt Continuation & Production Stub Evolution

**Clippy evolution (ZERO production warnings):**
- Fixed 8 remaining warnings: inner+outer attributes, borrowed format, empty doc line,
  missing Default derive, mul_add, let...else, map_or → is_some_and
- Production `--lib` target now emits zero clippy warnings

**Production stub evolution:**
- `generate_random()` in security_migration: evolved from `vec![0u8; length]` placeholder
  to real `getrandom` OS entropy (cryptographically secure)
- `JsonRpcService.send_request`/`subscribe`: evolved from fake success responses to
  proper `ServiceUnavailable` errors that guide callers toward capability discovery
- `capability_router` mock response: evolved from `success: true` fake to `routed: true`
  with provider/capability/status fields reflecting actual routing state

**Hardcoding evolution:**
- BearDog socket discovery: replaced hardcoded `/tmp/beardog-*` with env-driven
  `NESTGATE_SECURITY_SLUG` + XDG runtime directory scanning
- Socket dirs now follow XDG Base Directory Specification preference chain

**Dependency cleanup:**
- Removed dead `hyper` 0.14 from nestgate-api (no source usage)
- Deleted orphan `nestgate-mcp/src/client.rs` (reqwest-based, not compiled)
- Unified `mockall` to 0.13 workspace-wide (was 0.11/0.12/0.13)
- Unified `axum-test` to workspace (was 14.0/15.0)

**Smart file refactoring (test extraction):**
- `metrics_collector.rs`: 988 → 796 lines (tests → metrics_collector_tests.rs)
- `capability_resolver.rs`: 963 → 615 lines (tests → capability_resolver_tests.rs)
- `template_storage.rs`: 946 → 461 lines (tests → template_storage_tests.rs)
- All files now well under 1000-line limit (largest: 927)

---

### Session 1 — Comprehensive Codebase Audit & Deep Evolution

**Build fixes (critical):**
- `--all-features` build: Fixed 23 compile errors (consul/k8s discovery, adaptive-storage, ZFS init)
- Consul/K8s discovery: Replaced `reqwest` (removed dep) with pure-Rust bootstrap HTTP client
  using `tokio::net::TcpStream` — zero new dependencies, ecoBin compliant
- `services/storage/service.rs`: Removed dead `adaptive-storage` feature code referencing deleted module
- `nestgate-zfs/initialization.rs`: Replaced hardcoded `config.bind_address`/`port` with env-based discovery

**File size compliance (wateringHole: no files > 1000 lines):**
- `storage_paths.rs` (1046) → extracted `substrate_tiers.rs` (262 lines), original now 794 lines
- `security_migration.rs` (1006) → extracted tests to `security_migration_tests.rs` (455 lines), original now 553 lines

**Documentation:**
- Fixed 8 doc warnings: unresolved links, empty code blocks, unclosed HTML tags (`<T>`, `<u8>`)
- `cargo doc --workspace --no-deps` now produces 0 warnings

**TODO/FIXME removal (wateringHole §13):**
- Removed all TODO/FIXME from production `.rs` files (azure.rs, model_cache_handlers.rs,
  security.rs, server.rs, dev_stubs/zfs/types.rs)
- Implemented glob pattern socket scanning in transport/security.rs
- Implemented HTTP fallback server in transport/server.rs

**`#[allow]` reduction (nestgate-api):**
- Removed 10 crate-level `#![allow(clippy::...)]` suppressions (52→30 remaining)
- Removed: `too_many_lines`, `cognitive_complexity`, `double_must_use`, `collection_is_never_read`,
  `return_self_not_must_use`, `iter_on_single_items`, `impl_trait_in_params`,
  `to_string_trait_impl`, `missing_fields_in_debug`, `if_not_else`
- Refactored: `update_workspace_config`, `get_alerts`, `auto_configure`, `attach_standard_routes`,
  `UniversalZfsError::to_error_data` — extracted helper functions

**Stub → real implementation evolution:**
- `http_client_stub.rs`: Evolved from no-op to delegating to `DiscoveryHttpClient` (reqwest-like API)
- `connection_pool/factory.rs`: Evolved from broken placeholder to real `ConnectionPool` + config wiring
- `connection_pool/pool.rs`: Aligned with `NestGateCanonicalConfig` (semaphore, timeouts, bootstrap)

**Deprecated code migration:**
- `JsonRpcUnixServer`: Locally suppressed with `#[allow(deprecated)]` + migration doc comments
  (IsomorphicIpcServer not yet a drop-in replacement due to path layout differences)

**Pure-Rust discovery bootstrap:**
- New `discovery_mechanism/http.rs`: Minimal HTTP/1.1 client for discovery (GET, PUT+JSON)
- Consul discovery: Evolved to pure-Rust HTTP (no reqwest)
- K8s discovery: Evolved to HTTP via kubectl proxy / service mesh (HTTPS documented as needing TLS proxy)

**Test coverage:**
- Added unit tests: return_builders, config_builders, response_builders, canonical traits, MCP errors
- Coverage: 71.4% line (up from 69.6%), target 90%
- Identified high-ROI targets: unwrap-migrator CLI, ZFS modules, API handlers

---

## Deep Debt Evolution (Mar 27, 2026)

### Session — Comprehensive Audit & Evolution

**Build & lint fixes:**
- Clippy: Fixed `test_attr_in_doctest` in `env_isolation.rs`
- Format: `cargo fmt` applied across 3 files
- All workspace checks now pass with `-D warnings`

**Standards compliance:**
- tarpc version skew resolved (nestgate-api aligned to workspace 0.34)
- nestgate-api metadata aligned with workspace (ecoPrimals Collective)
- README license placeholder replaced with AGPL-3.0-only reference
- `rust-toolchain.toml` added (pinned to 1.94.1, clippy + rustfmt + llvm-tools)
- `LICENSING.md` added documenting scyBorg provenance trio alignment

**File size compliance (wateringHole: no files > 1000 lines):**
- `lifecycle.rs` (1095) → directory module: `lifecycle/mod.rs` (683) + `lifecycle/types.rs` (193)
- `metrics_collector.rs` (1034) → cleaned to 988 lines, stub evolved to /proc reads
- `analysis.rs` (1028) → cleaned to 927 lines, removed redundant delegation methods

**Unsafe code evolution:**
- `zero_copy_enhancements.rs`: Removed redundant manual `Send/Sync` impls
- `safe_ring_buffer.rs`: Removed redundant manual `Send/Sync` impls
- `advanced_optimizations.rs`: `LockFreeRingBuffer` aliased to `SafeRingBuffer`, `MemoryPool` rewritten with `parking_lot::Mutex`
- `safe_memory_pool.rs`: Replaced `UnsafeCell` with `parking_lot::Mutex` per slot
- `safe_alternatives.rs`: FFI demo evolved to safe RAII pattern
- `canonical_hierarchy.rs`: Removed unnecessary test `unsafe impl`

**Production stub evolution:**
- `nestgate-installer/download.rs`: Evolved from always-error to GitHub Releases API flow
- `hardware_tuning/handlers.rs`: Evolved from hardcoded resources to /proc-based real data
- `dev_stubs/zfs/config.rs`: Evolved to detect real ZFS via /proc/filesystems and zpool commands
- `metrics_collector.rs`: `get_system_resources` reads /proc/meminfo, /proc/net/dev, thread parallelism

**Hardcoding → capability-based:**
- Orchestrator registration: Uses env-driven fallback (`NESTGATE_ORCHESTRATOR_ADDR`)
- Port configuration: All ports configurable via environment variables
- `hardcoding.rs`: Centralized env-audit table with documented defaults
- Production test URLs use constants, not literals

**Semantic method naming (wateringHole compliance):**
- JSON-RPC: `health.liveness`, `health.readiness`, `health.check`, `health.info`
- `capabilities.list` registered and returning full method inventory
- Isomorphic IPC health methods aligned to `health.check`

**`#[allow]` reduction:**
- Consolidated deprecated canonical alias modules
- Fixed dead_code with `_`-prefixed fields
- Replaced per-item `#[allow(deprecated)]` with module-level

**Dependency evolution (ecoBin v3.0):**
- New `linux_proc` module: `/proc` + `rustix::fs::statvfs` for pure-Rust system metrics
- `sysinfo` annotated for removal; Linux paths prefer `/proc` with `sysinfo` fallback
- `rustix` added to workspace dependencies

**Chaos test stabilization:**
- `chaos_network_packet_loss_1_percent`: Increased trials from 100→1000 for statistical stability
- `chaos_network_packet_loss_10_percent`: Increased trials from 100→2000 for statistical stability

**Session 2 — Coverage & Module Evolution (Mar 27, 2026):**

- Semantic router module (`semantic_router/`) compiled and wired into `rpc/mod.rs`
  - Fixed `NestGateError` API alignment, `NestGateRpcClient` import, clippy lints
  - Routes: `storage.*`, `health.*`, `discovery.*`, `capabilities.list`, `metadata.*`, `crypto.*`
- Unignored 4 tests that don't need ZFS; properly ignored 40 ZFS-dependent tests in nestgate-zfs
- Fixed pre-existing `socket_config::test_biomeos_dir_second_priority` test
- Fixed `safe_memory_pool` doctest (bitmap capacity off-by-one, CAPACITY=1024 exceeded limit)
- New unit tests added across 15 modules:
  - `config/validation`, `constants/consolidated`, `rpc/template_storage`, `cache/manager`,
    `performance/connection_pool`, `security_provider_canonical`, `zero_cost_security_provider/authentication`,
    `traits/security_migration`, `traits/load_balancing/algorithms`, `traits/load_balancing/weighted`,
    `ecosystem_integration/capability_router`, `ai_first_refactored`, `response/mod`,
    `nestgate-zfs/config/tiers`, `nestgate-zfs/automation/actions`
- `sysinfo` further evolved: Linux-native `linux_proc` helpers added (uptime, loadavg, kernel),
  `utils/system.rs` prefers `/proc` on Linux, `sysinfo` demoted to fallback everywhere
- All clippy errors in test files fixed (needless_borrows, const_is_empty, dead_code, match patterns)

### Remaining Debt

| Area | Status |
|------|--------|
| Production unwrap/expect | CLEAN |
| Production TODO/FIXME | CLEAN (removed Mar 28) |
| unsafe blocks | EVOLVED (most replaced with safe alternatives) |
| Hardcoded primal names | EVOLVED (capability-based + env config) |
| TEMP_DISABLED modules | RESOLVED (infra-dependent only) |
| IMPLEMENTATION STUBs | EVOLVED (http_client, connection_pool, download, hardware, ZFS) |
| `#[allow]` suppression | 30 in nestgate-api (reduced from 52), targeting further reduction |
| Ignored tests | 468 (ZFS, E2E, cloud, chaos requiring real infrastructure) |
| Coverage gap to 90% | ~18.6 pp remaining |
| `sysinfo` removal | EVOLVED (Linux uses /proc first; sysinfo = cross-platform fallback) |
| Semantic router | COMPILED & WIRED (`data.*`, `nat.*` routes pending) |
| tarpc RPC manager | Placeholder (`dead_code`); needs completion or removal |
| `JsonRpcUnixServer` | Deprecated, locally suppressed; migration to IsomorphicIpcServer pending |
| reqwest dependency | REMOVED — pure-Rust HTTP client for discovery bootstrap |

### Coverage

```
Current:  71.4% line coverage (llvm-cov, Mar 28 2026)
          70.1% function coverage
          70.1% region coverage
Target:   90% line coverage
Gap:      ~18.6 percentage points
High-ROI targets:
  - tools/unwrap-migrator (~900 lines at 0%)
  - nestgate-zfs large modules (dataset, pool_setup, performance)
  - nestgate-api handlers (high line count, mid coverage)
  - nestgate-core enterprise modules
Path:     ZFS (needs real ZFS), installer (platform), cloud backends, binary entrypoints
```

### Dependency Purity

```
Production:    Mostly pure Rust; platform via rustix (replacing sysinfo)
Crypto:        100% RustCrypto
HTTP client:   Pure Rust (tokio TcpStream for discovery bootstrap)
No direct libc: uzers used instead
Discovery:     mDNS (mdns-sd), Consul/K8s (pure-Rust HTTP, feature-gated)
New:           rustix for /proc filesystem access
```

---

## Architecture

```
nestGate/ (13 crates)
├── nestgate-core       Core: IPC, config, crypto, discovery, linux_proc
├── nestgate-api        REST + JSON-RPC API server
├── nestgate-bin        CLI binary (unibin)
├── nestgate-zfs        ZFS integration (adaptive)
├── nestgate-mcp        MCP provider
├── nestgate-network    Network storage
├── nestgate-automation Automation engine (lifecycle/, analysis, types)
├── nestgate-installer  Platform installer (real GitHub releases download)
├── nestgate-canonical  Canonical types
├── nestgate-middleware Middleware stack
├── nestgate-nas        NAS integration
├── nestgate-fsmonitor  Filesystem monitoring
└── nestgate-performance Performance monitoring
```

---

## Compliance (wateringHole)

| Standard | Status |
|----------|--------|
| UniBin | PASS |
| ecoBin | PASS (sysinfo fallback being evolved) |
| JSON-RPC 2.0 | PASS |
| tarpc | PASS (feature-gated, version aligned) |
| Semantic naming | PASS (JSON-RPC server, IPC) |
| File size (<1000) | PASS (0 files over limit) |
| Sovereignty | EVOLVED (env-driven, capability-based) |
| mDNS Discovery | EVOLVED |
| Crypto delegation | EVOLVED |
| scyBorg license | DOCUMENTED (LICENSING.md) |

---

## Platform Support

| Platform     | Status | IPC   | Build | Tests |
|-------------|--------|-------|-------|-------|
| Linux       | Full   | Unix  | Yes   | Yes   |
| FreeBSD     | Full   | Unix  | Yes   | Yes   |
| macOS       | Full   | Unix  | Yes   | Yes   |
| Windows WSL2| Full   | TCP   | Yes   | Yes   |
| illumos     | Full   | Unix  | Yes   | Yes   |
| Android     | Full   | TCP   | Yes   | Yes   |

---

## Test Substrate (March 27, 2026)

Multi-filesystem testing environment on local hardware:

```
Warm tier (NVMe SSD, rotational=0):
  ext4       /                           1.8 TiB  Samsung 970 EVO Plus

Cold tier (HDD, rotational=1):
  ZFS 2.3.5  /mnt/nestgate/cold/zfs     12.7 TiB  sda+sdb mirror, sdf spare
  btrfs      /mnt/nestgate/cold/btrfs   12.7 TiB  sdc1
  xfs        /mnt/nestgate/cold/xfs     12.7 TiB  sdd1
  ext4       /mnt/nestgate/cold/ext4    12.7 TiB  sde1
```

ZFS datasets: `nestgate/{data,snapshots,cache,testing}`
btrfs subvolumes: `data`, `snapshots`
Substrate tier config: `SubstrateTiers::from_environment()` in `config/storage_paths.rs`
Setup script: `scripts/setup-test-substrate.sh`

---

**Created**: February 1, 2026  
**Latest**: March 27, 2026
