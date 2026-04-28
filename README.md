# NestGate - Sovereign Storage & Permanence Primal

**Version**: 4.7.0-dev  

**Verification (as of 2026-04-28, Session 48)**  
- **Build**: `cargo check --workspace --all-features --all-targets` — PASS  
- **Clippy**: `cargo clippy --workspace --all-targets -- -D warnings` — PASS (zero warnings)  
- **Tests**: `cargo test --workspace --lib` — 8,840 passing, 0 failures, 60 ignored  
- **Format**: `cargo fmt --check` — PASS  
- **Docs**: `cargo doc --workspace --no-deps` — PASS  
- **Supply chain**: `cargo deny check` — advisories ok, bans ok, licenses ok, sources ok  

**Metrics** (re-measure as needed; see [STATUS.md](./STATUS.md))  
- **Tests (last recorded)**: 8,840 passing, 60 ignored, 0 failures  
- **Coverage**: 84.12%+ line (`cargo llvm-cov --workspace --lib --summary-only`; wateringHole 80% met; 90% target pending)

**Technical debt (honest)**  
- **Open debt markers**: zero `TODO`/`FIXME`/`HACK`/`XXX` in production `.rs`  
- **Hardcoding**: `DEFAULT_SERVICE_NAME` constant used everywhere; zero hardcoded primal names in production  
- **Deprecated APIs**: 0 `#[deprecated]` markers (114 premature deprecations cleaned Session 43w; dead code removed)  
- **External deps**: Zero unused workspace deps; zero C-FFI `-sys` crates in production; `config` (crates.io) and `urlencoding` removed Session 43z  
- **Unsafe**: `#![forbid(unsafe_code)]` on ALL crate roots (zero exceptions)  
- **TLS/crypto**: `ring`/`reqwest` eliminated — `ureq` + vendored `rustls-rustcrypto` (pure Rust, `rustls-webpki` 0.103.12); installer uses system `curl`  
- **sysinfo**: Optional — Linux uses pure-Rust `/proc` parsing; `sysinfo` only on non-Linux  
- **File size**: All `.rs` files under 800 lines (largest files refactored Sessions 43–43p)  
- **`as` casts**: Dangerous narrowing casts evolved to `try_from`/`saturating`/`div_ceil`; benign widening casts remain  
- **Dead code**: zero unwired modules, zero `if false` stubs, zero `#[allow(dead_code)]` in production  
- **BTSP Phase 2**: server-side handshake wired into both UDS listeners (`is_btsp_required()` gate); JSON-line + length-prefixed dual framing; 6-tier security socket discovery; security provider wire contract aligned (`family_seed`, `session_token`, `btsp.session.verify` params); mode-aware error frames; `SECURITY_FAMILY_SEED` canonical env var (Session 45c)  
- **Mocks**: zero in production — `NoopStorage` is intentional null-object backend; all test doubles behind `#[cfg(test)]`; `ZfsBackendType::Mock` removed (dead code)  
- **Primal sovereignty**: zero hardcoded other-primal names in production; capability-based socket discovery (`security.sock`, `crypto.sock`); `DEFAULT_SERVICE_NAME` for self-references  
- **Streaming storage**: `storage.store_stream` / `retrieve_stream` chunked protocol for large tensors (neuralSpring/wetSpring)  
- **TCP alongside UDS**: `--port` / `NESTGATE_JSONRPC_TCP` activates TCP JSON-RPC listener (UniBin compliance)  
- **Cross-check tests**: `capability_registry.toml` ↔ dispatch invariant tests  
**Last Updated**: April 28, 2026 (Session 48)

---

## Quick Start

```bash
# Build
cargo build --release

# Configure
export NESTGATE_JWT_SECRET=$(openssl rand -base64 48)

# Run (socket-only by default — ecoBin compliant)
./target/release/nestgate daemon

# Or with HTTP enabled:
export NESTGATE_API_PORT=8085
./target/release/nestgate daemon --enable-http

# Verify (HTTP mode)
curl http://localhost:8085/health
```

### NEST Atomic Deployment

```bash
# Single-host deployment (all primals coexist)
export NESTGATE_API_PORT=8085  # Avoids port conflicts
export NESTGATE_JWT_SECRET=$(openssl rand -base64 48)
./nestgate daemon &

# Other primals discover NestGate via capability-based IPC
```

---

## Architecture

```
nestgate/ (23 workspace members: 20 code/crates + tools/unwrap-migrator + fuzz + root)
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
├── nestgate-discovery   Capability-based peer discovery (env + capability IPC; mDNS behind feature gate)
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
Deprecated/shed (removed from workspace): `nestgate-network`, `nestgate-automation`, `nestgate-mcp`.

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

See [STATUS.md](./STATUS.md) for measured metrics. Verified as of 2026-04-28 (Session 48).

| Area | Status |
|------|--------|
| Build | `cargo check --workspace --all-features --all-targets` — PASS |
| Clippy | `cargo clippy --workspace --all-targets --all-features -- -D warnings` — PASS (zero warnings) |
| Format | `cargo fmt --all --check` — PASS |
| Tests | `cargo test --workspace --lib` — 8,840 passing, 0 failures, 60 ignored |
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
| JSON-RPC 2.0 | Pass — Wire Standard L3 (Composable): `{primal, version, methods}` envelope, `provided_capabilities`, `consumed_capabilities` |
| tarpc | Pass — wired into daemon (feature-gated); `StorageBackend` trait injection via `nestgate-core` |
| Semantic naming | Pass — `health.*`, `storage.*`, `session.*`, `nat.*`, `beacon.*`, `capabilities.*`, `metadata.*`, `discovery.*`, `crypto.*`, `zfs.*`, `bonding.*`, `model.*`, `templates.*`, `audit.*`, `identity.*` |
| sysinfo evolution | Complete — Linux `/proc` primary, sysinfo optional non-Linux only |
| Coverage (80%+) | Pass — 84.12%+ line (wateringHole 80% met; 90% target pending) |
| File size (<1000 production) | Pass — all under 800 LOC (4 largest files refactored Sessions 43–43p) |
| BTSP Phase 1 | Pass — `BIOMEOS_INSECURE` guard, family-scoped socket naming (`nestgate-{fid}.sock`) |
| BTSP Phase 2 | Pass — server-side handshake wired into UDS accept (`btsp_server_handshake`); crypto delegated to BearDog |
| Sovereignty | Pass — capability-based discovery, zero hardcoded primals, family-scoped capability symlinks |
| Discovery | Env vars + capability IPC (mDNS behind `mdns` feature gate — delegated to biomeOS) |
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
cargo llvm-cov --workspace --summary-only --ignore-filename-regex 'tools/'

# Documentation
cargo doc --no-deps --workspace
```

### Root package feature flags (`nestgate` workspace package)

The repository root package is mainly for integration tests. Its `[features]` are:

- **`dev-stubs`** — enables dev/stub code in `nestgate-core` and `nestgate-zfs` for tests.
- **`streaming-rpc`** — reserved for streaming RPC work (see root `Cargo.toml`).

Other workspace crates define their own features (for example `mdns`, `sysinfo`, per-crate `dev-stubs`). Check each crate’s `Cargo.toml` for the authoritative list.

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
**Latest**: April 2026 (Session 48)
