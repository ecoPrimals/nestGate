# 🔌 Unix Socket Storage Backend Wiring - CRITICAL

**Date**: January 29, 2026  
**Status**: 🟡 **CRITICAL PRIORITY** - Production inter-primal comms  
**Complexity**: 2-3 hours (similar to tarpc wiring)  
**Grade Impact**: A+ 97.0 → A+ 98.0/100 (+1.0 point)

---

## 🎯 Executive Summary

Unix sockets are **PRODUCTION-CRITICAL** for inter-primal communication (JSON-RPC + tarpc). The Unix socket JSON-RPC server currently uses **in-memory DashMap** while tarpc uses **persistent StorageManagerService**.

**This creates a critical inconsistency**:
- Data stored via Unix socket → Lost on restart ❌
- Data stored via tarpc → Persists to filesystem ✅
- biomeOS integration → Broken persistence ❌

**MUST FIX**: Wire Unix socket JSON-RPC to `StorageManagerService` immediately.

---

## 📊 Current Architecture

### Storage Backend Status

| Interface | Current Backend | Persistent? | Status |
|-----------|----------------|-------------|--------|
| **tarpc RPC** | `StorageManagerService` | ✅ Filesystem | ✅ Complete (Jan 29) |
| **HTTP JSON-RPC** | `StorageManagerService` | ✅ Filesystem | ✅ Complete (Jan 29) |
| **Unix Socket JSON-RPC** | `DashMap` (in-memory) | ❌ Memory only | ❌ **CRITICAL GAP** |

### The Problem

```rust
// tarpc_server.rs (GOOD - Jan 29 fix)
pub struct NestGateRpcService {
    pub(crate) storage_manager: Arc<StorageManagerService>,  // ✅ Persistent!
    pub(crate) start_time: SystemTime,
}

// unix_socket_server.rs (BAD - Still in-memory!)
struct StorageState {
    storage: Arc<DashMap<String, DashMap<String, Value>>>,  // ❌ In-memory!
    blobs: Arc<DashMap<String, DashMap<String, Vec<u8>>>>,  // ❌ In-memory!
    // ...
}
```

---

## 🚀 Implementation Plan

### Phase 1: Update StorageState Structure (30min)

**File**: `code/crates/nestgate-core/src/rpc/unix_socket_server.rs`

```rust
// BEFORE:
#[derive(Debug, Clone)]
struct StorageState {
    storage: Arc<DashMap<String, DashMap<String, Value>>>,
    blobs: Arc<DashMap<String, DashMap<String, Vec<u8>>>>,
    templates: crate::rpc::template_storage::TemplateStorage,
    audits: crate::rpc::audit_storage::AuditStorage,
}

// AFTER:
#[derive(Debug, Clone)]
struct StorageState {
    storage_manager: Arc<StorageManagerService>,  // ✅ Real storage!
    templates: crate::rpc::template_storage::TemplateStorage,
    audits: crate::rpc::audit_storage::AuditStorage,
}
```

**Changes**:
1. Remove `storage` and `blobs` DashMap fields
2. Add `storage_manager: Arc<StorageManagerService>`
3. Update `Default` impl to initialize `StorageManagerService`

### Phase 2: Wire storage.store (30min)

```rust
// CURRENT (stores to DashMap):
async fn storage_store(params: &Option<Value>, state: &StorageState) -> Result<Value> {
    // ... validation ...
    let family_storage = state.storage.entry(family_id.to_string()).or_default();
    family_storage.insert(key.to_string(), data.clone());
    // ...
}

// TARGET (stores to StorageManagerService):
async fn storage_store(params: &Option<Value>, state: &StorageState) -> Result<Value> {
    // ... validation ...
    
    // Convert to storage backend format
    let dataset = family_id;  // Family = Dataset
    let object_id = key;
    let data_bytes = serde_json::to_vec(data)?;
    
    // Store via StorageManagerService
    state.storage_manager.store_object(
        dataset,
        object_id,
        data_bytes,
        Default::default()  // metadata
    ).await?;
    
    Ok(json!({"success": true, "key": key}))
}
```

### Phase 3: Wire storage.retrieve (20min)

```rust
// CURRENT (reads from DashMap):
async fn storage_retrieve(params: &Option<Value>, state: &StorageState) -> Result<Value> {
    // ... validation ...
    let data = state.storage
        .get(family_id)
        .and_then(|family_storage| family_storage.get(key).map(|v| v.clone()))
        .ok_or_else(|| NestGateError::not_found(...))?;
    // ...
}

// TARGET (reads from StorageManagerService):
async fn storage_retrieve(params: &Option<Value>, state: &StorageState) -> Result<Value> {
    // ... validation ...
    
    let dataset = family_id;
    let object_id = key;
    
    // Retrieve via StorageManagerService
    let data_bytes = state.storage_manager.retrieve_object(
        dataset,
        object_id
    ).await?;
    
    // Parse JSON
    let data: Value = serde_json::from_slice(&data_bytes)?;
    
    Ok(json!({"data": data}))
}
```

### Phase 4: Wire storage.list (20min)

```rust
// CURRENT (lists from DashMap):
async fn storage_list(params: &Option<Value>, state: &StorageState) -> Result<Value> {
    // ... validation ...
    let keys: Vec<String> = state.storage
        .get(family_id)
        .map(|family_storage| {
            family_storage
                .iter()
                .filter(|entry| prefix_match)
                .map(|entry| entry.key().clone())
                .collect()
        })
        .unwrap_or_default();
    // ...
}

// TARGET (lists from StorageManagerService):
async fn storage_list(params: &Option<Value>, state: &StorageState) -> Result<Value> {
    // ... validation ...
    
    let dataset = family_id;
    
    // List via StorageManagerService
    let objects = state.storage_manager.list_objects(
        dataset,
        prefix.as_deref()
    ).await?;
    
    let keys: Vec<String> = objects.iter()
        .map(|obj| obj.object_id.clone())
        .collect();
    
    Ok(json!({"keys": keys}))
}
```

### Phase 5: Wire storage.delete (15min)

```rust
// CURRENT (deletes from DashMap):
async fn storage_delete(params: &Option<Value>, state: &StorageState) -> Result<Value> {
    let deleted = state.storage
        .get(family_id)
        .and_then(|family_storage| family_storage.remove(key))
        .is_some();
    // ...
}

// TARGET (deletes from StorageManagerService):
async fn storage_delete(params: &Option<Value>, state: &StorageState) -> Result<Value> {
    // ... validation ...
    
    let dataset = family_id;
    let object_id = key;
    
    // Delete via StorageManagerService
    state.storage_manager.delete_object(
        dataset,
        object_id
    ).await?;
    
    Ok(json!({"success": true}))
}
```

### Phase 6: Wire storage_store_blob & storage_retrieve_blob (30min)

Similar pattern - convert to `store_object`/`retrieve_object` calls.

### Phase 7: Update JsonRpcUnixServer initialization (15min)

```rust
// BEFORE:
impl JsonRpcUnixServer {
    pub async fn new(family_id: &str) -> Result<Self> {
        // ...
        let state = StorageState::default();  // In-memory
        // ...
    }
}

// AFTER:
impl JsonRpcUnixServer {
    pub async fn new(family_id: &str) -> Result<Self> {
        // ...
        let storage_manager = Arc::new(
            StorageManagerService::new().await?
        );
        let state = StorageState {
            storage_manager,
            templates: TemplateStorage::new(),
            audits: AuditStorage::new(),
        };
        // ...
    }
}
```

### Phase 8: Update Tests (30min)

- Update test helpers to use temp directories
- Fix any test assertions
- Verify all Unix socket tests pass

---

## ⏱️ Time Estimate

| Phase | Task | Time |
|-------|------|------|
| 1 | Update StorageState structure | 30min |
| 2 | Wire storage.store | 30min |
| 3 | Wire storage.retrieve | 20min |
| 4 | Wire storage.list | 20min |
| 5 | Wire storage.delete | 15min |
| 6 | Wire blob methods | 30min |
| 7 | Update initialization | 15min |
| 8 | Update tests | 30min |
| **TOTAL** | **Complete wiring** | **2.5-3h** |

**Based on tarpc wiring**: We completed tarpc in 3h (estimated 8-12h), so this should be 2-3h.

---

## ✅ Success Criteria

### Functional Requirements

1. ✅ Unix socket JSON-RPC uses `StorageManagerService`
2. ✅ Data persists to filesystem
3. ✅ Data survives NestGate restart
4. ✅ All storage methods work correctly:
   - `storage.store` → Filesystem
   - `storage.retrieve` → Filesystem
   - `storage.list` → Filesystem
   - `storage.delete` → Filesystem
   - `storage_store_blob` → Filesystem
   - `storage_retrieve_blob` → Filesystem

### Testing Requirements

1. ✅ All Unix socket tests pass
2. ✅ Manual validation with biomeOS commands
3. ✅ Restart persistence test:
   ```bash
   # Store data
   echo '{"jsonrpc":"2.0","method":"storage.store",...}' | nc -U socket
   # Restart NestGate
   killall nestgate && ./nestgate server
   # Retrieve data (should work!)
   echo '{"jsonrpc":"2.0","method":"storage.retrieve",...}' | nc -U socket
   ```

### Quality Requirements

1. ✅ Clean compilation (zero errors)
2. ✅ Clean clippy (zero warnings)
3. ✅ Consistent error handling
4. ✅ Comprehensive logging

---

## 📈 Grade Impact

### Current State
- **Grade**: A+ 97.0/100
- **Issue**: Unix socket storage not persistent
- **Impact**: Production blocker for biomeOS

### After Wiring
- **Grade**: A+ 98.0/100 (+1.0 point!)
- **Status**: Full persistent storage across ALL interfaces
- **Impact**: Production-ready inter-primal comms

### Grade Breakdown
- Architecture: A++ (100) - Consistent backend ✅
- Storage Backend: A++ (100) - Full persistence ✅
- Test Suite: A++ (99) - All tests passing ✅
- **Overall**: **A++ (98.0/100)** 🎊

---

## 🎯 Priority

**PRIORITY**: **CRITICAL** 🚨

**Rationale**:
1. Unix sockets are PRODUCTION for inter-primal comms
2. biomeOS depends on persistent storage
3. Current state is production blocker
4. Only 2-3h to complete
5. Major grade impact (+1.0 point to A++)

**Recommendation**: **START IMMEDIATELY**

---

## 📚 References

### Code Files
- `code/crates/nestgate-core/src/rpc/unix_socket_server.rs` - Target file
- `code/crates/nestgate-core/src/rpc/tarpc_server.rs` - Reference implementation
- `code/crates/nestgate-core/src/services/storage/service.rs` - StorageManagerService

### Documentation
- `STORAGE_WIRING_PROGRESS_JAN_29_2026.md` - tarpc wiring reference
- `BIOMEOS_STORAGE_FIX_JAN_29_2026.md` - Recent bug fix
- `docs/session-archives/2026-01-29-storage-milestone/` - Wiring milestone docs

### Related Work
- ✅ tarpc server wiring (Jan 29, 3h)
- ✅ StorageManagerService creation (Jan 29, 2h)
- ✅ JSON-RPC test fixes (Jan 29, 1.5h)
- ⏳ Unix socket wiring (NEXT, 2-3h)

---

## 💡 Implementation Notes

### Key Patterns (from tarpc wiring)

1. **Error Conversion**:
   ```rust
   fn convert_error(err: NestGateError) -> JsonRpcError {
       // Map NestGateError to JSON-RPC error codes
   }
   ```

2. **Async Initialization**:
   ```rust
   pub async fn new() -> Result<Self> {
       let storage_manager = Arc::new(
           StorageManagerService::new().await?
       );
       // ...
   }
   ```

3. **Family → Dataset Mapping**:
   ```rust
   let dataset = family_id;  // Direct mapping
   let object_id = key;      // Direct mapping
   ```

4. **JSON Serialization**:
   ```rust
   let data_bytes = serde_json::to_vec(data)?;  // Store
   let data: Value = serde_json::from_slice(&bytes)?;  // Retrieve
   ```

### Testing Strategy

1. Run existing tests (should mostly pass with temp dirs)
2. Add restart persistence test
3. Manual validation with biomeOS commands
4. Integration test with full ecosystem

---

## 🚀 Execution Plan

### Immediate (Now)
1. Create working branch
2. Update `StorageState` structure
3. Wire 5 core methods
4. Test compilation

### Next (1-2h)
5. Wire blob methods
6. Update initialization
7. Fix tests
8. Full testing

### Final (30min)
9. Documentation update
10. Commit and push
11. biomeOS validation
12. Grade update to A++ (98.0)

---

**Status**: Ready to execute ✅  
**Priority**: CRITICAL - Production blocker  
**Time**: 2-3 hours  
**Impact**: A+ 97.0 → A++ 98.0/100 (+1.0 point!)

🦀 Let's wire this up and achieve A++ grade! 🚀
