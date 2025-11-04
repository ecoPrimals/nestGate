# nestgate-zfs Test Coverage Expansion Plan

**Current**: 8.79% (2,575 / 29,283 lines)  
**Target**: 30% (8,785 lines)  
**Gap**: +6,210 lines need test coverage  
**Estimated Time**: 8-10 hours

---

## 📊 Current State

### Files with Good Coverage (Keep)
- ✅ `automation/tests.rs` - 100%
- ✅ `pool/tests.rs` - 100%  
- ✅ `snapshot/tests.rs` - 90.27%
- ✅ `snapshot/policy.rs` - 100%
- ✅ `zero_cost_zfs_operations/utilities.rs` - 93.39%
- ✅ `zero_cost_zfs_operations/traits.rs` - 100%

### Files with 0% Coverage (Priority Targets)

**High Priority** (Core functionality):
- `dataset.rs` - 491 lines, 0% - **CRITICAL**
- `health.rs` - 278 lines, 0% - **IMPORTANT**
- `snapshot/manager.rs` - 438 lines, 0% - **CRITICAL**
- `snapshot/scheduler.rs` - 379 lines, 0% - **IMPORTANT**
- `native/pool_manager.rs` - 251 lines, 0%
- `native/dataset_manager.rs` - 265 lines, 0%
- `pool/manager.rs` - 417 lines, 23.50% - **NEEDS MORE**

**Medium Priority** (Advanced features):
- `advanced_features/cache.rs` - 37 lines, 0%
- `advanced_features/capacity.rs` - 139 lines, 0%
- `advanced_features/compression.rs` - 39 lines, 0%
- `advanced_features/snapshots.rs` - 27 lines, 0%
- `automation/actions.rs` - 229 lines, 0%
- `automation/engine.rs` - 251 lines, 0%
- `byob.rs` - 259 lines, 0%

**Low Priority** (Integration/Complex):
- `performance/*` modules - Various, mostly 0%
- `mcp_integration.rs` - 212 lines, 0%
- `orchestrator_integration.rs` - 235 lines, 0%

---

## 🎯 Test Strategy

### Phase 1: Core Functionality (Target: +15% coverage)
**Time**: 3-4 hours

1. **`dataset.rs` Tests** (~100 lines of tests)
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;
       
       #[test]
       fn test_dataset_creation() {
           // Test ZfsDatasetManager::new()
       }
       
       #[test]
       fn test_dataset_properties() {
           // Test property setting/getting
       }
       
       #[test]
       fn test_dataset_mounting() {
           // Test mount operations
       }
   }
   ```

2. **`health.rs` Tests** (~80 lines of tests)
   - Health check operations
   - Status monitoring
   - Alert conditions

3. **`snapshot/manager.rs` Tests** (~120 lines of tests)
   - Snapshot creation
   - Snapshot listing
   - Snapshot deletion
   - Rollback operations

### Phase 2: Advanced Features (Target: +10% coverage)
**Time**: 2-3 hours

4. **`advanced_features/cache.rs`** (~30 lines)
5. **`advanced_features/capacity.rs`** (~40 lines)
6. **`advanced_features/compression.rs`** (~30 lines)
7. **`automation/actions.rs`** (~60 lines)

### Phase 3: Native Operations (Target: +5% coverage)
**Time**: 2-3 hours

8. **`native/pool_manager.rs`** (~80 lines)
9. **`native/dataset_manager.rs`** (~80 lines)

---

## 📝 Test Template

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    fn setup_test_env() -> TempDir {
        TempDir::new().expect("Test setup failed")
    }
    
    #[test]
    fn test_basic_functionality() {
        let _temp = setup_test_env();
        // Test implementation
        assert!(true);
    }
    
    #[test]
    fn test_error_handling() {
        // Test error paths
        assert!(true);
    }
    
    #[test]
    fn test_edge_cases() {
        // Test boundaries
        assert!(true);
    }
}
```

---

## 🔧 Implementation Steps

### Step 1: Setup Test Infrastructure
```bash
cd /home/eastgate/Development/ecoPrimals/nestgate

# Ensure test dependencies
cargo add --dev tempfile
cargo add --dev mockall
```

### Step 2: Add Tests Module by Module
```bash
# For each module, add #[cfg(test)] section at end of file
# Example for dataset.rs:

# 1. Read current file
# 2. Add test module if not exists
# 3. Write basic tests for main functionality
# 4. Run: cargo test --package nestgate-zfs --lib
# 5. Check coverage: cargo llvm-cov --package nestgate-zfs --lib
```

### Step 3: Iterate Until 30%
```bash
# Keep adding tests until target reached
while [ $(cargo llvm-cov --package nestgate-zfs --lib 2>&1 | grep "TOTAL" | awk '{print $3}') < 30.00 ]; do
    # Add more tests
    cargo test --package nestgate-zfs --lib
done
```

---

## 📊 Expected Results

### After Phase 1 (3-4 hours)
- Coverage: 8.79% → ~24%
- Tests added: ~300 lines
- Files improved: 3 critical files

### After Phase 2 (2-3 hours)  
- Coverage: 24% → ~29%
- Tests added: ~160 lines
- Files improved: 4 advanced feature files

### After Phase 3 (2-3 hours)
- Coverage: 29% → ~32%
- Tests added: ~160 lines
- Files improved: 2 native operation files

### Total
- **Final Coverage**: 32% (exceeds 30% target!)
- **Total Test Lines**: ~620 lines added
- **Total Time**: 8-10 hours
- **Files Improved**: 9 major files

---

## 🚨 Challenges

1. **ZFS Dependencies**: Tests need mock ZFS commands or require actual ZFS
2. **Complex State**: Many operations involve system state
3. **Integration Points**: Some modules depend on external systems
4. **Time Investment**: 8-10 hours is significant

---

## 💡 Recommendations

### Quick Win Approach (2-3 hours, 20% coverage)
Focus ONLY on:
1. `dataset.rs` - Core functionality
2. `health.rs` - Production critical  
3. `snapshot/manager.rs` - User-facing feature

### Comprehensive Approach (8-10 hours, 30%+ coverage)
Follow full plan above

### Pragmatic Approach (4-5 hours, 25% coverage)
- Phase 1 only (core functionality)
- Skip advanced features
- Focus on most-used code paths

---

## 📚 Resources

- **Existing Tests**: `code/crates/nestgate-zfs/src/pool/tests.rs` (good example)
- **Test Helpers**: `dev_environment/` modules (mock infrastructure)
- **Coverage Tool**: `cargo llvm-cov --package nestgate-zfs --lib --html --open`

---

## ✅ Success Criteria

- [ ] Coverage ≥ 30%
- [ ] All new tests pass
- [ ] No test flakiness
- [ ] Build remains clean
- [ ] Documentation updated

---

**Status**: PLAN READY  
**Next**: Begin Phase 1 (dataset.rs tests)  
**Estimated Start-to-Finish**: 8-10 hours

---

*Note: This is a major undertaking. Consider breaking into multiple sessions.*

