# Developer Onboarding Guide

**Welcome to NestGate development!**

---

## What is NestGate?

**NestGate** is a **storage and discovery primal** in the ecoPrimals ecosystem.

**Core Responsibilities**:
- **Storage**: Dataset and object management with ZFS backend
- **Discovery**: Capability-based service discovery (runtime UDS socket resolution)
- **Registry**: Service metadata and capability tracking

**Architecture**: Primal Sovereignty (self-knowledge + runtime discovery)

---

## Codebase Structure

```
nestgate/
├── code/crates/
│   ├── nestgate-core/          # Core library (main development here)
│   │   ├── src/
│   │   │   ├── services/       # Service implementations
│   │   │   │   └── storage/    # Storage service (refactored)
│   │   │   ├── config/         # Configuration system (refactored)
│   │   │   ├── rpc/            # RPC and IPC
│   │   │   ├── primal_discovery/ # Capability discovery
│   │   │   ├── error/          # Error handling
│   │   │   └── lib.rs          # Library root
│   │   └── Cargo.toml
│   └── nestgate-bin/           # Binary entry point
├── tests/                      # Integration tests
├── benches/                    # Performance benchmarks
├── examples/                   # Usage examples
├── docs/                       # Documentation (39 guides)
├── config/                     # Configuration examples
└── README.md                   # Project overview
```

**Key Modules**:
- `services/storage` - Dataset/object operations (modular)
- `config/environment` - Environment-driven config (modular)
- `primal_discovery` - Capability-based discovery
- `rpc` - HTTP API + Unix socket JSON-RPC
- `error` - Comprehensive error handling

---

## Getting Started (30 minutes)

### **Step 1: Prerequisites** (5 min)

```bash
# Install Rust (if not already)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Verify version (need 1.75+)
rustc --version

# Install development tools
cargo install cargo-watch cargo-nextest cargo-audit

# Optional: ZFS for full functionality
sudo apt install zfsutils-linux  # Ubuntu/Debian
```

### **Step 2: Clone and Build** (10 min)

```bash
# Clone repository
git clone https://github.com/ecoPrimals/nestGate.git
cd nestGate

# Build workspace
cargo build --workspace

# Run tests (verify everything works)
cargo test --workspace

# Expected: 3670+ tests passing
```

### **Step 3: Understand the Architecture** (10 min)

**Read these docs** (in order):
1. `README.md` - Project overview
2. `QUICK_START.md` - 5-minute setup
3. `docs/architecture/COMPONENT_INTERACTIONS.md` - System design
4. `docs/guides/ENVIRONMENT_VARIABLES.md` - Configuration

### **Step 4: Run Locally** (5 min)

```bash
# Development mode (faster compile)
cargo run

# Or release mode (better performance)
cargo run --release

# Verify running
curl http://localhost:8080/health
```

---

## Development Workflow

### **Daily Development**:

```bash
# 1. Update from main
git pull origin main

# 2. Create feature branch
git checkout -b feature/my-awesome-feature

# 3. Make changes
# Edit files...

# 4. Auto-rebuild on save
cargo watch -x 'check' -x 'test --lib'

# 5. Run tests
cargo test

# 6. Check code quality
cargo clippy --all-targets
cargo fmt

# 7. Commit changes
git add .
git commit -m "feat: add awesome feature"

# 8. Push and create PR
git push -u origin feature/my-awesome-feature
# Then create PR on GitHub
```

### **Testing Workflow**:

```bash
# Run all tests
cargo test --workspace

# Run specific module tests
cargo test --package nestgate-core --lib services::storage

# Run with output (for debugging)
cargo test test_name -- --nocapture

# Fast testing with nextest
cargo nextest run

# Watch mode (auto-run tests)
cargo watch -x 'nextest run'

# Coverage report (primary — llvm-cov)
cargo llvm-cov --workspace --summary-only --ignore-filename-regex 'tools/'
# HTML report:
cargo llvm-cov --workspace --html --ignore-filename-regex 'tools/'
```

### **Debugging Workflow**:

```bash
# Enable debug logs
export RUST_LOG=nestgate=debug
cargo run

# Use rust-gdb
rust-gdb --args target/debug/nestgate

# Or lldb
rust-lldb target/debug/nestgate

# Print values
dbg!(variable);
println!("Debug: {:?}", value);

# Use tracing
use tracing::{debug, info, warn, error};
debug!("Processing request: {:?}", request);
```

---

## Coding Guidelines

### **Rust Style**:

**DO**:
- Use `rustfmt` (auto-format)
- Run `clippy` (catch issues)
- Write comprehensive tests
- Add documentation comments (`///`)
- Use `Result<T>` for fallible operations
- Prefer `&str` over `String` in function params

**DON'T**:
- Use `unwrap()` in production code (use `?` or `.ok()`)
- Hardcode values (use environment variables)
- Create large monolithic files (>800 lines = consider refactoring)
- Skip error handling
- Use `unsafe` without documentation and safety proof

### **Error Handling**:

```rust
// GOOD
pub async fn create_dataset(name: &str) -> Result<DatasetInfo> {
    validate_name(name)?;
    let dataset = storage.create(name).await?;
    Ok(dataset)
}

// BAD
pub async fn create_dataset(name: &str) -> DatasetInfo {
    storage.create(name).await.unwrap()  // Will panic!
}
```

### **Environment Variables**:

```rust
// GOOD - With fallback
let port = std::env::var("NESTGATE_PORT")
    .ok()
    .and_then(|p| p.parse().ok())
    .unwrap_or(8080);

// BAD - Hardcoded
let port = 8080;
```

### **Testing**:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dataset_creation() {
        let dataset = Dataset::new("test");
        assert_eq!(dataset.name(), "test");
    }

    #[tokio::test]
    async fn test_async_operation() {
        let service = StorageService::new().await.unwrap();
        let result = service.create_dataset("test", params).await;
        assert!(result.is_ok());
    }
}
```

---

## Common Development Tasks

### **Add a New API Endpoint**:

1. Define in `rpc/mod.rs`:
```rust
#[derive(Serialize, Deserialize)]
pub struct MyRequest {
    pub param: String,
}

pub async fn my_endpoint(req: MyRequest) -> Result<MyResponse> {
    // Implementation
}
```

2. Wire up in HTTP server
3. Add tests in `tests/api/`
4. Document in `docs/api/REST_API.md`

### **Add Environment Variable**:

1. Add to `config/environment.rs` (or submodule)
2. Document in `docs/guides/ENVIRONMENT_VARIABLES.md`
3. Add test in `config/environment/tests`
4. Update `README.md` if user-facing

### **Add a New Service**:

1. Create in `services/my_service/`
2. Implement service trait
3. Add configuration
4. Add comprehensive tests
5. Document in `docs/guides/`

---

## Best Practices

### **1. Test Coverage**

- Unit tests for all public functions
- Integration tests for workflows
- Property-based tests for complex logic
- Benchmark critical paths

**Target**: >80% coverage

### **2. Documentation**

- Public APIs: Comprehensive doc comments
- Complex logic: Inline comments
- Modules: Module-level docs (`//!`)
- Examples: Working code examples

**Target**: All public items documented

### **3. Performance**

- Profile before optimizing
- Benchmark changes
- Avoid premature optimization
- Use `cargo flamegraph` for profiling

**Target**: No regressions

### **4. Security**

- No secrets in code
- Validate all inputs
- Use constant-time comparisons for crypto
- Document all `unsafe` blocks

**Target**: Zero vulnerabilities

---

## Review Process

### **Before Creating PR**:

- [ ] All tests pass (`cargo test`)
- [ ] Code formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation updated
- [ ] CHANGELOG updated (if user-facing)
- [ ] Commit message follows convention

### **Commit Message Format**:

```
<type>: <description>

<body (optional)>

<footer (optional)>
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `refactor`: Code refactoring
- `docs`: Documentation
- `test`: Tests
- `perf`: Performance improvement
- `chore`: Maintenance

**Example**:
```
feat: add SHA-256 checksum calculation

Implements data integrity verification for stored objects.
Checksums are calculated on store and returned in ObjectInfo.

Closes #123
```

### **PR Checklist**:

- [ ] Descriptive title
- [ ] Linked to issue (if applicable)
- [ ] Tests included
- [ ] Documentation updated
- [ ] No breaking changes (or documented)
- [ ] Benchmarks run (for performance changes)

---

## Testing Guidelines

### **Test Pyramid**:

```
       ┌─────────────┐
       │  E2E Tests  │  (Few, slow, comprehensive)
       └─────────────┘
      ┌───────────────┐
      │ Integration   │   (Some, medium, workflows)
      │     Tests     │
      └───────────────┘
    ┌───────────────────┐
    │   Unit Tests      │    (Many, fast, focused)
    │   (70-80%)        │
    └───────────────────┘
```

### **Unit Test Example**:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dataset_validation() {
        assert!(validate_name("good-name").is_ok());
        assert!(validate_name("").is_err());
        assert!(validate_name("bad name").is_err());
    }

    #[tokio::test]
    async fn test_dataset_creation() {
        let mut config = StorageServiceConfig::development();
        config.auto_discover_pools = false;
        
        let service = StorageManagerService::with_config(config).await;
        assert!(service.is_ok());
    }
}
```

---

## Learning Resources

### **Internal Documentation**:
- `docs/architecture/` - System design
- `docs/guides/` - How-to guides
- `docs/api/` - API references
- `examples/` - Working code examples

### **External Resources**:
- [Rust Book](https://doc.rust-lang.org/book/) - Rust fundamentals
- [Async Book](https://rust-lang.github.io/async-book/) - Async Rust
- [tokio docs](https://tokio.rs/) - Async runtime
- [thiserror](https://docs.rs/thiserror/) - Error handling

---

## Your First Contribution

**Recommended starter tasks**:

1. **Fix a typo** - Good first PR!
2. **Improve documentation** - Always needed
3. **Add unit tests** - Increase coverage
4. **Fix a `good-first-issue`** - Check GitHub labels

**Medium tasks**:
1. Add new environment variable
2. Improve error messages
3. Add new API endpoint
4. Refactor a large file

**Advanced tasks**:
1. Implement new storage backend
2. Add clustering feature
3. Performance optimization
4. Security enhancement

---

## Community

### **Communication**:
- GitHub Issues - Bug reports, feature requests
- GitHub Discussions - Questions, ideas
- Pull Requests - Code contributions

### **Code of Conduct**:
- Be respectful and inclusive
- Provide constructive feedback
- Help newcomers
- Celebrate contributions

---

**Welcome to the team!**

**Questions?** See `CONTRIBUTING.md` or open a GitHub Discussion!

**NestGate Development** · Pure Rust · Production-Ready
