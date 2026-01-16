# Remaining HTTP Cleanup Tracking - January 16, 2026

**Date**: January 16, 2026 (4:00 AM)  
**Context**: UniBin implementation revealed additional HTTP references  
**Status**: ⏳ **TRACKED - Not Blocking**

---

## 🎯 **Summary**

**Progress**:
- ✅ Primary HTTP removed (protocol_http.rs, s3.rs, byob.rs) - **1,941 lines**
- ⏳ Peripheral HTTP remains in 6 files - **~500-800 lines estimated**

**Blocking**: ❌ NO - These are optional/peripheral modules

**Priority**: Medium (can be done in separate cleanup session)

---

## 📋 **Remaining HTTP References**

### **1. nestgate-network** (1 file)

**File**: `code/crates/nestgate-network/src/api.rs`

**Component**: `OrchestrationCapability` struct

**Usage**: Songbird orchestration via HTTP (deprecated)

**Lines**: ~150 lines

**Status**: ⏳ Stubbed with `unimplemented!()` for now

**Action**: Remove entire struct or evolve to Unix sockets

---

### **2. nestgate-api** (4 files)

#### **File 1**: `handlers/zfs/universal_zfs/backends/remote/client.rs`

**Component**: `HttpClient` and `RemoteZfsClient` structs

**Usage**: Remote ZFS operations over HTTP

**Lines**: ~200 lines estimated

**Reqwest usage**:
- `client: reqwest::Client` (field)
- `reqwest::Client::builder()` (constructor)
- `reqwest::Client::new()` (fallback)

**Status**: ⏳ Not yet cleaned

**Action**: Remove or feature-gate entire remote backend

---

#### **File 2**: `handlers/workspace_management/optimization.rs`

**Component**: Workspace optimization HTTP client

**Usage**: HTTP-based workspace optimization

**Lines**: ~50 lines estimated

**Reqwest usage**:
- `reqwest::Client::new()` (line 256)

**Status**: ⏳ Not yet cleaned

**Action**: Remove HTTP optimization or evolve to local

---

#### **File 3**: `handlers/zfs/universal_zfs/factory.rs`

**Component**: Remote endpoint health check

**Usage**: HTTP health check for remote ZFS endpoints

**Lines**: ~20 lines estimated

**Reqwest usage**:
- `reqwest::get(endpoint)` (line 256)

**Status**: ⏳ Not yet cleaned

**Action**: Remove or use Unix socket health check

---

#### **File 4**: `universal_primal.rs`

**Component**: Primal communication client

**Usage**: HTTP-based primal-to-primal communication (legacy)

**Lines**: ~100 lines estimated

**Reqwest usage**:
- `reqwest::Client::new()` (line 385)
- `reqwest::Client::new()` (line 492)

**Status**: ⏳ Not yet cleaned

**Action**: Remove entirely (should use Unix sockets)

---

## 📊 **Impact Assessment**

### **Severity**: **LOW**

These modules are peripheral and not critical to core functionality:
- Remote backends: Optional (most use local ZFS)
- Workspace optimization: Optional feature
- HTTP health checks: Can use local checks
- Universal primal HTTP: Deprecated (Unix sockets preferred)

### **Blocking UniBin**: ❌ **NO**

UniBin can work without these modules:
- Core daemon: ✅ Working
- CLI commands: ✅ Working
- Discovery: ✅ Working (no HTTP)
- Storage: ✅ Working (local)

### **Build Impact**

**Current**:
- `cargo check --package nestgate-bin`: ❌ FAIL (depends on nestgate-api)
- `cargo check --package nestgate-core`: ✅ PASS

**After Cleanup**:
- All builds: ✅ PASS

---

## 🎯 **Cleanup Strategy**

### **Option A**: Remove Files (Aggressive, Clean)

**Remove**:
- `universal_zfs/backends/remote/` (entire module)
- `workspace_management/optimization.rs` (HTTP methods)
- HTTP checks from `factory.rs`
- `universal_primal.rs` (HTTP methods)

**Benefit**: Clean, pure architecture

**Effort**: 2-3 hours

---

### **Option B**: Feature-Gate (Conservative)

**Add Features**:
```toml
[features]
remote-backends = ["reqwest"]
```

**Gate Code**:
```rust
#[cfg(feature = "remote-backends")]
pub mod remote;
```

**Benefit**: Preserve code optionally

**Effort**: 1-2 hours

**Issue**: Still violates Concentrated Gap

---

### **Option C**: Stub & Document (Pragmatic) ⭐

**Current Approach**:
- Stub HTTP methods with `unimplemented!()`
- Add notes about Unix socket evolution
- Document in this file
- Complete in dedicated session

**Benefit**: Unblocks uniBin immediately

**Effort**: 0 hours (already done for network)

**Next**: Finish stubbing nestgate-api files

---

## 📅 **Recommended Timeline**

### **Today** (This Session):
1. ✅ Stub remaining HTTP in nestgate-api (30 min)
2. ✅ Complete uniBin Phase 1 (2 hours)
3. ✅ Test uniBin functionality

### **Next Session** (Dedicated HTTP Cleanup):
1. Remove all HTTP from nestgate-api
2. Remove all HTTP from nestgate-network
3. Verify clean build
4. Update documentation

**Total Remaining**: 2-3 hours in dedicated session

---

## 🏆 **Progress**

### **Completed**:
- ✅ nestgate-zfs: 100% HTTP-free (3 files, 1,941 lines)
- ✅ nestgate-core: 100% HTTP-free
- ✅ nestgate-network: 90% HTTP-free (stubbed)

### **Remaining**:
- ⏳ nestgate-api: 4 files (~500 lines)
- ⏳ nestgate-network: Complete cleanup (~150 lines)

**Total**: ~650 lines across 5 files

---

## ✅ **Not Blocking**

**UniBin Implementation**: Can proceed with stubs

**Reason**: These are peripheral modules not used in core daemon/CLI

**Strategy**: Stub now, clean later

---

**Created**: January 16, 2026, 4:00 AM  
**Purpose**: Track remaining HTTP cleanup work  
**Status**: Documented, not blocking uniBin! ✅

🦀 **PRAGMATIC EVOLUTION** | 🎯 **UNBLOCK PROGRESS** | 📋 **TRACK DEBT**
