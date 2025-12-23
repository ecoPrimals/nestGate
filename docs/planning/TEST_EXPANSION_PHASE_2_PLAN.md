# 📝 TEST EXPANSION - PHASE 2 PLAN

**Date**: November 6, 2025 (Evening)  
**Status**: 🚀 **READY TO BEGIN**  
**Current Coverage**: 48.28%  
**Target Coverage**: 90%

---

## 🎯 OBJECTIVE

Write **1,000 high-quality tests** over the next 8-12 weeks to achieve 90% coverage.

---

## 📊 PHASE BREAKDOWN

### **Phase 2.1: Quick Wins** (Weeks 1-2) → 55% Coverage

**Target**: 100-150 tests  
**Focus**: Low-hanging fruit with high impact

#### **Week 1: Configuration Tests** (50-70 tests)
- `Default` implementations for all config structs
- Config builder patterns
- Config validation logic
- Config serialization/deserialization

#### **Week 2: Error Tests** (50-70 tests)
- Error creation helpers
- Error conversion (`From` implementations)
- Error display/debug implementations
- Error context methods

**Expected Gain**: +7% coverage (48% → 55%)

---

### **Phase 2.2: Core Functionality** (Weeks 3-8) → 75% Coverage

**Target**: 500-700 tests  
**Focus**: Core service logic and critical paths

#### **Weeks 3-4: Service Module Tests** (100-150 tests)
- Service initialization
- Service lifecycle methods
- Service discovery logic
- Service registry operations

#### **Weeks 5-6: Universal Adapter Tests** (100-150 tests)
- Adapter routing logic
- Capability discovery
- Dynamic endpoint resolution
- Fallback mechanisms

#### **Weeks 7-8: Storage Tests** (100-150 tests)
- Storage backend operations
- Pool management
- Snapshot handling
- Performance monitoring

**Expected Gain**: +20% coverage (55% → 75%)

---

### **Phase 2.3: Integration & Edge Cases** (Weeks 9-12) → 90% Coverage

**Target**: 300-400 tests  
**Focus**: Integration scenarios and edge cases

#### **Weeks 9-10: Integration Tests** (150-200 tests)
- Cross-module interactions
- E2E workflows
- Error propagation paths
- Concurrency scenarios

#### **Weeks 11-12: Edge Cases** (150-200 tests)
- Boundary conditions
- Error conditions
- Performance stress tests
- Chaos/fault scenarios

**Expected Gain**: +15% coverage (75% → 90%)

---

## 🚀 WEEK 1 ACTION PLAN

### **Starting Now: Configuration Tests**

#### **Day 1: Canonical Config Tests** (10-15 tests)

**Files to Test**:
1. `config/canonical_master/mod.rs`
2. `config/canonical_master/system_config.rs`
3. `config/canonical_master/storage_config.rs`

**Test Categories**:
```rust
// 1. Default implementations
#[test]
fn test_nestgate_canonical_config_default() { ... }

// 2. Builder patterns
#[test]
fn test_config_builder_with_environment() { ... }

// 3. Validation
#[test]
fn test_config_validation_passes() { ... }
#[test]
fn test_config_validation_fails_invalid_port() { ... }

// 4. Serialization
#[test]
fn test_config_serialization_roundtrip() { ... }
```

#### **Day 2: Network Config Tests** (10-15 tests)

**Files to Test**:
1. `config/canonical_master/domains/network/mod.rs`
2. `network/native_async/config.rs`

**Test Categories**:
- Default network settings
- Port validation
- Timeout configuration
- TLS settings

#### **Day 3: Error Config Tests** (10-15 tests)

**Files to Test**:
1. `error/mod.rs`
2. `error/helpers.rs`

**Test Categories**:
- Error creation helpers
- Error conversions
- Error context methods
- Error display formats

#### **Day 4: Storage Config Tests** (10-15 tests)

**Files to Test**:
1. `config/canonical_master/storage_config.rs`
2. `universal_storage/*/config.rs`

**Test Categories**:
- Storage backend configs
- Pool configurations
- Cache settings
- Performance tuning

#### **Day 5: Review & Polish** (10-15 tests)

- Run coverage measurement
- Identify gaps
- Write additional tests
- Verify all tests passing

**Week 1 Goal**: 50-70 tests, +3-5% coverage

---

## 📋 TEST TEMPLATE

### **Standard Test Structure**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_implementation() {
        // Arrange
        let config = MyConfig::default();
        
        // Assert
        assert_eq!(config.some_field, expected_value);
        assert!(config.validate().is_ok());
    }
    
    #[test]
    fn test_builder_pattern() {
        // Arrange & Act
        let config = MyConfig::builder()
            .with_field(value)
            .build()
            .expect("Config build failed");
        
        // Assert
        assert_eq!(config.some_field, value);
    }
    
    #[test]
    fn test_validation_failure() {
        // Arrange
        let mut config = MyConfig::default();
        config.some_field = invalid_value;
        
        // Act
        let result = config.validate();
        
        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected error message"));
    }
}
```

---

## 🎯 SUCCESS CRITERIA

### **Daily Metrics**
- [ ] 10-15 tests written
- [ ] All tests passing
- [ ] Coverage increasing
- [ ] No regressions

### **Weekly Metrics**
- [ ] 50-70 tests written
- [ ] +3-5% coverage gain
- [ ] All existing tests still passing
- [ ] Code review completed

### **Phase Metrics**
- [ ] 100-150 tests written (Weeks 1-2)
- [ ] +7% coverage gain (48% → 55%)
- [ ] No technical debt introduced
- [ ] Documentation updated

---

## 🔧 TOOLS & COMMANDS

### **Run Tests**
```bash
# Run all tests
cargo test --workspace --lib

# Run specific test file
cargo test --lib --package nestgate-core config::tests

# Run with output
cargo test --lib -- --nocapture
```

### **Measure Coverage**
```bash
# Quick summary
cargo llvm-cov --workspace --lib --summary-only

# Generate HTML report
cargo llvm-cov --workspace --lib --html

# View report
firefox target/llvm-cov/html/index.html
```

### **Watch Mode** (Optional)
```bash
# Install cargo-watch if not already installed
cargo install cargo-watch

# Watch and test on changes
cargo watch -x "test --lib"
```

---

## 📈 PROGRESS TRACKING

### **Week 1 Progress**

| Day | Tests Written | Coverage | Status |
|-----|--------------|----------|--------|
| Day 1 | ___ / 15 | ___% | 🔄 |
| Day 2 | ___ / 15 | ___% | ⏳ |
| Day 3 | ___ / 15 | ___% | ⏳ |
| Day 4 | ___ / 15 | ___% | ⏳ |
| Day 5 | ___ / 15 | ___% | ⏳ |
| **Total** | **___ / 70** | **___% → 55%** | ⏳ |

---

## 🎯 NEXT ACTIONS

### **Immediate** (Today)
1. ✅ Review this plan
2. ✅ Set up test workspace
3. 🔄 Begin Day 1: Canonical Config Tests

### **This Week**
1. Write 50-70 configuration tests
2. Measure coverage daily
3. Adjust plan based on results

### **Next Week**
1. Write 50-70 error tests
2. Reach 55% coverage milestone
3. Plan Week 3-4 (Service tests)

---

## 💡 TESTING BEST PRACTICES

### **DO**
- ✅ Test one thing per test
- ✅ Use descriptive test names
- ✅ Follow Arrange-Act-Assert pattern
- ✅ Test both success and failure cases
- ✅ Keep tests simple and focused
- ✅ Use test fixtures for complex setup

### **DON'T**
- ❌ Write mega-tests (test multiple things)
- ❌ Use magic numbers (use constants)
- ❌ Test implementation details
- ❌ Duplicate test logic
- ❌ Ignore test failures
- ❌ Write brittle tests

---

## 🏆 MOTIVATION

### **Where We Are**
- ✅ 48.28% coverage (solid foundation)
- ✅ 1,430 tests passing
- ✅ World-class architecture

### **Where We're Going**
- 🎯 55% coverage (Week 2)
- 🎯 75% coverage (Week 8)
- 🎯 90% coverage (Week 12)
- 🎯 Production ready!

### **Why This Matters**
- 🛡️ **Quality**: Catch bugs before production
- 📊 **Confidence**: Know your code works
- 🚀 **Velocity**: Refactor safely
- 💼 **Professional**: Meet industry standards

---

**Plan Created**: November 6, 2025 (Evening)  
**Status**: 🚀 **READY TO EXECUTE**  
**First Action**: Begin Day 1 - Canonical Config Tests

---

*Let's get to 90%! One test at a time.* 📝

