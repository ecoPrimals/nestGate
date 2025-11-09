# 🎯 Config Struct Consolidation - Phase 1 Execution Plan

**Date**: November 9, 2025  
**Phase**: 1 of 4 - Generic Config Renaming  
**Status**: 🚀 READY TO EXECUTE  
**Target**: Rename 109 generic "Config" structs to add domain context

---

## 📊 Executive Summary

**Problem**: 109 structs named simply `Config` without domain context  
**Impact**: High cognitive load, namespace conflicts, unclear purpose  
**Solution**: Rename to `{Domain}{Purpose}Config` pattern  
**Timeline**: 4 weeks  
**Effort**: ~30 configs per week

---

## 🎯 Goals

### Primary Goal
Eliminate ALL generic `Config` struct names by adding clear domain context.

### Success Criteria
- [ ] Zero structs named just "Config"
- [ ] All configs have domain-specific names
- [ ] Build stays GREEN throughout
- [ ] All tests pass
- [ ] Documentation updated

### Quality Metrics
- **Clarity**: Each config name immediately indicates its domain
- **Consistency**: Follow `{Domain}{Purpose}Config` pattern
- **No Regressions**: Zero test failures, zero new errors
- **Build Time**: Maintain or improve current build time

---

## 📋 Naming Convention

### Pattern: `{Domain}{Purpose}Config`

**Examples**:
```rust
// BEFORE (generic - unclear):
pub struct Config { ... }

// AFTER (domain-specific - clear):
pub struct NetworkCacheConfig { ... }
pub struct StoragePoolConfig { ... }
pub struct SecurityAuthConfig { ... }
pub struct MonitoringMetricsConfig { ... }
```

### Domain Extraction

Determine domain from file path:
```
code/crates/nestgate-core/src/network/cache.rs
                                  ^^^^^^^^ ^^^^^^
                                  Domain   Purpose
→ NetworkCacheConfig
```

### Special Cases

**1. Nested Modules**:
```
code/crates/nestgate-core/src/monitoring/alerts/manager.rs
                                  ^^^^^^^^^^^ ^^^^^^^ ^^^^^^^
                                  Domain      Sub      Purpose
→ MonitoringAlertManagerConfig (include sub-domain if needed)
OR → MonitoringAlertsConfig (if sub-domain is obvious)
```

**2. Generic Purpose**:
```
code/crates/nestgate-core/src/network/mod.rs
→ NetworkConfig (domain is sufficient if it's the main config)
```

**3. Very Specific Purpose**:
```
code/crates/nestgate-core/src/network/tls/cert_validator.rs
→ TlsCertValidatorConfig (be specific)
```

---

## 🗓️ Week-by-Week Plan

### Week 1: Network & Storage (Nov 11-15)

**Target**: 30 configs  
**Focus**: High-visibility modules

**Modules**:
- `network/` - ~15 configs
  - cache.rs → NetworkCacheConfig
  - metrics.rs → NetworkMetricsConfig
  - compression.rs → NetworkCompressionConfig
  - security.rs → NetworkSecurityConfig
  - auth.rs → NetworkAuthConfig
  - tls.rs → NetworkTlsConfig
  - etc.

- `storage/` - ~15 configs
  - Similar pattern for storage modules

**Daily Goals**:
- Monday: 6 configs (network cache, metrics, compression)
- Tuesday: 6 configs (network security, auth, tls)
- Wednesday: 6 configs (network retry, timeout, pool)
- Thursday: 6 configs (storage modules)
- Friday: 6 configs (storage modules) + verify & document

---

### Week 2: Monitoring & Services (Nov 18-22)

**Target**: 30 configs  
**Focus**: Observability and service modules

**Modules**:
- `monitoring/` - ~15 configs
- `services/` - ~15 configs

**Approach**: Same as Week 1

---

### Week 3: Config, Traits & Utils (Nov 25-29)

**Target**: 30 configs  
**Focus**: Core infrastructure

**Modules**:
- `config/` - ~10 configs (meta-configs)
- `traits/` - ~10 configs
- `utils/` - ~10 configs

**Approach**: Same as Week 1

---

### Week 4: Remaining & Verification (Dec 2-6)

**Target**: 19 remaining configs + full verification

**Tasks**:
- Complete remaining configs
- Full workspace build verification
- Run complete test suite
- Update all documentation
- Create migration guide

---

## 🔧 Execution Process

### For Each Config Struct

#### Step 1: Identify Domain & Purpose
```bash
# Example: network/cache.rs line 25
# File path: code/crates/nestgate-core/src/network/cache.rs
# Line: pub struct Config { ... }

# Extract:
# Domain: network
# Purpose: cache
# New name: NetworkCacheConfig
```

#### Step 2: Rename Struct
```rust
// BEFORE:
pub struct Config {
    pub capacity: usize,
    pub ttl: Duration,
}

// AFTER:
pub struct NetworkCacheConfig {
    pub capacity: usize,
    pub ttl: Duration,
}
```

#### Step 3: Update All References

**In same file**:
```rust
// BEFORE:
impl Config {
    pub fn new() -> Self { ... }
}

impl Default for Config { ... }

// AFTER:
impl NetworkCacheConfig {
    pub fn new() -> Self { ... }
}

impl Default for NetworkCacheConfig { ... }
```

**In other files**:
```bash
# Find all usages
grep -r "cache::Config" code/crates/nestgate-core/src

# Update imports
# BEFORE: use crate::network::cache::Config;
# AFTER: use crate::network::cache::NetworkCacheConfig;
```

#### Step 4: Verify

```bash
# Build check
cargo check -p nestgate-core

# Run tests for affected module
cargo test -p nestgate-core --lib -- network::cache

# If GREEN, proceed to next config
```

---

## 📝 Tracking Document

### Template for Each Config

```markdown
### NetworkCacheConfig
- **Original**: Config (network/cache.rs:25)
- **New Name**: NetworkCacheConfig
- **Domain**: network
- **Purpose**: cache
- **Status**: ✅ Complete
- **References Updated**: 3 files
- **Tests**: ✅ Passing
- **Date**: Nov 11, 2025
```

---

## 🚨 Risk Mitigation

### Build Breaks
- **Risk**: References not updated → compilation errors
- **Mitigation**: 
  - Work in small batches (6 per day)
  - Verify build after each rename
  - Use compiler errors to find missed references

### Test Failures
- **Risk**: Tests hardcode config names
- **Mitigation**:
  - Run tests after each rename
  - Update test fixtures
  - Keep test coverage at 100%

### Merge Conflicts
- **Risk**: Long-running branch causes conflicts
- **Mitigation**:
  - Work in feature branch
  - Merge main frequently
  - Small, focused PRs per module

### External Crate Dependencies
- **Risk**: Other crates import configs
- **Mitigation**:
  - Check with `cargo tree`
  - Coordinate with other crate owners
  - Add type aliases for compatibility if needed

---

## 🔍 Verification Commands

### Find Generic Configs
```bash
# Count remaining generic configs
grep -rn "^pub struct Config[[:space:]]" code/crates/nestgate-core/src --include="*.rs" | wc -l

# Should decrease each day: 109 → 103 → 97 → ...
```

### Build Verification
```bash
# After each rename
cargo check -p nestgate-core

# After each batch (6 configs)
cargo check --workspace

# End of day
cargo test --workspace --lib
```

### Progress Tracking
```bash
# Daily status
echo "Configs renamed today: X"
echo "Total remaining: Y"
echo "On track: $(if [ Y -le Z ]; then echo YES; else echo NO; fi)"
```

---

## 📚 Documentation Updates

### Files to Update

1. **CONTRIBUTING.md** - Add config naming guidelines
```markdown
## Config Struct Naming

✅ **DO**: Use domain-specific names
```rust
pub struct NetworkCacheConfig { ... }
```

❌ **DON'T**: Use generic names
```rust
pub struct Config { ... }  // Too generic!
```
```

2. **ARCHITECTURE_OVERVIEW.md** - Document config organization

3. **CONFIG_STRUCT_INVENTORY_NOV_9_2025.md** - Update with progress

4. **Create MIGRATION_CONFIG_NAMES.md** - Guide for users

---

## 🎯 Success Milestones

### Week 1 Success
- [ ] 30 network & storage configs renamed
- [ ] Build GREEN
- [ ] All tests passing
- [ ] Documentation updated

### Week 2 Success
- [ ] 60 total configs renamed (50% complete)
- [ ] Build GREEN
- [ ] All tests passing
- [ ] Mid-point review complete

### Week 3 Success
- [ ] 90 total configs renamed (82% complete)
- [ ] Build GREEN
- [ ] All tests passing
- [ ] Almost done review

### Week 4 Success (FINAL)
- [ ] 109 configs renamed (100% complete) ✅
- [ ] Build GREEN
- [ ] All tests passing
- [ ] Documentation complete
- [ ] Migration guide published
- [ ] UNIFICATION: 99.5% → 99.6%

---

## 🚀 Getting Started

### Monday Morning (Nov 11)

**1. Create feature branch**
```bash
git checkout -b feature/config-naming-phase1
```

**2. First config: network/cache.rs**
```bash
# Open file
code code/crates/nestgate-core/src/network/cache.rs

# Find Config struct (around line 25)
# Rename to NetworkCacheConfig
# Update all references in file
# Search for imports: grep -r "network::cache::Config"
# Update all imports
# Verify: cargo check -p nestgate-core
# Test: cargo test -p nestgate-core --lib -- network::cache
```

**3. Track progress**
```bash
# Add to tracking doc
echo "### NetworkCacheConfig" >> CONFIG_PHASE1_PROGRESS.md
echo "- Status: ✅ Complete" >> CONFIG_PHASE1_PROGRESS.md
echo "- Date: $(date)" >> CONFIG_PHASE1_PROGRESS.md
```

**4. Commit small, commit often**
```bash
git add -A
git commit -m "config: Rename network::cache::Config to NetworkCacheConfig

- Renamed Config to NetworkCacheConfig in network/cache.rs
- Updated all references (3 files)
- Tests passing
- Build GREEN

Part of Phase 1 config consolidation (#1/109)"
```

---

## 📊 Metrics to Track

### Daily Metrics
- Configs renamed: X
- Files modified: Y
- Build time: Z seconds
- Test time: W seconds

### Weekly Metrics
- Total configs renamed: X/109
- Percentage complete: Y%
- Build stability: GREEN/YELLOW/RED
- Test pass rate: Z%

### Quality Metrics
- Zero compilation errors maintained
- 100% test pass rate maintained
- Build time impact: <5% slower acceptable
- No new clippy warnings

---

## 🎉 Completion Criteria

### Phase 1 Complete When:
- [ ] Zero structs named just "Config"
- [ ] All 109 configs have domain context
- [ ] Build GREEN across workspace
- [ ] All 1,026 tests passing
- [ ] Documentation updated
- [ ] Migration guide published
- [ ] Lessons learned documented

---

## 📞 Support & Questions

### If You Get Stuck

**Build Error**: 
- Check compiler output for missed references
- Use `grep -r "::Config"` to find imports
- Verify module path is correct

**Test Failure**:
- Check test fixtures for hardcoded names
- Update test data files
- Verify mock configs updated

**Unclear Domain**:
- Look at parent module names
- Check file purpose from code
- Ask: "What does this module do?"

### Communication

**Daily Standup Template**:
```
Yesterday: Renamed 6 configs (network cache, metrics, compression)
Today: Will rename 6 configs (network security, auth, tls)
Blockers: None, all GREEN
```

---

## 🏁 Ready to Start!

**Status**: ✅ READY  
**Start Date**: Monday, November 11, 2025  
**First Target**: NetworkCacheConfig  
**Goal**: 6 configs per day  
**End**: December 6, 2025 (100% complete)

**Let's achieve 99.6% unification!** 🚀

---

**Document Version**: 1.0  
**Created**: November 9, 2025  
**Owner**: Config Consolidation Team  
**Phase**: 1 of 4

