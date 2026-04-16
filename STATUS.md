# NestGate - Current Status

**Last Updated**: April 16, 2026 (Session 43v — coverage 84.12%, deprecated 114, flaky test fix, storage_tests split)  
**Version**: 4.7.0-dev

---

## Quick Metrics

```
Build:              PASS — cargo check --workspace --all-features --all-targets (0 errors), as of 2026-04-16
Clippy:             PASS — cargo clippy --workspace --lib -- -W clippy::all -W clippy::pedantic -W clippy::nursery (zero warnings), as of 2026-04-16
Format:             CLEAN (cargo fmt --check passes), as of 2026-04-16
Docs:               PASS — cargo doc --workspace --no-deps (zero warnings), as of 2026-04-16
Tests:              8,636 passing, 0 failures, 60 ignored (cargo test --workspace --lib), as of 2026-04-16
Coverage:           84.12% line (cargo llvm-cov --workspace --lib --summary-only, 2026-04-16) — wateringHole 80% met; 90% target pending
Files > 800 lines:  0 (all .rs files under 800 LOC; 4 large files refactored Session 43p)
Unwrap/Expect:      ZERO in production library code
Inline markers:     none in committed production `.rs` (wateringHole policy — verified 2026-04-11)
Unsafe code:        #![forbid(unsafe_code)] on ALL crate roots (zero exceptions — env-process-shim uses edition 2021 safe wrappers)
println! in lib:    ZERO in core libs; installer retains stdout for interactive wizard UX (documented)
Dead code:          ZERO unwired modules, ZERO `if false` stubs, ZERO #[allow(dead_code)] in production
Box<dyn Error>:     ZERO in production library code (last one evolved Session 43k — ConfigError::ParseError → String)
async-trait:        ZERO compiled usages, ZERO dependency (not in any Cargo.toml)
Mocks in prod:      ZERO — all mocks test-only (#[cfg(test)]) or feature-gated (dev-stubs)
Stubs:              Feature-gated behind `dev-stubs` cargo feature (opt-in only, zero production leakage)
TLS/crypto:         ureq + rustls-rustcrypto (pure Rust); ring/reqwest/openssl/native-tls ELIMINATED from dep tree AND lockfile; installer uses system curl
Discovery:          Environment variables + capability IPC (mDNS/Consul/K8s discovery_mechanism removed; delegated to orchestration provider)
MCP:                Not a workspace member — use biomeOS `capability.call` / capability IPC instead
IPC routes (UDS):   storage.*, session.*, model.*, templates.*, audit.*, nat.*, beacon.*, zfs.*, bonding.ledger.*, health.*, capabilities.*, identity.*, discovery.* — 51 methods (UNIX_SOCKET_SUPPORTED_METHODS const)
IPC routes (HTTP):  storage.dataset.*, storage.object.*, storage.*_stream*, discovery.capability.*, health.*, capabilities.*, identity.* — 23 methods (JSON_RPC_CAPABILITIES_METHODS const)
IPC routes (tarpc): storage.*, metadata.*, crypto.*, session.*, discovery.*, health.*, capabilities.* — 42 explicit semantic-routed methods (`semantic_router/mod.rs` match arms)
data.* delegation:  Removed from router — callers should discover data capability provider via `capabilities.list`
Wire Standard:      Level 3 (Composable) — {primal, version, methods} envelope, provided_capabilities (12 groups, 51 methods), consumed_capabilities (3), protocol, transport
Emoji in logs:      ZERO in library tracing — professional structured logging only (Session 43f)
Registry:           capability_registry.toml — machine-readable self-knowledge, cross-check invariant tests
Capability symlink: storage[-{fid}].sock → nestgate[-{fid}].sock (auto-managed lifecycle, family-scoped per BTSP Phase 1)
BTSP Phase 1:      PASS — BIOMEOS_INSECURE guard, family-scoped socket naming, generic FAMILY_ID fallback
TCP JSON-RPC:      Functional — `--port`, `--listen`, NESTGATE_API_PORT, or NESTGATE_JSONRPC_TCP=1 activates TcpFallbackServer alongside UDS
UDS keep-alive:    PASS — persistent connections (multiple sequential requests per connection); flush after every response (LD-03 resolved)
sysinfo:            OPTIONAL — Linux uses pure-Rust /proc parsing; sysinfo on non-Linux only
Platforms:          6+ (Linux, FreeBSD, macOS, WSL2, illumos, Android)
Decomposition:      nestgate-core split into 13 crates (295K→52K lines, core deps 51→44)
Primal self-knowledge: Re-exported through nestgate-core from nestgate-discovery (single import path)
Primal sovereignty: DEFAULT_SERVICE_NAME constant; env-overridable; zero other-primal refs in production code
Workspace deps:     100% hoisted to workspace = true (zero version drift)
Workspace members:  23 (20 code/crates + tools/unwrap-migrator + fuzz + root nestgate)
Serial tests:       #[serial]: scoped to ZFS command stub tests (temp_env::with_vars elsewhere)
Numeric casts:      Dangerous narrowing `as` casts evolved to try_from/saturating; benign widening casts remain
Supply chain:       cargo deny check — advisories ok, bans ok, licenses ok, sources ok (rustls-webpki 0.103.12 vendored ring-free, rand 0.9.x, CDLA-Permissive-2.0 allowed)
ring:               ELIMINATED — zero stanzas in Cargo.lock (rustls-webpki vendored without ring optional dep)
dyn audit:          317 matches: 154 test-only, 39 comments, ~65 dyn Error, ~22 std traits, ~37 intentional DI/strategy/plugin patterns — zero stadial debt
async-trait:        ZERO #[async_trait] attrs, ZERO dep in Cargo.toml; 73 text matches are comments/docs/migration templates only
CONTEXT.md:         Present (per wateringHole PUBLIC_SURFACE_STANDARD)
```

**Note:** The fossil placeholder `tests/mdns_discovery_integration_tests.rs` was removed April 2026; mDNS removal and delegation are documented in `CHANGELOG.md` and project handoffs.

---

## Session 43 — Deep Debt Evolution & primalSpring Compliance (Apr 12, 2026)

### primalSpring Audit Response
- **Doc drift**: STATUS.md method counts reconciled to code — per-surface: UDS 51 (`UNIX_SOCKET_SUPPORTED_METHODS`), HTTP 23 (`JSON_RPC_CAPABILITIES_METHODS`), tarpc semantic router 42 explicit routes (`semantic_router/mod.rs`)
- **TCP/`--port` wiring**: Socket-only mode now resolves port from `NESTGATE_API_PORT` env; activates TCP alongside UDS when env port differs from default
- **Domain symlink**: Confirmed already implemented (`storage[-{fid}].sock` → `nestgate[-{fid}].sock`); compliance matrix update proposed
- **Deprecated APIs**: 210→187 (17 zero-caller items removed Sessions 43–43g; 6 dead port constants Session 43g; 6 dead functions Session 43j)
- **Box\<dyn Error\>**: 5 production function signatures evolved to typed `NestGateError` / `Result<T>` (Session 43g)

### Smart file refactoring (4 largest files)
- `jsonrpc_server/mod.rs` 794→185 lines (extracted `storage_methods.rs`, `capability_methods.rs`, `monitoring_methods.rs`)
- `storage_handlers.rs` 771→446 lines (extracted `blob_handlers.rs`, `external_handlers.rs`)
- `crud.rs` 762→433 lines (extracted `crud_properties.rs`, `crud_helpers.rs`, `crud_list.rs`)
- `tarpc_types/mod.rs` 738→463 lines (extracted `storage.rs`, `metadata.rs` DTOs)

### `as` cast evolution (production)
- `btsp_server_handshake.rs`: `len as usize` → `usize::try_from(len)` with error mapping
- `websocket.rs`: `usize`/`u64` → `u32` casts to `u32::try_from(...).unwrap_or(u32::MAX)`
- `storage/service.rs`: `u64` → `f64` division → `u128` integer math (no precision loss)
- `observability/metrics.rs`: Added division-by-zero guard for empty histograms

### Clone optimization
- `JsonRpcServer::start`: Removed unnecessary `state.clone()` (moved into scope)
- `InMemoryBackend::announce`: Build `PrimalInfo` from `&SelfKnowledge` instead of cloning entire struct

### Test coverage push (+42 tests)
- `pool_ops.rs` 59→99%, `trait_impl.rs` 62→99%, `tier.rs` 64→86%
- `metadata_backend.rs`: error paths, edge cases, concurrency
- `unix_socket_server/mod.rs`: unknown method, malformed request handling
- `registry.rs`: register/deregister cycles, capability queries, concurrent access
- Flaky fake-ZFS tests stabilized with `can_spawn_fake_zfs` pre-flight check

### Verification (all PASS)
- `cargo fmt --all --check` — PASS
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` — PASS (zero warnings)
- `cargo doc --workspace --no-deps` — PASS (zero warnings)
- `cargo test --workspace` — 11,792 passing, 0 failures, 451 ignored

---

## Ground truth refresh (Apr 12, 2026 — Session 43)

Measured with `cargo check` / `cargo clippy --workspace --all-targets --all-features -- -D warnings` / `cargo fmt --all --check` / `cargo test --workspace` / `cargo deny check bans` / `cargo doc --workspace --no-deps`.

- **Production file size**: All production `.rs` files under **750** lines. Session 43 refactored 4 largest: `jsonrpc_server/mod.rs` 794→185, `storage_handlers.rs` 771→446, `crud.rs` 762→433, `tarpc_types/mod.rs` 738→463.
- **Workspace**: **23** members (20 code/crates + tools + fuzz + root). Zero clippy warnings. Zero C-FFI `-sys` crates in production.
- **Concurrency**: Zero lock-across-await. All `Mutex` in async context uses `tokio::sync::Mutex` or `parking_lot::Mutex` (sub-microsecond). Zero `std::sync::Mutex` in async. `DiagnosticsManager` migrated to `tokio::sync::RwLock`.
- **Testing**: Zero `thread::sleep` waits (except chaos/timeout). `#[serial]`: **0**. `EnvSource` trait injection for env isolation. Fake-ZFS tests have `can_spawn_fake_zfs` pre-flight checks for stability under parallel load.
- **Defaults**: Bind defaults to `127.0.0.1` (secure-by-default). Fallback port is `0` (ephemeral, OS-assigned). Hardcoded ports centralized to `runtime_fallback_ports` constants with env-var overrides.
- **Stubs**: Production mock builders gated behind `#[cfg(any(test, feature = "dev-stubs"))]`. `NoopStorage` documented as intentional null-object backend. All test doubles behind `#[cfg(test)]`.
- **Numeric safety**: Dangerous narrowing `as` casts evolved to `try_from`/`saturating`/`u128` integer math. Benign widening casts remain. Custom `unix_secs()` helper for timestamp conversions.
- **Dependency injection**: `StorageBackend` and `MetadataBackend` traits in RPC layer allow `nestgate-core`'s filesystem-backed storage to back tarpc/semantic router (NG-01 resolved).
- **Copyright**: 2025-2026 across all source files. SPDX on all .rs files.
- **Coverage**: ~81.7% line (llvm-cov). 42 new tests added Session 43 targeting low-coverage files.

---

## Trait Excision, Flaky Test Fix & Idiomacy Session (Mar 30, 2026)

### Deprecated trait excision (~2,300 lines removed)
- Deleted `canonical_provider_unification.rs` (798 lines), `security_migration.rs` (558 lines),
  `security_migration_tests.rs` (553 lines) — all zero-consumer closed cycles
- Deleted `canonical_hierarchy/` (7 files, ~400 lines) and `migration/` (7 files, ~500 lines)
- Rewrote `traits/mod.rs` — removed 5 module declarations, 80+ lines of stale migration comments
- Updated deprecation notes in 5 files to point to `CanonicalSecurity` (not removed path)

### Clippy zero (6 warnings → 0)
- Fixed: unused import, `map_or` → `is_none_or`, `.err().expect()` → `.expect_err()`,
  strict float comparison → epsilon, double-nested `mod tests`, redundant clones

### Flaky test evolution (env race conditions)
- Replaced raw `set_var`/`remove_var` with `temp_env::with_var` + `#[serial_test::serial]`
  across 8 test functions in 5 files (`fault_injection_tests.rs`, `test_support.rs`,
  `agnostic_config.rs`, `capability_port_discovery.rs`, `environment_error_tests.rs`, `defaults.rs`)

### Hardcoded primal names → self-knowledge constant
- Replaced `"nestgate"` string literals with `DEFAULT_SERVICE_NAME` in 8 production files:
  RPC health, IPC discover_capabilities, capability list, self-knowledge, tracing, JWT issuer

### TEMPORARY/TEMP_DISABLED cleanup
- Removed dead `capability_auth` compat module + `SecurityModularizationComplete` marker struct
- Removed dead `DiscoveryManager` type alias; cleaned 4 stale TEMP_DISABLED comments

### Allow-block reduction (nestgate-api, nestgate-core)
- **nestgate-api**: 67 lints → 31 (22 production + 16 test-only); fixed 8 real bugs
  (unused imports, dead field, redundant clones, unfulfilled expect)
- **nestgate-core**: Eliminated nuclear `clippy::all/pedantic/nursery/restriction` in test cfg
  → 12 targeted test-only lints

### Dependency cleanup
- Removed 4 orphaned workspace deps: `gethostname`, `ipnetwork`, `tungstenite`, `tokio-tungstenite`

### Debris cleanup
- Removed 2 empty directories: `nestgate-zfs/{data,config}`

---

## Deep Debt Execution & Evolution Session (Mar 30, 2026)

### Allow-block reduction (4 crates)
- **nestgate-bin**: Evolved from blanket `clippy::all/pedantic/nursery/restriction/cargo` to 7 targeted allows.
  Fixed: `option_if_let_else`, `branches_sharing_code`, `redundant_pattern_matching`, `let...else`,
  `cast_possible_truncation`, `empty_line_after_doc_comments`, `needless_pass_by_value`.
- **nestgate-network/mcp/performance**: Removed `clippy::unwrap_used`, `clippy::expect_used`,
  `clippy::redundant_clone`, `clippy::type_complexity`, `clippy::float_cmp`, `clippy::ip_constant`,
  `clippy::manual_string_new`, `clippy::manual_range_contains`, `clippy::needless_collect` from
  production crate-level allows. Moved to `#[cfg_attr(test, allow(...))]` since zero production uses exist.

### Production stub evolution (6 files, 0 fake-success paths remaining)
- **Universal storage adapter HTTP**: Fake data on read/silent no-op on write → honest `not_implemented` errors
- **REST metrics ZFS fields**: Hardcoded arc_size=2GB, throughput=100/50Mbps → zeroed defaults until real ZFS wiring
- **REST metrics history**: Fabricated time-series data → empty response with TSDB-not-wired log
- **MCP AI optimization**: Silent `Ok(())` → `not_implemented` error when no engine available
- **REST storage benchmark**: 500ms sleep + fake scores → `BENCHMARK_NOT_IMPLEMENTED` error
- **REST cloud scan mock**: "Cloud Storage (Mock)" fabricated backend → removed (logged when requested)
- **Remote ZFS HTTP client**: 157 lines of dead unreachable HTTP code → 70-line honest error stub

### Zero-copy improvements
- **RPC list_datasets**: `format!("{pool}/")` per-item in filter → hoisted outside loop
- **JSON-RPC transport**: `String::from_utf8_lossy` + `from_str` → direct `serde_json::from_slice`

### Dependency evolution
- **tokio**: `features = ["full"]` → minimal `[fs, io-util, macros, net, process, rt-multi-thread, signal, sync, time]`
- **gethostname** (libc wrapper): Eliminated → replaced with `rustix::system::uname().nodename()`
- **rustix**: Added `system` feature for hostname resolution
- **nestgate-mcp**: Removed 12 unused deps (sysinfo, config, rand, tracing-subscriber, sha2, futures, dashmap, chrono, anyhow, fastrand)
- **nestgate-network**: Removed 8 unused deps (anyhow, futures-util, tokio-tungstenite, tungstenite, async-stream, tower, tower-http, ipnetwork)
- **nestgate-platform**: Removed unused gethostname dep

### Test stabilization
- **service_integration_tests**: Fixed parallel env-var race condition with `#[serial_test::serial]`
- All monitoring tests updated for honest metrics (no longer assert on fabricated ZFS data)
- Storage adapter tests updated (HTTP returns not_implemented, not fake bytes)

---

## Comprehensive audit & deep debt evolution session (Mar 29, 2026)

### Full workspace clippy clean (pedantic + nursery + -D warnings)
- **ALL 25 crates + root tests/benches/examples** pass `cargo clippy --workspace --all-targets -- -D warnings`
- Crate-level `#![cfg_attr(test, allow(...))]` for test-only patterns (unwrap, expect, panic, float_cmp)
- Targeted `#![allow(...)]` with documented reasons for config-heavy and deprecated-migration code
- Fixed real issues: unused async (40+ functions de-asynced), numeric cast precision, format strings, redundant clones

### Safety evolution
- **`#![forbid(unsafe_code)]`** on ALL crate roots (zero exceptions — env-process-shim uses edition 2021 safe wrappers)
- **println!/eprintln!** eliminated from library code — migrated to `tracing::info!/debug!/warn!/error!`
- **Production stubs** feature-gated behind `dev-stubs` cargo feature (opt-in only)
- **serde_yaml_ng** (unsafe-libyaml) removed from core/config — dead dependency eliminated

### Dependency evolution
- **ring**: ELIMINATED — installer switched from `rustls-tls` to `rustls-tls-webpki-roots-no-provider` + `aws-lc-rs`
- **sysinfo**: OPTIONAL — Linux uses pure-Rust `/proc` parsing; sysinfo only for non-Linux
- **serde_yaml_ng**: removed from production crates (only fuzz retains for YAML fuzzing)

### Coverage evolution
- **74.5% → 80.25% line** (llvm-cov), **73.3% → 79.67% function** — target: 90%
- 1,457 lib tests (from baseline ~1,100), 0 failures
- Targeted coverage for 0% files: ZFS, API, config modernization, cache, automation, types, discovery
- RPC coverage raised to 84.2%, types to 83.9%, observe to 87.6%
- E2e (20+ files), chaos (20+ files), fault injection (7 files) test suites present

### Zero-copy evolution
- `Cow<'static, str>` migration completed for `core_errors.rs` error types
- Zero-copy constructor cleanup resolved — constructors accept `impl Into<Cow<'static, str>>`
- Zero allocation on hot error paths with static string literals

---

## Deep debt evolution & idiomatic Rust session (Mar 29, 2026)

### Clippy pedantic zero-warning pass
- **13 pedantic lint categories zeroed**: `const_fn`, `must_use`, `Self` pattern, `match_same_arms`, `unnecessary_wraps`, `significant_drop_tightening`, `unused_self`, `uninlined_format_args`, `derive_partial_eq_without_eq`, `ref_option_ref`, `used_underscore_binding`, `missing_const_for_fn`, `return_self_not_must_use`
- **Warnings**: 4,642 → 2,972 (remaining: docs linting, numeric casts on non-hot paths)
- **unused_async**: 229 → 122 (hot paths de-asynced, trait-required kept with `#[expect]`)
- **Cast precision**: Hot paths use `f64_to_u64_saturating` / `u64_to_f64_approximate` (nestgate-zfs numeric module)

### API evolution
- **Deprecated `JsonRpcUnixServer`** migrated to `IsomorphicIpcServer` in all production entry points
- **`EventsErrorConfig`** migrated to `CanonicalNetworkConfig`
- **ZFS config types** (`DatasetConfig`, `SnapshotConfig`, etc.) un-deprecated (they are the real types, not network types)
- **`hardcoding::ports`** → 15 call sites migrated to `runtime_fallback_ports`
- **`env!("CARGO_PKG_NAME")`** → literal `"nestgate"` for primal identity throughout all crates

### Zero-copy evolution
- Reused `Vec<u8>` buffers + `serde_json::from_slice` in IPC line readers
- `Cow::Borrowed` for tarpc endpoint strings
- Eliminated redundant `.to_string()` / `.clone()` in socket scanning and snapshot parsing

### Test coverage
- **8,177 tests passing** (lib), 0 failures, 64 ignored
- Re-enabled 10 environment-sensitive tests in `nestgate-config` (temp-env + serial_test)
- All `#[ignore]` reasons documented with run instructions
- `# Errors` doc sections added to 52 top public APIs

### Dependency audit
- **No openssl** — uses rustls (good)
- **ring** (C/ASM crypto) — transitive only via reqwest→rustls in installer; RustCrypto backend not yet stable upstream
- **unsafe-libyaml** via serde_yaml_ng — REMOVED from core/config; only fuzz retains
- **inotify-sys** via notify — kernel FFI, expected for Linux

---

## Documentation & quality session (Mar 29, 2026)

### Ground-truth refresh

- **Doctests**: Fixed 65 failing doctests across 7 crates; `cargo test --doc` clean
- **Tests**: +704 net new tests (11,003 → 11,707), all passing
- **Coverage**: 68.4% → 74.3% line (llvm-cov)
- **Clippy**: Cleanup pass — unnecessary `unsafe`, `Send` issues, trivial regex, bulk auto-fixes; warnings 8,227 → 4,642
- **Types**: `Arc<str>` adoption for discovery/RPC identifiers; `Cow<'static, str>` for JSON-RPC wire types
- **New crate**: `nestgate-env-process-shim` — safe env mutation for parallel tests (Rust 2024 `set_var` rules)
- **Root docs**: README, DOCUMENTATION_INDEX, QUICK_REFERENCE, QUICK_START, STATUS, CHANGELOG aligned to 4.7.0-dev and paths under `code/crates/`, `docs/`, `config/`

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

**Debt marker removal (wateringHole §13):**
- Removed migration/debt markers from production `.rs` files (azure.rs, model_cache_handlers.rs,
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
- README license placeholder replaced with AGPL-3.0-or-later reference
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
| Production unwrap/expect | CLEAN (library `src/`; tests may use unwrap/expect) |
| Production inline markers (wateringHole §13) | CLEAN |
| unsafe blocks | ZERO — `#![forbid(unsafe_code)]` on all crate roots (including env-process-shim) |
| Hardcoded primal names | CLEAN (DEFAULT_SERVICE_NAME + env config) |
| Production stubs | EVOLVED (routes return real AppState data; dev stubs feature-gated) |
| TLS/crypto | Delegated to security capability provider via IPC; installer uses system curl (ring/rustls/reqwest ELIMINATED) |
| `sysinfo` dependency | OPTIONAL (Linux: pure-Rust /proc; non-Linux: sysinfo) |
| Coverage gap to 90% | ~8.3 pp remaining (81.7% current; Session 43 targeted low-coverage files +42 tests) |
| Semantic router | COMPILED & WIRED — `data.*` delegates; `nat.*`, `beacon.*` routes active; `discovery` overstep modules deprecated |
| `#[allow(dead_code)]` | 0 production `#[allow(dead_code)]` — dead code removed rather than suppressed |
| MCP in-tree | REMOVED from workspace — external biomeOS / capability.call |
| Automation in-tree | DEPRECATED — zero production consumers; all deps removed from nestgate-zfs/api/fuzz |
| mDNS in-tree | Feature-gated behind `mdns` — biomeOS for production discovery |
| Capability symlink | `storage.sock` → `nestgate.sock` auto-managed with guard pattern |

### Coverage

```
Current:  ~81.7% line coverage (llvm-cov, Apr 12 2026)
          (evolution: 68.4% → 71.4% → 74.3% → 77.1% → 80% → 81.4% → 81.7%)
Target:   90% line coverage
Gap:      ~8.3 percentage points
Path:     ZFS (needs real ZFS), installer (platform), cloud backends, binary entrypoints
Session 43: pool_ops 59→99%, trait_impl 62→99%, tier 64→86%, metadata_backend +edge tests, registry +concurrent tests
```

### Dependency Purity

```
Production:    Pure Rust; platform via rustix + /proc parsing
Crypto:        Delegated to security capability provider via IPC; local JWT uses RustCrypto (hmac, sha2)
TLS:           ELIMINATED from dep tree — installer uses system curl; security capability provider supplies ecosystem TLS
HTTP client:   Pure Rust (tokio TcpStream bootstrap; reqwest/rustls/ring all removed)
No direct libc: rustix replaces uzers
Hostname:      rustix::system::uname (gethostname eliminated)
Tokio:         Minimal features (9 specific, not "full")
Discovery:     Env vars + capability IPC (mDNS feature-gated, not default)
sysinfo:       Optional, non-Linux only
```

---

## Architecture

High-level layout; full member list is `[workspace].members` in root `Cargo.toml` (**23** packages). MCP is not a workspace member (delegated to biomeOS `capability.call`).

```
nestGate/ (23 workspace members: 20 code/crates + tools + fuzz + root)
├── nestgate-types / nestgate-platform / nestgate-env-process-shim  Foundation
├── nestgate-config / nestgate-storage / nestgate-rpc / nestgate-discovery
├── nestgate-security / nestgate-observe / nestgate-cache
├── nestgate-core       Traits, network, services, adapters (re-exports; primal_self_knowledge from nestgate-discovery)
├── nestgate-canonical  Canonical modernization
├── nestgate-api        REST + JSON-RPC API server
├── nestgate-bin        CLI binary (UniBin)
├── nestgate-zfs        ZFS integration (adaptive)
├── nestgate-installer  Platform installer (real GitHub releases download)
├── nestgate-middleware Middleware stack
├── nestgate-nas        NAS integration
├── nestgate-fsmonitor  Filesystem monitoring
├── nestgate-performance Performance monitoring
├── tools/unwrap-migrator  Helper CLI
└── fuzz/               Fuzz targets
Deprecated/shed (removed from workspace): nestgate-network, nestgate-automation, nestgate-mcp
```

---

## Compliance (wateringHole)

| Standard | Status |
|----------|--------|
| UniBin | PASS |
| ecoBin | PASS (pure Rust app code; ring/rustls/reqwest eliminated; sysinfo optional non-Linux only) |
| JSON-RPC 2.0 | PASS — Wire Standard Level 3 (Composable) |
| tarpc | PASS (feature-gated, version aligned) |
| Semantic naming | PASS (storage.*, data.*, nat.*, beacon.*, health.*, capabilities.*) |
| File size (<1000 production) | PASS (max ~463 lines after Session 43 refactors) |
| Sovereignty | PASS (capability-based discovery, storage.sock symlink, zero hardcoded primals) |
| mDNS Discovery | Feature-gated (`mdns`); production via biomeOS |
| Crypto delegation | PASS — SecurityProviderClient |
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
**Latest**: April 16, 2026 (Session 43v)
