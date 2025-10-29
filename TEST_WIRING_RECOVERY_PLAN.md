# 🔧 Test Wiring Recovery Plan

**Date**: October 29, 2025  
**Status**: 🚀 READY TO EXECUTE  
**Priority**: 🔥 CRITICAL - Unlock 65-75% Coverage  
**Timeline**: 1-2 weeks

---

## 📊 **PROBLEM SUMMARY**

### **Discovery**
- **5,667 test functions** exist in codebase
- **Only 1,036 tests** are actually running (18%)
- **4,631 tests (82%)** are orphaned - not wired into build system
- **Current coverage**: 18% measured
- **Potential coverage**: 65-75% if all tests run

### **Root Cause**
Test code exists but isn't imported into module tree:
- Test files exist but parent modules don't import them
- `#[cfg(test)]` blocks in files not included in `lib.rs`
- Only 6 lib.rs files actually import test modules

---

## 🎯 **OBJECTIVES**

1. ✅ Wire up all 4,631 orphaned tests
2. ✅ Achieve 65-75% code coverage (from 18%)
3. ✅ Fix compilation errors in reactivated tests
4. ✅ Document test organization for future maintainability
5. ✅ Maintain 100% test pass rate

---

## 📋 **EXECUTION PHASES**

### **Phase 1: Discovery & Audit** ⏱️ 2-3 hours

**Objective**: Identify all orphaned test files and their status

**Tasks**:
1. ✅ Scan for all test files in codebase
2. ✅ Identify which are imported vs orphaned
3. ✅ Generate comprehensive orphan list per crate
4. ✅ Prioritize by crate importance
5. ✅ Create tracking spreadsheet

**Deliverables**:
- `orphaned_tests_inventory.txt` - Complete list
- `crate_test_status.csv` - Per-crate breakdown
- `priority_order.txt` - Execution order

**Success Criteria**:
- Know exact count of orphaned files
- Have clear execution plan per crate

---

### **Phase 2: Core Crates Wiring** ⏱️ 3-4 days

**Objective**: Wire up tests in critical crates first

**Priority Order**:
1. 🔥 **nestgate-core** (518→2,500 tests)
   - Largest impact on coverage
   - Foundation for other crates
   
2. 🔥 **nestgate-api** (105→1,200 tests)
   - Critical for integration testing
   - API contract validation
   
3. 🔥 **nestgate-zfs** (99→600 tests)
   - Core storage functionality
   - Production-critical

4. 🟡 **nestgate-network** (22→250 tests)
   - Network layer validation
   
5. 🟡 **nestgate-performance** (54→300 tests)
   - Benchmark validation

**Process Per Crate**:
```bash
# 1. Find orphaned tests
find code/crates/nestgate-CRATE -name "*test*.rs" -o -name "*tests.rs"

# 2. For each orphaned file:
#    a. Identify parent module
#    b. Add module import
#    c. Fix compilation errors
#    d. Run tests
#    e. Verify pass rate

# 3. Verify crate tests
cargo test --package nestgate-CRATE --lib

# 4. Measure coverage increase
cargo tarpaulin --package nestgate-CRATE
```

**Success Criteria Per Crate**:
- ✅ All test files imported
- ✅ All tests compile
- ✅ >95% test pass rate
- ✅ Coverage increase measured

---

### **Phase 3: Supporting Crates** ⏱️ 2-3 days

**Crates**:
- nestgate-automation (28→150 tests)
- nestgate-mcp (28→200 tests)
- nestgate-fsmonitor (26→100 tests)
- nestgate-nas (34→150 tests)
- nestgate-installer (12→100 tests)
- nestgate-middleware (5→50 tests)
- nestgate-bin (0→100 tests)

**Process**: Same as Phase 2

**Success Criteria**:
- All supporting crates at >90% test activation
- Overall workspace coverage >60%

---

### **Phase 4: Integration & Validation** ⏱️ 1-2 days

**Objective**: Ensure all tests work together

**Tasks**:
1. ✅ Run full workspace test suite
   ```bash
   cargo test --workspace --all-targets
   ```

2. ✅ Generate comprehensive coverage report
   ```bash
   cargo tarpaulin --workspace --out Html --output-dir coverage-reports
   ```

3. ✅ Verify test pass rate >95%

4. ✅ Document test organization
   - Test naming conventions
   - Module structure
   - Import patterns

5. ✅ Update CI/CD to catch orphaned tests

**Success Criteria**:
- ✅ 4,500+ tests running (from 1,036)
- ✅ 65-75% code coverage (from 18%)
- ✅ >95% test pass rate
- ✅ Zero orphaned test files
- ✅ CI/CD validates test imports

---

### **Phase 5: Documentation & Cleanup** ⏱️ 1 day

**Tasks**:
1. ✅ Update ROOT_DOCS_INDEX.md with corrected coverage
2. ✅ Update CURRENT_STATUS.md
3. ✅ Document test organization standards
4. ✅ Create test contribution guidelines
5. ✅ Archive this plan to sessions/

**Deliverables**:
- Updated documentation
- Test organization guide
- CI/CD lint rules for test imports

---

## 🔧 **TECHNICAL APPROACH**

### **Pattern 1: Inline Test Modules**

```rust
// File: foo.rs with inline tests
pub struct Foo { }

#[cfg(test)]
mod tests {
    #[test]
    fn test_foo() { }
}
```

**Fix**: Ensure `foo.rs` is imported in parent `mod.rs` or `lib.rs`

```rust
// In lib.rs or mod.rs
pub mod foo;  // ← This makes the #[cfg(test)] block compile
```

### **Pattern 2: Separate Test Files**

```rust
// File: foo_tests.rs or foo_test.rs
#[test]
fn test_foo() { }
```

**Fix**: Explicitly import test module

```rust
// In lib.rs or mod.rs
#[cfg(test)]
mod foo_tests;  // ← Add this
```

### **Pattern 3: Tests Directory**

```
crate/
├── src/
│   └── lib.rs
└── tests/
    └── integration_tests.rs
```

**Fix**: These should work automatically. If not, check Cargo.toml

```toml
[dev-dependencies]
# Ensure test dependencies are here
```

---

## 📊 **PROGRESS TRACKING**

### **Per-Crate Metrics**

| Crate | Tests Before | Tests After | Coverage Before | Coverage After | Status |
|-------|--------------|-------------|-----------------|----------------|--------|
| nestgate-core | 518 | Target: 2,500 | ~19% | Target: 70% | 🚧 Pending |
| nestgate-api | 105 | Target: 1,200 | ~15% | Target: 65% | 🚧 Pending |
| nestgate-zfs | 99 | Target: 600 | ~17% | Target: 68% | 🚧 Pending |
| nestgate-network | 22 | Target: 250 | ~12% | Target: 60% | 🚧 Pending |
| nestgate-performance | 54 | Target: 300 | ~20% | Target: 65% | 🚧 Pending |
| Others | 238 | Target: 917 | ~15% | Target: 55% | 🚧 Pending |
| **TOTAL** | **1,036** | **5,767** | **18%** | **65-75%** | 🚧 **Pending** |

### **Daily Progress Log**

```
Day 1: Phase 1 Complete
- [ ] Orphan inventory created
- [ ] Priority order established
- [ ] Tracking system set up

Day 2-3: nestgate-core
- [ ] Core tests wired up
- [ ] Compilation errors fixed
- [ ] Tests passing

Day 4-5: nestgate-api & nestgate-zfs
- [ ] API tests wired up
- [ ] ZFS tests wired up
- [ ] Integration verified

Day 6-7: Supporting crates
- [ ] All remaining crates wired
- [ ] Coverage targets met

Day 8-9: Integration & validation
- [ ] Full workspace tests passing
- [ ] Coverage report generated
- [ ] Documentation updated
```

---

## 🚨 **RISK MITIGATION**

### **Risk 1: Tests Won't Compile**

**Likelihood**: HIGH  
**Impact**: MEDIUM  
**Mitigation**:
- Many tests may be outdated
- Fix incrementally, file by file
- Comment out broken tests temporarily
- Track broken tests for later fix

**Contingency**: 
- If >20% of tests fail compilation, create separate "broken_tests" branch
- Fix compilation issues in parallel
- Don't block wiring on compilation fixes

### **Risk 2: Tests Fail When Run**

**Likelihood**: MEDIUM  
**Impact**: LOW  
**Mitigation**:
- Prioritize getting tests to run over passing
- Track failing tests separately
- Fix test logic after wiring complete

**Contingency**:
- Accept 85% pass rate during wiring
- Create issues for each failing test
- Target 95%+ pass rate by end of Phase 4

### **Risk 3: Coverage Lower Than Expected**

**Likelihood**: LOW  
**Impact**: MEDIUM  
**Mitigation**:
- Some tests may test same code
- Some code may be truly uncovered
- Re-measure expectations after Phase 2

**Contingency**:
- If coverage <55% after full wiring, reassess
- May need to write 500-1,000 new tests
- Still better than 1,800+ we thought

---

## ✅ **SUCCESS CRITERIA**

### **Must Have** (Phase 1-4)
- ✅ 4,500+ tests running (4.3x increase)
- ✅ 60%+ code coverage (3.3x increase)
- ✅ >90% test pass rate
- ✅ Zero orphaned test files
- ✅ All crates have tests wired

### **Should Have** (Phase 5)
- ✅ 65-75% code coverage
- ✅ >95% test pass rate
- ✅ Updated documentation
- ✅ CI/CD validation
- ✅ Test organization guide

### **Nice to Have**
- ✅ 5,667 tests running (100% activation)
- ✅ 75%+ code coverage
- ✅ >98% test pass rate
- ✅ Automated orphan detection

---

## 🎯 **POST-COMPLETION ACTIONS**

After achieving 65-75% coverage:

1. **Identify Remaining Gaps** (1-2 days)
   - Generate coverage heat map
   - Find uncovered critical paths
   - Prioritize by risk

2. **Write Targeted Tests** (2-3 weeks)
   - Focus on uncovered hot paths
   - Edge cases and error handling
   - Integration scenarios
   - Target: 85-90% coverage

3. **E2E & Chaos Testing** (2-3 weeks)
   - Expand E2E test suite
   - Implement chaos scenarios
   - Fault injection tests
   - Production readiness validation

4. **Achieve A+ Grade** (Final Goal)
   - 85-90% coverage
   - >98% test pass rate
   - Comprehensive test suite
   - Production-ready validation

---

## 📝 **EXECUTION CHECKLIST**

### **Pre-Execution**
- [x] Plan document created
- [ ] Team review and approval
- [ ] Git branch created: `test-wiring-recovery`
- [ ] Tracking spreadsheet set up
- [ ] Backup current test results

### **Phase 1: Discovery**
- [ ] Run orphan detection script
- [ ] Generate inventory files
- [ ] Create priority list
- [ ] Set up progress tracking

### **Phase 2: Core Crates**
- [ ] nestgate-core wired and tested
- [ ] nestgate-api wired and tested
- [ ] nestgate-zfs wired and tested
- [ ] nestgate-network wired and tested
- [ ] nestgate-performance wired and tested

### **Phase 3: Supporting Crates**
- [ ] All remaining crates wired
- [ ] All tests compiling
- [ ] Pass rate >90%

### **Phase 4: Integration**
- [ ] Full workspace tests passing
- [ ] Coverage report generated
- [ ] Results documented

### **Phase 5: Cleanup**
- [ ] Documentation updated
- [ ] Plan archived
- [ ] PR created and merged

---

## 🚀 **EXECUTION START**

**Assigned**: AI Assistant  
**Start Date**: October 29, 2025  
**Target Completion**: November 12, 2025 (2 weeks)  
**Status**: 🟢 READY TO BEGIN

---

**Next Step**: Execute Phase 1 - Discovery & Audit

