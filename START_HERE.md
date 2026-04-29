# NestGate - Start Here

## Current Status

```
Build:       PASS — cargo check --workspace --all-features --all-targets (as of Session 49)
Tests:       PASS — cargo test --workspace --lib, 0 failures (8,841 passing, 60 ignored — STATUS.md)
Coverage:    84.12%+ line (llvm-cov) — wateringHole 80% met; 90% target pending
Clippy:      PASS — cargo clippy --workspace --all-targets -- -D warnings (as of Session 49)
Docs:        cargo doc --workspace --no-deps — clean in routine runs
Unsafe:      #![forbid(unsafe_code)] on ALL crate roots (zero exceptions)
Crypto:      Delegated to security capability provider via IPC; installer uses system curl (no in-tree TLS stack for downloads)
sysinfo:     Optional — Linux uses pure-Rust /proc; sysinfo on non-Linux only
Serial:      #[serial]: scoped to ZFS stub tests; temp_env elsewhere
Debt markers: none in production library sources (wateringHole; see STATUS.md)
Binary:      ~4.7MB musl static
Platforms:   6+ (Linux, FreeBSD, macOS, WSL2, illumos, Android)
```

See [STATUS.md](./STATUS.md) for full measured metrics.

---

## Quick Start

### 1. Build

```bash
cargo build --release --workspace
```

#### Cross-compile for aarch64 (static musl)

```bash
rustup target add aarch64-unknown-linux-musl
sudo apt install gcc-aarch64-linux-gnu   # Debian/Ubuntu
cargo build-arm64                         # alias from .cargo/config.toml
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
- **Capability-based discovery** — Discovers other primals at runtime by capability (storage.sock symlink)
- **Isomorphic IPC** — Auto-adapts Unix sockets or TCP based on platform
- **JSON-RPC 2.0 + tarpc** — Dual IPC with semantic method naming (storage.*, session.*, nat.*, beacon.*, zfs.*, bonding.*)
- **ZFS integration** — Adaptive backend, graceful fallback to standard filesystem

### NEST Atomic Composition

```
NEST Atomic = TOWER + nestgate + squirrel
            = (security provider + network provider) + nestgate + squirrel
            = Security + Network + Storage + AI
```

---

## Architecture

```
nestgate/ (23 workspace members — see README Architecture)
├── nestgate-types … nestgate-platform … (foundation)
├── nestgate-config, nestgate-storage, nestgate-rpc, nestgate-discovery, …
├── nestgate-core       Traits, network, services, adapters
├── nestgate-api        REST + JSON-RPC API server
├── nestgate-bin        CLI binary (UniBin)
├── nestgate-zfs        ZFS integration (adaptive)
├── nestgate-installer  Platform installer (system curl, ecoBin compliant)
├── nestgate-canonical  Canonical types
├── nestgate-middleware Middleware stack
├── nestgate-nas        NAS integration
├── nestgate-fsmonitor  Filesystem monitoring
├── nestgate-performance Performance monitoring
└── tools/unwrap-migrator, fuzz (workspace)
Note: nestgate-network, nestgate-automation, nestgate-mcp are deprecated/shed (removed from workspace).
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
NESTGATE_ORCHESTRATION_IPC_PATH=...  # Optional capability/orchestration IPC socket override
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
**Last Updated**: April 29, 2026 (Session 49)
