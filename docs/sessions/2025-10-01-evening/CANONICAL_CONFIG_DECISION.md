# 📋 **Canonical Configuration System Decision**

**Date**: September 30, 2025  
**Status**: ✅ **APPROVED - Active Migration Phase**  
**Last Updated**: September 30, 2025 (Build fixes complete, consolidation phase begins)  
**Decision**: Use `config/canonical_master/NestGateCanonicalConfig` as THE single source of truth

---

## 🎯 **THE Canonical Config System**

**Location**: `code/crates/nestgate-core/src/config/canonical_master/`

**Primary Type**: `NestGateCanonicalConfig`

**Structure**:
```rust
pub struct NestGateCanonicalConfig {
    pub domains: ConsolidatedDomainConfigs,
    pub environment: Environment,
    pub metadata: ConfigMetadata,
}

pub struct ConsolidatedDomainConfigs {
    pub zfs: ZfsDomainConfig,
    pub api: ApiDomainConfig,
    pub mcp: McpDomainConfig,
    pub network_services: NetworkServicesDomainConfig,
    pub automation: AutomationDomainConfig,
    pub fsmonitor: FsMonitorDomainConfig,
    pub installer: InstallerDomainConfig,
    pub performance: PerformanceDomainConfig,
    pub binary: BinaryDomainConfig,
}
```

---

## ✅ **Why This System?**

1. **Comprehensive Domain Coverage**: Covers all 15 crates systematically
2. **Clean Extension Pattern**: Each domain has clear extension points
3. **Best Aligned with Crate Structure**: Maps directly to our crate organization
4. **Already in Use**: Examples and tests already reference this system
5. **Migration Framework**: Has built-in migration helpers

---

## 🔴 **DEPRECATED - Mark for Removal**

The following config systems are **DEPRECATED** and should NOT be used:

### **1. config/canonical/types.rs - CanonicalConfig**
```rust
// ❌ DEPRECATED
pub struct CanonicalConfig { ... }
```
**Reason**: Too generic, lacks domain-specific organization

### **2. unified_config_consolidation.rs - StandardDomainConfig<T>**
```rust
// ❌ DEPRECATED
pub struct StandardDomainConfig<T> { ... }
```
**Reason**: Overly complex generic pattern, harder to maintain

### **3. config/domains_legacy.rs**
```rust
// ✅ ALREADY REMOVED (Sep 30, 2025)
```
**Status**: Deleted - was not imported anywhere

### **4. Per-Crate Config Duplicates**
Each crate should **EXTEND** the canonical config, not duplicate it:

```rust
// ❌ DON'T DO THIS
// In nestgate-api/src/config.rs
pub struct ApiConfig { ... }  // Duplicates domain config

// ✅ DO THIS INSTEAD
// In nestgate-api/src/config.rs
pub struct ApiExtensions {
    // Only API-specific extensions that don't belong in core
}
```

---

## 📋 **Migration Guidelines**

### **For Core Developers**

1. **Use THE canonical config**:
   ```rust
   use nestgate_core::config::canonical_master::NestGateCanonicalConfig;
   ```

2. **Access domain configs**:
   ```rust
   let config = NestGateCanonicalConfig::load()?;
   let api_config = &config.domains.api;
   let zfs_config = &config.domains.zfs;
   ```

3. **Create extensions ONLY when necessary**:
   ```rust
   // Only create domain extensions for truly crate-specific needs
   pub struct ApiDomainExtensions {
       pub advanced_routing: AdvancedRoutingConfig,
       // ... other API-only features
   }
   ```

### **For Crate Maintainers**

**DO**:
- ✅ Use `NestGateCanonicalConfig` from nestgate-core
- ✅ Extend with domain-specific configs in your crate
- ✅ Document any extensions you add
- ✅ Update your crate's README when using canonical config

**DON'T**:
- ❌ Create new Config structs that duplicate domain configs
- ❌ Fork the canonical config into your own version
- ❌ Use deprecated config systems
- ❌ Hardcode values that belong in config

---

## 🗺️ **Configuration Hierarchy**

```
NestGateCanonicalConfig (THE canonical)
├── domains: ConsolidatedDomainConfigs
│   ├── zfs: ZfsDomainConfig           → nestgate-zfs uses this
│   ├── api: ApiDomainConfig           → nestgate-api uses this
│   ├── mcp: McpDomainConfig           → nestgate-mcp uses this
│   ├── network_services               → nestgate-network uses this
│   ├── automation                     → nestgate-automation uses this
│   ├── fsmonitor                      → nestgate-fsmonitor uses this
│   ├── installer                      → nestgate-installer uses this
│   ├── performance                    → nestgate-performance uses this
│   └── binary                         → nestgate-bin uses this
├── environment: Environment
└── metadata: ConfigMetadata
```

---

## 📊 **Progress Tracking**

### **Completed** ✅
- [x] Decision documented
- [x] Deprecated systems identified
- [x] domains_legacy.rs removed
- [x] Duplicate constants consolidated
- [x] Shared constants module created

### **In Progress** 🔄
- [x] Build syntax errors fixed (Week 1, Day 1 - COMPLETE)
- [ ] NetworkConfig consolidation (33+ duplicates → 1 canonical) - Week 2, Day 1-2
- [ ] StorageConfig consolidation (30+ duplicates → 1 canonical) - Week 2, Day 3-4
- [ ] SecurityConfig consolidation (20+ duplicates → 1 canonical) - Week 2, Day 5
- [ ] Update all 15 crates to use canonical - Week 3

### **Next Steps** 📋
1. **Week 1, Day 2-3** (Current): Documentation & Planning
   - ✅ Update this document
   - [ ] Create NetworkConfig migration map
   - [ ] Set up validation scripts
   - [ ] Document consolidation patterns

2. **Week 2**: Configuration Consolidation
   - Consolidate NetworkConfig duplicates (see NETWORKCONFIG_MIGRATION_MAP.md)
   - Consolidate StorageConfig duplicates
   - Consolidate SecurityConfig duplicates
   
3. **Week 3**: Universal Adoption
   - Update crate dependencies to use canonical
   - Migrate error systems
   - Validate all functionality

4. **Week 4**: Cleanup
   - Remove LegacyNetworkConfig definitions
   - Remove migration helpers after all migrations complete
   - Remove deprecated markers

---

## 🔍 **Validation**

After migration, verify:
```bash
# Should find ONLY canonical config
grep -r "pub struct.*NetworkConfig" code/crates/nestgate-core/src/config/canonical_master

# Should find NO legacy configs
grep -r "LegacyNetworkConfig\|domains_legacy" code/crates --include="*.rs"

# Should use shared constants
grep -r "pub const MODULE_VERSION" code/crates/nestgate-core/src/canonical_types
# (Should show "use crate::constants::shared::MODULE_VERSION")
```

---

## 📞 **Questions?**

If you're unsure whether to:
- Use canonical config → **YES, always**
- Create a new Config struct → **Probably NO** (extend canonical instead)
- Duplicate a domain config → **NO, never**

See: `UNIFICATION_ASSESSMENT_REPORT.md` for detailed migration guidance.

---

**Next Document**: See `CONSOLIDATION_PROGRESS.md` for ongoing work tracking  
**Architecture**: See `ARCHITECTURE_OVERVIEW.md` for system design  
**Assessment**: See `UNIFICATION_ASSESSMENT_REPORT.md` for full analysis

---

*Last Updated: September 30, 2025*  
*Status: Active Implementation* 