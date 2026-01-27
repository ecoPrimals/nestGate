# 🧹 Archive Code Cleanup Audit - January 27, 2026

**Date**: January 27, 2026  
**Purpose**: Identify archive code, outdated TODOs, and dead code for cleanup  
**Status**: Ready for execution  
**Safety**: All cleanups preserve fossil record in ecoPrimals/

---

## 📊 **AUDIT SUMMARY**

### **Findings**

| Category | Count | Status | Action |
|----------|-------|--------|--------|
| **Commented-out code** | 105+ blocks | Safe to remove | Clean |
| **Deprecated markers** | 389 files | Review needed | Audit |
| **Development stubs** | 15 instances | Feature-gated | Keep (documented) |
| **TODO/FIXME** | 38 instances | Review needed | Update |
| **Archive docs** | 3 files | Move to fossil | Relocate |

---

## 🎯 **SAFE TO REMOVE** (Immediate)

### **1. Commented-Out Module Declarations** (105 instances)

**Pattern**: `// pub mod something` or `// pub use something`

**Status**: These are already removed from compilation, comments can be deleted.

**Files with most occurrences**:
- `code/crates/nestgate-core/src/monitoring/tracing_setup/mod.rs` (16 instances)
- `code/crates/nestgate-api/src/handlers/mod.rs` (14 instances)
- `code/crates/nestgate-core/src/services/storage/mod.rs` (13 instances)
- `code/crates/nestgate-core/src/lib.rs` (7 instances)
- `code/crates/nestgate-zfs/src/config/mod.rs` (6 instances)
- `code/crates/nestgate-installer/src/config/mod.rs` (5 instances)

**Action**: Remove all commented-out declarations **EXCEPT**:
- Lines with migration notices (keep for 1 version)
- Lines documenting WHY something was removed

**Example - REMOVE**:
```rust
// pub mod old_module;
pub mod new_module;
```

**Example - KEEP** (has explanation):
```rust
// pub mod songbird_registration; // REMOVED: Deprecated since v2.3.0, zero production usage
```

---

### **2. Archive Documentation** (3 files)

**Location**: `docs/archive/old-status/`

**Files**:
1. `CAPABILITY_DISCOVERY_MIGRATION_GUIDE.md`
2. `CURRENT_STATUS.md`
3. `HARDCODING_ELIMINATION_STRATEGY.md`

**Status**: Old status files from earlier sessions

**Action**: Move to `ecoPrimals/wateringHole/fossils/nestGate/` for historical record

**Reason**: These are superseded by:
- Current `CURRENT_STATUS.md` (root, updated Jan 27)
- `CAPABILITY_MAPPINGS.md` (root)
- Session archives (docs/session-archives/2026-01-27/)

---

## ⚠️ **REVIEW BEFORE REMOVING**

### **3. DEPRECATED Markers** (389 files)

**Status**: Files contain "deprecated", "DEPRECATED", or deprecation attributes

**Breakdown by Type**:

#### **A. Already Handled** ✅:
- `songbird_registration.rs` - Already commented out in mod.rs
- Port helper functions - Already have `#[deprecated]` attributes with migration paths

#### **B. Need Review** (examine these):

**Critical Files**:
1. **`code/crates/nestgate-core/src/config/external/services.rs`**
   - Status: Deprecated in favor of capability-based config
   - Action: Verify no production usage, then remove

2. **`code/crates/nestgate-core/src/constants/ports.rs`**
   - Status: Some helpers marked deprecated
   - Action: Keep file, just remove deprecated helper functions
   - Reason: Modern `get_api_server_addr()` should stay

3. **`code/crates/nestgate-zfs/src/lib.rs`**
   - Check for deprecated exports
   - Keep deprecation warnings (guide users)

---

### **4. Development Stubs** (15 instances)

**Pattern**: `DEVELOPMENT STUB` marker

**Status**: Properly feature-gated with `#[cfg(feature = "dev-stubs")]`

**Action**: **KEEP** - These are intentional and documented

**Examples**:
- `code/crates/nestgate-core/src/crypto/mod.rs` (delegationplanned)
- `code/crates/nestgate-core/src/dev_stubs/mod.rs` (test infrastructure)
- `code/crates/nestgate-api/src/dev_stubs/zfs/config.rs` (local dev)

**Why Keep**:
- All properly gated (not in production builds)
- Clear migration path documented
- Evolution plan exists (Week 2-3: BearDog delegation)

---

### **5. TODO/FIXME Markers** (38 instances)

**Files with TODOs** (21 files):

**High Priority** (remove outdated):
1. `code/crates/nestgate-core/src/rpc/semantic_router.rs` (6 TODOs)
   - Status: These are **VALID** placeholders for Week 1-2 work
   - Action: **KEEP** - Part of roadmap

2. `code/crates/nestgate-core/src/crypto/mod.rs` (1 TODO)
   - "TODO: Either complete implementation or remove in favor of BearDog"
   - Action: **KEEP** - Week 2-3 work (crypto delegation)

3. `code/crates/nestgate-core/src/discovery_mechanism.rs` (1 TODO)
   - Check if still relevant
   
4. `code/crates/nestgate-core/src/capability_discovery.rs` (2 TODOs)
   - Check if still relevant after Week 1-2 wiring

**Low Priority** (likely outdated):
- Check files in `universal_storage/`, `performance/`, `services/`
- May have TODOs from earlier iterations

**Action**:
1. Read each TODO
2. If part of current roadmap (Week 1-8) → **KEEP**
3. If superseded by new architecture → **REMOVE**
4. If unclear → Ask user

---

## 🧪 **SAFE (INTENTIONAL PATTERNS)**

### **6. Commented-Out Security Module**

**File**: `code/crates/nestgate-core/src/lib.rs:147-150`

```rust
// ⚠️ Security module temporarily disabled - has integration errors
// ✅ JWT validation extracted to standalone module below
// Re-enable after integration fixes are complete
// pub mod security;
```

**Status**: **KEEP** - Clear temporary disable with reason

**Why**: This is a known temporary state with clear re-enable plan.

---

### **7. Test-Only Code**

**Pattern**: `#[cfg(test)]` modules

**Status**: **KEEP** - These are production test infrastructure

**Example**:
```rust
#[cfg(test)]
mod error_path_coverage_tests;
```

---

## 📋 **EXECUTION PLAN**

### **Phase 1: Safe Removals** (30-45 minutes)

```bash
# 1. Remove commented-out declarations without explanations
# Manual review of 105 instances, remove ~70-80 safe ones

# 2. Move archive docs to fossil record
mkdir -p /path/to/ecoPrimals/wateringHole/fossils/nestGate/2026-01-27/
mv docs/archive/old-status/*.md /path/to/ecoPrimals/wateringHole/fossils/nestGate/2026-01-27/

# 3. Remove old-status folder (after move)
rmdir docs/archive/old-status/
```

### **Phase 2: Deprecated Code Review** (1-2 hours)

**Priority Order**:
1. **config/external/services.rs** - Check usage, likely remove
2. **constants/ports.rs** - Remove deprecated helpers only
3. Review other deprecated markers

**Process**:
1. Search for imports: `grep -r "config::external::services" code/`
2. If zero usage → Remove
3. If has usage → Update TODO for migration

### **Phase 3: TODO Audit** (1 hour)

**Process**:
1. Read each of 38 TODOs
2. Category:
   - **Roadmap** (Week 1-8) → Keep
   - **Superseded** → Remove
   - **Unclear** → Flag for user

### **Phase 4: Git Push** (via SSH)

```bash
# After cleanup
git add .
git commit -m "chore: archive code cleanup - remove dead code, outdated TODOs

- Removed 70-80 commented-out declarations without explanations
- Moved 3 archive docs to ecoPrimals/wateringHole/fossils
- Removed outdated TODOs (verified not in current roadmap)
- Removed deprecated code with zero production usage
- Kept: Development stubs (feature-gated), roadmap TODOs, deprecation notices with migration paths

All fossil records preserved in ecoPrimals/ per ecosystem standards."

git push origin main
```

---

## 🎯 **SPECIFIC RECOMMENDATIONS**

### **High-Confidence Removals**

1. **Remove** (after verifying zero usage):
   - `config/external/services.rs` (deprecated, capability-based config replaces it)
   - Deprecated port helper functions in `constants/ports.rs`
   - Commented-out declarations in monitoring/tracing_setup (16 lines)

2. **Keep** (intentional):
   - All `DEVELOPMENT STUB` markers (feature-gated)
   - TODOs in `semantic_router.rs` (current roadmap)
   - Deprecation notice for `songbird_registration` (one version grace period)

3. **Move to Fossil**:
   - 3 files in `docs/archive/old-status/`

---

## 📊 **IMPACT ANALYSIS**

### **Lines Removed** (Estimated):

| Category | Lines | Impact |
|----------|-------|--------|
| Commented-out declarations | ~100 | Cleaner, easier to read |
| Deprecated code (if unused) | ~500-1000 | Smaller binaries |
| Outdated TODOs | ~20-30 | Clearer roadmap |
| Archive docs (moved) | 0 (moved) | Better organization |

**Total Cleanup**: ~120-130 lines removed + 3 files relocated

### **Benefits**:

1. **Cleaner Codebase**: Easier to navigate
2. **Faster Compilation**: Less dead code to compile
3. **Clear Intent**: Only keep what's intentional
4. **Better Documentation**: Outdated TODOs removed

---

## ✅ **SAFETY CHECKLIST**

Before each removal:
- [ ] Verify not imported anywhere (`grep -r "module_name" code/`)
- [ ] Check for deprecation migration path (keep those)
- [ ] Verify not in current roadmap (Week 1-8)
- [ ] Preserve fossil record in ecoPrimals/wateringHole/

**Rule**: When in doubt, keep it for one more version cycle.

---

## 🚦 **DECISION MATRIX**

| Pattern | Action | Reason |
|---------|--------|--------|
| `// pub mod x` (no explanation) | **REMOVE** | Already removed from compilation |
| `// pub mod x // REMOVED: reason` | **KEEP** | Documents migration |
| `DEVELOPMENT STUB` | **KEEP** | Intentional, feature-gated |
| `TODO` (in roadmap) | **KEEP** | Current work plan |
| `TODO` (superseded) | **REMOVE** | No longer relevant |
| `#[deprecated]` (with path) | **KEEP** | User migration guide |
| `#[deprecated]` (unused) | **REMOVE** | Dead code |
| Archive docs | **MOVE** | Fossil record |

---

## 🔍 **NEXT STEPS**

### **User Decision Required**:

1. **Approve Phase 1** (safe removals)?
   - Remove commented-out declarations without explanations
   - Move archive docs to ecoPrimals/wateringHole/fossils/

2. **Execute Phase 2** (deprecated code review)?
   - Need to verify zero usage of deprecated modules
   - Can proceed with analysis

3. **Execute Phase 3** (TODO audit)?
   - Read all 38 TODOs, categorize
   - Remove outdated, keep roadmap ones

**Ready to proceed when you confirm.** All changes are safe and reversible (fossil records preserved).

---

**Status**: ✅ **AUDIT COMPLETE - AWAITING APPROVAL**  
**Safety**: 🔒 All fossil records preserved  
**Impact**: 🎯 ~120-130 lines cleaned, 3 files relocated  
**Risk**: ⚠️ LOW (only removing already-removed code)

---

*🦀 Clean codebase · Clear intent · Fossil record preserved · Production-ready 🧹*
