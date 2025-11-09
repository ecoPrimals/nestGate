# NetworkConfig Migration Analysis
**Date**: September 30, 2025  
**Target Crate**: nestgate-network

---

## 📊 CURRENT STATE

### nestgate-network Current Usage
**Location**: `code/crates/nestgate-network/src/`

**Current NetworkConfig Type**:
```rust
// types.rs:18
pub type NetworkConfig = StandardDomainConfig<NetworkExtensions>;
```

**Current Imports**:
- `nestgate_core::unified_config_consolidation::StandardDomainConfig`
- `nestgate_core::unified_config_primary::NestGatePrimaryConfig`

**Problem**: Using OLD unified config system, not the NEW canonical_primary

---

## 🎯 CANONICAL TARGET

**THE Canonical NetworkConfig**:
```rust
// canonical_primary/domains/network/mod.rs:48
pub struct CanonicalNetworkConfig {
    pub api: NetworkApiConfig,
    pub orchestration: NetworkOrchestrationConfig,
    pub protocols: NetworkProtocolConfig,
    pub vlan: NetworkVlanConfig,
    pub discovery: NetworkDiscoveryConfig,
    pub performance: NetworkPerformanceConfig,
    pub security: NetworkSecurityConfig,
    pub monitoring: NetworkMonitoringConfig,
    pub environment: NetworkEnvironmentConfig,
}
```

---

## 🔄 MIGRATION STRATEGY

### Phase 1: Update Type Aliases
1. Change type alias in `types.rs` to use CanonicalNetworkConfig
2. Keep backward-compatible extensions if needed

### Phase 2: Update Imports
1. Replace `unified_config_consolidation` imports
2. Use `canonical_primary::domains::network` instead
3. Update `config.rs` module

### Phase 3: Update Extensions
1. Map `NetworkExtensions` fields to CanonicalNetworkConfig sub-modules
2. Ensure all functionality is preserved
3. Add migration helpers if needed

### Phase 4: Validate
1. Run cargo check
2. Run tests
3. Document changes

---

## 📋 FILES TO MODIFY

### Priority 1 (Core Type Aliases)
1. `src/types.rs` - Update NetworkConfig type alias
2. `src/config.rs` - Update to use canonical_primary
3. `src/lib.rs` - Update re-exports

### Priority 2 (Extensions & Compatibility)
4. `src/unified_network_config/` - Map to canonical or deprecate
5. `src/unified_network_extensions/` - Map to canonical or deprecate

### Priority 3 (Usage Sites)
6. Update all files using NetworkConfig to be compatible

---

## ✅ SUCCESS CRITERIA

- [ ] NetworkConfig type alias points to CanonicalNetworkConfig
- [ ] All imports from canonical_primary
- [ ] Zero compilation errors
- [ ] All tests pass
- [ ] Backward compatibility maintained where needed

---

**Status**: 📋 ANALYSIS COMPLETE - READY TO MIGRATE
