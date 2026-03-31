# NestGate - Sovereign Storage & Permanence Primal

**Version**: 4.7.0-dev  
**Build**: All workspace members — `cargo check --workspace --all-features --all-targets` (0 errors)  
**Tests**: 8,376 lib tests passing, 0 failures (workspace `--lib`)  
**Coverage**: 80.95% line (workspace `--lib`, llvm-cov) — last full measurement; re-run to refresh  
**Clippy**: ZERO warnings — `cargo clippy --workspace --all-features --all-targets`  
**Docs**: Zero warnings (`cargo doc --workspace --no-deps`)  
**Production TODO/FIXME**: Zero  
**Unsafe**: None in application crates; `#![forbid(unsafe_code)]` on all crate roots except `nestgate-env-process-shim`  
**TLS/crypto**: Delegated to bearDog via IPC; installer uses system `curl` (zero C crypto deps)  
**sysinfo**: Optional — Linux uses pure-Rust `/proc` parsing; `sysinfo` only on non-Linux  
**File size**: All production `.rs` files under 1,000 lines (max 879)  
**Last Updated**: March 31, 2026

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
nestgate/ (24 workspace members: 22 code/crates + tools/unwrap-migrator + fuzz)
│
│  Foundation Layer (zero internal deps, compiles first)
├── nestgate-types       Error types, result aliases, unified enums
├── nestgate-platform    env_process, linux_proc, OS abstractions (rustix/uzers)
├── nestgate-env-process-shim  Safe env mutation (tests; isolates set_var / remove_var)
│
│  Domain Layer (depends on types/platform)
├── nestgate-config      Config, constants, defaults, canonical modernization
├── nestgate-storage     Universal + temporal storage abstractions
├── nestgate-rpc         JSON-RPC + tarpc IPC layer (isomorphic UDS/TCP, storage.sock symlink)
├── nestgate-discovery   Capability-based peer discovery (env + songBird IPC; mDNS behind feature gate)
├── nestgate-security    Crypto delegation (bearDog IPC), JWT, certs, zero-cost auth
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
├── nestgate-network     Network storage (admin router; HTTP shed to songBird)
├── nestgate-automation  Storage-specific automation (tiering, lifecycle)
├── nestgate-installer   Platform installer (system curl, ecoBin compliant)
├── nestgate-middleware  Middleware stack
├── nestgate-nas         NAS integration
├── nestgate-fsmonitor   Filesystem monitoring
└── nestgate-performance Performance monitoring
```

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

**Primal Self-Knowledge** — Runtime capability discovery, zero hardcoding. Capabilities are discovered at runtime via environment variables and songBird IPC.

**Capability-Based Discovery** — NestGate discovers other primals by capability (e.g., "crypto", "security"), not by hardcoded names or ports. Any primal providing a capability works.

---

## Current State

See [STATUS.md](./STATUS.md) for measured metrics.

| Area | Status |
|------|--------|
| Build | `cargo check --workspace --all-features --all-targets` — 0 errors |
| Clippy | ZERO warnings (`cargo clippy --workspace --all-features --all-targets`) |
| Format | Clean (`cargo fmt --check`) |
| Tests | 8,376 lib tests passing, 0 failures |
| Coverage | 80.95% line (llvm-cov) — wateringHole 80% minimum met; not re-measured Mar 31 |
| Docs | Zero warnings (`cargo doc --workspace --no-deps`) |
| Production TODO/FIXME | Zero |
| Production unwrap/expect | Zero in library `src/` (`#[cfg(test)]` / integration tests may use unwrap; clippy warns workspace-wide) |
| Unsafe | Only `nestgate-env-process-shim` (env bridge); `#![forbid(unsafe_code)]` elsewhere |
| TLS/crypto | Delegated to bearDog IPC; installer uses system `curl` (zero C crypto deps; ring/rustls/reqwest eliminated) |
| sysinfo | Optional — Linux uses pure-Rust `/proc`; sysinfo on non-Linux only |
| File size (production < 1000) | All compliant (max 879 lines) |
| http_client_stub | Self-contained (no removed `discovery_mechanism` dependency) |
| Env-var race conditions | Fixed (temp-env + serial_test) |

### Compliance (wateringHole)

| Standard | Status |
|----------|--------|
| UniBin | Pass — single `nestgate` binary |
| ecoBin | Pass — pure Rust application code, socket-only default, zero C crypto deps (ring/rustls/reqwest eliminated) |
| JSON-RPC 2.0 | Pass |
| tarpc | Pass — wired into daemon (feature-gated) |
| Semantic naming | Pass — `health.*`, `storage.*`, `data.*`, `nat.*`, `beacon.*`, `capabilities.*` |
| sysinfo evolution | Complete — Linux `/proc` primary, sysinfo optional non-Linux only |
| Coverage (80%+) | Pass — 80.95% line (wateringHole minimum met) |
| File size (<1000 production) | Pass (max 879 lines) |
| Sovereignty | Pass — capability-based discovery, zero hardcoded primals, storage.sock symlink |
| Discovery | Env vars + songBird IPC (mDNS behind `mdns` feature gate — delegated to biomeOS/songBird) |
| Crypto delegation | Pass — bearDog IPC via capability-based `SecurityProviderClient` |

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

# Linting (must pass with zero warnings; matches STATUS.md ground truth)
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Format
cargo fmt --all

# Code coverage
cargo llvm-cov --workspace --summary-only --ignore-filename-regex 'tools/'

# Documentation
cargo doc --no-deps --workspace
```

### Key Technologies

- **Language**: Rust (stable toolchain)
- **Async Runtime**: Tokio
- **HTTP**: Axum
- **Serialization**: Serde, serde_json
- **Concurrency**: DashMap, std::sync::LazyLock, pin-project
- **Security**: Delegated to bearDog via IPC; local JWT via RustCrypto (hmac, sha2)
- **IPC**: Unix sockets + TCP fallback (JSON-RPC 2.0, storage.sock capability symlink)
- **CLI**: Clap 4 (derive mode)
- **Discovery**: Environment variables + songBird IPC (capability-based)

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

1. Push test coverage toward 90% target (currently 80.95% last measured)
2. Multi-filesystem substrate testing (ZFS, btrfs, xfs, ext4 on real hardware)
3. Cross-gate replication (multi-node data orchestration)
4. Profile and optimize `.clone()` hotspots in RPC layer with real benchmark data

For details: See [STATUS.md](./STATUS.md).

---

## License

AGPL-3.0-only — see [LICENSE](LICENSE) for the full text.

All ecoPrimals software is licensed under the strictest copyleft.
Humans accessing this software through beardog entropy systems are granted
free use rights for personal, educational, and non-commercial purposes.

---

**Created**: January 31, 2026  
**Latest**: March 31, 2026
