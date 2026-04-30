# NestGate Quick Start

**Version**: 4.7.0-dev  
**Last Updated**: April 30, 2026 (Session 50)

---

## Prerequisites

- **Rust**: stable toolchain (1.94+) via `rustup`
- **OS**: Linux, macOS, FreeBSD, Windows (WSL2), illumos, Android

**Optional**: ZFS (for tiered storage features)

**Current build status** (Session 50): 23 workspace members; 8,841 tests passing (lib), 60 ignored, 0 failures; 84.12%+ line coverage; clippy clean with `-D warnings` — details in [STATUS.md](./STATUS.md).

---

## Build

```bash
cargo build --release --workspace
```

---

## Configure

NestGate uses sensible defaults. Set environment variables to override:

```bash
export NESTGATE_JWT_SECRET=$(openssl rand -base64 48)  # Required for auth
export NESTGATE_API_PORT=8085                          # HTTP port (default: 8080)
export NESTGATE_BIND=127.0.0.1                         # Bind address
```

**Storage**: XDG-compliant (`$HOME/.local/share/nestgate` by default)

---

## Run

```bash
# Socket-only (default, ecoBin compliant)
./target/release/nestgate daemon

# With HTTP enabled
./target/release/nestgate daemon --enable-http

# Verify (HTTP mode)
curl http://localhost:8085/health
```

NestGate auto-selects the best IPC transport:
1. Unix domain socket (optimal on Linux/macOS/FreeBSD)
2. TCP fallback (Windows WSL2, Android)

---

## Verify

```bash
# Health check (HTTP)
curl http://localhost:8085/health

# Store an object
curl -X PUT http://localhost:8085/api/datasets/test/objects/hello \
  --data-binary "Hello NestGate"

# Retrieve
curl http://localhost:8085/api/datasets/test/objects/hello
```

---

## Configuration Priority

1. Environment variables (highest)
2. `$XDG_CONFIG_HOME/nestgate/config.toml`
3. `$HOME/.config/nestgate/config.toml`
4. `/etc/nestgate/config.toml`
5. Built-in defaults (lowest)

---

## Key Environment Variables

```bash
NESTGATE_API_PORT=8085          # HTTP port
NESTGATE_BIND=0.0.0.0           # Bind address
NESTGATE_JWT_SECRET=...          # JWT authentication secret
NESTGATE_STORAGE_PATH=...       # Override storage location
NESTGATE_IPC_MODE=auto           # auto, unix, tcp
RUST_LOG=info                    # Logging level
```

---

## Troubleshooting

**Port in use**: `lsof -i :8085` then change `NESTGATE_API_PORT`

**Socket permission**: Set `NESTGATE_SOCKET_DIR=$HOME/.nestgate/sockets`

**ZFS not found**: NestGate gracefully falls back to standard filesystem

---

## Next Steps

- [STATUS.md](./STATUS.md) -- Current measured metrics
- [QUICK_REFERENCE.md](./QUICK_REFERENCE.md) -- Essential commands
- [CONTRIBUTING.md](./CONTRIBUTING.md) -- Development guidelines
- [CAPABILITY_MAPPINGS.md](./CAPABILITY_MAPPINGS.md) -- Primal capabilities

---

**License**: AGPL-3.0-or-later  
**Last Updated**: April 30, 2026 (Session 50)
