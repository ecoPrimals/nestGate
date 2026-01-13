# 🚀 START HERE - NestGate Quick Start Guide

**Welcome to NestGate!** This guide will get you up and running in minutes.

**Last Updated**: January 13, 2026  
**Status**: Production Ready (Grade A+ - 95/100)  
**Tests**: 1,235+ passing | **Coverage**: 69.7%

---

## 📋 Quick Navigation

**New Developer?** → Start with [Prerequisites](#prerequisites)  
**Returning Developer?** → Jump to [Development Workflow](#development-workflow)  
**Production Deployment?** → See [Deployment Guide](#production-deployment)  
**Need Help?** → Check [Troubleshooting](#troubleshooting)

---

## ✅ Prerequisites

### Required

1. **Rust 1.75 or later**
   ```bash
   # Install/update Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup update stable
   
   # Verify installation
   rustc --version  # Should be 1.75.0 or higher
   ```

2. **Git**
   ```bash
   git --version  # Should be 2.0 or higher
   ```

### Optional (but recommended)

3. **ZFS** (for native storage backend)
   ```bash
   # Ubuntu/Debian
   sudo apt-get install zfsutils-linux
   
   # Check installation
   zfs --version
   ```

4. **cargo-llvm-cov** (for coverage reports)
   ```bash
   cargo install cargo-llvm-cov
   ```

---

## 🏁 Quick Start (5 minutes)

### 1. Clone Repository
```bash
git clone https://github.com/ecoprimals/nestgate
cd nestgate
```

### 2. Build Project
```bash
# Development build (~30 seconds)
cargo build

# Or release build (~60 seconds)
cargo build --release
```

### 3. Run Tests
```bash
# Quick smoke test
cargo test --lib

# Full test suite (~2 minutes)
cargo test --workspace
```

### 4. Start Local Instance
```bash
# Set required environment variables
export NESTGATE_FAMILY_ID="local-dev"

# Start server
./start_local_dev.sh

# Or manually:
cargo run --bin nestgate
```

### 5. Verify Running
```bash
# Check health endpoint
curl http://localhost:8080/health

# Expected response:
# {"status": "healthy"}
```

**🎉 Success!** NestGate is now running locally.

---

## 💻 Development Workflow

### Daily Development

```bash
# 1. Pull latest changes
git pull origin main

# 2. Run tests
cargo test --workspace

# 3. Make your changes
# ... edit files ...

# 4. Format code
cargo fmt --all

# 5. Check lints
cargo clippy --workspace -- -D warnings

# 6. Run affected tests
cargo test --lib  # Or specific tests

# 7. Commit changes
git add .
git commit -m "feat: your feature description"
git push
```

### Essential Commands

```bash
# Build
cargo build                    # Debug build
cargo build --release          # Optimized build

# Test
cargo test                     # All tests
cargo test --lib              # Library tests only
cargo test integration_test   # Specific test

# Check
cargo check                    # Fast type check
cargo clippy                   # Lint check
cargo fmt --check              # Format check

# Documentation
cargo doc --open               # Generate & open docs

# Clean
cargo clean                    # Remove build artifacts
```

---

## 🔧 Configuration

### Environment Variables

Required:
```bash
export NESTGATE_FAMILY_ID="your-family-id"
```

Optional (with defaults):
```bash
# Ports
export NESTGATE_API_PORT=8080
export NESTGATE_METRICS_PORT=9090
export NESTGATE_HEALTH_PORT=8082

# Storage
export NESTGATE_STORAGE_PATH="/data/nestgate"

# Discovery (optional)
export NESTGATE_CAPABILITY_STORAGE_ENDPOINT="127.0.0.1:5000"
```

### Configuration File

Create `nestgate.toml`:
```toml
[service]
family_id = "nestgate-dev"
node_id = "node1"

[api]
port = 8080
bind = "127.0.0.1"

[storage]
backend = "filesystem"
path = "/tmp/nestgate-data"

[discovery]
enabled = true
```

Load configuration:
```bash
cargo run -- --config nestgate.toml
```

---

## 📚 Key Concepts

### 1. Capability-Based Discovery

NestGate discovers services by capability, not hardcoded location:

```rust
use nestgate_core::config::capability_based::CapabilityConfigBuilder;

let config = CapabilityConfigBuilder::new().build()?;
let storage = config.discover(PrimalCapability::Storage).await?;
```

**Benefits**:
- Zero hardcoding
- Runtime flexibility
- Sovereignty-compliant

### 2. Universal Storage

Single API for multiple storage backends:

```rust
use nestgate_core::universal_storage::UniversalStorage;

let storage = UniversalStorage::auto_detect().await?;
storage.create_dataset("my-data").await?;
```

**Supported Backends**:
- ZFS (native)
- Filesystem (POSIX)
- Object storage (S3-compatible)
- Memory (testing)

### 3. Error Handling

All public APIs return `Result`:

```rust
pub fn process(id: &str) -> Result<Data> {
    let item = storage.get(id)?;  // Propagate errors
    Ok(item)
}
```

**Never use `.unwrap()` in production code!**

---

## 🧪 Testing

### Running Tests

```bash
# All tests
cargo test --workspace

# Specific modules
cargo test --package nestgate-core
cargo test --lib --package nestgate-api

# Integration tests
cargo test --test '*'

# With output
cargo test -- --nocapture

# Single test
cargo test test_storage_creation
```

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operation() {
        let result = process_data("test");
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_async_operation() {
        let storage = create_storage().await.unwrap();
        assert!(storage.is_healthy().await);
    }
}
```

---

## 🔍 Project Structure

```
nestgate/
├── code/crates/          # Main crates
│   ├── nestgate-core/    # Core functionality
│   ├── nestgate-api/     # API server
│   └── nestgate-zfs/     # ZFS backend
├── examples/             # Usage examples
├── tests/                # Integration tests
├── docs/                 # Documentation
├── config/               # Configuration templates
└── scripts/              # Utility scripts
```

### Important Directories

- **`code/crates/nestgate-core/src/`** - Core implementation
- **`examples/`** - Working code examples
- **`tests/`** - Integration & E2E tests
- **`docs/`** - Comprehensive documentation

---

## 🚢 Production Deployment

### Pre-Deployment Checklist

```bash
# 1. Run all tests
cargo test --workspace --release

# 2. Check formatting
cargo fmt --all --check

# 3. Run lints
cargo clippy --workspace -- -D warnings

# 4. Verify deployment readiness
./verify_deployment_readiness.sh

# 5. Build release
cargo build --release --workspace
```

### Deploy

```bash
# Using deploy script
./deploy/production-deploy.sh

# Or manually
sudo systemctl start nestgate
sudo systemctl enable nestgate
```

See **[Operations Runbook](./OPERATIONS_RUNBOOK.md)** for detailed deployment instructions.

---

## 🐛 Troubleshooting

### Build Failures

**Problem**: Compilation errors
```bash
# Solution 1: Update Rust
rustup update stable

# Solution 2: Clean build
cargo clean
cargo build

# Solution 3: Check dependencies
cargo update
```

### Test Failures

**Problem**: Tests failing
```bash
# Solution 1: Run specific test with output
cargo test failing_test -- --nocapture

# Solution 2: Check environment
env | grep NESTGATE

# Solution 3: Reset test environment
rm -rf /tmp/nestgate-test-*
```

### Runtime Issues

**Problem**: Service won't start
```bash
# Check configuration
echo $NESTGATE_FAMILY_ID

# Check port availability
sudo lsof -i :8080

# Check logs
./logs/nestgate.log
```

---

## 📖 Next Steps

After getting started:

1. **Read Architecture** → [ARCHITECTURE_OVERVIEW.md](./ARCHITECTURE_OVERVIEW.md)
2. **Explore Examples** → [examples/](./examples/)
3. **Review API Docs** → [JSONRPC_API_DOCUMENTATION.md](./JSONRPC_API_DOCUMENTATION.md)
4. **Join Development** → [CONTRIBUTING.md](./CONTRIBUTING.md)

---

## 🆘 Getting Help

- **Documentation**: [docs/](./docs/)
- **Examples**: [examples/](./examples/)
- **API Reference**: Run `cargo doc --open`
- **Issues**: [GitHub Issues](https://github.com/ecoprimals/nestgate/issues)

---

## ✅ Checklist

- [ ] Rust 1.75+ installed
- [ ] Repository cloned
- [ ] Project builds successfully
- [ ] Tests pass
- [ ] Environment variables set
- [ ] Local instance running
- [ ] Health endpoint responds

**All checked?** You're ready to develop! 🚀

---

**Quick Links**:
- [README](./README.md) - Project overview
- [Architecture](./ARCHITECTURE_OVERVIEW.md) - System design
- [Contributing](./CONTRIBUTING.md) - How to contribute
- [API Docs](./JSONRPC_API_DOCUMENTATION.md) - API reference

**Status**: Production Ready ✅  
**Grade**: A (95/100)  
**Last Updated**: January 13, 2026
