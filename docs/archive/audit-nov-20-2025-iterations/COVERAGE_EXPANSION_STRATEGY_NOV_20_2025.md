# Test Coverage Expansion Strategy
**Date**: November 20, 2025  
**Current Coverage**: 68.89% (line), 66.91% (function)  
**Target**: 90%+ coverage  
**Gap**: Need to cover ~21% more lines (~29,000 lines)

---

## 📊 Current Status

### Overall Metrics (from llvm-cov)
- **Line Coverage**: 68.89% (95,539 / 138,679 lines)
- **Function Coverage**: 66.91% (9,405 / 14,057 functions)
- **Region Coverage**: 67.02% (68,599 / 102,358 regions)

**Note**: This is significantly better than the previously reported 48.65%!

---

## 🎯 Critical Modules with Low Coverage

### Priority 1: Ultra-Low Coverage (<10%)

| Module | Lines | Covered | Coverage | Priority |
|--------|-------|---------|----------|----------|
| **snapshot/scheduler.rs** | 379 | 12 | 3.17% | 🔴 CRITICAL |
| **zero_cost_zfs_operations/manager.rs** | 585 | 17 | 2.91% | 🔴 CRITICAL |
| **native/pool_manager.rs** | 464 | 16 | 3.45% | 🔴 CRITICAL |
| **failover.rs** | 782 | 38 | 4.86% | 🔴 CRITICAL |
| **backup.rs** | 585 | 34 | 5.81% | 🔴 CRITICAL |
| **native/dataset_manager.rs** | 443 | 31 | 7.00% | 🔴 CRITICAL |

**Impact**: These 6 modules alone represent **3,238 lines** with only **148 covered**.  
**Potential Gain**: Covering these to 80% would add **~2,500 lines** = **+1.8% total coverage**

### Priority 2: Low Coverage (10-30%)

| Module | Lines | Covered | Coverage | Priority |
|--------|-------|---------|----------|----------|
| **pool_setup/mod.rs** | 402 | 57 | 14.18% | 🟠 HIGH |
| **dataset/manager.rs** | 430 | 61 | 14.19% | 🟠 HIGH |
| **pool_setup/device_detection.rs** | 305 | 73 | 23.93% | 🟠 HIGH |
| **dataset/creation.rs** | 279 | 69 | 24.73% | 🟠 HIGH |
| **orchestrator_integration.rs** | 997 | 276 | 27.28% | 🟠 HIGH |
| **pool/manager.rs** | 417 | 120 | 28.78% | 🟠 HIGH |

**Impact**: These 6 modules represent **2,830 lines** with only **656 covered**.  
**Potential Gain**: Covering these to 80% would add **~1,600 lines** = **+1.2% total coverage**

### Priority 3: Medium Coverage (30-50%)

| Module | Lines | Covered | Coverage | Action |
|--------|-------|---------|----------|--------|
| **pool_setup/config.rs** | 124 | 86 | 30.65% | 🟡 MEDIUM |
| **pool_setup/creation.rs** | 356 | 126 | 35.39% | 🟡 MEDIUM |
| **tier.rs** | 195 | 71 | 36.41% | 🟡 MEDIUM |
| **production_readiness.rs** | 424 | 167 | 39.39% | 🟡 MEDIUM |
| **error.rs** | 498 | 196 | 39.36% | 🟡 MEDIUM |

**Impact**: These 5 modules represent **1,597 lines** with only **646 covered**.  
**Potential Gain**: Covering these to 80% would add **~630 lines** = **+0.5% total coverage**

---

## 🚀 Expansion Strategy

### Phase 1: Low-Hanging Fruit (Week 1-2)
**Target**: +5% coverage (68.89% → 73.89%)

#### Focus Areas:
1. **Snapshot Scheduler** (`snapshot/scheduler.rs`)
   - Add lifecycle tests (start, stop, pause)
   - Add scheduling logic tests
   - Add error handling tests
   - **Estimated**: 15-20 tests, +3% module coverage

2. **Zero-Cost ZFS Manager** (`zero_cost_zfs_operations/manager.rs`)
   - Add operation tests (create, delete, list)
   - Add performance validation tests
   - Add error path tests
   - **Estimated**: 20-25 tests, +3% module coverage

3. **Native Pool Manager** (`native/pool_manager.rs`)
   - Add pool lifecycle tests
   - Add health check tests
   - Add capacity monitoring tests
   - **Estimated**: 15-20 tests, +3% module coverage

**Deliverable**: ~50-65 new tests, ~2,500 lines covered

### Phase 2: Critical Paths (Week 3-4)
**Target**: +5% coverage (73.89% → 78.89%)

#### Focus Areas:
4. **Failover Logic** (`failover.rs`)
   - Add failover scenario tests
   - Add recovery tests
   - Add state machine tests
   - **Estimated**: 20-25 tests

5. **Pool Setup** (`pool_setup/mod.rs`, `pool_setup/device_detection.rs`)
   - Add device detection tests
   - Add configuration validation tests
   - Add setup workflow tests
   - **Estimated**: 25-30 tests

6. **Dataset Management** (`dataset/manager.rs`, `dataset/creation.rs`)
   - Add CRUD operation tests
   - Add property management tests
   - Add quota/reservation tests
   - **Estimated**: 20-25 tests

**Deliverable**: ~65-80 new tests, ~2,200 lines covered

### Phase 3: Integration & Edge Cases (Week 5-6)
**Target**: +5% coverage (78.89% → 83.89%)

#### Focus Areas:
7. **Orchestrator Integration** (`orchestrator_integration.rs`)
   - Add end-to-end workflow tests
   - Add multi-service coordination tests
   - Add error propagation tests
   - **Estimated**: 25-30 tests

8. **Error Handling** (`error.rs`)
   - Add error conversion tests
   - Add context propagation tests
   - Add recovery strategy tests
   - **Estimated**: 15-20 tests

9. **Production Readiness** (`production_readiness.rs`)
   - Add health check tests
   - Add metrics validation tests
   - Add readiness criteria tests
   - **Estimated**: 15-20 tests

**Deliverable**: ~55-70 new tests, ~1,500 lines covered

### Phase 4: Polish to 90% (Week 7-8)
**Target**: +6% coverage (83.89% → 90%+)

#### Focus Areas:
- Edge cases in medium-coverage modules
- Error paths in high-coverage modules
- Integration scenarios
- Performance edge cases
- Chaos/fault scenarios

**Deliverable**: ~80-100 new tests, ~2,000 lines covered

---

## 📈 Coverage Targets by Phase

| Phase | Timeline | Coverage Target | New Tests | Lines Covered |
|-------|----------|-----------------|-----------|---------------|
| **Current** | - | 68.89% | - | 95,539 |
| **Phase 1** | Week 1-2 | 73.89% | 50-65 | +2,500 |
| **Phase 2** | Week 3-4 | 78.89% | 65-80 | +2,200 |
| **Phase 3** | Week 5-6 | 83.89% | 55-70 | +1,500 |
| **Phase 4** | Week 7-8 | 90%+ | 80-100 | +2,000 |
| **Total** | 8 weeks | **90%+** | **250-315** | **+8,200** |

---

## 🎯 Quick Wins (Today)

### Immediate Actions (Next 2-3 Hours)
Focus on modules with the lowest coverage and highest impact:

1. **Snapshot Scheduler Tests** (30 min)
   - Basic lifecycle: start, stop, pause, resume
   - Expected: +200 lines covered

2. **Zero-Cost Manager Tests** (45 min)
   - Basic operations: create, delete, list, get
   - Expected: +350 lines covered

3. **Native Pool Manager Tests** (45 min)
   - Pool lifecycle: create, destroy, import, export
   - Expected: +300 lines covered

4. **Failover Basic Tests** (30 min)
   - Simple failover scenario
   - Expected: +200 lines covered

**Total**: ~2.5 hours, ~1,050 lines, +0.75% coverage (68.89% → 69.64%)

---

## 🛠️ Implementation Guidelines

### Test Structure
```rust
#[cfg(test)]
mod {module}_tests {
    use super::*;
    use crate::common::env_isolation::IsolatedEnvironment;
    
    #[test]
    fn test_{functionality}_success() {
        // Arrange
        let mut env = IsolatedEnvironment::new("test_name");
        env.set("VAR", "value");
        
        // Act
        let result = do_something();
        
        // Assert
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_{functionality}_error_handling() {
        // Test error paths
    }
}
```

### Coverage Best Practices
1. **Focus on Critical Paths**: Test happy path + major error paths
2. **Test Error Handling**: Every `Result` return should have error tests
3. **Test Edge Cases**: Boundary conditions, empty inputs, null cases
4. **Test State Transitions**: For state machines and lifecycle management
5. **Integration Tests**: For cross-module interactions

### What NOT to Test
- Simple getters/setters (unless they have logic)
- Trivial derive implementations
- Third-party library code
- Dead code / deprecated code marked for removal

---

## 📊 Measurement & Tracking

### Coverage Commands
```bash
# Generate full report
cargo llvm-cov --html --output-dir coverage-latest --workspace

# Quick summary
cargo llvm-cov --workspace --summary-only

# Per-module coverage
cargo llvm-cov --workspace | grep "specific_module"
```

### Success Metrics
- **Coverage %**: Track total line coverage
- **Test Count**: Track number of tests added
- **Module Coverage**: Track per-module improvements
- **Test Reliability**: Maintain 100% pass rate
- **Build Time**: Keep CI/CD times reasonable

---

## 🎯 Priority Matrix

### High Priority (Do First)
- ✅ Modules <10% coverage
- ✅ Critical path code (main workflows)
- ✅ Error handling paths
- ✅ Public API surface

### Medium Priority (Do Second)
- ⏳ Modules 10-50% coverage
- ⏳ Internal utilities
- ⏳ Edge cases
- ⏳ Performance-critical code

### Low Priority (Do Last)
- ⏸️ Modules >80% coverage
- ⏸️ Test utilities
- ⏸️ Deprecated code
- ⏸️ Simple data structures

---

## 🚀 Quick Start (Today)

To immediately boost coverage, run:

```bash
# 1. Review coverage report
firefox coverage-nov-20/html/index.html

# 2. Add tests for snapshot scheduler
# Edit: code/crates/nestgate-zfs/src/snapshot/scheduler.rs

# 3. Add tests for zero-cost manager
# Edit: code/crates/nestgate-zfs/src/zero_cost_zfs_operations/manager.rs

# 4. Run coverage to verify
cargo llvm-cov --workspace

# 5. Commit improvements
git add . && git commit -m "test: expand coverage for scheduler and zero-cost manager"
```

---

## 📚 Resources

### Documentation
- **Modern Testing Guide**: `MODERN_CONCURRENCY_PATTERNS_GUIDE.md`
- **Environment Isolation**: `tests/common/env_isolation.rs`
- **Test Migration**: `docs/session-reports/nov-20-2025/TEST_MIGRATION_SUMMARY_NOV_20_2025.md`

### Examples
- **Practical Tests**: `tests/practical_integration_tests.rs`
- **Unit Tests**: `tests/unit/`
- **Integration Tests**: `tests/integration/`

---

## 🎯 Bottom Line

**Current**: 68.89% coverage (better than expected!)  
**Target**: 90%+ coverage  
**Gap**: ~21% (~29,000 lines)  
**Strategy**: 4 phases over 8 weeks, ~250-315 new tests  
**Quick Win Today**: Add 50-80 tests for ultra-low coverage modules → +0.75-1.5% coverage

**Next Action**: Start with `snapshot/scheduler.rs` tests (easiest, highest impact)

---

*Coverage analysis completed November 20, 2025*

