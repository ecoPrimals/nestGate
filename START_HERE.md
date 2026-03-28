# NestGate - Start Here

## Current Status

```
Build:       22/22 crates compiling (0 errors)
Tests:       12,383 passing, 0 failures, 469 ignored
Coverage:    ~72% line (target: 90%)
Clippy:      ZERO production warnings (pedantic+nursery)
Platforms:   6+ (Linux, FreeBSD, macOS, WSL2, illumos, Android)
```

See [STATUS.md](./STATUS.md) for full measured metrics.

---

## Quick Start

### 1. Build

```bash
cargo build --release --workspace
```

### 2. Configure

```bash
export NESTGATE_JWT_SECRET=$(openssl rand -base64 48)  # Required
export NESTGATE_API_PORT=8085                          # Optional (default: 8080)
```

### 3. Run

```bash
# Socket-only mode (default, ecoBin compliant)
./target/release/nestgate daemon

# Or with HTTP API:
./target/release/nestgate daemon --enable-http --port 8085
```

### 4. Verify

```bash
# HTTP mode:
curl http://localhost:8085/health

# Socket mode: Use JSON-RPC over Unix socket
```

---

## What Is NestGate?

NestGate is a **storage and discovery primal** in the ecoPrimals ecosystem. It provides:

- **Universal storage** — Works on 6+ platforms out of the box
- **Capability-based discovery** — Discovers other primals at runtime by capability
- **Isomorphic IPC** — Auto-adapts Unix sockets or TCP based on platform
- **JSON-RPC 2.0 + tarpc** — Dual IPC with semantic method naming
- **MCP provider** — Exposes storage via Model Context Protocol
- **ZFS integration** — Adaptive backend, graceful fallback to standard filesystem

### NEST Atomic Composition

```
NEST Atomic = TOWER + nestgate + squirrel
            = (beardog + songbird) + nestgate + squirrel
            = Security + Network + Storage + AI
```

---

## Architecture

```
nestGate/ (22 crates)
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

### Key Patterns

- **Try-Detect-Adapt-Succeed** — Try optimal path, detect constraints, adapt, always succeed
- **Runtime discovery** — Capabilities are data discovered at runtime, not compile-time config
- **Environment-driven** — 4-tier fallback: env vars -> XDG -> home -> system defaults
- **Zero hardcoding** — No primal names or ports in production code

---

## Testing

```bash
# All tests
cargo test --workspace

# Coverage
cargo llvm-cov --workspace --summary-only --ignore-filename-regex 'tools/'

# Linting
cargo clippy --all-targets --all-features -- -D warnings

# Format check
cargo fmt --all -- --check
```

---

## Configuration

NestGate auto-discovers everything. Override with environment variables:

```bash
NESTGATE_API_PORT=8085             # API port
NESTGATE_BIND=0.0.0.0              # Bind address
NESTGATE_JWT_SECRET=...             # JWT secret
NESTGATE_STORAGE_PATH=...          # Storage path
NESTGATE_ZFS_BINARY=...            # ZFS binary override
NESTGATE_CAPABILITY_CRYPTO_ENDPOINT=...  # Override crypto provider
SONGBIRD_IPC_PATH=...              # Override Songbird socket
RUST_LOG=info                       # Logging level
```

---

## Troubleshooting

**Build fails**: `cargo clean && cargo build --release`

**Tests fail**: `cargo test --workspace -- --nocapture` for verbose output

**Port in use**: Change `NESTGATE_API_PORT` or `lsof -i :8080`

**IPC connection fails**: Check `ls -la $XDG_RUNTIME_DIR/nestgate.*`

---

## Documentation

- [README.md](./README.md) — Project overview
- [STATUS.md](./STATUS.md) — Current measured metrics (ground truth)
- [QUICK_REFERENCE.md](./QUICK_REFERENCE.md) — Essential commands
- [CONTRIBUTING.md](./CONTRIBUTING.md) — Development guidelines
- [CAPABILITY_MAPPINGS.md](./CAPABILITY_MAPPINGS.md) — Primal capabilities
- [CHANGELOG.md](./CHANGELOG.md) — Version history
- [specs/](./specs/) — Protocol specifications
- [docs/](./docs/) — Architecture, guides, session archives

---

**Created**: January 31, 2026  
**Last Updated**: March 27, 2026
