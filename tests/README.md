# 🧪 **NestGate Testing Guide**

**Last Updated**: March 29, 2026  
**Coverage**: 74.3% (workspace, all features)  
**Goal**: Maintain high coverage with clean, maintainable tests

---

## 📊 **Quick Stats**

```
Tests passing:          11,707
Failures:               0
Ignored:                563 (mostly e2e/chaos; see tests/DISABLED_TESTS_REFERENCE.md)
Crates with tests:      Workspace-wide
```

---

## 🚀 **Quick Start**

### **Running Tests**

Primary command for the full suite (all crates, all features):

```bash
cargo test --workspace --all-features
```

Other useful invocations:

```bash
# Library tests only (often faster during development)
cargo test --workspace --all-features --lib

# Specific test name filter
cargo test --workspace --all-features test_name

# Run ignored tests (e2e, chaos, etc.—needs environment/setup)
cargo test --workspace --all-features -- --ignored

# With verbose output
cargo test --workspace --all-features -- --nocapture

# Single-threaded (for debugging)
cargo test --workspace --all-features -- --test-threads=1
```

For focused suites (e2e, chaos, fault, performance), use the test framework and module layout under `tests/` (see **Test Organization** below).

### **Coverage Reports**

```bash
# Workspace coverage (requires cargo-llvm-cov)
cargo llvm-cov --workspace --all-features

# HTML report (typical llvm-cov options)
cargo llvm-cov --workspace --all-features --html
```

---

## 📁 **Test Organization**

Top-level `tests/` is organized by category. Many scenarios also live as `*.rs` integration targets at the root of `tests/`; shared helpers live under `tests/common/`.

```
tests/
├── common/                # Shared utilities (sync, config, mocks, test doubles, …)
├── unit/                  # Focused unit-style tests
├── integration/           # Multi-component / API integration
├── e2e/                   # End-to-end workflows and framework
├── chaos/                 # Chaos / resilience scenarios
├── fault/                 # Fault-injection suites
├── penetration_testing/   # Security-oriented tests
├── performance/           # Performance and load-oriented tests
├── ecosystem/             # Cross-component / live-style integration
├── templates/             # Spec / doc templates for tests
├── specs/                 # Contract and integration spec docs
├── dashmap/               # Concurrent collection / stress coverage
└── unibin/                # CLI / binary-level scenarios
```

Crate-local tests remain under `code/crates/<crate>/tests/` and `#[cfg(test)]` modules inside each crate.

---

## ✍️ **Writing Tests**

### **Unit Test Pattern**

```rust
// In your module: code/crates/my-crate/src/my_module.rs

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_function_with_valid_input() {
        // Arrange
        let input = create_test_input();
        
        // Act
        let result = my_function(input);
        
        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_value);
    }
    
    #[test]
    fn test_function_returns_error_on_invalid_input() {
        let result = my_function(invalid_input());
        assert!(result.is_err());
    }
}
```

### **Integration Test Pattern**

```rust
// In tests/integration/my_feature.rs

use nestgate_core::*;
use crate::common::*;

#[tokio::test]
async fn test_feature_integration() {
    // Setup
    let config = test_config::create_test_config();
    let service = MyService::new(config).await.unwrap();
    
    // Test
    let result = service.perform_action().await;
    
    // Assert
    assert!(result.is_ok());
    
    // Cleanup happens automatically
}
```

### **E2E Test Pattern**

```rust
// In tests/e2e/workflows/my_workflow.rs

use crate::e2e::framework::*;

#[tokio::test]
#[ignore] // Run explicitly with --ignored
async fn test_complete_user_workflow() {
    let runner = Runner::new().await;
    
    let scenario = Scenario::new()
        .step("User creates pool", create_pool)
        .step("User creates dataset", create_dataset)
        .step("User configures replication", setup_replication);
    
    runner.run_scenario(scenario).await.expect("Workflow failed");
}
```

---

## 🎯 **Testing Guidelines**

### **DO ✅**

- ✅ **Write tests for new code** - Every new feature needs tests
- ✅ **Test edge cases** - Invalid input, boundaries, error conditions
- ✅ **Use descriptive names** - `test_config_builder_creates_valid_config`
- ✅ **Keep tests focused** - One concept per test
- ✅ **Use common helpers** - Don't duplicate test setup
- ✅ **Clean up resources** - Use RAII guards or explicit cleanup
- ✅ **Test behavior, not implementation** - Focus on what, not how

### **DON'T ❌**

- ❌ **Use unwrap() in production code** - Tests can unwrap, production can't
- ❌ **Depend on test execution order** - Tests must be independent
- ❌ **Use hardcoded paths** - Use temp dirs and relative paths
- ❌ **Leave flaky tests** - Fix or skip flaky tests immediately
- ❌ **Write mega-tests** - Break large tests into smaller focused ones
- ❌ **Forget to test errors** - Error paths need testing too
- ❌ **Duplicate test logic** - Extract to common helpers

---

## 🛠️ **Test Utilities**

### **Common Helpers** (`tests/common/`)

```rust
use crate::common::*;

// Test configuration
let config = test_config::create_test_config();
let config = test_config::create_production_like_config();

// Test environment
let _guard = test_environment::setup(); // Auto cleanup on drop

// Mock services
let mock_storage = mocks::mock_storage_service();
let mock_security = mocks::mock_security_provider();

// Test data builders
let pool = test_doubles::storage::pool_builder()
    .with_name("test-pool")
    .with_capacity_gb(100)
    .build();
```

### **Assertions**

```rust
// Use descriptive assertions
assert!(result.is_ok(), "Operation should succeed");
assert_eq!(value, expected, "Value mismatch");

// For Results, consider expect() in tests
let value = result.expect("Should return value");

// Custom assertions (to be added)
// assert_test_config_valid(&config);
// assert_storage_success(result);
// assert_sovereignty_compliant(&component);
```

---

## 📈 **Coverage Targets**

### **By Crate**

Per-crate percentages change with `cargo llvm-cov --workspace --all-features`. Treat **~74% workspace coverage** as the current baseline; raise or maintain bars per crate as features land.

### **By Module Type**

- **Core Logic**: 90%+ coverage required
- **API Handlers**: 90%+ coverage required
- **Storage Operations**: 85%+ coverage required
- **Network Operations**: 85%+ coverage required
- **Utilities**: 80%+ coverage required

---

## 🐛 **Debugging Tests**

### **Test Failures**

```bash
# Run single test with full output
cargo test --workspace --all-features test_name -- --nocapture

# Run with logging
RUST_LOG=debug cargo test --workspace --all-features test_name

# Run in single thread (avoids concurrency issues)
cargo test --workspace --all-features -- --test-threads=1
```

### **Common Issues**

1. **Test Pollution**
   - Tests affecting each other
   - Solution: Ensure proper cleanup, use unique names/IDs

2. **Async Runtime Issues**
   - `#[tokio::test]` not working
   - Solution: Ensure `tokio` dev dependency with `macros` feature

3. **Environment Variables**
   - Tests failing due to env vars
   - Solution: Use `serial_test` crate or mock environment

4. **Resource Leaks**
   - Tests hanging or running out of resources
   - Solution: Add cleanup, use RAII guards

---

## 📚 **Testing Strategy**

### **Test Pyramid**

```
         /\
        /  \  E2E Tests (~5%)
       /    \  - Full workflows
      /------\  - Slow, comprehensive
     /        \ 
    /          \ Integration Tests (~25%)
   /            \ - Service interactions
  /              \ - Medium speed
 /----------------\
/                  \ Unit Tests (~70%)
--------------------  - Fast, focused
                      - Most coverage here
```

### **What to Test**

1. **Unit Tests** - Test individual functions/methods
   - Pure functions
   - Business logic
   - Error conditions
   - Edge cases

2. **Integration Tests** - Test component interactions
   - API endpoints
   - Service coordination
   - Database operations
   - External service mocks

3. **E2E Tests** - Test complete user workflows
   - Critical user journeys
   - Multi-step processes
   - System behavior under load

4. **Chaos Tests** - Test resilience
   - Network failures
   - Resource exhaustion
   - Service crashes
   - Recovery scenarios

---

## 🔧 **Maintenance**

### **Regular Tasks**

- **Daily**: Run test suite before committing
- **Weekly**: Review coverage reports
- **Monthly**: Audit flaky tests
- **Quarterly**: Review test architecture

### **Test Hygiene**

```bash
# Remove unused test code
cargo test --workspace --all-features -- --list | wc -l  # Count tests

# Check for duplicate test names
cargo test --workspace --all-features -- --list | sort | uniq -d

# Find slow tests
cargo test --workspace --all-features -- --show-output | grep "test result"
```

---

## 📖 **Resources**

### **Internal**
- [Disabled / ignored tests](DISABLED_TESTS_REFERENCE.md)
- [Sleep migration patterns](SLEEP_MIGRATION_GUIDE.md)
- Coverage output: use `cargo llvm-cov` (HTML or lcov as needed)

### **External**
- [Rust Testing Book](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Tokio Testing Guide](https://tokio.rs/tokio/topics/testing)
- [proptest](https://docs.rs/proptest/) - Property-based testing
- [rstest](https://docs.rs/rstest/) - Parameterized testing

---

## 🆘 **Getting Help**

- Check this README first
- Look at existing tests for patterns
- Review `tests/common/` for utilities
- See [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines

---

**Status**: Workspace test suite green; ignored tests documented separately  
**Coverage**: ~74.3% workspace (all features); re-measure after large changes

---

*Keep tests simple, focused, and maintainable. Happy testing! 🧪*

