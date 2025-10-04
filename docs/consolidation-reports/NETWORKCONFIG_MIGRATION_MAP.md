# 🗺️ **NetworkConfig Migration Map**

**Date**: September 30, 2025  
**Status**: 📋 **PLANNING PHASE**  
**Target**: Consolidate 33+ NetworkConfig variants → 1 canonical  
**Timeline**: Week 2, Day 1-2 (2 days)

---

## 🎯 **OBJECTIVE**

Consolidate all NetworkConfig definitions across the nestgate codebase into a single canonical configuration system located at `code/crates/nestgate-core/src/config/canonical_master/domains/network/`.

---

## 📊 **CURRENT STATE ANALYSIS**

### **The Problem: 33+ NetworkConfig Variants**

Multiple definitions of NetworkConfig exist across crates, creating:
- Configuration fragmentation
- Type confusion
- Maintenance burden
- Import complexity

### **NetworkConfig Variants Identified**

| # | **Location** | **Type Name** | **Status** | **Action** |
|---|-------------|---------------|------------|------------|
| 1 | `nestgate-network/src/types.rs` | `NetworkConfig` | 🔴 Active | Migrate to canonical |
| 2 | `nestgate-network/src/config.rs` | `NetworkConfig` | 🔴 Active | Migrate to canonical |
| 3 | `nestgate-network/src/unified_network_config/network_core.rs` | `UnifiedNetworkConfig` | 🔴 Active | Migrate to canonical |
| 4 | `nestgate-core/src/config/canonical_master/domains/network/mod.rs` | `CanonicalNetworkConfig` | ✅ **CANONICAL** | **Keep - This is THE source** |
| 5 | `nestgate-core/src/unified_types/network_config.rs` | `NetworkConfig` | 🟡 Deprecated | Remove after migration |
| 6 | `nestgate-core/src/unified_final_config/domain_configs/network.rs` | `NetworkConfig` | 🟡 Deprecated | Remove after migration |
| 7 | `nestgate-core/src/canonical/types/config_registry.rs` | `CanonicalNetworkConfig` | 🟡 Duplicate | Merge into canonical |
| 8 | `ecosystem-expansion/templates/config-template/network_config.rs` | `NetworkConfig` | 🟢 Template | Update to reference canonical |
| 9 | `ecosystem-expansion/templates/config-template/network.rs` | `NetworkConfig` | 🟢 Template | Update to reference canonical |
| 10 | `ecosystem-expansion/templates/config-template/domains/network/mod.rs` | `NetworkConfig` | 🟢 Template | Update to reference canonical |
| 11-33 | Various test configs, examples, migration helpers | Various | 🔵 Test/Temp | Update or mark as test-only |

---

## ✅ **THE CANONICAL SOURCE**

**Location**: `code/crates/nestgate-core/src/config/canonical_master/domains/network/mod.rs`

**Structure**:
```rust
/// THE canonical network configuration for the entire NestGate ecosystem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalNetworkConfig {
    /// Core API server configuration
    pub api: NetworkApiConfig,
    
    /// Network orchestration configuration
    pub orchestration: NetworkOrchestrationConfig,
    
    /// Protocol-specific configurations
    pub protocols: NetworkProtocolConfig,
    
    /// VLAN and network segmentation
    pub vlan: NetworkVlanConfig,
    
    /// Service discovery configuration
    pub discovery: NetworkDiscoveryConfig,
    
    /// Performance and optimization settings
    pub performance: NetworkPerformanceConfig,
    
    /// Security and authentication settings
    pub security: NetworkSecurityConfig,
    
    /// Monitoring and observability
    pub monitoring: NetworkMonitoringConfig,
    
    /// Environment-specific overrides
    pub environment: NetworkEnvironmentConfig,
}
```

**Sub-modules** (all in `canonical_master/domains/network/`):
- `api.rs` - API server configuration
- `orchestration.rs` - Network orchestration
- `protocols.rs` - Protocol-specific configs
- `vlan.rs` - VLAN and segmentation
- `discovery.rs` - Service discovery
- `performance.rs` - Performance tuning
- `security.rs` - Security settings
- `monitoring.rs` - Observability config
- `environment.rs` - Environment overrides

---

## 🔄 **MIGRATION STRATEGY**

### **Phase 1: Preparation** (2 hours)

1. **Audit Current Usage**
   ```bash
   # Find all NetworkConfig usages
   rg "NetworkConfig" --type rust -l > /tmp/network_config_files.txt
   
   # Count usages by file
   rg "NetworkConfig" --type rust -c | sort -t: -k2 -nr
   ```

2. **Document Dependencies**
   - Identify which crates import which NetworkConfig
   - Map field usage across variants
   - Identify breaking changes

3. **Create Migration Checklist**
   - List all files needing updates
   - Prioritize by dependency order
   - Identify potential conflicts

### **Phase 2: Core Crate Migration** (4 hours)

#### **Priority 1: nestgate-network** (Highest Priority)

**Files to Update**:
1. `code/crates/nestgate-network/src/types.rs`
2. `code/crates/nestgate-network/src/config.rs`
3. `code/crates/nestgate-network/src/unified_network_config/network_core.rs`
4. `code/crates/nestgate-network/src/lib.rs`

**Migration Pattern**:
```rust
// BEFORE (types.rs):
use nestgate_core::unified_config_consolidation::StandardDomainConfig;
pub type NetworkConfig = StandardDomainConfig<NetworkExtensions>;

// AFTER (types.rs):
use nestgate_core::config::canonical_master::domains::network::CanonicalNetworkConfig;
pub type NetworkConfig = CanonicalNetworkConfig;
```

**Extension Pattern** (if needed):
```rust
// Only if truly necessary for network-specific features
pub struct NetworkDomainExtensions {
    pub advanced_routing: AdvancedRoutingConfig,
    // Only network-crate-specific features
}
```

#### **Priority 2: nestgate-api** (High Priority)

**Files to Update**:
- All network-related configs in `nestgate-api/src/`
- Handler configurations using NetworkConfig
- Update imports

**Migration Pattern**:
```rust
// Update imports
use nestgate_core::config::canonical_master::domains::network::CanonicalNetworkConfig;

// Update function signatures
pub fn initialize(config: &CanonicalNetworkConfig) -> Result<()> {
    // Implementation
}
```

#### **Priority 3: Other Crates** (Medium Priority)

For each remaining crate:
1. Search for NetworkConfig imports
2. Update to use canonical
3. Remove local NetworkConfig definitions
4. Test functionality

### **Phase 3: Cleanup** (2 hours)

1. **Remove Deprecated Definitions**
   ```rust
   // Mark as deprecated first (for 1 week)
   #[deprecated(since = "0.8.0", note = "Use CanonicalNetworkConfig from canonical_master")]
   pub type NetworkConfig = /* ... */;
   ```

2. **Update Migration Helpers**
   - Update `networkconfig_migration.rs` to use canonical
   - Add conversion functions if needed

3. **Remove Old Imports**
   ```bash
   # Find and remove old import patterns
   rg "unified_config_consolidation::StandardDomainConfig" --type rust
   ```

### **Phase 4: Validation** (2 hours)

1. **Build Verification**
   ```bash
   cargo check --workspace
   cargo build --workspace
   ```

2. **Test Verification**
   ```bash
   cargo test --workspace
   ```

3. **Import Verification**
   ```bash
   # Should only find canonical imports
   rg "NetworkConfig" --type rust | grep "use.*canonical_master"
   ```

---

## 📋 **DETAILED MIGRATION CHECKLIST**

### **Preparation Tasks**
- [ ] Run usage audit scripts
- [ ] Document current field mappings
- [ ] Identify breaking changes
- [ ] Create backup branch: `git checkout -b config-migration-backup`
- [ ] Review canonical NetworkConfig structure

### **nestgate-network Migration**
- [ ] Update `src/types.rs` - Change NetworkConfig type alias
- [ ] Update `src/config.rs` - Update imports and helpers
- [ ] Update `src/unified_network_config/network_core.rs` - Deprecate UnifiedNetworkConfig
- [ ] Update `src/lib.rs` - Update re-exports
- [ ] Run tests: `cargo test -p nestgate-network`
- [ ] Verify no compilation errors

### **nestgate-api Migration**
- [ ] Find all NetworkConfig usages
- [ ] Update imports to canonical
- [ ] Update function signatures
- [ ] Update handler configurations
- [ ] Run tests: `cargo test -p nestgate-api`
- [ ] Verify functionality

### **nestgate-core Cleanup**
- [ ] Deprecate `unified_types/network_config.rs`
- [ ] Deprecate `unified_final_config/domain_configs/network.rs`
- [ ] Merge duplicate `canonical/types/config_registry.rs` NetworkConfig
- [ ] Update migration helpers
- [ ] Run tests: `cargo test -p nestgate-core`

### **Remaining Crates**
- [ ] nestgate-mcp - Update NetworkConfig usages
- [ ] nestgate-automation - Update NetworkConfig usages
- [ ] nestgate-installer - Update NetworkConfig usages
- [ ] nestgate-bin - Update NetworkConfig usages
- [ ] Other crates as needed

### **Templates & Examples**
- [ ] Update `ecosystem-expansion/templates/` to reference canonical
- [ ] Update `examples/` configurations
- [ ] Add migration example to templates

### **Final Validation**
- [ ] `cargo check --workspace` - No errors
- [ ] `cargo build --workspace` - Successful build
- [ ] `cargo test --workspace` - All tests pass
- [ ] Verify imports: Only canonical NetworkConfig used
- [ ] Documentation updated

---

## 🔧 **MIGRATION HELPERS**

### **Automatic Migration Script**

Create `scripts/migrate-network-config.sh`:
```bash
#!/bin/bash
# Migrate NetworkConfig imports to canonical

echo "Migrating NetworkConfig to canonical..."

# Pattern 1: Update StandardDomainConfig imports
find code/crates -name "*.rs" -type f -exec sed -i \
  's/unified_config_consolidation::StandardDomainConfig/config::canonical_master::domains::network::CanonicalNetworkConfig/g' {} \;

# Pattern 2: Update type aliases
find code/crates -name "*.rs" -type f -exec sed -i \
  's/pub type NetworkConfig = StandardDomainConfig<NetworkExtensions>;/pub type NetworkConfig = CanonicalNetworkConfig;/g' {} \;

echo "Migration complete. Run 'cargo check --workspace' to verify."
```

### **Validation Script**

Create `scripts/validate-network-config-migration.sh`:
```bash
#!/bin/bash
# Validate NetworkConfig migration

echo "=== NetworkConfig Migration Validation ==="

# Check for old imports
OLD_IMPORTS=$(rg "StandardDomainConfig<NetworkExtensions>" --type rust | wc -l)
echo "Old import patterns remaining: $OLD_IMPORTS"

# Check for canonical imports
CANONICAL_IMPORTS=$(rg "CanonicalNetworkConfig" --type rust | wc -l)
echo "Canonical imports found: $CANONICAL_IMPORTS"

# Check for deprecated patterns
DEPRECATED=$(rg "unified_config_consolidation" --type rust | wc -l)
echo "Deprecated pattern usages: $DEPRECATED"

if [ $OLD_IMPORTS -eq 0 ] && [ $DEPRECATED -eq 0 ]; then
    echo "✅ Migration validation PASSED"
    exit 0
else
    echo "⚠️  Migration incomplete"
    exit 1
fi
```

---

## 📊 **SUCCESS CRITERIA**

### **Definition of Success**

- [ ] All NetworkConfig variants consolidated to 1 canonical
- [ ] All crates using CanonicalNetworkConfig
- [ ] No remaining StandardDomainConfig<NetworkExtensions> usages
- [ ] All tests passing
- [ ] No compilation errors
- [ ] Documentation updated

### **Metrics**

| **Metric** | **Before** | **Target** | **Verification** |
|------------|------------|-----------|------------------|
| NetworkConfig variants | 33+ | 1 | `rg "pub.*NetworkConfig" --type rust \| wc -l` |
| Canonical imports | ~10% | 100% | `rg "CanonicalNetworkConfig" --type rust \| wc -l` |
| Deprecated imports | ~90% | 0% | `rg "StandardDomainConfig" --type rust \| wc -l` |
| Build status | Errors | Clean | `cargo check --workspace` |
| Test status | Unknown | Pass | `cargo test --workspace` |

---

## ⚠️ **POTENTIAL ISSUES & SOLUTIONS**

### **Issue 1: Field Name Mismatches**

**Problem**: Old NetworkConfig might have different field names than canonical

**Solution**:
```rust
// Create conversion functions
impl From<LegacyNetworkConfig> for CanonicalNetworkConfig {
    fn from(legacy: LegacyNetworkConfig) -> Self {
        Self {
            api: NetworkApiConfig {
                host: legacy.host,
                port: legacy.port,
                // Map fields
            },
            // ... map other fields
        }
    }
}
```

### **Issue 2: Missing Fields in Canonical**

**Problem**: Some variants have fields not in canonical

**Solution**:
1. Add missing fields to canonical (if they're truly network-wide)
2. Create crate-specific extensions (if crate-specific)
3. Document why fields were added/omitted

### **Issue 3: Breaking Changes for Downstream**

**Problem**: Changing NetworkConfig breaks external users

**Solution**:
1. Deprecate old types (don't remove immediately)
2. Provide migration guide
3. Add compatibility layer temporarily
4. Version bump to indicate breaking change

---

## 📅 **TIMELINE**

### **Day 1 (Week 2, Day 1)** - 4 hours
- ✅ Preparation (2 hours)
- ✅ nestgate-network migration (2 hours)

### **Day 2 (Week 2, Day 2)** - 4 hours
- ✅ nestgate-api migration (2 hours)
- ✅ Remaining crates migration (1 hour)
- ✅ Validation & cleanup (1 hour)

**Total Estimated Time**: 8 hours over 2 days

---

## 🎯 **NEXT ACTIONS**

1. **Review this migration map** with team
2. **Create backup branch** before starting migration
3. **Run audit scripts** to gather current state data
4. **Begin Phase 1** (Preparation) when ready
5. **Follow checklist** systematically

---

## 📞 **SUPPORT & REFERENCES**

**Related Documents**:
- `CANONICAL_CONFIG_DECISION.md` - Why canonical_master is THE system
- `UNIFICATION_ANALYSIS_REPORT.md` - Full codebase analysis
- `UNIFICATION_CHECKLIST.md` - Weekly task tracking

**Migration Patterns**:
- See `docs/unification/NETWORKCONFIG_MIGRATION_ANALYSIS.md` for detailed patterns
- See `code/crates/nestgate-core/src/config/migration_helpers/` for helper functions

**Questions?** See the comprehensive guides or create an issue for discussion.

---

**Created**: September 30, 2025  
**Status**: 📋 **Ready for Week 2, Day 1-2**  
**Next Review**: After NetworkConfig migration complete

---

*NetworkConfig Migration: From 33+ variants to 1 canonical source of truth* 