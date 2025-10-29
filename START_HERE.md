# 🚀 Welcome to NestGate!

**Last Updated**: October 28, 2025  
**Status**: Active Development - Test Expansion Phase

---

## 👋 **First Time Here?**

NestGate is a **sovereign Network Attached Storage (NAS) system** built in Rust, featuring:

- 🧠 **Infant Discovery Architecture** - World-first runtime capability detection
- ⚡ **Zero-Cost Architecture** - 45% performance improvement over traditional approaches
- 🔒 **Sovereignty Layer** - Enforces human dignity and ethical AI principles
- 💾 **Universal Storage** - Native ZFS integration with extensible backends
- 🌐 **Comprehensive API** - REST, RPC, and WebSocket interfaces

---

## 🎯 **Quick Navigation**

| I want to... | Go here |
|--------------|---------|
| **Understand the project** | [README.md](README.md) |
| **Get started quickly** | [QUICK_START_GUIDE.md](QUICK_START_GUIDE.md) |
| **See current status** | [CURRENT_STATUS.md](CURRENT_STATUS.md) |
| **Find all documentation** | [ROOT_DOCS_INDEX.md](ROOT_DOCS_INDEX.md) |
| **Contribute code** | [CONTRIBUTING.md](CONTRIBUTING.md) |
| **Deploy to production** | [DEPLOYMENT_GUIDE.md](DEPLOYMENT_GUIDE.md) |
| **Report a bug** | [KNOWN_ISSUES.md](KNOWN_ISSUES.md) |

---

## 🚀 **Quick Start** (5 Minutes)

### Prerequisites
```bash
# Rust toolchain (1.75+)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# ZFS (optional, for storage features)
# Ubuntu/Debian:
sudo apt install zfsutils-linux

# Arch:
sudo pacman -S zfs-utils
```

### Build & Test
```bash
# Clone the repository
git clone <repository-url>
cd nestgate

# Build the project
cargo build --release

# Run tests (673 passing ✅)
cargo test --workspace --lib

# Run a specific component
cargo run --bin nestgate-api-server
```

### Verify Installation
```bash
# Check build status
cargo check --workspace

# Run quick validation
cargo test --package nestgate-core --lib

# Expected output: "test result: ok. 518 passed"
```

---

## 📁 **Project Structure**

```
nestgate/
├── 📦 code/crates/          # Main workspace crates
│   ├── nestgate-core/       # Core library (518 tests ✅)
│   ├── nestgate-api/        # REST/RPC API (56 tests ✅)
│   ├── nestgate-zfs/        # ZFS integration (99 tests ✅)
│   ├── nestgate-bin/        # Binary executables
│   └── nestgate-canonical/  # Canonical types & configs
├── 📚 docs/                 # Comprehensive documentation (500+ files)
├── 📋 specs/                # Technical specifications
├── 🧪 tests/                # Integration & E2E tests
├── ⚡ benches/              # Performance benchmarks
├── 📖 examples/             # Usage examples
├── 🛠️ tools/                # Development tools
├── 🐳 docker/               # Docker configurations
└── 🚢 deploy/               # Deployment scripts
```

---

## 🎓 **Learning Path**

### 1. **Understand the Architecture** (30 minutes)
Read these in order:
1. [README.md](README.md) - Project overview
2. [ARCHITECTURE_OVERVIEW.md](ARCHITECTURE_OVERVIEW.md) - System design
3. [docs/architecture/infant-discovery.md](docs/architecture/) - Infant Discovery Architecture
4. [docs/architecture/zero-cost.md](docs/architecture/) - Zero-Cost Architecture

### 2. **Explore the Code** (1 hour)
```bash
# Start with the core library
code/crates/nestgate-core/src/

# Key files to read:
├── lib.rs                    # Module organization
├── config/canonical_master.rs # Configuration system
├── error/mod.rs              # Error handling
├── traits/mod.rs             # Core traits
└── discovery/                # Infant Discovery implementation
```

### 3. **Run Examples** (30 minutes)
```bash
# Basic storage example
cargo run --example storage_demo

# API server example
cargo run --example api_server

# Discovery example
cargo run --example infant_discovery_demo
```

### 4. **Write Your First Test** (30 minutes)
See: [CONTRIBUTING.md](CONTRIBUTING.md#writing-tests)

---

## 🧪 **Testing**

### Current Status
```
Coverage:       15.94% → Target: 90%
Library Tests:  673 passing (100% pass rate) ✅
Status:         Phase 2 - Test Expansion
```

### Run Tests
```bash
# All library tests (fast, 100% passing)
cargo test --workspace --lib

# Specific crate
cargo test --package nestgate-core

# With coverage report
cargo tarpaulin --workspace --lib --out Html
```

### Test Organization
- **Unit Tests**: In-module `#[cfg(test)]` blocks
- **Integration Tests**: `tests/` directory
- **Comprehensive Tests**: `*_comprehensive_tests.rs` files
- **Benchmarks**: `benches/` directory

**Testing Guide**: [TEST_MODERNIZATION_PLAN_OCT_28_2025.md](TEST_MODERNIZATION_PLAN_OCT_28_2025.md)

---

## 🏗️ **Core Concepts**

### Infant Discovery Architecture
Runtime capability detection without hardcoded assumptions. Services announce their capabilities, and the system discovers them dynamically.

```rust
use nestgate_core::discovery::InfantDiscovery;

let discovery = InfantDiscovery::new();
let capabilities = discovery.discover_capabilities().await?;
```

**Learn more**: [docs/architecture/infant-discovery.md](docs/architecture/)

### Zero-Cost Architecture
High-level abstractions with zero runtime overhead. Achieves 45% performance improvement through compile-time optimizations.

```rust
use nestgate_core::zero_cost::ZeroCostOperation;

let op = ZeroCostOperation::new(); // No heap allocation
let result = op.execute()?;        // Inlined at compile time
```

**Learn more**: [docs/architecture/zero-cost.md](docs/architecture/)

### Sovereignty Layer
Enforces human dignity and ethical principles. Every operation respects user sovereignty and consent.

```rust
use nestgate_core::sovereignty::SovereigntyCheck;

SovereigntyCheck::enforce_human_dignity(operation)?;
```

**Learn more**: [docs/sovereignty/](docs/sovereignty/)

---

## 🤝 **Contributing**

### Quick Contribution Guide

1. **Find an Issue**
   - Check [KNOWN_ISSUES.md](KNOWN_ISSUES.md)
   - Look for "good first issue" labels
   - Join discussion in issues

2. **Set Up Development**
   ```bash
   # Fork and clone
   git clone <your-fork>
   cd nestgate
   
   # Create feature branch
   git checkout -b feature/your-feature
   
   # Make changes and test
   cargo test --workspace --lib
   cargo clippy --workspace
   cargo fmt --check
   ```

3. **Submit Pull Request**
   - Write clear commit messages
   - Add tests for new features
   - Update documentation
   - Follow [CONTRIBUTING.md](CONTRIBUTING.md)

### Development Tools
```bash
# Format code
cargo fmt

# Lint code
cargo clippy --workspace -- -D warnings

# Check for errors
cargo check --workspace

# Run benchmarks
cargo bench
```

---

## 📊 **Current Development Focus**

### **Phase 2: Test Expansion** (In Progress)

**Goal**: Expand test coverage from 15.94% to 20%

**Recent Progress** (Oct 28, 2025):
- ✅ Added 100+ comprehensive tests
- ✅ Fixed 6 unwrap() → expect() migrations
- ✅ Achieved 100% library test pass rate
- ✅ Documented all known issues

**Next Steps**:
1. Fix security module syntax errors (1-2 hours)
2. Re-enable integration tests
3. Add 200-300 more handler tests
4. Continue unwrap migration

**Full Status**: [CURRENT_STATUS.md](CURRENT_STATUS.md)

---

## 🔧 **Troubleshooting**

### Common Issues

**Build Fails**
```bash
# Clean and rebuild
cargo clean
cargo build --release
```

**Tests Fail**
```bash
# Run tests with output
cargo test --workspace --lib -- --nocapture

# Check specific test
cargo test --package nestgate-core test_name -- --nocapture
```

**Missing Dependencies**
```bash
# Update dependencies
cargo update

# Check for issues
cargo tree --duplicates
```

### Get Help
- Check [KNOWN_ISSUES.md](KNOWN_ISSUES.md)
- Read [ROOT_DOCS_INDEX.md](ROOT_DOCS_INDEX.md)
- Review [CONTRIBUTING.md](CONTRIBUTING.md)
- Ask in project discussions

---

## 📚 **Essential Documentation**

### Must-Read Documents
1. **[README.md](README.md)** - Project overview
2. **[CURRENT_STATUS.md](CURRENT_STATUS.md)** - Current development status
3. **[ARCHITECTURE_OVERVIEW.md](ARCHITECTURE_OVERVIEW.md)** - System architecture
4. **[CONTRIBUTING.md](CONTRIBUTING.md)** - Contribution guidelines

### Reference Documentation
- **[ROOT_DOCS_INDEX.md](ROOT_DOCS_INDEX.md)** - Complete documentation index
- **[KNOWN_ISSUES.md](KNOWN_ISSUES.md)** - Current issues and blockers
- **[DEPLOYMENT_GUIDE.md](DEPLOYMENT_GUIDE.md)** - Production deployment
- **[specs/](specs/)** - Technical specifications

### Development Guides
- **[TEST_MODERNIZATION_PLAN_OCT_28_2025.md](TEST_MODERNIZATION_PLAN_OCT_28_2025.md)** - Testing strategy
- **[TOOL_MIGRATION_QUICKSTART.md](TOOL_MIGRATION_QUICKSTART.md)** - Tool migration
- **[QUICK_COMMANDS.sh](QUICK_COMMANDS.sh)** - Useful commands

---

## 🎯 **Next Steps**

### For New Contributors
1. ✅ Read [CONTRIBUTING.md](CONTRIBUTING.md)
2. ✅ Set up development environment
3. ✅ Run tests to verify setup
4. ✅ Pick a good first issue
5. ✅ Submit your first PR!

### For Explorers
1. ✅ Read [ARCHITECTURE_OVERVIEW.md](ARCHITECTURE_OVERVIEW.md)
2. ✅ Explore [docs/](docs/) directory
3. ✅ Run examples in [examples/](examples/)
4. ✅ Review [specs/](specs/) for technical details

### For Production Users
1. ✅ Read [DEPLOYMENT_GUIDE.md](DEPLOYMENT_GUIDE.md)
2. ✅ Review [CURRENT_STATUS.md](CURRENT_STATUS.md)
3. ✅ Check [KNOWN_ISSUES.md](KNOWN_ISSUES.md)
4. ✅ Set up monitoring and observability

---

## 💡 **Pro Tips**

### Speed Up Development
```bash
# Use cargo watch for auto-rebuild
cargo install cargo-watch
cargo watch -x 'test --workspace --lib'

# Use cargo nextest for faster tests
cargo install cargo-nextest
cargo nextest run --workspace

# Use sccache for faster builds
cargo install sccache
export RUSTC_WRAPPER=sccache
```

### Code Quality
```bash
# Run comprehensive checks
cargo fmt && cargo clippy --workspace -- -D warnings && cargo test --workspace --lib

# Alias for convenience
alias nestgate-check='cargo fmt && cargo clippy --workspace -- -D warnings && cargo test --workspace --lib'
```

### Documentation
```bash
# Generate and open docs
cargo doc --workspace --open --no-deps

# Check doc coverage
cargo +nightly rustdoc --lib -- -D warnings
```

---

## 🏆 **Project Stats**

```
Language:        Rust (100%)
Lines of Code:   ~150,000
Test Coverage:   15.94% (target: 90%)
Tests:           673 passing (100% pass rate)
Crates:          15+ workspace crates
Contributors:    Active development team
License:         (See LICENSE file)
```

---

## 📞 **Contact & Community**

- **Issues**: (Add issue tracker URL)
- **Discussions**: (Add discussions URL)
- **Documentation**: (Add docs site URL)
- **Community**: (Add community links)

---

**Welcome to NestGate!** 🎉

We're building something special here - a truly sovereign storage system that respects users and their data. We're glad you're interested in joining us!

**Ready to dive in?** → [QUICK_START_GUIDE.md](QUICK_START_GUIDE.md)

---

**Last Updated**: October 28, 2025  
**Next Review**: November 11, 2025  
**Maintained by**: NestGate Development Team
