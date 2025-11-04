# 🧪 **NestGate Testing Guide**

**Last Updated**: October 29, 2025  
**Coverage**: 16.61% (2,755/16,588 lines)  
**Goal**: 90%+ coverage with clean, maintainable tests

---

## 📊 **Quick Stats**

```
Total Test Functions:   ~5,000
Passing Tests:          1,180/1,181 (99.8%)
Test Files:             149+
Crates with Tests:      All major crates
```

---

## 🚀 **Quick Start**

### **Running Tests**

```bash
# All library tests (fastest, recommended for dev)
cargo test --workspace --lib

# All tests including integration
cargo test --workspace

# Specific test
cargo test --workspace test_name

# E2E tests (requires setup, slower)
cargo test --workspace --test '*' --ignored

# Chaos tests (manual execution)
cargo test --workspace --test 'chaos*' --ignored

# With verbose output
cargo test --workspace -- --nocapture

# Single-threaded (for debugging)
cargo test --workspace -- --test-threads=1
```

### **Coverage Reports**

```bash
# Generate HTML coverage report
cargo tarpaulin --workspace --lib --out Html --output-dir coverage-reports

# Quick coverage check
cargo tarpaulin --workspace --lib --out Stdout --skip-clean

# Per-crate coverage
scripts/check-coverage.sh  # (if exists)
```

---

## 📁 **Test Organization**

```
tests/
├── common/                # Shared test utilities
│   ├── mod.rs             # Re-exports all helpers
│   ├── mocks.rs           # Mock implementations
│   ├── test_config.rs     # Test configuration
│   ├── test_environment.rs # Test environment setup
│   ├── test_helpers.rs    # Generic test helpers
│   └── test_doubles/      # Test doubles for services
│
├── unit/                  # Pure unit tests
│   ├── config_system_tests.rs
│   ├── core_error_system_tests.rs
│   ├── traits_system_tests.rs
│   └── ... (add tests here by module)
│
├── integration/           # Service integration tests
│   ├── api_integration.rs
│   ├── storage_integration.rs
│   ├── security/          # Security integration
│   └── ... (tests that use multiple components)
│
├── e2e/                   # End-to-end workflows
│   ├── framework/         # E2E test framework
│   ├── workflows/         # User workflow tests
│   └── mod.rs
│
├── chaos/                 # Chaos engineering
│   ├── chaos_testing_framework.rs
│   └── mod.rs
│
├── penetration_testing/   # Security penetration tests
│   ├── attacks.rs
│   ├── scanner.rs
│   └── tests.rs
│
├── performance/           # Performance tests
│   └── mod.rs
│
└── archive/               # Deprecated tests (reference only)
    ├── demos/             # Demo/example tests
    └── duplicates/        # Consolidated duplicates
```

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

| Crate | Current | Target | Priority |
|-------|---------|--------|----------|
| nestgate-core | ~19% | 90% | 🔴 High |
| nestgate-api | ~15% | 90% | 🔴 High |
| nestgate-zfs | ~17% | 85% | 🔴 High |
| nestgate-network | ~20% | 85% | 🟡 Medium |
| nestgate-mcp | ~18% | 80% | 🟡 Medium |

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
cargo test --workspace test_name -- --nocapture

# Run with logging
RUST_LOG=debug cargo test --workspace test_name

# Run in single thread (avoids concurrency issues)
cargo test --workspace -- --test-threads=1
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
cargo test --workspace -- --list | wc -l  # Count tests

# Check for duplicate test names
cargo test --workspace -- --list | sort | uniq -d

# Find slow tests
cargo test --workspace -- --show-output | grep "test result"
```

---

## 📖 **Resources**

### **Internal**
- [Test Modernization Plan](../TEST_MODERNIZATION_PLAN.md)
- [Comprehensive Audit](../COMPREHENSIVE_AUDIT_OCT_29_2025.md)
- Coverage Reports: `coverage-reports/`

### **External**
- [Rust Testing Book](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Tokio Testing Guide](https://tokio.rs/tokio/topics/testing)
- [proptest](https://docs.rs/proptest/) - Property-based testing
- [rstest](https://docs.rs/rstest/) - Parameterized testing

---

## 🎯 **Next Steps**

### **Immediate (This Week)**
1. Add tests to uncovered modules
2. Consolidate duplicate test files
3. Standardize test naming

### **Short Term (This Month)**
1. Reach 25% coverage
2. Create standard test templates
3. Document all test patterns

### **Medium Term (3 Months)**
1. Reach 75% coverage
2. Add comprehensive chaos tests
3. Automate coverage tracking

### **Long Term (4 Months)**
1. Achieve 90%+ coverage
2. < 1% test flakiness
3. < 5min full test suite

---

## 🆘 **Getting Help**

- Check this README first
- Look at existing tests for patterns
- Review `tests/common/` for utilities
- See [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines

---

**Status**: Test infrastructure modernization in progress  
**Coverage Goal**: 90%+ by Week 16  
**Priority**: 🔴 HIGH - Foundation for quality

---

*Keep tests simple, focused, and maintainable. Happy testing! 🧪*

