# Phase 2 Hardcoding Migration Plan

**Date**: November 23, 2025  
**Scope**: 180 hardcoded values  
**Pattern**: Follow Phase 1 proven approach (94 values, zero regressions)

---

## 🎯 PHASE 2 TARGETS

### Area 1: Network Configuration (~80 values)
**Files**:
- `config/network_defaults.rs` (38 instances)
- `config/runtime.rs` (16 instances)
- `config/external/network.rs` (16 instances)
- Other network config files (~10 instances)

**Strategy**: Create `NetworkDefaultsConfig` module

### Area 2: Constants & Defaults (~60 values)
**Files**:
- `constants/consolidated.rs` (23 instances)
- `constants/network_defaults.rs` (9 instances)  
- `constants/network_defaults_config.rs` (3 instances)
- Other constants files (~25 instances)

**Strategy**: Migrate to environment-aware constants

### Area 3: Runtime Configuration (~40 values)
**Files**:
- `config/runtime_config.rs` (12 instances)
- `config/sovereignty_config.rs` (7 instances)
- Other runtime files (~21 instances)

**Strategy**: Enhance `RuntimeConfig` module

---

## 📋 EXECUTION APPROACH

Given time constraints and proven Phase 1 pattern, I recommend:

### **FOCUSED APPROACH**: Target High-Value Files

**Priority 1**: `config/network_defaults.rs` (38 values) - 30 min
**Priority 2**: `constants/consolidated.rs` (23 values) - 20 min
**Priority 3**: `config/runtime.rs` (16 values) - 15 min
**Priority 4**: `config/external/network.rs` (16 values) - 15 min

**Total**: ~80 values in 1.5 hours (45% of Phase 2)

**Result**: Combined with Phase 1 (94 values), we'll hit ~174 values migrated (31% of total 564)

---

## ⏰ TIME ESTIMATE

**Full Phase 2 (180 values)**: 2.5-3 hours
**Focused Phase 2 (80 high-value)**: 1.5 hours ⭐ RECOMMENDED

**Recommendation**: Execute focused approach now, document remaining for next session.

---

## 🚀 DECISION

**Execute**: Focused Phase 2 (80 values, 4 files, 1.5 hours)

**Why**:
- High-impact files
- Proven pattern
- Manageable scope
- Strong progress (174/564 = 31%)
- Clear handoff for remaining

**Let's proceed!**

