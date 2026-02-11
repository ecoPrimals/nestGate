# NestGate - Universal Storage & Discovery Primal

**Version**: 4.1.0-dev  
**Build**: 13/13 crates compiling, 0 errors  
**Tests**: 12,155 passing, 0 failures, 431 ignored  
**Coverage**: 70.07% line (llvm-cov, target: 90%)  
**Clippy**: Clean under `-D warnings`  
**Last Updated**: February 11, 2026

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
nestGate/ (13 crates)
├── nestgate-core       Core: IPC, config, crypto, discovery
├── nestgate-api        REST + JSON-RPC API server
├── nestgate-bin        CLI binary (unibin)
├── nestgate-zfs        ZFS integration (adaptive)
├── nestgate-mcp        MCP provider
├── nestgate-network    Network storage
├── nestgate-automation Automation engine
├── nestgate-installer  Platform installer
├── nestgate-canonical  Canonical types
├── nestgate-middleware Middleware stack
├── nestgate-nas        NAS integration
├── nestgate-fsmonitor  Filesystem monitoring
└── nestgate-performance Performance monitoring
```

### Key Design Patterns

**Isomorphic IPC** — Same binary auto-adapts transport:
1. Try Unix domain socket (optimal)
2. Detect platform constraints
3. Fall back to TCP if needed
4. Always functional, zero configuration

**Adaptive Backend (Try-Optimize-Fallback)** — Platform-optimized paths with universal fallbacks. Applied to storage detection, service management, filesystem detection, ZFS backend, IPC transport.

**Primal Self-Knowledge** — Runtime capability discovery, zero hardcoding. Capabilities are discovered at runtime via environment variables, mDNS, or capability-based IPC.

**Capability-Based Discovery** — NestGate discovers other primals by capability (e.g., "crypto", "security"), not by hardcoded names or ports. Any primal providing a capability works.

---

## Current State

See [STATUS.md](./STATUS.md) for measured metrics.

| Area | Status |
|------|--------|
| Build | 13/13 crates, 0 errors |
| Clippy | Clean (`-D warnings`) |
| Format | Clean |
| Tests | 12,155 passing, 0 failures |
| Coverage | 70.07% line (excluding tools/) |
| Production unwrap/expect | Zero |
| File size limit (1000 lines) | All compliant |
| Env-var race conditions | Fixed (80+ tests with save/restore) |

### Compliance (wateringHole)

| Standard | Status |
|----------|--------|
| UniBin | Pass — single `nestgate` binary |
| ecoBin | Pass — pure Rust, socket-only default |
| JSON-RPC 2.0 | Pass |
| tarpc | Pass — wired into daemon (feature-gated) |
| Semantic naming | Partial — `storage.*`, `crypto.*` compliant |
| File size (<1000) | Pass |
| Sovereignty | Evolved — capability-based discovery |
| mDNS Discovery | Evolved — real mdns-sd with cache fallback |
| Crypto delegation | Evolved — capability-based, compiles clean |

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

# Linting (must pass with zero warnings)
cargo clippy --all-targets --all-features -- -D warnings

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
- **Serialization**: Serde, serde_json, serde_yaml_ng
- **Concurrency**: DashMap, std::sync::LazyLock
- **Security**: RustCrypto stack (AES-256-GCM, ed25519-dalek, hmac, argon2, sha2)
- **IPC**: Unix sockets + TCP fallback (JSON-RPC 2.0)
- **CLI**: Clap 4 (derive mode)
- **Discovery**: mdns-sd, capability-based IPC

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
- [docs/](./docs/) — Architecture, API, guides, session archives

### Session Archives (Fossil Record)

- `docs/sessions/feb_2026/` — Deep-debt evolution, model cache, upstream bugs
- `docs/sessions/jan_2026/` — Isomorphic IPC, platform evolution, genomeBin

---

## What's Active

1. Push test coverage toward 90% target (currently 70.07%)
2. Evolve remaining IMPLEMENTATION STUB sections to full implementations
3. Complete semantic method naming across all domains
4. Cross-gate replication (multi-node data orchestration)

For details: See `STATUS.md` and `docs/sessions/feb_2026/`.

---

## License

[Add your license here]

---

**Created**: January 31, 2026  
**Latest**: February 11, 2026
