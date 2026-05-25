# NestGate - Sovereign Storage & Permanence Primal

**Version**: 4.7.0-dev (internal iteration; workspace Cargo.toml `0.1.0`, binary `2.1.0`)  

> **Versioning scheme**: `4.7.0-dev` is the internal iteration version tracking
> 63+ development sessions. The workspace `Cargo.toml` uses `0.1.0` (pre-1.0 API
> surface). The `nestgate-bin` crate uses `2.1.0` for the CLI binary. plasmidBin
> `manifest.toml` tracks `0.1.0` (workspace root). These will unify on the first
> tagged public release.  

**Verification (as of 2026-05-24, Session 72)**  
- **Build**: `cargo check --workspace --all-features --all-targets` — PASS  
- **Clippy**: `cargo clippy --workspace -- -D warnings` — PASS (zero warnings)  
- **Tests**: 682 RPC lib tests, 12,399+ full workspace — 0 failures  
- **Format**: `cargo fmt --check` — PASS  
- **Docs**: `cargo doc --workspace --no-deps` — PASS  
- **Supply chain**: `cargo deny check` — advisories ok, bans ok, licenses ok, sources ok

**Metrics** (re-measure as needed; see [STATUS.md](./STATUS.md))  
- **Tests (last recorded)**: 682 RPC / 12,399+ full workspace, 0 failures
- **Coverage**: 84.12%+ line (`cargo llvm-cov --workspace --lib --summary-only`; wateringHole 80% met; 90% target pending)

**Technical debt (honest)**  
- **Open debt markers**: zero `TODO`/`FIXME`/`HACK`/`XXX` in production `.rs`  
- **Hardcoding**: `DEFAULT_SERVICE_NAME` constant used everywhere; zero hardcoded primal names in production  
- **Deprecated APIs**: 0 `#[deprecated]` markers (114 premature deprecations cleaned Session 43w; dead code removed)  
- **External deps**: Zero unused workspace deps (3 removed Session 61: `toml`, `async-stream`, `sha2`; `fastrand` → dev-dep); zero C-FFI `-sys` crates in production; `config` (crates.io) and `urlencoding` removed Session 43z  
- **Unsafe**: `#![forbid(unsafe_code)]` on ALL crate roots (zero exceptions); nestgate-zfs uses unconditional forbid (formerly `cfg`-gated outside tests)  
- **TLS/crypto**: `ring`/`reqwest` eliminated — `ureq` + vendored `rustls-rustcrypto` (pure Rust, `rustls-webpki` 0.103.12); installer uses system `curl`
- **Vendored crates** (`vendor/`): `rustls-rustcrypto` and `rustls-webpki` are vendored via `[patch.crates-io]` in the workspace `Cargo.toml`. Rationale: upstream `rustls-rustcrypto` optionally depends on `ring`; vendoring lets us build with pure-Rust crypto only, eliminating `ring` from the lockfile entirely. `rustls-webpki` is pinned at 0.103.12+ for RUSTSEC-2023-0071 mitigation. Both are consistent with `deny.toml` banning `ring`, `openssl`, `aws-lc-sys`  
- **sysinfo**: Optional — Linux uses pure-Rust `/proc` parsing; `sysinfo` only on non-Linux  
- **File size**: All `.rs` files under 800 lines (largest files refactored Sessions 43–43p, 58)  
- **`as` casts**: Dangerous narrowing casts evolved to `try_from`/`saturating`/`div_ceil`; benign widening casts remain  
- **Dead code**: zero unwired modules, zero `if false` stubs, zero `#[allow(dead_code)]` in production  
- **BTSP Phase 2**: server-side handshake wired into both UDS listeners (`is_btsp_required()` gate); JSON-line + length-prefixed dual framing; 6-tier security socket discovery; security provider wire contract aligned (`family_seed`, `session_token`, `btsp.session.verify` params); mode-aware error frames; `SECURITY_FAMILY_SEED` canonical env var (Session 45c)  
- **BTSP Phase 3**: `btsp.negotiate` server-side encrypted channel (ChaCha20-Poly1305 AEAD, HKDF-SHA256 key derivation, length-prefixed framing); wired into both UDS and isomorphic IPC listeners; transport hardened (decrypt/read errors propagate as Err, not silent Ok)
- **JWT NUCLEUS bypass**: BTSP composition auto-detected via `is_btsp_required()` — skips `NESTGATE_JWT_SECRET` validation when FAMILY_ID signals a NUCLEUS stack
- **`is_btsp_required` unified**: client delegates to canonical server version (eliminates env-var and `"standalone"` divergence)  
- **Mocks**: zero in production — `NoopStorage` is intentional null-object backend; all test doubles behind `#[cfg(test)]`; `ZfsBackendType::Mock` removed (dead code)  
- **Primal sovereignty**: zero hardcoded other-primal names in production; capability-based socket discovery (`security.sock`, `crypto.sock`); `DEFAULT_SERVICE_NAME` for self-references  
- **Streaming storage**: `storage.store_stream` / `retrieve_stream` chunked protocol for large tensors (neuralSpring/wetSpring)  
- **TCP alongside UDS**: `--port` / `NESTGATE_JSONRPC_TCP` activates TCP JSON-RPC listener (UniBin compliance)  
- **Cross-check tests**: `capability_registry.toml` ↔ dispatch invariant tests  
- **Lint mega-list narrowing**: Crate-level lint suppressions narrowed: nestgate-core 22→16, nestgate-zfs 24→17, nestgate-api 14→12, nestgate-installer 12→2/4, nestgate-bin 6→4 (real code fixes, not just moved)
- **Stale features removed**: `btsp` (nestgate-rpc), `cli` (nestgate-installer), `zfs`/`advanced`/`ai`/`performance` (nestgate-zfs — declared but never cfg-gated)
- **Commented-out code removed**: pool.rs HTTP block, operations.rs S3 stub, production_capability_bridge.rs K8s/Consul futures
- **Visibility narrowed**: `pub mod protocol` → `pub(crate)` in nestgate-rpc
- **Migration commentary cleaned**: nestgate-api config.rs — 120 lines of duplicated alias banners collapsed
- **BEARDOG refs evolved**: `SECURITY_SOCKET` added to discovery chain; doc/comment references updated to capability-agnostic language
- **Doc drift fixed**: STATUS method counts corrected (HTTP 22→23, semantic 42→43); JSON-RPC server info log dynamically derives method count via `module.method_names()`
- **Workspace dep consistency**: `crossbeam` centralized from local pin to `[workspace.dependencies]`
- **Wire Standard L3 on all surfaces**: `protocol` + `transport` fields added to ALL four `capabilities.list` implementations (UDS, HTTP, semantic router, isomorphic adapter); transport: `["uds", "tcp", "http"]`
- **`consumed_capabilities` aligned**: `"discovery"` → `"discovery_mesh"` in code to match `capability_registry.toml`; stale `CAPABILITY_MAPPINGS.md` consumed entries corrected
- **Discovery tiers documented**: Tier 3 (UDS convention), Tier 4 (manifest), Tier 5 (TCP probing) natively; Tiers 1-2 via orchestration
- **MethodGate JH-0**: Pre-dispatch authorization gate — Public/Protected method classification, `NESTGATE_AUTH_MODE` enforcement, `auth.check`/`auth.mode`/`auth.peer_info` introspection (Session 59)
- **content.* transport parity**: All 8 content-addressed methods routed through all transport paths — UDS dispatch, SemanticRouter, isomorphic IPC, HTTP API (Session 60)
- **lifecycle.status**: Public primal status probe on all transport paths, BTSP-exempt (Session 60)
- **Dep hygiene**: 3 unused deps removed from nestgate-api, `"biomeos"` socket-dir literal replaced with canonical `ecosystem_path_segment()` (Session 61)  
- **`primal.announce`**: JSON-RPC self-registration with biomeOS Neural API on startup — `capabilities`, `methods`, `signal_tiers`, `cost_hints`, `latency_estimates` (Session 70, Wave 43)
- **`--socket PATH` CLI flag**: Uniform launcher convergence — sets `NESTGATE_SOCKET` env (Session 71, Wave 47)
- **`health.liveness` normalized**: `{"status":"alive","primal":"nestgate"}` across all 5 transport surfaces (Session 71, Wave 47)
- **`btsp.capabilities`**: New method wired on all transport paths (Session 69)
- **Refactored `unix_adapter_handlers`**: 790L split into handlers (440L) + `storage_handlers.rs` (369L) (Session 72)
- **`primal_sovereignty` honesty**: `execute_capability_request` returns `not_implemented` error instead of fake success (Session 72)  
**Last Updated**: May 24, 2026

---

## Quick Start

### Production (plasmidBin — post-primordial)

All NUCLEUS primal binaries come from `plasmidBin`. No `target/release/`,
`which`, or `cargo install` paths in production deployment.

```bash
# Fetch pre-built binary
curl -sSL https://raw.githubusercontent.com/ecoPrimals/plasmidBin/main/fetch.sh | bash

# Binary lands in $XDG_DATA_HOME/ecoPrimals/plasmidBin/primals/{triple}/nestgate
# Or use primalSpring's composition launcher:
COMPOSITION_NAME=nest primalSpring/tools/composition_nucleus.sh start
```

### Local Development

```bash
# Build from source
cargo build --release

# Configure
export NESTGATE_JWT_SECRET=$(openssl rand -base64 48)

# Run (socket-only by default — ecoBin compliant)
./target/release/nestgate daemon

# Or with explicit socket path and HTTP:
./target/release/nestgate daemon --socket /tmp/nestgate.sock --enable-http

# Verify (HTTP mode)
curl http://localhost:8085/health
```

---

## Architecture

```
nestgate/ (22 workspace packages: 20 code/crates + fuzz + root)
│  default-members: root + nestgate-bin (cross-arch binary production)
│
│  Foundation Layer (zero internal deps, compiles first)
├── nestgate-types       Error types, result aliases, unified enums
├── nestgate-platform    env_process, linux_proc, OS abstractions (rustix)
├── nestgate-env-process-shim  Safe env mutation (tests; isolates set_var / remove_var)
│
│  Domain Layer (depends on types/platform)
├── nestgate-config      Config, constants, defaults, canonical modernization
├── nestgate-storage     Universal + temporal storage abstractions
├── nestgate-rpc         JSON-RPC + tarpc IPC layer (isomorphic UDS/TCP, storage.sock symlink)
├── nestgate-discovery   Capability-based peer discovery (env + capability IPC; runtime socket resolution)
├── nestgate-security    Crypto delegation (security capability provider), JWT, certs, zero-cost auth
├── nestgate-observe     Observability, diagnostics, event system
├── nestgate-cache       Multi-tier cache, UUID cache, cache math
│
│  Integration Layer
├── nestgate-core        Traits, network, services, adapters (re-exports primal_self_knowledge)
├── nestgate-canonical   Canonical modernization patterns
│
│  Application Layer
├── nestgate-api         REST + JSON-RPC API server
├── nestgate-bin         CLI binary (UniBin)
├── nestgate-zfs         ZFS integration (adaptive)
├── nestgate-installer   Platform installer (system curl, ecoBin compliant)
├── nestgate-middleware  Middleware stack
├── nestgate-nas         NAS integration
├── nestgate-fsmonitor   Filesystem monitoring
└── nestgate-performance Performance monitoring
```
Deprecated/shed (removed from workspace): `nestgate-network`, `nestgate-automation`, `nestgate-mcp`, `tools/unwrap-migrator`.

The core was decomposed across two phases from a 295K-line monolith (488s check)
into 13 focused crates that compile in parallel. `nestgate-core` re-exports all
extracted modules for zero downstream breakage (including `primal_self_knowledge` from `nestgate-discovery`). Core is now ~52K lines with 24
core-only modules and 44 dependencies (down from 51).

### Key Design Patterns

**Isomorphic IPC** — Same binary auto-adapts transport:
1. Try Unix domain socket (optimal)
2. Detect platform constraints
3. Fall back to TCP if needed
4. Always functional, zero configuration

**Adaptive Backend (Try-Optimize-Fallback)** — Platform-optimized paths with universal fallbacks. Applied to storage detection, service management, filesystem detection, ZFS backend, IPC transport.

**Primal Self-Knowledge** — Runtime capability discovery, zero hardcoding. Capabilities are discovered at runtime via environment variables and capability IPC.

**Capability-Based Discovery** — NestGate discovers other primals by capability (e.g., "crypto", "security"), not by hardcoded names or ports. Any primal providing a capability works.

---

## Current State

See [STATUS.md](./STATUS.md) for measured metrics. Verified as of 2026-05-24 (Session 72).

| Area | Status |
|------|--------|
| Build | `cargo check --workspace --all-features --all-targets` — PASS |
| Clippy | `cargo clippy --workspace --all-targets --all-features -- -D warnings` — PASS (zero warnings) |
| Format | `cargo fmt --all --check` — PASS |
| Tests | 682 RPC lib tests, 12,399+ full workspace — 0 failures |
| Coverage | 84.12%+ line (llvm-cov) — wateringHole 80% met; 90% target pending |
| Docs | `cargo doc --workspace --no-deps` — zero warnings |
| Deprecated | 0 `#[deprecated]` markers (114 premature deprecations cleaned Session 43w) |
| unwrap/expect | Zero in production library code; tests may use |
| Unsafe | `#![forbid(unsafe_code)]` on ALL crate roots |
| TLS/crypto | `ureq` + `rustls-rustcrypto` (pure Rust); zero C-FFI `-sys` in production |
| File size | All `.rs` files under 800 LOC (wateringHole limit 1000) |
| Env-var isolation | `EnvSource` / `MapEnv` primary; `#[serial]` scoped to ZFS command stub tests only |

### Compliance (wateringHole)

| Standard | Status |
|----------|--------|
| UniBin | Pass — single `nestgate` binary |
| ecoBin | Pass — pure Rust application code, socket-only default, zero C crypto deps (ring/rustls/reqwest eliminated) |
| JSON-RPC 2.0 | Pass — Wire Standard L3 (Composable): `{primal, version, capabilities}` envelope, `provided_capabilities`, `consumed_capabilities` |
| tarpc | Pass — wired into daemon (feature-gated); `StorageBackend` trait injection via `nestgate-core` |
| Semantic naming | Pass — `health.*`, `storage.*`, `content.*`, `session.*`, `nat.*`, `beacon.*`, `capabilities.*`, `metadata.*`, `discovery.*`, `crypto.*`, `zfs.*`, `bonding.*`, `model.*`, `templates.*`, `audit.*`, `identity.*`, `lifecycle.*`, `auth.*`, `btsp.*` |
| sysinfo evolution | Complete — Linux `/proc` primary, sysinfo optional non-Linux only |
| Coverage (80%+) | Pass — 84.12%+ line (wateringHole 80% met; 90% target pending) |
| File size (<1000 production) | Pass — all under 800 LOC (4 largest files refactored Sessions 43–43p) |
| BTSP Phase 1 | Pass — `BIOMEOS_INSECURE` guard, family-scoped socket naming (`nestgate-{fid}.sock`) |
| BTSP Phase 2 | Pass — server-side handshake wired into UDS accept (`btsp_server_handshake`); crypto delegated to BearDog |
| Sovereignty | Pass — capability-based discovery, zero hardcoded primals, family-scoped capability symlinks |
| Discovery | Env vars + capability IPC (runtime socket resolution — mDNS removed) |
| Crypto delegation | Pass — capability-based `SecurityProviderClient` |

### Platform Support

| Platform | Status | IPC | Build | Tests |
|----------|--------|-----|-------|-------|
| Linux | Full | Unix | Yes | Yes |
| FreeBSD | Full | Unix | Yes | Yes |
| macOS | Full | Unix | Yes | Yes |
| Windows WSL2 | Full | TCP | Yes | Yes |
| illumos | Full | Unix | Yes | Yes |
| Android | Full | TCP | Yes | Yes |

---

## Development

### Build & Test

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run all tests
cargo test --workspace

# Linting (CI-style: deny warnings; matches STATUS.md)
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Format
cargo fmt --all

# Code coverage
cargo llvm-cov --workspace --summary-only

# Documentation
cargo doc --no-deps --workspace
```

### Root package feature flags (`nestgate` workspace package)

The repository root package is mainly for integration tests. Its `[features]` are:

- **`dev-stubs`** — enables dev/stub code in `nestgate-core` and `nestgate-zfs` for tests.

Other workspace crates define their own features (for example `sysinfo`, per-crate `dev-stubs`). Check each crate’s `Cargo.toml` for the authoritative list.

### Key Technologies

- **Language**: Rust (stable toolchain)
- **Async Runtime**: Tokio
- **HTTP**: Axum
- **Serialization**: Serde, serde_json
- **Concurrency**: DashMap, tokio::sync, parking_lot, std::sync::LazyLock, pin-project
- **Security**: Delegated to security capability provider via IPC; local JWT via RustCrypto (hmac, sha2)
- **IPC**: Unix sockets + TCP fallback (JSON-RPC 2.0, storage.sock capability symlink)
- **CLI**: Clap 4 (derive mode)
- **Discovery**: Environment variables + capability IPC (capability-based)

### Configuration

**Priority order**:
1. Environment variables (highest)
2. `$XDG_CONFIG_HOME/nestgate/config.toml`
3. `$HOME/.config/nestgate/config.toml`
4. `/etc/nestgate/config.toml`
5. Built-in defaults (lowest)

**Common variables**:
```bash
NESTGATE_API_PORT=8085       # HTTP port (default: 8080)
NESTGATE_BIND=0.0.0.0        # Bind address (default: 127.0.0.1)
NESTGATE_JWT_SECRET=...       # JWT secret (required)
RUST_LOG=info                 # Logging level
```

---

## Documentation

- [STATUS.md](./STATUS.md) — Current measured metrics (ground truth)
- [QUICK_REFERENCE.md](./QUICK_REFERENCE.md) — Essential commands & configuration
- [CONTRIBUTING.md](./CONTRIBUTING.md) — Development guidelines
- [CAPABILITY_MAPPINGS.md](./CAPABILITY_MAPPINGS.md) — Primal capability mappings
- [CHANGELOG.md](./CHANGELOG.md) — Version history
- [DOCUMENTATION_INDEX.md](./DOCUMENTATION_INDEX.md) — Full doc index
- [specs/](./specs/) — Protocol specifications
- [docs/](./docs/) — Architecture, API, guides

### Fossil Record

Session archives and historical docs preserved in `ecoPrimals/infra/wateringHole/fossilRecord/nestgate/`.

---

## What's Active

1. Push test coverage toward 90% target (currently 84.12%+)
2. Track vendored `rustls-rustcrypto` + `rustls-webpki` upstream for drop opportunity
3. Multi-filesystem substrate testing (ZFS, btrfs, xfs, ext4 on real hardware)
4. Cross-gate replication (multi-node data orchestration)
5. aarch64 musl cross-compile CI (config exists; pipeline not wired)

For details: See [STATUS.md](./STATUS.md).

---

## License

AGPL-3.0-or-later — see [LICENSE](LICENSE) for the full text.

All ecoPrimals software is licensed under the strictest copyleft.
Humans accessing this software through the ecosystem's security and entropy
capabilities are granted free use rights for personal, educational, and
non-commercial purposes.

---

**Created**: January 31, 2026  
**Latest**: May 2026 (Session 72)
