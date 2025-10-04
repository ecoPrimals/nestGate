# 🚀 **WEEK 2 EXECUTION PLAN: Configuration Consolidation Sprint**

**Sprint Dates**: October 7-11, 2025 (Week 2)  
**Status**: 🎯 **READY TO EXECUTE**  
**Goal**: Consolidate NetworkConfig (33+→1), StorageConfig (30+→1), SecurityConfig (20+→1)  
**Backup Created**: ✅ `backups/pre-week2-consolidation-20250930/crates`

---

## 📋 **PRE-SPRINT CHECKLIST**

### **Infrastructure Ready** ✅
- [x] Backup created: `backups/pre-week2-consolidation-20250930/`
- [x] Validation scripts verified
- [x] NetworkConfig migration map reviewed
- [x] Canonical systems identified
- [ ] All documentation reviewed
- [ ] Team aligned on approach

### **Environment Setup**
```bash
# Working directory
cd /home/eastgate/Development/ecoPrimals/nestgate

# Validation available
ls scripts/validation/*.sh

# Backup location
ls -lh backups/pre-week2-consolidation-20250930/
```

---

## 📅 **DAY-BY-DAY EXECUTION PLAN**

## **DAY 1: NetworkConfig Consolidation (Monday)**

### **Morning Session (4 hours): Core Migration**

#### **Task 1.1: Audit Current State** (30 min)
```bash
# Find all NetworkConfig usages
rg "NetworkConfig" --type rust code/crates/ > /tmp/networkconfig_audit.txt

# Count by file
rg "NetworkConfig" --type rust -c code/crates/ | sort -t: -k2 -nr > /tmp/networkconfig_counts.txt

# Review results
cat /tmp/networkconfig_counts.txt
```

**Expected Output**: List of ~33 files with NetworkConfig usage

#### **Task 1.2: Migrate nestgate-network** (2 hours)

**Step 1: Update types.rs**
```bash
# Backup file
cp code/crates/nestgate-network/src/types.rs backups/types.rs.before

# File: code/crates/nestgate-network/src/types.rs
# Change:
#   FROM: use nestgate_core::unified_config_consolidation::StandardDomainConfig;
#         pub type NetworkConfig = StandardDomainConfig<NetworkExtensions>;
#   TO:   use nestgate_core::config::canonical_master::domains::network::CanonicalNetworkConfig;
#         pub type NetworkConfig = CanonicalNetworkConfig;
```

**Step 2: Update config.rs**
```bash
# File: code/crates/nestgate-network/src/config.rs
# Update imports to reference canonical
```

**Step 3: Update network_core.rs**
```bash
# File: code/crates/nestgate-network/src/unified_network_config/network_core.rs
# Line 34: pub type UnifiedNetworkConfig = crate::types::NetworkConfig;
# This should now resolve to CanonicalNetworkConfig via the alias
```

**Validation:**
```bash
cargo check -p nestgate-network
cargo test -p nestgate-network --no-run
```

**Success Criteria**: ✅ nestgate-network compiles with canonical config

#### **Task 1.3: Migrate nestgate-api Network Configs** (1.5 hours)

**Files to Update**:
```bash
# Find API network configs
rg "NetworkConfig" --type rust code/crates/nestgate-api/ -l

# Update imports in each file to use:
# use nestgate_core::config::canonical_master::domains::network::CanonicalNetworkConfig;
```

**Validation:**
```bash
cargo check -p nestgate-api
cargo test -p nestgate-api --no-run
```

### **Afternoon Session (4 hours): Template & Test Updates**

#### **Task 1.4: Update Templates** (1 hour)

**Files**:
```bash
# Update template references
code/crates/nestgate-core/src/unified_types/network_config.rs
ecosystem-expansion/templates/config-template/network_config.rs
ecosystem-expansion/templates/config-template/network.rs
ecosystem-expansion/templates/config-template/domains/network/mod.rs
```

**Action**: Add deprecation notices or update to reference canonical

#### **Task 1.5: Update Test Configs** (1 hour)

**Pattern**:
```rust
// In test files:
#[cfg(test)]
mod tests {
    use nestgate_core::config::canonical_master::domains::network::CanonicalNetworkConfig as NetworkConfig;
    // Rest of test code...
}
```

**Validation:**
```bash
cargo test --workspace --lib
```

#### **Task 1.6: Full Workspace Validation** (1 hour)

```bash
# Full build check
cargo check --workspace

# Run tests
cargo test --workspace --no-run

# Check for NetworkConfig references to deprecated modules
rg "unified_types::network_config" --type rust code/crates/
rg "unified_final_config.*network" --type rust code/crates/

# Validate config unification
./scripts/validation/validate-config-unification.sh
```

#### **Task 1.7: Documentation** (1 hour)

Create `NETWORKCONFIG_MIGRATION_COMPLETE.md`:
```markdown
# NetworkConfig Migration Complete

**Date**: [Date]
**Files Updated**: [Count]
**Tests Passing**: [Status]

## Changes Made
- Migrated nestgate-network to canonical
- Updated nestgate-api network configs
- Updated templates to reference canonical
- Updated test configurations

## Validation Results
[Paste validation output]

## Breaking Changes
[Document any API changes]

## Next Steps
- StorageConfig consolidation (Day 3-4)
```

**End of Day 1 Success Criteria**:
- ✅ NetworkConfig consolidated to 1 canonical source
- ✅ nestgate-network using canonical
- ✅ nestgate-api using canonical
- ✅ All tests passing
- ✅ Build clean

---

## **DAY 2: NetworkConfig Cleanup & Validation (Tuesday)**

### **Morning Session (4 hours): Comprehensive Testing**

#### **Task 2.1: Integration Testing** (2 hours)

```bash
# Run full test suite
cargo test --workspace

# Run integration tests
cargo test --workspace --test '*'

# Run doc tests
cargo test --workspace --doc

# Verify examples compile
cargo check --examples
```

#### **Task 2.2: Deprecated Module Marking** (1 hour)

**Mark deprecated (don't remove yet - Week 4)**:
```rust
// code/crates/nestgate-core/src/unified_types/network_config.rs
#[deprecated(since = "0.8.0", note = "Use canonical_master::domains::network::CanonicalNetworkConfig")]
pub struct NetworkConfig { ... }

// code/crates/nestgate-core/src/unified_final_config/domain_configs/network.rs
#[deprecated(since = "0.8.0", note = "Use canonical_master::domains::network::CanonicalNetworkConfig")]
pub struct NetworkConfig { ... }
```

**Validation**:
```bash
# Should now show deprecation warnings
cargo check --workspace 2>&1 | grep "deprecated"
```

#### **Task 2.3: Update Migration Progress** (1 hour)

Update tracking documents:
- `UNIFICATION_CHECKLIST.md` - Check off NetworkConfig tasks
- `WEEK2_PROGRESS_UPDATE.md` - Document Day 1-2 completion
- `NETWORKCONFIG_MIGRATION_MAP.md` - Update status to COMPLETE

### **Afternoon Session (3 hours): StorageConfig Preparation

#### **Task 2.4: StorageConfig Audit** (1.5 hours)

```bash
# Find all StorageConfig usages
rg "StorageConfig" --type rust code/crates/ > /tmp/storageconfig_audit.txt

# Count by file
rg "StorageConfig" --type rust -c code/crates/ | sort -t: -k2 -nr > /tmp/storageconfig_counts.txt

# Identify the canonical source
find code/crates/nestgate-core -name "*storage*" -type f | grep config
```

**Expected**: ~30 StorageConfig variants

#### **Task 2.5: Create StorageConfig Migration Plan** (1.5 hours)

Create `STORAGECONFIG_MIGRATION_PLAN.md`:
```markdown
# StorageConfig Migration Plan

## Canonical Source
Location: code/crates/nestgate-core/src/config/canonical_master/storage_config.rs

## Variants Found
[List all 30+ variants]

## Migration Order
1. nestgate-zfs (highest priority)
2. nestgate-nas
3. nestgate-api (storage-related)
4. Other crates

## Timeline
- Day 3: Core migrations
- Day 4: Cleanup and validation
```

**End of Day 2 Success Criteria**:
- ✅ NetworkConfig fully consolidated
- ✅ All tests passing
- ✅ Deprecated modules marked
- ✅ StorageConfig migration plan ready

---

## **DAY 3: StorageConfig Consolidation (Wednesday)**

### **Morning Session (4 hours): Core Storage Migration**

#### **Task 3.1: Identify Canonical StorageConfig** (30 min)

```bash
# Find canonical storage config
find code/crates/nestgate-core/src/config/canonical_master -name "*storage*"

# Review structure
cat code/crates/nestgate-core/src/config/canonical_master/storage_config.rs | head -100
```

#### **Task 3.2: Migrate nestgate-zfs** (2 hours)

**Primary Target**: ZFS crate is main storage consumer

```bash
# Backup
cp -r code/crates/nestgate-zfs backups/nestgate-zfs.before

# Find all storage configs in zfs crate
rg "StorageConfig" --type rust code/crates/nestgate-zfs/ -l
```

**Update Pattern**:
```rust
// Change imports to:
use nestgate_core::config::canonical_master::storage_config::CanonicalStorageConfig;
pub type StorageConfig = CanonicalStorageConfig;
```

**Validation**:
```bash
cargo check -p nestgate-zfs
cargo test -p nestgate-zfs --no-run
```

#### **Task 3.3: Migrate nestgate-nas** (1.5 hours)

```bash
# Backup
cp -r code/crates/nestgate-nas backups/nestgate-nas.before

# Update storage configs
rg "StorageConfig" --type rust code/crates/nestgate-nas/ -l
```

**Validation**:
```bash
cargo check -p nestgate-nas
cargo test -p nestgate-nas --no-run
```

### **Afternoon Session (4 hours): API & Other Crates**

#### **Task 3.4: Migrate nestgate-api Storage Configs** (2 hours)

```bash
# Find API storage configs
rg "StorageConfig|storage_config" --type rust code/crates/nestgate-api/ -l

# Update each file
# Pattern: Use canonical import
```

**Validation**:
```bash
cargo check -p nestgate-api
```

#### **Task 3.5: Update Remaining Crates** (2 hours)

```bash
# Check other crates for storage configs
for crate in nestgate-automation nestgate-installer nestgate-middleware nestgate-performance; do
    echo "Checking $crate..."
    rg "StorageConfig" --type rust code/crates/$crate/ -l || echo "  No matches"
done
```

**Update each crate found**

**Validation**:
```bash
cargo check --workspace
```

**End of Day 3 Success Criteria**:
- ✅ StorageConfig consolidated to 1 canonical
- ✅ nestgate-zfs using canonical
- ✅ nestgate-nas using canonical
- ✅ All storage consumers updated
- ✅ Build passes

---

## **DAY 4: StorageConfig Cleanup & SecurityConfig Start (Thursday)**

### **Morning Session (3 hours): Storage Validation**

#### **Task 4.1: Comprehensive Storage Testing** (2 hours)

```bash
# Run all storage-related tests
cargo test --workspace -- storage

# Run ZFS tests
cargo test -p nestgate-zfs

# Run NAS tests
cargo test -p nestgate-nas

# Full workspace test
cargo test --workspace
```

#### **Task 4.2: Mark Deprecated Storage Configs** (1 hour)

```rust
// Mark old storage configs as deprecated
#[deprecated(since = "0.8.0", note = "Use canonical_master::storage_config::CanonicalStorageConfig")]
```

### **Afternoon Session (4 hours): SecurityConfig Consolidation**

#### **Task 4.3: SecurityConfig Audit** (1 hour)

```bash
# Find all SecurityConfig usages
rg "SecurityConfig" --type rust code/crates/ > /tmp/securityconfig_audit.txt

# Count variants
rg "SecurityConfig" --type rust -c code/crates/ | sort -t: -k2 -nr

# Expected: ~20 variants
```

#### **Task 4.4: Identify Canonical SecurityConfig** (30 min)

```bash
# Find canonical security config
find code/crates/nestgate-core/src/config/canonical_master -name "*security*"

# Review structure
cat code/crates/nestgate-core/src/config/canonical_master/security_config.rs | head -100
```

#### **Task 4.5: Begin Security Migration** (2.5 hours)

**Priority Order**:
1. nestgate-api (auth/authz configs)
2. nestgate-middleware (security middleware)
3. Other crates

```bash
# Start with API security configs
rg "SecurityConfig" --type rust code/crates/nestgate-api/ -l

# Update to canonical
# Pattern: use nestgate_core::config::canonical_master::security_config::CanonicalSecurityConfig;
```

**Validation**:
```bash
cargo check -p nestgate-api
cargo check --workspace
```

**End of Day 4 Success Criteria**:
- ✅ StorageConfig fully validated
- ✅ SecurityConfig migration started
- ✅ 50%+ of security configs migrated
- ✅ Build passes

---

## **DAY 5: SecurityConfig Completion & Week Wrap-up (Friday)**

### **Morning Session (3 hours): Complete Security Migration**

#### **Task 5.1: Finish Security Config Migration** (2 hours)

```bash
# Complete remaining security configs
for crate in nestgate-middleware nestgate-core nestgate-mcp; do
    echo "Migrating $crate security configs..."
    rg "SecurityConfig" --type rust code/crates/$crate/ -l
done
```

**Update all remaining files**

**Validation**:
```bash
cargo check --workspace
cargo test --workspace --no-run
```

#### **Task 5.2: Mark Deprecated Security Configs** (1 hour)

```rust
// Mark old security configs as deprecated
#[deprecated(since = "0.8.0", note = "Use canonical_master::security_config::CanonicalSecurityConfig")]
```

### **Afternoon Session (4 hours): Validation & Documentation**

#### **Task 5.3: Comprehensive Week 2 Validation** (2 hours)

```bash
# Full test suite
cargo test --workspace

# All doc tests
cargo test --workspace --doc

# Clippy check
cargo clippy --workspace -- -D warnings

# Build all targets
cargo build --workspace --all-targets

# Run validation scripts
./scripts/validation/validate-build-health.sh
./scripts/validation/validate-config-unification.sh
./scripts/validation/run-all-validations.sh
```

#### **Task 5.4: Update Documentation** (1 hour)

**Update Files**:
- `UNIFICATION_CHECKLIST.md` - Check off Week 2 tasks
- `WEEK2_PROGRESS_UPDATE.md` - Document achievements
- `ARCHITECTURE_OVERVIEW.md` - Update unification percentage

**Create**:
- `WEEK2_COMPLETION_REPORT.md` - Summary of Week 2 achievements

#### **Task 5.5: Prepare Week 3 Plan** (1 hour)

Create `WEEK3_EXECUTION_PLAN.md`:
```markdown
# Week 3: Error System & Crate Migration

## Goals
- Consolidate error enums (57 → 15)
- Migrate all 15 crates to canonical
- Achieve 100% test pass rate

## Preparation
- Review error migration helpers
- Identify domain-specific vs. cross-cutting errors
- Plan migration order by dependency
```

**End of Week 2 Success Criteria**: ✅
- ✅ NetworkConfig: 33+ → 1 canonical
- ✅ StorageConfig: 30+ → 1 canonical
- ✅ SecurityConfig: 20+ → 1 canonical
- ✅ Config structs: 525 → ~350 (33% reduction)
- ✅ All tests passing
- ✅ Build with 0 errors
- ✅ Documentation updated
- ✅ Week 3 plan ready

---

## 🛠️ **UTILITY COMMANDS**

### **During Migration**

**Quick Validation**:
```bash
# Fast check
cargo check --workspace

# Quick test
cargo test --workspace --lib --no-run

# Count remaining duplicates
rg "NetworkConfig" --type rust code/crates/ | wc -l
rg "StorageConfig" --type rust code/crates/ | wc -l
rg "SecurityConfig" --type rust code/crates/ | wc -l
```

**Rollback** (if needed):
```bash
# Restore from backup
rm -rf code/crates
cp -r backups/pre-week2-consolidation-20250930/crates code/

# Verify
cargo check --workspace
```

**Progress Tracking**:
```bash
# Create daily snapshot
cp -r code/crates backups/week2-day[1-5]-snapshot-$(date +%Y%m%d-%H%M)/
```

### **Common Issues & Solutions**

**Issue**: Trait bound errors after config change
```rust
// Solution: Update trait imports
use nestgate_core::config::canonical_master::domains::network::CanonicalNetworkConfig;
```

**Issue**: Missing fields in canonical config
```rust
// Solution: Use domain extensions
pub struct NetworkDomainExtensions {
    pub custom_field: CustomType,
}
```

**Issue**: Tests failing after migration
```bash
# Solution: Update test imports
cargo test --workspace -- --nocapture
# Review error messages and update test configs
```

---

## 📊 **SUCCESS METRICS TRACKING**

### **Daily Checkpoints**

**Day 1**:
- [ ] NetworkConfig variants: 33+ → 1 ✅
- [ ] nestgate-network migrated ✅
- [ ] nestgate-api network configs updated ✅

**Day 2**:
- [ ] All tests passing ✅
- [ ] Deprecated modules marked ✅
- [ ] StorageConfig plan created ✅

**Day 3**:
- [ ] StorageConfig variants: 30+ → 1 ✅
- [ ] nestgate-zfs migrated ✅
- [ ] nestgate-nas migrated ✅

**Day 4**:
- [ ] StorageConfig validated ✅
- [ ] SecurityConfig 50% migrated ✅

**Day 5**:
- [ ] SecurityConfig variants: 20+ → 1 ✅
- [ ] All validations passing ✅
- [ ] Documentation complete ✅

### **Week 2 Final Metrics**

**Before Week 2**:
- Config structs: 525
- NetworkConfig variants: 33+
- StorageConfig variants: 30+
- SecurityConfig variants: 20+
- Unification: 85%

**After Week 2** (Target):
- Config structs: ~350 (33% reduction) ✅
- NetworkConfig variants: 1 ✅
- StorageConfig variants: 1 ✅
- SecurityConfig variants: 1 ✅
- Unification: ~92% ✅

---

## 🎯 **NEXT STEPS AFTER WEEK 2**

1. **Week 3 Planning** (Day 5 afternoon)
   - Error system consolidation strategy
   - Crate-by-crate migration plan
   - Dependency order mapping

2. **Week 3 Preparation**
   - Review error migration helpers
   - Test migration patterns
   - Prepare error consolidation scripts

3. **Communication**
   - Update team on Week 2 achievements
   - Share lessons learned
   - Align on Week 3 approach

---

**Sprint Owner**: [Your Name]  
**Start Date**: October 7, 2025  
**Target Completion**: October 11, 2025  
**Backup Location**: `backups/pre-week2-consolidation-20250930/`

---

*Week 2 Execution Plan - Ready for action! 🚀* 