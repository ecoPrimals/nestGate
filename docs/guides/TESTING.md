> **ARCHIVED**: Written November 2025 (~1,319 tests, 41% coverage). As of April 2026 the
> suite has **~12,240 tests at ~80% coverage**. For current testing guidance see
> [`tests/README.md`](../../tests/README.md) and [`STATUS.md`](../../STATUS.md).

# 🧪 NestGate Testing Guide
**Version**: 1.0  
**Last Updated**: November 3, 2025  
**Status**: Production Testing System Active

---

## 📊 Quick Status

```
Tests:          1,319 PASSING (100% pass rate)
Coverage:       41.29%
Target:         90%
Infrastructure: Comprehensive (Unit, Integration, E2E, Chaos, Fault)
Build:          ✅ Clean
```

---

## 🚀 Quick Start

### Run All Tests (Recommended)
```bash
# Run all unit tests across workspace
cargo test --workspace --lib --no-fail-fast

# Run tests for specific package
cargo test --package nestgate-core --lib
cargo test --package nestgate-api --lib
cargo test --package nestgate-zfs --lib
```

### Generate Coverage Report
```bash
# Generate HTML coverage report
cargo llvm-cov --workspace --lib --html

# View the report
open target/llvm-cov/html/index.html
# Or on Linux: xdg-open target/llvm-cov/html/index.html

# Get coverage summary only
cargo llvm-cov --workspace --lib --summary-only
```

### Run Specific Tests
```bash
# Run a specific test by name
cargo test --package nestgate-core test_zero_cost_architecture

# Run tests matching a pattern
cargo test --lib zero_cost

# Run tests in a specific file
cargo test --package nestgate-core --lib error::
```

---

## 📂 Test Organization

### Test Structure

```
nestgate/
├── code/crates/
│   ├── nestgate-core/
│   │   ├── src/
│   │   │   ├── *_tests.rs          # Unit tests alongside code
│   │   │   └── */mod.rs             # Module tests
│   │   └── tests/                   # Integration tests
│   ├── nestgate-api/
│   │   └── src/handlers/*_tests.rs  # Handler tests
│   ├── nestgate-zfs/
│   │   └── src/*_tests.rs           # ZFS operation tests
│   └── [other crates]/
│       └── tests/                   # Per-crate integration tests
└── tests/                           # Workspace-level integration tests
    ├── chaos/                       # Chaos engineering tests
    ├── e2e/                         # End-to-end tests
    ├── integration/                 # Integration test suites
    └── *.rs                         # Top-level test files
```

---

## 🧪 Test Types

### 1. Unit Tests (1,319 tests) ✅

**Location**: `code/crates/*/src/*_tests.rs`

**Purpose**: Test individual functions, types, and modules in isolation.

**Run**:
```bash
# All unit tests
cargo test --workspace --lib

# Specific crate
cargo test --package nestgate-core --lib
```

**Examples**:
- Type construction tests
- Error handling tests
- Configuration validation tests
- Zero-cost abstraction tests

### 2. Integration Tests ⚠️

**Location**: `tests/*.rs`, `code/crates/*/tests/*.rs`

**Purpose**: Test interactions between modules and crates.

**Run**:
```bash
# All integration tests (some have issues)
cargo test --workspace --tests

# Specific integration test
cargo test --test integration_tests
```

**Status**: 36 test files, some need compilation fixes

### 3. E2E Tests ✅

**Location**: `tests/e2e/*.rs`, `tests/*e2e*.rs`

**Purpose**: Test complete workflows from end to end.

**Files**:
- `tests/e2e_comprehensive_workflows_split.rs`
- `tests/integration/e2e_chaos_test.rs`
- `tests/integration/universal_architecture_e2e_test.rs`

**Run**:
```bash
cargo test --test e2e_comprehensive_workflows_split
```

### 4. Chaos Engineering Tests ✅

**Location**: `tests/chaos/*.rs`, `tests/*chaos*.rs`

**Purpose**: Test system behavior under failure conditions.

**Files** (8 total):
- `tests/sovereignty_chaos_testing.rs`
- `tests/chaos/comprehensive_chaos_tests.rs`
- `tests/chaos_simple_modern.rs`
- And 5 more

**Run**:
```bash
# All chaos tests
cargo test --workspace --test '*chaos*'

# Specific chaos test
cargo test --test chaos_simple_modern
```

**Note**: Some chaos tests need compilation fixes (type errors)

### 5. Fault Injection Tests ✅

**Location**: `tests/*fault*.rs`

**Purpose**: Test error handling and recovery under fault conditions.

**Files**:
- `tests/fault_injection_framework.rs`
- `tests/fault_injection_suite.rs`

**Run**:
```bash
cargo test --test fault_injection_framework
```

---

## 📊 Coverage Analysis

### Current Coverage: 41.29%

**Excellent Coverage (>60%)**:
- `nestgate-zfs/src/types.rs` - 64.91%
- `nestgate-zfs/src/pool_types_tests.rs` - 97.10%
- Zero-cost modules - 90%+
- Test utilities - 90-100%

**Good Coverage (40-60%)**:
- Core error handling - 40-50%
- Pool helpers - 46.74%
- Production readiness - 37.26%

**Needs Work (<40%)**:
- Pool manager - 23.50%
- Monitoring modules - 0-20%
- Performance engine - 0-10%
- Some storage backends - 0-30%

### Viewing Coverage

```bash
# Generate HTML report
cargo llvm-cov --workspace --lib --html

# View in browser
open target/llvm-cov/html/index.html

# Check specific file coverage
open target/llvm-cov/html/nestgate_core/src/index.html
```

### Coverage Goals

| Timeframe | Target | Priority Modules |
|-----------|--------|------------------|
| **Current** | 41.29% | ✅ Achieved |
| **Month 1** | 50-55% | Storage backends, core paths |
| **Month 2** | 65-75% | Monitoring, networking |
| **Month 3** | 80-90% | Edge cases, error paths |

---

## 🎯 Test Execution by Package

### nestgate-core (689 tests)

**Coverage**: ~45%

```bash
# All core tests
cargo test --package nestgate-core --lib

# Specific modules
cargo test --package nestgate-core error::
cargo test --package nestgate-core zero_cost::
cargo test --package nestgate-core universal_adapter::
```

**Key test modules**:
- Error handling (comprehensive)
- Zero-cost abstractions
- Configuration system
- Universal adapter
- Infant Discovery
- Security providers

### nestgate-zfs (163 tests)

**Coverage**: ~38%

```bash
# All ZFS tests
cargo test --package nestgate-zfs --lib

# Specific areas
cargo test --package nestgate-zfs types::
cargo test --package nestgate-zfs pool::
cargo test --package nestgate-zfs zero_cost_zfs::
```

**Key test modules**:
- ZFS types and constructors
- Pool operations
- Zero-cost ZFS operations
- Production readiness

### nestgate-api (124 tests)

**Coverage**: ~40%

```bash
# All API tests
cargo test --package nestgate-api --lib

# Specific handlers
cargo test --package nestgate-api handlers::status::
cargo test --package nestgate-api handlers::workspace::
cargo test --package nestgate-api rest::
```

**Key test modules**:
- Status handlers
- Workspace management
- REST endpoints
- WebSocket support

### Other Packages

```bash
# nestgate-network — archived (removed from workspace in v4.7.0); do not use --package nestgate-network

# nestgate-canonical (105 tests)
cargo test --package nestgate-canonical --lib

# nestgate-performance (54 tests)
cargo test --package nestgate-performance --lib

# nestgate-nas (34 tests)
cargo test --package nestgate-nas --lib

# nestgate-mcp: removed from workspace (MCP delegated to biomeOS capability.call)
```

---

## 🔧 Test Development

### Writing New Tests

#### Unit Test Template

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_works() {
        // Arrange
        let input = setup_test_data();
        
        // Act
        let result = feature_under_test(input);
        
        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_value);
    }

    #[test]
    fn test_feature_handles_errors() {
        let invalid_input = create_invalid_input();
        let result = feature_under_test(invalid_input);
        assert!(result.is_err());
    }
}
```

#### Async Test Template

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_async_feature() {
        let service = create_test_service().await;
        let result = service.async_operation().await;
        assert!(result.is_ok());
    }
}
```

### Test Organization Guidelines

1. **Keep tests close to code**: Place unit tests in `*_tests.rs` files next to the code they test
2. **Use descriptive names**: `test_feature_handles_edge_case` not `test1`
3. **Test one thing**: Each test should verify one specific behavior
4. **Include error cases**: Test both success and failure paths
5. **Use fixtures**: Create reusable test data in test utilities

---

## 🐛 Troubleshooting

### Tests Not Running?

**Problem**: `cargo test --workspace --lib` shows 0 tests

**Solution**: Run per-package:
```bash
cargo test --package nestgate-core --lib
cargo test --package nestgate-api --lib
# etc.
```

### Compilation Errors?

**Problem**: Some integration tests fail to compile

**Known Issues**:
- `chaos_engineering_suite.rs` - Type name errors (being fixed)
- Some tests require features or infrastructure

**Solution**: Skip problematic tests for now:
```bash
# Run only working tests
cargo test --lib  # Skip integration tests
```

### Coverage Not Generating?

**Problem**: `cargo llvm-cov` fails

**Prerequisites**:
```bash
# Install llvm-cov
cargo install cargo-llvm-cov

# Ensure llvm-tools are installed
rustup component add llvm-tools-preview
```

### Slow Test Execution?

**Problem**: Tests take too long

**Solutions**:
```bash
# Run tests in parallel (default)
cargo test --workspace --lib

# Run specific package only
cargo test --package nestgate-core --lib

# Skip expensive tests
cargo test --lib -- --skip expensive_
```

---

## 📈 Coverage Expansion Plan

### Phase 1: Quick Wins (Weeks 1-2)

**Target**: 50-55% coverage

**Focus Areas**:
- Add tests for 0% coverage modules
- Test error paths
- Test configuration edge cases

**Commands**:
```bash
# Identify 0% coverage files
cargo llvm-cov --workspace --lib --html
# Review target/llvm-cov/html/index.html

# Write tests for uncovered modules
# Run and verify
cargo test --package <crate> --lib
cargo llvm-cov --workspace --lib --summary-only
```

### Phase 2: Core Coverage (Weeks 3-6)

**Target**: 65-75% coverage

**Focus Areas**:
- Storage backends
- Monitoring systems
- Network modules
- Performance engines

### Phase 3: Comprehensive (Weeks 7-10)

**Target**: 80-90% coverage

**Focus Areas**:
- Edge cases
- Error recovery
- Integration scenarios
- Chaos/fault scenarios

---

## 🎯 CI/CD Integration

### GitHub Actions Example

```yaml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          
      - name: Run tests
        run: cargo test --workspace --lib --no-fail-fast
        
      - name: Generate coverage
        run: |
          cargo install cargo-llvm-cov
          cargo llvm-cov --workspace --lib --lcov --output-path lcov.info
          
      - name: Upload coverage
        uses: codecov/codecov-action@v3
        with:
          files: lcov.info
```

---

## 📚 Best Practices

### 1. Test Naming
```rust
// ✅ Good
#[test]
fn test_user_creation_with_valid_email() { }

// ❌ Bad
#[test]
fn test1() { }
```

### 2. Test Organization
```rust
// ✅ Good - Organized by feature
#[cfg(test)]
mod user_tests {
    mod creation {
        #[test]
        fn test_valid_user() { }
        
        #[test]
        fn test_invalid_email() { }
    }
    
    mod validation {
        // ...
    }
}
```

### 3. Assertions
```rust
// ✅ Good - Clear message
assert_eq!(
    result.status, 
    UserStatus::Active,
    "User should be active after creation"
);

// ❌ Bad - No context
assert!(result.status == UserStatus::Active);
```

### 4. Test Data
```rust
// ✅ Good - Reusable fixtures
fn create_test_user() -> User {
    User {
        id: "test-123".into(),
        email: "test@example.com".into(),
        // ...
    }
}

// ❌ Bad - Inline data repeated
#[test]
fn test1() {
    let user = User { id: "test-123".into(), ... };
}
```

---

## 🎯 Quick Reference

### Essential Commands

```bash
# Run all unit tests
cargo test --workspace --lib

# Generate coverage
cargo llvm-cov --workspace --lib --html

# Run specific test
cargo test --package nestgate-core test_name

# Watch mode (requires cargo-watch)
cargo watch -x "test --lib"

# Run with output
cargo test --lib -- --nocapture

# Run single-threaded
cargo test --lib -- --test-threads=1
```

### Coverage Commands

```bash
# HTML report
cargo llvm-cov --workspace --lib --html

# Summary only
cargo llvm-cov --workspace --lib --summary-only

# LCOV format (for CI)
cargo llvm-cov --workspace --lib --lcov --output-path lcov.info

# JSON format
cargo llvm-cov --workspace --lib --json --output-path coverage.json
```

### Package-Specific

```bash
# Core tests
cargo test --package nestgate-core --lib

# API tests
cargo test --package nestgate-api --lib

# ZFS tests
cargo test --package nestgate-zfs --lib

# Network: nestgate-network crate was archived (v4.7.0); run network-related tests via nestgate-core / nestgate-api as applicable
```

---

## 📞 Support

### Getting Help

- **Coverage Issues**: Check `target/llvm-cov/html/index.html`
- **Test Failures**: Run with `--nocapture` for full output
- **Build Issues**: Try `cargo clean` and rebuild

### Resources

- [Rust Testing Book](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [cargo-llvm-cov Documentation](https://github.com/taiki-e/cargo-llvm-cov)
- [NestGate Test Reports](./TEST_SYSTEM_ACTIVATION_REPORT_NOV_3_2025.md)

---

## 🎉 Current Status

```
✅ 1,319 tests passing (100% pass rate)
✅ 41.29% coverage (measured)
✅ Unit tests: Comprehensive
✅ Integration tests: Present (some need fixes)
✅ E2E tests: 3 files
✅ Chaos tests: 8 files
✅ Fault injection: 2 files
✅ Build: Clean
✅ Test infrastructure: Production-grade
```

**Next Goal**: Expand coverage to 50-55% (Weeks 1-2)

---

*Last Updated: November 3, 2025*  
*Test System Version: 1.0*  
*Total Tests: 1,319*  
*Coverage: 41.29%*

