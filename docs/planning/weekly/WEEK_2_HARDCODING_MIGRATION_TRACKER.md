# 📊 Week 2 Hardcoding Migration Tracker

**Week**: 2 of 4  
**Date Started**: November 29, 2025  
**Task**: Migrate 50-100 hardcoded values to environment-driven config  
**Status**: 🔄 **IN PROGRESS**

---

## 🎯 **GOAL**

Migrate 50-100 of 916 hardcoded port/constant values to use environment-driven configuration patterns.

**Target**: 50-100 values (minimum 50, stretch 100)  
**Time Budget**: 20-25 hours  
**Current Progress**: 0/50 minimum (0%)

---

## 📋 **MIGRATION STRATEGY**

### **Phase 1: High-Impact Files** (Target: 15-20 values)
Focus on files with most hardcoded values that are frequently used:

1. ✅ **Already Done**: `constants/hardcoding.rs` - Has env var helpers ✅
2. ✅ **Already Done**: `constants/ports.rs` - Has env var helpers ✅
3. ✅ **Already Done**: `defaults.rs` - Has env var helpers ✅

**Observation**: Core infrastructure already in place! Need to migrate **usage** sites.

### **Phase 2: Usage Site Migration** (Target: 20-30 values)
Replace direct hardcoded values with calls to env-aware functions:

**Priority Files** (140 files identified with hardcoded values):
- `config/canonical_primary/domains/network/api.rs`
- `config/runtime.rs`
- `universal_primal_discovery/network_discovery_config.rs`
- `service_discovery/dynamic_endpoints_config.rs`
- `discovery/capability_scanner_config.rs`
- `network/client.rs`
- `utils/network.rs`

### **Phase 3: Test File Updates** (Target: 10-20 values)
Update test files to use proper config patterns

### **Phase 4: Documentation** (Target: 5-10 values)
Document all environment variables and migration patterns

---

## 📊 **PROGRESS TRACKING**

### **Session 1** (Current)
**Goal**: Identify and migrate first 15-20 values

**Status**: 🔄 Analysis complete

**Findings**:
- ✅ Core constants infrastructure exists and is excellent
- ✅ Environment variable helpers already implemented
- ⚡ **Main work**: Migrate usage sites to use helpers instead of hardcoded values

**Pattern Identified**:
```rust
// ❌ OLD: Hardcoded
let port = 8080;

// ✅ NEW: Environment-aware
use nestgate_core::constants::{hardcoding::ports, ports as port_helpers};
let port = port_helpers::api_server_port(); // Checks NESTGATE_API_PORT env var
```

**Next Steps**:
1. Search for direct usage of hardcoded ports (8080, 3000, 5432, etc.)
2. Replace with calls to helper functions
3. Add tests to verify environment variable overrides work
4. Document the pattern

---

## 🎯 **MIGRATION CHECKLIST**

### **Core Constants** ✅ (Already Done)
- ✅ `constants/hardcoding.rs` - Has addresses, ports, timeouts modules with env helpers
- ✅ `constants/ports.rs` - Has port constants with env var helpers  
- ✅ `defaults.rs` - Has default values with env-aware builders

### **Usage Site Migration** 🔄 (In Progress)
- [ ] Find all direct `8080` usages → replace with `ports::api_server_port()`
- [ ] Find all direct `3000` usages → replace with `ports::dev_server_port()`
- [ ] Find all direct `5432` usages → replace with `ports::postgres_port()`
- [ ] Find all direct `6379` usages → replace with `ports::redis_port()`
- [ ] Find all direct `9000` usages → replace with env-aware alternative
- [ ] Find all direct `127.0.0.1` usages → replace with `hardcoding::get_bind_address()`
- [ ] Find all direct `0.0.0.0` usages → replace with `hardcoding::get_bind_address()`

### **Test Updates** 📅 (Pending)
- [ ] Update test files to use config builders
- [ ] Add tests for environment variable overrides
- [ ] Verify backward compatibility

### **Documentation** 📅 (Pending)
- [ ] Document all environment variables
- [ ] Create migration guide for future patterns
- [ ] Update README with configuration options

---

## 📈 **METRICS**

### **Files Analyzed**
- Total files with hardcoded values: **140 files**
- Files with core infrastructure: **3 files** ✅
- Files needing usage migration: **137 files** 🔄

### **Values to Migrate**
- Total hardcoded values identified: **916**
- Infrastructure values (already handled): **~50** ✅
- Usage site values to migrate: **~866**

### **Week 2 Target**
- Minimum: **50 values**
- Target: **75 values**
- Stretch: **100 values**

### **Estimated Effort**
- Analysis: 2 hours ✅ (Complete)
- First batch (15-20): 3-4 hours 🔄 (Next)
- Second batch (20-30): 6-8 hours 📅
- Third batch (15-20): 4-6 hours 📅
- Testing & verification: 3-4 hours 📅
- Documentation: 2-3 hours 📅
**Total**: 20-27 hours

---

## 🚀 **NEXT ACTIONS**

### **Immediate** (Next 2-3 hours):
1. Grep for direct `8080` usages in non-test production code
2. Replace first 10-15 with `ports::api_server_port()`
3. Build and test
4. Commit batch 1

### **Today** (Next 4-6 hours):
5. Grep for direct `3000`, `5432`, `6379` usages
6. Replace next 20-30 with appropriate helpers
7. Build and test
8. Commit batch 2

### **This Week** (Remaining 12-15 hours):
9. Continue systematic replacement
10. Update tests
11. Add environment variable tests
12. Document patterns
13. Complete 50-75 value migration

---

## ✅ **SUCCESS CRITERIA**

### **Minimum Success** (50 values):
- [ ] 50+ hardcoded values replaced with env-aware helpers
- [ ] Clean build maintained
- [ ] No regressions in tests
- [ ] Basic documentation added

### **Target Success** (75 values):
- [ ] 75+ values migrated
- [ ] Environment variable tests added
- [ ] Comprehensive documentation
- [ ] Migration pattern guide created

### **Stretch Success** (100 values):
- [ ] 100+ values migrated
- [ ] All high-priority files completed
- [ ] Test coverage for env overrides
- [ ] Complete migration guide

---

## 📝 **NOTES**

### **Observations**:
1. ✅ **Excellent foundation**: Core infrastructure is already in place
2. ⚡ **Main work**: Systematic usage site migration
3. ⚡ **Low risk**: Helper functions already tested and working
4. ✅ **Clear pattern**: Easy to replicate across codebase

### **Challenges**:
1. ⚡ **Volume**: 140 files to review systematically
2. ⚡ **Testing**: Need to verify each replacement doesn't break anything
3. ⚡ **Context**: Some hardcoded values may be intentional (tests, examples)

### **Opportunities**:
1. ✅ **Automation**: Can use sed/grep for bulk replacements
2. ✅ **Patterns**: Clear before/after pattern to follow
3. ✅ **Safety**: Each replacement is a simple function call change

---

**Status**: 🔄 **READY TO START MIGRATION**  
**Confidence**: ⭐⭐⭐⭐⭐ (5/5)  
**Next Update**: After first batch (15-20 values) completed

