# NetworkConfig Consolidation Baseline

**Date**: September 30, 2025  
**Phase**: Week 2, Day 1 - Pre-Migration Baseline

## 📊 Current State

### Metrics
- **StandardDomainConfig usages**: 67 (needs migration)
- **CanonicalNetworkConfig usages**: 42 (good, but needs more)
- **unified_config_consolidation imports**: 8 (deprecated)
- **Total files with NetworkConfig**: 56
- **Canonical file**: ✅ EXISTS (136 lines)

### Status
⚠️  **Migration Required**: StandardDomainConfig > CanonicalNetworkConfig

## 🎯 Priority Files for Migration

### High Priority (Crate-level)
1. `code/crates/nestgate-network/src/config.rs` - Uses StandardDomainConfig
2. `code/crates/nestgate-network/src/types.rs` - Uses StandardDomainConfig
3. `code/crates/nestgate-mcp/src/config.rs` - Uses unified_config_consolidation
4. `code/crates/nestgate-automation/` - Uses StandardDomainConfig

### Medium Priority (Core cleanup)
- Migration helpers (can stay until Week 4)
- Unified config consolidation module
- Legacy config definitions

### Low Priority (Test/Example)
- Test configurations
- Example files
- Templates

## �� Target State (Post-Migration)

- StandardDomainConfig usages: 67 → 0
- CanonicalNetworkConfig usages: 42 → 100+
- unified_config_consolidation: 8 → 0
- Total files: 56 → ~20 (consolidation)

## 📅 Timeline

- **Day 1 AM**: nestgate-network migration
- **Day 1 PM**: nestgate-mcp, nestgate-automation  
- **Day 2**: Validation and cleanup

---

*Baseline established: Ready for migration*
