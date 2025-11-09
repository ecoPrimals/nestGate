# ZFS Config Migration Plan
**Date**: November 7, 2025  
**Task**: Migrate ZFS configuration to canonical_primary  
**Pattern**: Following proven NetworkConfig migration success  
**References**: 138 across 38 files in nestgate-zfs

---

## 📊 CURRENT STATE ANALYSIS

### Four ZfsConfig Types Found

#### 1. **Runtime Command Config** ✅ KEEP
**Location**: `nestgate-core/src/services/storage/config.rs`
```rust
pub struct ZfsConfig {
    pub zfs_binary: String,        // "/usr/sbin/zfs"
    pub zpool_binary: String,       // "/usr/sbin/zpool"
    pub use_sudo: bool,
    pub command_timeout: Duration,
}
```
**Purpose**: Runtime command execution settings  
**Decision**: **KEEP** - This is operational config, not domain config

#### 2. **OLD Unified Pattern** ❌ REMOVE
**Location**: `nestgate-zfs/src/canonical_zfs_config.rs` (271 lines)
```rust
pub type ZfsConfig = StandardDomainConfig<ZfsExtensions>;

pub struct ZfsExtensions {
    pub pools: ZfsPoolConfig,
    pub datasets: ZfsDatasetConfig,
    pub performance: ZfsPerformanceConfig,
    pub monitoring: ZfsMonitoringConfig,
    pub snapshots: ZfsSnapshotConfig,
    pub migration: ZfsMigrationConfig,
}
```
**Problem**: Uses OLD `unified_config_consolidation::StandardDomainConfig`  
**Decision**: **MIGRATE** to canonical_primary

#### 3. **NEW Canonical (Comprehensive)** ✅ TARGET
**Location**: `nestgate-core/src/config/canonical_primary/domains/storage_canonical/zfs.rs` (138 lines)
```rust
pub struct ZfsStorageConfig {
    pub enabled: bool,
    pub pools: Vec<ZfsPoolConfig>,
    pub datasets: ZfsDatasetConfig,
    pub snapshots: ZfsSnapshotConfig,
    pub maintenance: ZfsMaintenanceConfig,
    pub performance: ZfsPerformanceConfig,
    pub security: ZfsSecurityConfig,
}
```
**Purpose**: Comprehensive ZFS domain configuration  
**Decision**: **EXTEND** with missing fields from #2, then use as primary

#### 4. **Simple Canonical Config** ✅ PART OF STORAGE
**Location**: `nestgate-core/src/config/canonical_primary/storage_config.rs`
```rust
pub struct ZfsConfig {
    pub enabled: bool,
    pub pools: Vec<ZfsPool>,
    pub zfs_settings: HashMap<String, serde_json::Value>,
}
```
**Purpose**: Part of StorageConfig  
**Decision**: **KEEP** as wrapper, delegates to ZfsStorageConfig

#### 5. **Wrapper/Re-export** ❌ REMOVE
**Location**: `nestgate-zfs/src/config/unified_zfs_config.rs` (106 lines)
```rust
pub use crate::canonical_zfs_config::{ZfsConfig, ZfsExtensions};

pub struct InternalZfsExtensions {
    pub pools: ZfsPoolSettings,
    pub datasets: ZfsDatasetSettings,
    pub snapshots: ZfsSnapshotSettings,
}
```
**Purpose**: Re-exports + additional internal types  
**Decision**: **REMOVE** after migration

---

## 🎯 MIGRATION STRATEGY

### Phase 1: Understand & Map (Completed ✅)

**Findings**:
- 138 references to ZfsConfig across 38 files
- 2 importing from `unified_zfs_config` (nestgate-core, nestgate-zfs/config/mod.rs)
- Most use `canonical_zfs_config` or `services/storage/config`
- Some specialized configs in nestgate-zfs/config/ (automation, health, metrics, pool, security, tiers)

**Key Insight**: The specialized configs in `nestgate-zfs/config/` are COMPLEMENTARY, not competing. They add ZFS-specific operational settings on top of the domain config.

### Phase 2: Extend Canonical Target

**Missing from `storage_canonical/zfs.rs`** (need to add from `canonical_zfs_config`):

1. **Monitoring Config** (from ZfsMonitoringConfig):
   - `health_check_enabled: bool`
   - `health_check_interval: Duration`
   - `metrics_collection: bool`
   - `alert_thresholds: AlertThresholds`

2. **Migration Config** (from ZfsMigrationConfig):
   - `migration_enabled: bool`
   - `bandwidth_limit_mbps: Option<u64>`
   - `compression_during_migration: bool`
   - `verification_enabled: bool`

3. **Pool Config Extensions** (from ZfsPoolConfig):
   - `default_pool_name: String`
   - `max_pools: u32`
   - `auto_discovery: bool`
   - `health_check_interval: Duration`
   - `auto_pool_creation: bool`

4. **Performance Details** (from ZfsPerformanceConfig):
   - `arc_cache: ArcCacheConfig`
   - `l2arc: L2ArcConfig`
   - `zil: ZilConfig`
   - `prefetch: PrefetchConfig`

5. **Dataset Extensions** (from ZfsDatasetConfig):
   - `max_datasets_per_pool: u32`
   - `quota_enforcement: bool`

6. **Snapshot Extensions** (from ZfsSnapshotConfig):
   - `retention_policy: RetentionPolicy` (with hourly/daily/weekly/monthly)
   - `naming_convention: String`

### Phase 3: Type Mapping

| OLD Type | NEW Location | Notes |
|----------|--------------|-------|
| `canonical_zfs_config::ZfsConfig` | `storage_canonical::ZfsStorageConfig` | Main domain config |
| `canonical_zfs_config::ZfsExtensions` | Merged into `ZfsStorageConfig` | No longer needed |
| `canonical_zfs_config::ZfsPoolConfig` | `storage_canonical::zfs::ZfsPoolConfig` | Extend with missing fields |
| `canonical_zfs_config::ZfsPerformanceConfig` | `storage_canonical::zfs::ZfsPerformanceConfig` | Extend with sub-configs |
| `canonical_zfs_config::ZfsMonitoringConfig` | `storage_canonical::zfs::ZfsMonitoringConfig` | NEW - add to canonical |
| `canonical_zfs_config::ZfsMigrationConfig` | `storage_canonical::zfs::ZfsMigrationConfig` | NEW - add to canonical |
| `services::storage::config::ZfsConfig` | KEEP AS IS | Runtime command config |

---

## 📋 STEP-BY-STEP EXECUTION PLAN

### Step 1: Extend Canonical ZfsStorageConfig (2 hours)

**File**: `code/crates/nestgate-core/src/config/canonical_primary/domains/storage_canonical/zfs.rs`

**Actions**:
1. Add `ZfsMonitoringConfig` struct
2. Add `ZfsMigrationConfig` struct  
3. Extend `ZfsPoolConfig` with missing fields
4. Extend `ZfsPerformanceConfig` with sub-configs (ARC, L2ARC, ZIL, Prefetch)
5. Extend `ZfsDatasetConfig` with missing fields
6. Extend `ZfsSnapshotConfig` with RetentionPolicy
7. Update `ZfsStorageConfig` to include new fields
8. Add Default implementations
9. Add factory methods (production(), development(), testing())
10. Add validation methods

### Step 2: Update Exports (30 min)

**File**: `code/crates/nestgate-core/src/config/canonical_primary/domains/mod.rs`

**Actions**:
1. Ensure `storage_canonical::ZfsStorageConfig` is exported
2. Add re-exports for all ZFS sub-configs
3. Update documentation

### Step 3: Create Type Alias in nestgate-zfs (15 min)

**File**: `code/crates/nestgate-zfs/src/types.rs` or new file

**Actions**:
```rust
// Re-export canonical ZFS config for backward compatibility
pub use nestgate_core::config::canonical_primary::domains::storage_canonical::{
    ZfsStorageConfig as ZfsConfig,
    ZfsPoolConfig,
    ZfsDatasetConfig,
    ZfsSnapshotConfig,
    ZfsPerformanceConfig,
    ZfsMonitoringConfig,
    ZfsMigrationConfig,
    ZfsMaintenanceConfig,
    ZfsSecurityConfig,
};
```

### Step 4: Update nestgate-zfs Imports (2 hours)

**Target files** (38 files with 138 references):

**High-priority files** (most references):
1. `src/config/mod.rs` - Update main config re-exports
2. `src/lib.rs` - Update public exports
3. `src/canonical_zfs_config.rs` - Mark deprecated, add migration note
4. `src/config/unified_zfs_config.rs` - Mark deprecated, add migration note

**Systematic update**:
```bash
# Find all imports
grep -r "use.*canonical_zfs_config" code/crates/nestgate-zfs/src --include="*.rs" > zfs_imports.txt

# For each file, update:
# OLD: use crate::canonical_zfs_config::ZfsConfig;
# NEW: use nestgate_core::config::canonical_primary::domains::storage_canonical::ZfsStorageConfig as ZfsConfig;
# OR:  use crate::types::ZfsConfig; (if we add type alias)
```

### Step 5: Update Tests (1 hour)

**Test files to update**:
- `tests/unit_tests/config_tests.rs`
- `tests/extensive_zfs_tests.rs`
- `tests/unit_tests_simple.rs`
- `tests/pool_tests.rs`
- And 4 more test files

**Actions**:
1. Update imports
2. Update config construction
3. Update assertions for new field names
4. Run tests: `cargo test -p nestgate-zfs`

### Step 6: Remove Old Files (30 min)

**After all tests pass**:
```bash
# Mark as deprecated first (safety)
# In canonical_zfs_config.rs, add:
#[deprecated(since = "0.2.0", note = "Use nestgate_core::config::canonical_primary::domains::storage_canonical::ZfsStorageConfig")]

# After verification, remove:
rm code/crates/nestgate-zfs/src/canonical_zfs_config.rs
rm code/crates/nestgate-zfs/src/config/unified_zfs_config.rs

# Update config/mod.rs to remove references
```

### Step 7: Final Validation (30 min)

```bash
# Full workspace check
cargo check --workspace

# ZFS crate tests
cargo test -p nestgate-zfs

# Integration tests
cargo test --workspace

# Verify no OLD imports remain
grep -r "canonical_zfs_config\|unified_zfs_config" code/crates --include="*.rs"
# Should return 0 results (or only deprecation notices)
```

---

## 🚨 RISK MITIGATION

### Known Risks

1. **Breaking existing code** (138 references!)
   - Mitigation: Use deprecation warnings first
   - Mitigation: Create type aliases for backward compatibility
   - Mitigation: Test incrementally

2. **Complex field mappings**
   - Mitigation: Document all field mappings
   - Mitigation: Test each mapping

3. **Test failures**
   - Mitigation: Fix tests as we go
   - Mitigation: Run tests after each file update

### Rollback Plan

```bash
# If anything breaks:
git stash  # Save work
# OR
git reset --hard HEAD  # Full rollback

# Verify working state
cargo test -p nestgate-zfs
cargo test --workspace
```

---

## ✅ SUCCESS CRITERIA

- [ ] All fields from `canonical_zfs_config` are in `storage_canonical/zfs.rs`
- [ ] All 138 ZfsConfig references updated or using type aliases
- [ ] Zero imports from `canonical_zfs_config` remain
- [ ] Zero imports from `unified_zfs_config` remain
- [ ] All tests passing: `cargo test -p nestgate-zfs` ✅
- [ ] All tests passing: `cargo test --workspace` ✅
- [ ] Old files removed: `canonical_zfs_config.rs`, `unified_zfs_config.rs`
- [ ] Build clean: `cargo check --workspace` ✅
- [ ] Zero breaking changes for users

---

## 📊 ESTIMATED TIME

| Phase | Task | Time |
|-------|------|------|
| 1 | Extend canonical ZfsStorageConfig | 2h |
| 2 | Update exports | 30min |
| 3 | Create type aliases | 15min |
| 4 | Update nestgate-zfs imports | 2h |
| 5 | Update tests | 1h |
| 6 | Remove old files | 30min |
| 7 | Final validation | 30min |
| **TOTAL** | | **6h 45min** |

---

## 🎯 DECISION

**Recommendation**: Given the complexity (138 references, 38 files), I recommend:

1. **Option A** (SAFER): Create a detailed migration document (this one), then pause for review
2. **Option B** (AGGRESSIVE): Execute the migration now following this plan

**My Recommendation**: **Option A** - This is more complex than NetworkConfig. Let's ensure we have the right approach before making changes.

**Next Steps**:
1. Review this migration plan
2. Verify the field mappings are correct
3. Decide: Proceed with migration OR adjust plan
4. If proceeding: Execute Step 1 (Extend canonical config)

---

**Document Status**: ✅ COMPLETE - Ready for Review  
**Confidence**: MEDIUM-HIGH (more complex than NetworkConfig)  
**Recommendation**: Review before executing

