# PER-CRATE MIGRATION PLAN

This document provides specific migration steps for each of the 15 crates.

## Migration Pattern

For each crate, follow this pattern:

### 1. Identify Local Config Structs
```bash
# Find config structs in the crate
rg "pub struct.*Config" code/crates/nestgate-{CRATE}/src/
```

### 2. Determine if Extension Needed
- If the config is truly crate-specific → Create extension
- If the config duplicates canonical → Remove entirely

### 3. Update Imports
```rust
// OLD
pub struct ApiConfig { ... }

// NEW
use nestgate_core::config::canonical_master::{
    NestGateCanonicalConfig,
    domains::ApiDomainConfig,
};

// Only if truly needed:
pub struct ApiExtensions {
    // Crate-specific fields only
}
```

### 4. Update Usage Sites
- Replace local config with canonical config
- Update method signatures
- Update initialization code

---

## Crate-Specific Plans

### nestgate-api

**Current Config Files**:
- `src/config.rs` - Defines ApiConfig
- `src/unified_api_config/` - Directory of config modules

**Analysis**:
- Most fields already exist in canonical ApiDomainConfig
- Some API-specific routing config may need extension

**Action**:
1. Remove `ApiConfig` struct
2. Import `ApiDomainConfig` from canonical_master
3. Create `ApiExtensions` for routing-specific config if needed
4. Update all usage sites

**Estimated Effort**: 2-3 hours

---

### nestgate-network

**Current Config Files**:
- `src/config.rs` - Defines NetworkConfig
- `src/types.rs` - Defines NetworkConfig again (duplicate!)
- `src/unified_network_config/` - Another network config

**Analysis**:
- THREE NetworkConfig definitions in one crate!
- Clear consolidation target
- Most fields exist in canonical NetworkServicesDomainConfig

**Action**:
1. Remove all three NetworkConfig definitions
2. Import NetworkServicesDomainConfig from canonical_master
3. Create NetworkExtensions ONLY if needed
4. Update all usage sites

**Estimated Effort**: 3-4 hours

---

### nestgate-zfs

**Current Config Files**:
- `src/config.rs` - Defines ZfsConfig
- Multiple config structs in various modules

**Analysis**:
- ZFS-specific config is appropriate
- Should extend canonical StorageDomainConfig

**Action**:
1. Import StorageDomainConfig from canonical_master
2. Keep ZfsExtensions for ZFS-specific features
3. Ensure no duplication with canonical storage config
4. Update all usage sites

**Estimated Effort**: 2-3 hours

---

### nestgate-mcp

**Current Config Files**:
- `src/config/` - Directory with MCP config modules

**Analysis**:
- MCP-specific config is appropriate
- Should use canonical McpDomainConfig

**Action**:
1. Import McpDomainConfig from canonical_master
2. Keep MCP-specific extensions
3. Remove any duplicated fields
4. Update all usage sites

**Estimated Effort**: 2 hours

---

### nestgate-automation

**Current Config Files**:
- `src/types/mod.rs` - Defines AutomationConfig

**Analysis**:
- Should use canonical AutomationDomainConfig

**Action**:
1. Remove AutomationConfig
2. Import AutomationDomainConfig from canonical_master
3. Create extensions only if needed
4. Update usage sites

**Estimated Effort**: 1-2 hours

---

### nestgate-installer

**Current Config Files**:
- `src/config.rs` - Defines InstallerConfig

**Analysis**:
- Installer-specific, but may duplicate system config

**Action**:
1. Review overlap with canonical InstallerDomainConfig
2. Remove duplicates
3. Keep installer-specific extensions
4. Update usage sites

**Estimated Effort**: 2 hours

---

### nestgate-fsmonitor

**Current Config Files**:
- `src/config.rs` - Defines FsMonitorConfig

**Analysis**:
- Should use canonical FsMonitorDomainConfig

**Action**:
1. Import FsMonitorDomainConfig from canonical_master
2. Create extensions if needed
3. Update usage sites

**Estimated Effort**: 1-2 hours

---

### nestgate-performance

**Current Config Files**:
- `src/config/` - Performance monitoring config

**Analysis**:
- Should use canonical PerformanceDomainConfig

**Action**:
1. Import PerformanceDomainConfig
2. Remove duplicates
3. Update usage sites

**Estimated Effort**: 1-2 hours

---

### Other Crates

**nestgate-bin**, **nestgate-canonical**, **nestgate-middleware**, **nestgate-nas**:

Similar pattern:
1. Identify local configs
2. Map to canonical equivalents
3. Create extensions if truly needed
4. Remove duplicates

**Total Estimated Effort**: 6-8 hours

---

## Total Migration Timeline

**Week 3 Schedule**:
- Day 1: nestgate-api, nestgate-network (largest configs)
- Day 2: nestgate-zfs, nestgate-mcp
- Day 3: nestgate-automation, nestgate-installer, nestgate-fsmonitor
- Day 4: nestgate-performance + 4 smaller crates
- Day 5: Testing, fixes, documentation

**Success Criteria**:
- All 15 crates import from canonical_master
- Zero local config struct duplicates
- All tests pass
- Documentation updated
