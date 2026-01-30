# 🚀 NestGate Quick Reference

**Fast commands for daily development** · v3.4.0 · A+++ 110/100 LEGENDARY

---

## 🏃 **Most Common Commands**

### **Build & Run**
```bash
# Build (release)
cargo build --release

# Run NestGate
./target/release/nestgate

# Run with environment
export NESTGATE_PORT=8080
export NESTGATE_DATA_DIR=$HOME/.local/share/nestgate
./target/release/nestgate
```

### **Development**
```bash
# Build (dev)
cargo build

# Watch mode
cargo watch -x 'build' -x 'test --lib'

# Run tests
cargo test --workspace

# Fast tests
cargo nextest run
```

---

## 🧪 **Testing**

```bash
# All tests
cargo test --workspace

# Specific module
cargo test --package nestgate-core --lib services::storage

# With output
cargo test test_name -- --nocapture

# Coverage
cargo tarpaulin --out Html
```

---

## 🔍 **Code Quality**

```bash
# Clippy (linting)
cargo clippy --all-targets

# Format check
cargo fmt --check

# Apply formatting
cargo fmt

# All checks
cargo build && cargo test && cargo clippy
```

---

## 🌐 **API Testing**

### **Health Check**
```bash
curl http://localhost:8080/health
```

### **Create Dataset**
```bash
curl -X POST http://localhost:8080/api/datasets \
  -H "Content-Type: application/json" \
  -d '{"name":"test","description":"Test dataset"}'
```

### **Store Object**
```bash
echo "Hello" | curl -X PUT \
  http://localhost:8080/api/datasets/test/objects/greeting \
  --data-binary @-
```

### **Retrieve Object**
```bash
curl http://localhost:8080/api/datasets/test/objects/greeting
```

### **List Datasets**
```bash
curl http://localhost:8080/api/datasets
```

---

## 🔧 **Environment Variables** (Common)

```bash
# Network
export NESTGATE_PORT=8080
export NESTGATE_HOST=127.0.0.1

# Storage (XDG-compliant)
export NESTGATE_DATA_DIR=$HOME/.local/share/nestgate
export NESTGATE_CACHE_DIR=$HOME/.cache/nestgate

# Discovery
export NESTGATE_DISCOVERY_ENABLED=true

# Logging
export RUST_LOG=info
export RUST_LOG=nestgate=debug  # Verbose
```

**Complete list**: See `docs/guides/ENVIRONMENT_VARIABLES.md`

---

## 📊 **Status Checks**

```bash
# Health
curl http://localhost:8080/health

# Statistics
curl http://localhost:8080/api/stats

# Metrics (Prometheus)
curl http://localhost:9090/metrics

# Discovered services
curl http://localhost:8080/api/services
```

---

## 🐳 **Docker**

```bash
# Build image
docker build -t nestgate:latest .

# Run container
docker run -d -p 8080:8080 \
  -e NESTGATE_ZFS_ENABLED=false \
  nestgate:latest

# Logs
docker logs -f nestgate

# Compose
docker-compose up -d
```

---

## 🔍 **Debugging**

```bash
# Enable debug logging
export RUST_LOG=nestgate=debug
./target/debug/nestgate

# Trace logging
export RUST_LOG=nestgate=trace

# Specific module
export RUST_LOG=nestgate::storage=debug

# Check configuration
curl http://localhost:8080/api/config
```

---

## 🛠️ **Development Shortcuts**

```bash
# Clean build
cargo clean && cargo build

# Update dependencies
cargo update

# Check for updates
cargo outdated

# Security audit
cargo audit

# Benchmark
cargo bench
```

---

## 📂 **File Locations**

### **Source Code**:
- Core library: `code/crates/nestgate-core/src/`
- Binary: `code/crates/nestgate-bin/src/`
- Tests: `tests/`
- Examples: `examples/`

### **Configuration**:
- XDG data: `~/.local/share/nestgate`
- XDG cache: `~/.cache/nestgate`
- XDG config: `~/.config/nestgate`
- Runtime: `/run/user/{UID}/nestgate`

### **Documentation**:
- Root: `QUICK_START.md`, `README.md`
- Guides: `docs/guides/`
- API: `docs/api/`
- Architecture: `docs/architecture/`

---

## 🔗 **Key Documents**

- **Quick Start**: `QUICK_START.md`
- **API Reference**: `docs/api/REST_API.md`
- **Architecture**: `docs/architecture/COMPONENT_INTERACTIONS.md`
- **Troubleshooting**: `docs/guides/TROUBLESHOOTING.md`
- **Contributing**: `CONTRIBUTING.md`
- **Environment Vars**: `docs/guides/ENVIRONMENT_VARIABLES.md`

---

## 🎯 **Common Workflows**

### **Add New Feature**:
```bash
git checkout -b feature/my-feature
# Make changes
cargo test
cargo clippy
git commit -m "feat: add my feature"
git push -u origin feature/my-feature
# Create PR on GitHub
```

### **Fix Bug**:
```bash
git checkout -b fix/bug-description
# Make fix
cargo test
git commit -m "fix: resolve bug"
git push
```

### **Update Docs**:
```bash
# Edit markdown files
git add docs/
git commit -m "docs: update documentation"
git push
```

---

## 🚨 **Emergency Commands**

### **Service Down**:
```bash
# Kill process
pkill nestgate

# Restart
./target/release/nestgate &

# Check status
curl http://localhost:8080/health
```

### **Reset Storage**:
```bash
# WARNING: Deletes all data!
rm -rf ~/.local/share/nestgate/*
./target/release/nestgate
```

### **Clear Cache**:
```bash
rm -rf ~/.cache/nestgate/*
```

---

## 📞 **Get Help**

| Issue | Resource |
|-------|----------|
| Quick questions | `docs/guides/TROUBLESHOOTING.md` |
| API usage | `docs/api/REST_API.md` |
| Configuration | `docs/guides/ENVIRONMENT_VARIABLES.md` |
| Architecture | `docs/architecture/COMPONENT_INTERACTIONS.md` |
| Bug reports | GitHub Issues |

---

**NestGate v3.4.0** · A+++ 110/100 LEGENDARY · Quick Reference 🦀

**For detailed guides**: See `docs/` directory
