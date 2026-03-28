# NestGate - Quick Reference

**Version**: 4.6.0-dev  
**Tests**: 12,383 passing, 0 failures, 469 ignored  
**Coverage**: ~72% line (llvm-cov, target: 90%)  
**Clippy**: ZERO production warnings (pedantic+nursery)  
**Last Updated**: March 28, 2026

---

## Quick Start

```bash
# Build
cargo build --release

# Run (socket-only by default — ecoBin compliant)
./target/release/nestgate daemon

# Or with HTTP enabled:
export NESTGATE_API_PORT=8085
./target/release/nestgate daemon --enable-http

# Verify (HTTP mode)
curl http://localhost:8085/health
```

---

## Essential Commands

### Build

```bash
cargo build                         # Development
cargo build --release               # Release
cargo build --workspace             # All crates
cargo build --package nestgate-core # Specific crate
```

### Test

```bash
cargo test --workspace                 # All tests
cargo test --package nestgate-core     # Specific crate
cargo test test_name                   # Single test
cargo test -- --nocapture              # With output
```

### Quality

```bash
cargo clippy --all-targets --all-features -- -D warnings  # Lint
cargo fmt --all -- --check                                 # Format check
cargo fmt --all                                            # Auto-format
cargo doc --no-deps --workspace                            # Docs
```

### Coverage

```bash
cargo llvm-cov --workspace --summary-only --ignore-filename-regex 'tools/'
```

### Run

```bash
cargo run -- daemon                               # Dev mode
./target/release/nestgate daemon                  # Release
NESTGATE_API_PORT=9000 ./target/release/nestgate daemon --enable-http
```

---

## Configuration

### Environment Variables

**API Server**:
```bash
export NESTGATE_API_PORT=8085         # Port (default: 8080)
export NESTGATE_BIND=0.0.0.0          # Bind address (default: 127.0.0.1)
export NESTGATE_JWT_SECRET=...         # JWT secret (required)
```

**Storage**:
```bash
export NESTGATE_STORAGE_PATH=/var/lib/nestgate
export NESTGATE_CACHE_SIZE=1073741824   # 1GB
```

**IPC** (auto-selects transport, but overridable):
```bash
export NESTGATE_IPC_MODE=auto          # auto, unix, tcp
export NESTGATE_SOCKET_PATH=$XDG_RUNTIME_DIR/nestgate.sock
```

**ZFS** (optional):
```bash
export NESTGATE_ZFS_BINARY=/usr/sbin/zfs
export NESTGATE_ZPOOL_BINARY=/usr/sbin/zpool
```

**Discovery**:
```bash
export NESTGATE_DISCOVERY_ENABLED=true
export NESTGATE_CAPABILITY_CRYPTO_ENDPOINT=...    # Override crypto provider
export NESTGATE_CAPABILITY_SECURITY_ENDPOINT=...  # Override security provider
export NESTGATE_SECURITY_PROVIDER=...             # BearDog override
export SONGBIRD_IPC_PATH=...                      # Songbird socket override
```

**Logging**:
```bash
export RUST_LOG=info                   # error, warn, info, debug, trace
export RUST_BACKTRACE=1                # Enable backtrace
```

### Configuration Files (Priority Order)

1. Environment variables (highest)
2. `$XDG_CONFIG_HOME/nestgate/config.toml`
3. `$HOME/.config/nestgate/config.toml`
4. `/etc/nestgate/config.toml`
5. Built-in defaults (lowest)

---

## Testing

### Categories

```bash
cargo test --lib --workspace          # Unit tests
cargo test --test '*'                 # Integration tests
cargo bench                           # Benchmarks
```

### Coverage

```bash
cargo llvm-cov --workspace --summary-only --ignore-filename-regex 'tools/'
cargo llvm-cov --workspace --html --ignore-filename-regex 'tools/'  # HTML report
```

---

## Deployment

### Production Build

```bash
# Linux (static linking)
cargo build --release --target x86_64-unknown-linux-musl
cargo build --release --target aarch64-unknown-linux-musl

# macOS
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
```

### Systemd Service

```ini
# /etc/systemd/system/nestgate.service
[Unit]
Description=NestGate Storage & Discovery Primal
After=network.target

[Service]
Type=simple
User=nestgate
ExecStart=/usr/local/bin/nestgate daemon
Restart=on-failure
RestartSec=5s

[Install]
WantedBy=multi-user.target
```

---

## Debugging

```bash
RUST_LOG=debug ./target/release/nestgate daemon       # Verbose
RUST_LOG=trace ./target/release/nestgate daemon       # Very verbose
RUST_LOG=nestgate_core=debug ./target/release/nestgate daemon  # Module-specific
RUST_BACKTRACE=1 ./target/release/nestgate daemon     # With backtrace
```

### IPC Endpoint Check

```bash
ls -la $XDG_RUNTIME_DIR/nestgate.*   # Should show .sock or .tcp
```

---

## Troubleshooting

**Port in use**: `lsof -i :8080` then `kill <PID>` or change `NESTGATE_API_PORT`.

**IPC connection fails**: Check `$XDG_RUNTIME_DIR/nestgate.*` for socket/tcp files.

**Tests failing**: `cargo clean && cargo test --workspace` for clean rebuild.

**ZFS not found**: Set `NESTGATE_ZFS_BINARY` or ensure `zfs` is in PATH. NestGate gracefully falls back to standard filesystem.

---

## Documentation

- [README.md](./README.md) — Project overview
- [STATUS.md](./STATUS.md) — Current measured metrics
- [CONTRIBUTING.md](./CONTRIBUTING.md) — Development guidelines
- [CAPABILITY_MAPPINGS.md](./CAPABILITY_MAPPINGS.md) — Primal capabilities
- [CHANGELOG.md](./CHANGELOG.md) — Version history
- [specs/](./specs/) — Protocol specifications

---

**Last Updated**: March 27, 2026
