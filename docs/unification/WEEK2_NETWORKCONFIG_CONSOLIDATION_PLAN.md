# NetworkConfig Consolidation Plan
**Date**: September 30, 2025  
**Goal**: Consolidate 32+ NetworkConfig variants → 1 canonical

---

## 📊 CURRENT STATE

### Canonical NetworkConfig (THE Target)
**Location**: `code/crates/nestgate-core/src/config/canonical_primary/domains/network/mod.rs`
**Name**: `CanonicalNetworkConfig`
**Status**: ✅ This is THE canonical config to use

### Variants Found (32+)

#### Category 1: Legacy (Deprecated - Safe to Remove)
- LegacyNetworkConfig (10+ instances)
- LegacyNetworkConfigBuilder
- LegacyNetworkConfigFragment

#### Category 2: Migration Helpers (Temporary)
- EnhancedNetworkConfigFragment
- DynamicNetworkConfig
- networkconfig_migration.rs variants

#### Category 3: Duplicates in Other Crates
- nestgate-network/src/types.rs
- nestgate-canonical/src/types.rs
- nestgate-api/src/ecoprimal_sdk/config.rs

#### Category 4: Old Unified Types
- UnifiedNetworkConfig
- MinimalNetworkConfig
- FuzzNetworkConfigData

---

## 🎯 CONSOLIDATION STRATEGY

### Phase 1: Verify Canonical (✅ Done)
- Canonical exists in `canonical_primary/domains/network`
- Well-structured and comprehensive
- Ready to be THE single source

### Phase 2: Update Exports (Next)
1. Ensure canonical_primary exports CanonicalNetworkConfig
2. Add type alias: `NetworkConfig = CanonicalNetworkConfig`
3. Update config/mod.rs to re-export

### Phase 3: Migration (This Week)
1. Update crate dependencies to use canonical
2. Replace local NetworkConfig definitions
3. Add crate-specific extensions if needed

### Phase 4: Cleanup (Week 4)
1. Remove deprecated LegacyNetworkConfig variants
2. Remove migration helpers
3. Remove old unified types

---

## 📝 ACTION ITEMS

### Today
- [x] Analyze all NetworkConfig variants
- [ ] Read canonical CanonicalNetworkConfig structure
- [ ] Create migration script
- [ ] Test in one crate first

### This Week  
- [ ] Update nestgate-network crate
- [ ] Update nestgate-api crate
- [ ] Update other crates using NetworkConfig
- [ ] Validate all imports work

---

## ✅ SUCCESS CRITERIA

- All production code uses `canonical_primary::domains::network::CanonicalNetworkConfig`
- Zero local NetworkConfig definitions (except deprecated)
- All tests pass
- Clean compilation

---

**Status**: 📋 PLAN COMPLETE - READY TO EXECUTE
