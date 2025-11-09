# Phase 1 Unification Action Plan - Error System

**Timeline**: Weeks 1-2  
**Goal**: Establish single canonical error system  
**Status**: 🔴 NOT STARTED  
**Priority**: 🚨 CRITICAL

---

## 📊 CURRENT STATE ANALYSIS

### Fragmented Result Types (9 Total)

#### 1. **PRIMARY AUTHORITY** (Keep)
```
File: code/crates/nestgate-core/src/error/mod.rs
Type: pub type Result<T> = std::result::Result<T, NestGateError>
Status: ✅ CANONICAL - This is THE authority
Action: KEEP and use everywhere
```

#### 2-5. **DOMAIN-SPECIFIC** (Migrate)
```
File: code/crates/nestgate-core/src/universal_storage/compression/mod.rs:55
Type: pub type Result<T> = std::result::Result<T, CompressionError>
Action: REMOVE - Use NestGateError::Storage variant instead

File: code/crates/nestgate-core/src/universal_storage/snapshots/mod.rs:74
Type: pub type Result<T> = std::result::Result<T, SnapshotError>
Action: REMOVE - Use NestGateError::Storage variant instead

File: code/crates/nestgate-core/src/universal_storage/filesystem_backend/mod.rs:111
Type: pub type Result<T> = std::result::Result<T, FilesystemError>
Action: REMOVE - Use NestGateError::Storage variant instead

File: code/crates/nestgate-core/src/universal_storage/checksums/mod.rs:56
Type: pub type Result<T> = std::result::Result<T, ChecksumError>
Action: REMOVE - Use NestGateError::Storage variant instead
```

#### 6-7. **REDUNDANT** (Merge)
```
File: code/crates/nestgate-core/src/error/unified_result_system.rs:39
Type: pub type Result<T, E = NestGateError> = std::result::Result<T, E>
Action: MERGE into mod.rs or REMOVE (redundant with mod.rs)

File: code/crates/nestgate-core/src/error/idiomatic/mod.rs:33
Type: pub type Result<T> = IdioResult<T>
Action: EVALUATE - May be specialized wrapper, merge or remove
```

#### 8-9. **RE-EXPORTS** (Update)
```
File: code/crates/nestgate-canonical/src/error.rs:102
Type: pub type Result<T, E = NestGateError> = std::result::Result<T, E>
Action: UPDATE to re-export from nestgate-core::error::Result

File: code/crates/nestgate-bin/src/error.rs:71
Type: pub type Result<T> = nestgate_core::error::Result<T>
Action: ✅ GOOD - Already re-exporting, keep this pattern
```

---

## 🎯 MIGRATION TASKS

### Task 1: Update Domain-Specific Error Files

#### 1.1 Universal Storage - Compression Module
**File**: `code/crates/nestgate-core/src/universal_storage/compression/mod.rs`

**Current** (Lines 1-60):
```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompressionError {
    #[error("Compression failed: {0}")]
    CompressionFailed(String),
    // ... more variants
}

pub type Result<T> = std::result::Result<T, CompressionError>;
```

**Target**:
```rust
use crate::error::{Result, NestGateError, StorageErrorDetails};

// Remove local Result type alias
// Use NestGateError::Storage variant for all errors

impl From<CompressionError> for NestGateError {
    fn from(err: CompressionError) -> Self {
        NestGateError::Storage(StorageErrorDetails {
            message: format!("Compression error: {}", err),
            operation: "compress".to_string(),
            // ... map fields
        })
    }
}
```

**Migration Steps**:
1. Add `use crate::error::{Result, NestGateError};` at top
2. Remove `pub type Result<T> = std::result::Result<T, CompressionError>;`
3. Update all function signatures to use `crate::error::Result<T>`
4. Add `From<CompressionError>` impl for NestGateError
5. Update callers to handle NestGateError instead

**Estimated Impact**: ~50 lines to update in compression module + ~20 call sites

---

#### 1.2 Universal Storage - Snapshots Module
**File**: `code/crates/nestgate-core/src/universal_storage/snapshots/mod.rs`

**Current** (Line 74):
```rust
pub type Result<T> = std::result::Result<T, SnapshotError>;
```

**Target**:
```rust
// Remove Result type alias
// Use crate::error::Result<T> everywhere
// Add From<SnapshotError> for NestGateError conversion
```

**Migration Steps**:
1. Remove Result type alias on line 74
2. Add `use crate::error::{Result, NestGateError};`
3. Update all function signatures
4. Add error conversion impl
5. Update ~30 call sites

---

#### 1.3 Universal Storage - Filesystem Backend
**File**: `code/crates/nestgate-core/src/universal_storage/filesystem_backend/mod.rs`

**Current** (Line 111):
```rust
pub type Result<T> = std::result::Result<T, FilesystemError>;
```

**Target**: Same pattern as above

**Migration Steps**: Same as 1.2

**Estimated Impact**: ~40 call sites

---

#### 1.4 Universal Storage - Checksums Module
**File**: `code/crates/nestgate-core/src/universal_storage/checksums/mod.rs`

**Current** (Line 56):
```rust
pub type Result<T> = std::result::Result<T, ChecksumError>;
```

**Target**: Same pattern as above

**Migration Steps**: Same as 1.2

**Estimated Impact**: ~25 call sites

---

### Task 2: Consolidate Error System Core

#### 2.1 Merge or Remove unified_result_system.rs
**File**: `code/crates/nestgate-core/src/error/unified_result_system.rs`

**Analysis**:
```rust
// Line 39: Redundant with mod.rs
pub type Result<T, E = NestGateError> = std::result::Result<T, E>;

// Lines 10-38: Domain-specific result types
pub type StorageResult<T> = std::result::Result<T, StorageError>;
pub type ZfsResult<T> = std::result::Result<T, ZfsError>;
pub type NetworkResult<T> = std::result::Result<T, NetworkError>;
// ... etc
```

**Decision Options**:
1. **KEEP** if domain-specific results are useful for type clarity
2. **MERGE** into mod.rs if they should be in main error module
3. **REMOVE** if not being used

**Recommended Action**: AUDIT USAGE FIRST
```bash
# Check if domain-specific result types are used
grep -r "StorageResult\|ZfsResult\|NetworkResult" code/crates --include="*.rs" | wc -l
```

**If used**: Keep but document as specialized wrappers  
**If not used**: Remove and use canonical `Result<T>`

---

#### 2.2 Evaluate Idiomatic Error System
**File**: `code/crates/nestgate-core/src/error/idiomatic/mod.rs`

**Current** (Line 33):
```rust
pub type Result<T> = IdioResult<T>;
```

**Analysis Needed**:
1. What is `IdioResult<T>`? (Check definition)
2. Is it used outside error/idiomatic/?
3. Can it be merged into main error system?

**Action**: 
```bash
# Check usage
grep -r "IdioResult\|error::idiomatic::Result" code/crates --include="*.rs"
```

**If specialized**: Keep as opt-in alternative  
**If unused**: Remove entire idiomatic submodule  
**If widely used**: Evaluate merge into main system

---

### Task 3: Update Re-exports

#### 3.1 Canonical Crate
**File**: `code/crates/nestgate-canonical/src/error.rs`

**Current** (Line 102):
```rust
pub type Result<T, E = NestGateError> = std::result::Result<T, E>;
```

**Target**:
```rust
// Re-export from core instead of redefining
pub use nestgate_core::error::{Result, NestGateError};
```

**Benefit**: Single source of truth, no redefinition

---

#### 3.2 Binary Crate (Already Good!)
**File**: `code/crates/nestgate-bin/src/error.rs` (Line 71)

**Current**:
```rust
pub type Result<T> = nestgate_core::error::Result<T>;
```

**Status**: ✅ Perfect pattern - this is how all crates should re-export

---

### Task 4: Update Import Statements

**Estimated Scope**: ~500+ import statements to update

#### Automated Migration Script
```bash
#!/bin/bash
# scripts/migrate_result_imports.sh

echo "🔄 Migrating Result type imports to canonical..."

# Replace universal_storage Result imports
find code/crates -name "*.rs" -type f -exec sed -i \
    's|use crate::universal_storage::compression::Result|use crate::error::Result|g' {} \;

find code/crates -name "*.rs" -type f -exec sed -i \
    's|use crate::universal_storage::snapshots::Result|use crate::error::Result|g' {} \;

find code/crates -name "*.rs" -type f -exec sed -i \
    's|use crate::universal_storage::filesystem_backend::Result|use crate::error::Result|g' {} \;

find code/crates -name "*.rs" -type f -exec sed -i \
    's|use crate::universal_storage::checksums::Result|use crate::error::Result|g' {} \;

# Verify no old imports remain
echo "✅ Checking for remaining fragmented imports..."
grep -r "use.*universal_storage.*::Result" code/crates --include="*.rs" || echo "✅ Clean!"

echo "✅ Result import migration complete!"
```

---

## 🧪 VALIDATION & TESTING

### Step 1: Pre-Migration Validation
```bash
# 1. Ensure build is clean
cargo check --workspace

# 2. Ensure tests pass
cargo test --workspace

# 3. Baseline test coverage
cargo llvm-cov --lib --workspace --summary-only > baseline_coverage.txt

# 4. Document current Result usage
grep -r "pub type Result" code/crates --include="*.rs" > baseline_results.txt
```

### Step 2: Post-Migration Validation
```bash
# 1. Build should still be clean
cargo check --workspace || echo "❌ Build broken, rollback!"

# 2. Tests should still pass
cargo test --workspace || echo "❌ Tests broken, rollback!"

# 3. Coverage should not decrease
cargo llvm-cov --lib --workspace --summary-only > post_migration_coverage.txt
diff baseline_coverage.txt post_migration_coverage.txt

# 4. Verify single Result definition
RESULT_COUNT=$(grep -r "pub type Result" code/crates --include="*.rs" | wc -l)
if [ $RESULT_COUNT -eq 1 ]; then
    echo "✅ SUCCESS: Only 1 Result type definition found!"
else
    echo "⚠️  WARNING: Found $RESULT_COUNT Result type definitions"
fi
```

### Step 3: Comprehensive Testing
```bash
# Run full test suite
cargo test --workspace --all-features

# Run clippy to catch issues
cargo clippy --workspace --all-targets

# Check documentation builds
cargo doc --workspace --no-deps

# Run benchmarks to verify no performance regression
cargo bench
```

---

## 📋 MIGRATION CHECKLIST

### Week 1: Domain Error Migration

- [ ] **Day 1**: Analysis & Planning
  - [ ] Run usage audits on all Result types
  - [ ] Create detailed migration order
  - [ ] Set up rollback branch
  - [ ] Document all current usages

- [ ] **Day 2**: Compression Module
  - [ ] Remove Result type alias
  - [ ] Add error conversion impl
  - [ ] Update all function signatures
  - [ ] Update ~50 call sites
  - [ ] Run tests

- [ ] **Day 3**: Snapshots Module
  - [ ] Same steps as Day 2
  - [ ] Update ~30 call sites
  - [ ] Run tests

- [ ] **Day 4**: Filesystem Backend Module
  - [ ] Same steps as Day 2
  - [ ] Update ~40 call sites
  - [ ] Run tests

- [ ] **Day 5**: Checksums Module
  - [ ] Same steps as Day 2
  - [ ] Update ~25 call sites
  - [ ] Run tests
  - [ ] Comprehensive week 1 testing

### Week 2: Core Consolidation

- [ ] **Day 1**: unified_result_system.rs
  - [ ] Audit usage of domain-specific results
  - [ ] Decide: Keep/Merge/Remove
  - [ ] Implement decision
  - [ ] Update documentation

- [ ] **Day 2**: Idiomatic Module
  - [ ] Audit IdioResult usage
  - [ ] Decide: Keep/Merge/Remove
  - [ ] Implement decision
  - [ ] Update documentation

- [ ] **Day 3**: Re-export Updates
  - [ ] Update nestgate-canonical
  - [ ] Verify nestgate-bin pattern
  - [ ] Update other crates if needed
  - [ ] Run cross-crate tests

- [ ] **Day 4**: Import Statement Migration
  - [ ] Run automated migration script
  - [ ] Manual review of changes
  - [ ] Fix any script misses
  - [ ] Comprehensive testing

- [ ] **Day 5**: Final Validation
  - [ ] Complete test suite
  - [ ] Performance benchmarks
  - [ ] Documentation updates
  - [ ] Code review
  - [ ] Merge to main

---

## 🎯 SUCCESS CRITERIA

### Must Have (Blocking)
- ✅ All tests passing (248/248 → 248/248)
- ✅ Zero compilation errors
- ✅ Only 1 canonical Result type definition
- ✅ Zero clippy errors introduced
- ✅ Test coverage maintained or improved

### Should Have (Important)
- ✅ All 500+ import statements updated
- ✅ All domain errors converted to NestGateError
- ✅ Documentation updated
- ✅ Migration guide created

### Nice to Have (Optional)
- ✅ Performance benchmarks show no regression
- ✅ Code review approved by 2+ reviewers
- ✅ Example migrations documented
- ✅ Rollback procedure tested

---

## 🚨 RISKS & MITIGATION

### Risk 1: Breaking Changes
**Impact**: HIGH  
**Probability**: MEDIUM  
**Mitigation**:
- Work in feature branch
- Comprehensive testing at each step
- Keep rollback option ready
- Update code in small, testable chunks

### Risk 2: Import Statement Misses
**Impact**: MEDIUM  
**Probability**: MEDIUM  
**Mitigation**:
- Use automated script for bulk changes
- Manual review of all changes
- Grep for remaining old patterns
- Let compiler catch remaining issues

### Risk 3: Performance Regression
**Impact**: LOW  
**Probability**: LOW  
**Mitigation**:
- Benchmark before and after
- NestGateError is already zero-cost
- No allocations in hot paths
- Same memory layout

### Risk 4: Hidden Dependencies
**Impact**: MEDIUM  
**Probability**: LOW  
**Mitigation**:
- Full workspace build after each change
- Cross-crate test suite
- Check all crate dependencies
- Verify re-exports work correctly

---

## 📞 SUPPORT & RESOURCES

### Documentation
- `docs/guides/ERROR_STANDARDIZATION_MIGRATION_PLAN.md` - Comprehensive guide
- `code/crates/nestgate-core/src/error/mod.rs` - Canonical error system
- This document - Phase 1 action plan

### Tools
- `scripts/migrate_result_imports.sh` - Automated migration (create this)
- `grep`/`ripgrep` - Finding patterns
- `sed` - Bulk replacements
- `cargo` - Building and testing

### Getting Help
- Check existing error migration guide
- Review error system documentation
- Test each change incrementally
- Use git to rollback if needed

---

**Phase 1 Start Date**: TBD  
**Phase 1 End Date**: TBD (2 weeks from start)  
**Status**: 📋 READY TO BEGIN  
**Blocker**: None - can start immediately  

---

*This action plan provides a detailed, step-by-step approach to unifying the error system. Follow it systematically to achieve clean, debt-free error handling.*

