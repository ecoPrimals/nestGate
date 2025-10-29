# 🧪 **E2E TEST RESTORATION PLAN**
## NestGate - End-to-End Testing Strategy

**Created**: October 26, 2025 (Evening)  
**Status**: Ready for Implementation  
**Priority**: HIGH (Critical for production readiness)

---

## 📊 **CURRENT SITUATION**

### **Disabled Test Inventory**:
```
Total Disabled Tests:     11 test files
Total Disabled Examples:  3 example files
Total Test Directories:   3 disabled directories

BREAKDOWN:
- Integration tests:      1 file  (nestgate-bin)
- Network tests:          2 files (nestgate-network)
- ZFS tests:              4 files (nestgate-zfs)
- API tests:              3 files (nestgate-api)
- Core benchmarks:        1 file  (nestgate-core)
```

### **Disabled Files Found**:

#### **Integration Tests**:
1. `code/crates/nestgate-bin/tests-disabled-oct-20-2025/integration_tests.rs`
   - **Lines**: 424 lines
   - **Purpose**: Binary integration tests
   - **Status**: Deprecated hardcoded localhost patterns
   - **Priority**: 🔴 HIGH

#### **Network Tests**:
2. `code/crates/nestgate-network/tests/connection_manager_tests.rs.disabled`
3. `code/crates/nestgate-network/tests/types_tests.rs.disabled`
   - **Purpose**: Network layer testing
   - **Priority**: 🔴 HIGH

#### **ZFS Tests**:
4. `code/crates/nestgate-zfs/tests/pool_tests.rs.disabled`
5. `code/crates/nestgate-zfs/tests/performance_comprehensive_tests.rs.disabled`
6. `code/crates/nestgate-zfs/tests/unit_tests.rs.disabled`
7. `code/crates/nestgate-zfs/tests/basic_functionality_tests.rs.disabled`
   - **Purpose**: ZFS operations testing
   - **Priority**: 🟡 MEDIUM-HIGH

#### **API Tests**:
8. `code/crates/nestgate-api/tests/zfs_api_tests.rs.disabled`
9. `code/crates/nestgate-api/tests/hardware_tuning_test_helpers.rs.disabled`
10. `code/crates/nestgate-api/tests/hardware_tuning_handlers_tests.rs.disabled`
    - **Purpose**: API endpoint testing
    - **Priority**: 🔴 HIGH

#### **Performance Benchmarks**:
11. `code/crates/nestgate-core/benches/unified_performance_validation.rs.disabled`
12. `code/crates/nestgate-zfs/benches/performance_benchmarks.rs.disabled`
    - **Purpose**: Performance validation
    - **Priority**: 🟢 MEDIUM

#### **Example Code** (Disabled):
- `code/crates/nestgate-api/examples-disabled-oct-20-2025/`
  - `dev_server.rs`
  - `modern_nestgate_demo.rs`
  - `rpc_demo.rs`

- `code/crates/nestgate-zfs/examples-disabled-oct-20-2025/`
  - `pool_setup_demo.rs`
  - `production_config.toml`

---

## 🎯 **RESTORATION STRATEGY**

### **Phase 1: Foundation** (Week 1 - 2-3 days)
**Goal**: Restore basic integration test infrastructure

#### **Tasks**:
1. **Analyze disabled test code**:
   - Read all 11 disabled test files
   - Document why each was disabled
   - Identify common patterns and blockers

2. **Fix hardcoded localhost patterns**:
   - Update to environment-driven endpoint resolution
   - Implement pattern: `resolve_service_endpoint("api").await`
   - Replace all `"http://localhost:8080"` patterns

3. **Update imports and dependencies**:
   - Fix unresolved module imports
   - Update to current API structure
   - Fix deprecated dependencies

#### **Expected Output**:
- ✅ Analysis document of all disabled tests
- ✅ Common blocker patterns identified
- ✅ Localhost resolution strategy implemented

---

### **Phase 2: Priority Tests** (Week 1-2 - 5-7 days)
**Goal**: Restore critical integration and E2E tests

#### **Priority Order**:

##### **🔴 CRITICAL (Restore First)**:
1. **`integration_tests.rs`** (nestgate-bin)
   - **Why**: Tests complete binary functionality
   - **Effort**: 4-6 hours
   - **Blockers**: Hardcoded localhost (424 lines to review)
   - **Tests**: Binary help, service lifecycle, config handling

2. **`zfs_api_tests.rs`** (nestgate-api)
   - **Why**: Tests critical ZFS API endpoints
   - **Effort**: 3-4 hours
   - **Blockers**: API evolution, import changes
   - **Tests**: ZFS operations, pool management, snapshot handling

3. **`connection_manager_tests.rs`** (nestgate-network)
   - **Why**: Tests network layer stability
   - **Effort**: 2-3 hours
   - **Blockers**: Module reorganization
   - **Tests**: Connection pooling, retry logic, timeouts

##### **🟡 HIGH (Restore Second)**:
4. **`types_tests.rs`** (nestgate-network)
   - **Why**: Tests type definitions and serialization
   - **Effort**: 2 hours
   - **Tests**: Network types, enum variants, validation

5. **`hardware_tuning_handlers_tests.rs`** (nestgate-api)
   - **Why**: Tests hardware auto-configuration
   - **Effort**: 3-4 hours
   - **Tests**: Hardware detection, optimization, recommendations

##### **🟢 MEDIUM (Restore Third)**:
6-8. **ZFS test suite** (4 files)
   - `pool_tests.rs`
   - `basic_functionality_tests.rs`
   - `unit_tests.rs`
   - `performance_comprehensive_tests.rs`
   - **Why**: Tests ZFS backend operations
   - **Effort**: 6-8 hours total
   - **Tests**: Pool operations, snapshots, performance

#### **Expected Output**:
- ✅ 3-5 critical tests restored and passing
- ✅ Integration test framework validated
- ✅ Clear patterns established for remaining tests

---

### **Phase 3: Comprehensive Coverage** (Week 2-3 - 5-7 days)
**Goal**: Restore all remaining tests and examples

#### **Tasks**:
1. Restore remaining ZFS tests (4 files)
2. Restore performance benchmarks (2 files)
3. Restore example code (5 files)
4. Create new E2E scenarios for uncovered workflows

#### **New E2E Scenarios to Create** (20-30 tests):
- **User Workflows**:
  - Complete pool creation and management workflow
  - Snapshot lifecycle (create, clone, rollback, delete)
  - Dataset operations end-to-end
  - Backup and restore workflows

- **System Integration**:
  - API → ZFS → Storage complete flow
  - Multi-user concurrent operations
  - Error recovery and resilience
  - Configuration changes and reloading

- **Performance Scenarios**:
  - Load testing under various conditions
  - Concurrent request handling
  - Large-scale operations
  - Resource constraint scenarios

#### **Expected Output**:
- ✅ All 11 disabled tests restored
- ✅ 20-30 new E2E scenarios created
- ✅ 100% of critical workflows covered
- ✅ Example code updated and working

---

### **Phase 4: Chaos & Fault Testing** (Week 3-4 - 5-7 days)
**Goal**: Implement chaos engineering and fault injection tests

#### **Chaos Test Scenarios** (40-60 tests):

##### **Network Chaos**:
- Packet loss simulation (1%, 5%, 10%, 50%)
- Network latency injection (10ms, 100ms, 1s, 5s)
- Connection timeouts
- DNS failures
- Firewall rules

##### **Resource Chaos**:
- CPU stress (50%, 75%, 90%, 100%)
- Memory pressure (low memory conditions)
- Disk I/O saturation
- File descriptor exhaustion
- Thread pool exhaustion

##### **Service Chaos**:
- Service crashes and restarts
- Partial service availability
- Dependency failures
- Configuration corruption
- State inconsistencies

##### **Storage Chaos**:
- Disk full conditions
- ZFS pool degradation
- Snapshot failures
- Replication errors
- Data corruption scenarios

#### **Fault Injection Tests** (40-60 tests):
- Database connection failures
- API endpoint failures
- Invalid request handling
- Malformed data processing
- Authentication failures
- Authorization edge cases
- Rate limiting scenarios
- Circuit breaker triggering

#### **Expected Output**:
- ✅ 40-60 chaos tests implemented
- ✅ 40-60 fault injection tests implemented
- ✅ Resilience validated
- ✅ Recovery mechanisms tested

---

## 📋 **DETAILED RESTORATION STEPS**

### **Step 1: Analyze Disabled Test**
```bash
# For each disabled test file:
1. Read the test file
2. Identify compilation errors
3. Document hardcoded values
4. List deprecated imports
5. Note API changes needed
```

### **Step 2: Fix Common Blockers**
```rust
// Pattern 1: Fix hardcoded localhost
// BEFORE:
let url = "http://localhost:8080";

// AFTER:
use crate::config::network_defaults;
let port = std::env::var("NESTGATE_API_PORT")
    .ok()
    .and_then(|p| p.parse::<u16>().ok())
    .unwrap_or_else(network_defaults::api_port);
let url = format!("http://localhost:{}", port);
```

```rust
// Pattern 2: Fix deprecated imports
// BEFORE:
use crate::old_module::Type;

// AFTER:
use crate::new_module::Type;
```

### **Step 3: Update Test to Current API**
```rust
// Update test assertions to match current types
// Update test setup to use current factories
// Update mocks to current interfaces
```

### **Step 4: Enable and Verify**
```bash
# Rename .disabled to .rs
mv test_file.rs.disabled test_file.rs

# Run the test
cargo test --test test_file

# Fix any remaining issues
# Iterate until passing
```

---

## 🎯 **SUCCESS METRICS**

### **Week 1 Targets**:
```
Tests Analyzed:      11/11 (100%)
Priority Tests Restored: 3-5 tests
Pass Rate:           100%
Documentation:       Analysis complete
```

### **Week 2 Targets**:
```
Total Tests Restored:    6-8 tests
Integration Coverage:    50-70%
Pass Rate:              100%
New E2E Scenarios:      10-15 tests
```

### **Week 3 Targets**:
```
Total Tests Restored:    11/11 (100%)
New E2E Scenarios:      20-30 tests
Chaos Tests:            20-30 tests
Coverage:               80-90% of workflows
```

### **Week 4 Targets**:
```
Chaos Tests:            40-60 tests
Fault Tests:            40-60 tests
Total E2E Coverage:     100% critical workflows
Production Ready:       ✅
```

---

## 📊 **EFFORT ESTIMATION**

### **Time Breakdown**:
```
Phase 1 (Analysis):          2-3 days  (16-24 hours)
Phase 2 (Priority Restore):  5-7 days  (40-56 hours)
Phase 3 (Complete Restore):  5-7 days  (40-56 hours)
Phase 4 (Chaos/Fault):       5-7 days  (40-56 hours)

TOTAL:                       17-24 days (136-192 hours)
CALENDAR TIME:               3-4 weeks (with focused effort)
```

### **Confidence**: ⭐⭐⭐⭐ HIGH

---

## 🚨 **BLOCKERS & RISKS**

### **Known Blockers**:
1. ✅ **Hardcoded localhost patterns** - Solution documented
2. ✅ **Deprecated imports** - Requires API updates
3. ✅ **API evolution** - Tests need alignment with current API
4. ⚠️ **ZFS availability** - May need mock/stub on non-ZFS systems

### **Mitigation Strategies**:
1. Implement environment-driven endpoint resolution
2. Update all imports to current modules
3. Create adapter layer for API changes
4. Implement dual-mode testing (real ZFS + mocks)

### **Risk Assessment**:
- **Technical Risk**: LOW (patterns clear, solutions known)
- **Time Risk**: MEDIUM (dependent on API stability)
- **Complexity Risk**: LOW (well-structured existing tests)

---

## 🎯 **IMMEDIATE NEXT STEPS**

### **This Week** (Start Phase 1):
1. ✅ Create this restoration plan
2. Read and analyze `integration_tests.rs` (first file)
3. Document all hardcoded values found
4. Create localhost resolution utility
5. Fix imports in first test file
6. Attempt restoration of first test

### **Next Week** (Complete Phase 1 + Start Phase 2):
1. Complete analysis of all 11 disabled tests
2. Implement common fixes across all tests
3. Restore 3-5 priority tests
4. Validate test infrastructure works
5. Document restoration patterns

---

## 📚 **RESOURCES**

### **Documentation**:
- Current API structure: `code/crates/nestgate-api/src/`
- Network defaults: `code/crates/nestgate-core/src/config/network_defaults.rs`
- Test patterns: `tests/working_integration_*.rs`

### **Similar Working Tests**:
- `tests/working_integration_basic.rs` - Pattern for basic tests
- `tests/working_integration_api_patterns.rs` - API testing patterns
- `tests/working_integration_comprehensive.rs` - Complex scenarios

### **Migration Guides**:
- `HARDCODED_PORT_MIGRATION_PLAN_STRATEGIC.md` - Port migration patterns
- `UNWRAP_MIGRATION_PLAN_STRATEGIC.md` - Error handling patterns

---

## ✅ **COMPLETION CRITERIA**

The E2E test restoration is complete when:
- [x] All 11 disabled tests analyzed
- [ ] All 11 disabled tests restored and passing
- [ ] 20-30 new E2E scenarios created
- [ ] 40-60 chaos tests implemented
- [ ] 40-60 fault injection tests implemented
- [ ] 100% of critical workflows covered
- [ ] Documentation updated
- [ ] CI/CD pipeline includes E2E tests
- [ ] Production readiness validated

---

**Status**: 📋 **PLAN COMPLETE**  
**Priority**: 🔴 **HIGH** (Critical for production)  
**Timeline**: 3-4 weeks (focused effort)  
**Confidence**: ⭐⭐⭐⭐ HIGH

*E2E testing is crucial for production confidence. This plan provides a systematic approach to restoration and expansion.*

---

**Created**: October 26, 2025  
**Author**: Comprehensive Audit & Planning Session  
**Next Review**: After Phase 1 completion

