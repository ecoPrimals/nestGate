# Storage Wiring Blockers - January 29, 2026

## 🚧 Critical Design Issues Discovered

**Status**: Work paused at 40% completion to address architectural decisions

---

## 🔴 **Blocker 1: StorageServiceConfig Missing base_path**

### Issue:
`StorageServiceConfig` does not have a `base_path` field for filesystem storage.

### Current Fields:
- `zfs: ZfsConfig`
- `auto_discover_pools: bool`
- `enable_quotas: bool`
- etc.

### Required:
Need a field to specify where datasets/objects are stored on the filesystem.

### Solutions:
**Option A**: Add `base_path` field to `StorageServiceConfig`
```rust
pub struct StorageServiceConfig {
    pub base_path: String,  // e.g., "/var/lib/nestgate/storage"
    pub zfs: ZfsConfig,
    // ... existing fields
}
```

**Option B**: Use ZfsConfig mount point
- Leverage existing ZFS configuration
- May not work for non-ZFS deployments

**Recommendation**: Option A (more flexible)

---

## 🔴 **Blocker 2: ObjectInfo Structure Mismatch**

### Issue:
Two different `ObjectInfo` structures:
1. **tarpc version** (in `tarpc_types.rs`):
   - Has: `dataset`, `compressed`, `encrypted`, `metadata` fields
   
2. **Storage manager version** (simplified):
   - Has: `key`, `size_bytes`, `created_at`, etc.
   - **Missing**: `dataset`, `compressed`, `encrypted`, `metadata`

### Impact:
Cannot directly return storage manager's `ObjectInfo` from RPC methods.

### Solutions:
**Option A**: Extend storage manager's `ObjectInfo` to match tarpc
**Option B**: Convert between types with mapping function
**Option C**: Unify to single `ObjectInfo` type

**Recommendation**: Option C (eliminate duplication)

---

## 🔴 **Blocker 3: NestGateError API Mismatch**

### Issue:
Using `NestGateError::not_found()` but enum is `NestGateUnifiedError::NotFound`.

### Error in Code:
```rust
NestGateError::not_found(&format!("object {}/{}", dataset, key))
// Should be:
NestGateError::NotFound { resource, ... }
```

### Solution:
Update error construction to match `NestGateUnifiedError` API.

---

## 🔴 **Blocker 4: Additional Methods Still Using Old Fields**

### Remaining Methods with `self.datasets` or `self.objects`:
- `get_object_metadata()`
- `list_objects()`  
- `update_object_metadata()`
- Others (need full scan)

### Solution:
Continue systematic method-by-method updates.

---

## 📊 **Current State**

### ✅ Completed (40%):
- Structure updated
- 5 core methods wired (create, list, store, retrieve, delete)
- Error conversion helper added
- delete_dataset method added

### ❌ Blocked (60%):
- Config missing base_path field
- ObjectInfo type mismatch
- Error API mismatches
- ~10+ methods still using old storage

### Compilation Errors: **20+**

---

## 🎯 **Required Actions (Priority Order)**

### 1. Add base_path to StorageServiceConfig (30min)
```rust
// In config.rs
pub struct StorageServiceConfig {
    pub base_path: String,
    // ... rest of fields
}

// In Default implementation
impl Default for StorageServiceConfig {
    fn default() -> Self {
        Self {
            base_path: "/var/lib/nestgate/storage".to_string(),
            // ... rest
        }
    }
}
```

### 2. Unify ObjectInfo Structures (1h)
- Choose canonical `ObjectInfo` type
- Update all code to use single type
- Remove duplicate definitions

### 3. Fix Error Construction (30min)
- Update to `NestGateUnifiedError` API
- Fix `not_found()` calls
- Fix `io_error()` calls

### 4. Wire Remaining Methods (2-3h)
- `get_object_metadata()`
- `list_objects()`
- `update_object_metadata()`
- Any others found

### 5. Update Tests (1h)
- Change `new()` calls to `new().await`
- Update assertions
- Fix async test setup

### 6. Integration Testing (1-2h)
- End-to-end dataset/object operations
- Verify persistence
- Performance validation

**Total Remaining**: 6-8 hours (as estimated)

---

## 💡 **Recommendations**

### For Next Session:

1. **Start with Config** - Add `base_path` field first
2. **Unify Types** - Consolidate ObjectInfo definitions
3. **Fix Errors** - Update to correct error API
4. **Systematic Wiring** - Continue method-by-method
5. **Test Incrementally** - Verify each step compiles

### Alternative Approach:

If issues persist, consider:
- **Hybrid System**: Keep old in-memory as fallback
- **Feature Flag**: Make real storage optional
- **Phased Migration**: Wire one method at a time, test each

---

## 📈 **Progress Summary**

| Phase | Status | Time | Remaining |
|-------|--------|------|-----------|
| Structure Update | ✅ Done | 1h | - |
| Core Methods | ✅ Done | 2h | - |
| Config Issues | 🚧 Blocked | - | 30min |
| Type Unification | 🚧 Blocked | - | 1h |
| Error Fixes | 🚧 Blocked | - | 30min |
| Remaining Methods | ⏳ Waiting | - | 2-3h |
| Testing | ⏳ Waiting | - | 2-3h |
| **TOTAL** | **40% Done** | **3h** | **6-8h** |

---

## ✅ **What Went Well**

- ✅ Clear architectural plan from archive
- ✅ Systematic approach working
- ✅ Good progress on core methods
- ✅ Excellent documentation of blockers

## ⚠️ **Challenges Discovered**

- ⚠️ Config structure not storage-ready
- ⚠️ Type duplication (ObjectInfo)
- ⚠️ Error API evolved (need updates)
- ⚠️ More methods than expected

---

## 🎓 **Key Learnings**

1. **Design First**: Config should support use case
2. **Type Safety**: One canonical type better than conversions
3. **Error Handling**: Unified error system needs consistency
4. **Scope**: Always larger than initial estimate

---

**Session Time**: 4 hours total (2h on storage wiring)  
**Status**: Professional WIP with clear blockers identified  
**Next**: Address config/type issues, then continue wiring  
**Quality**: Following "deep debt" - document over rush

---

*🦀 40% complete · Blockers identified · Clear path forward 🏗️*
