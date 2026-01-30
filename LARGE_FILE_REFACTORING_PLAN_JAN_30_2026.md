# 🔨 Large File Smart Refactoring Plan

**Date**: January 30, 2026  
**Phase**: 2 (Foundation Cleanup)  
**Status**: IN PROGRESS  
**Target**: 10+ files >900 lines

---

## 🎯 **Smart Refactoring Philosophy**

**NOT**: Just splitting files arbitrarily  
**YES**: Logical module extraction with clear responsibilities

### **Principles**

1. **Logical Cohesion**: Group related code together
2. **Single Responsibility**: Each module has one clear purpose
3. **Preserve Performance**: Zero runtime overhead
4. **Maintain API**: Backward compatibility via re-exports
5. **Improve Testability**: Easier to test smaller modules

---

## 📊 **Target Files**

| File | Lines | Status | Priority |
|------|-------|--------|----------|
| `unix_socket_server.rs` | 1,067 | Skip (Phase 3) | N/A |
| **`discovery_mechanism.rs`** | **973** | **✅ COMPLETE** | **🔴 HIGH** |
| `zero_copy_networking.rs` | 961 | Pending | 🟡 MEDIUM |
| `semantic_router.rs` | 929 | Pending | 🟡 MEDIUM |
| `consolidated_canonical.rs` | 928 | Pending | 🟡 MEDIUM |
| `unified_api_config/handlers.rs` | 921 | Pending | 🟡 MEDIUM |
| `auto_configurator.rs` | 917 | Pending | 🟡 MEDIUM |
| `installer/lib.rs` | 915 | Pending | 🟢 LOW |
| `production_discovery.rs` | 910 | Pending | 🟢 LOW |
| `hardware_tuning/types.rs` | 907 | Pending | 🟢 LOW |

**Note**: `unix_socket_server.rs` will be REPLACED entirely in Phase 3 with biomeos-ipc

---

## 🔨 **File 1: discovery_mechanism.rs** (973 lines) ✅ **COMPLETE**

### **Current Structure**

```
discovery_mechanism.rs (973 lines)
├── Module docs (1-221)
│   ├── Documentation (1-66)
│   ├── Imports (67-75)
│   ├── Capability type (76-87)
│   ├── ServiceInfo struct (88-104)
│   ├── DiscoveryMechanism trait (105-128)
│   └── DiscoveryBuilder (129-219)
├── mdns module (222-388) [167 lines]
│   ├── MdnsDiscovery struct
│   ├── impl MdnsDiscovery
│   └── impl DiscoveryMechanism for MdnsDiscovery
├── consul module (389-653) [265 lines]
│   ├── ConsulDiscovery struct
│   ├── impl ConsulDiscovery
│   └── impl DiscoveryMechanism for ConsulDiscovery
├── k8s module (654-914) [261 lines]
│   ├── K8sDiscovery struct
│   ├── impl K8sDiscovery
│   └── impl DiscoveryMechanism for K8sDiscovery
├── testing module (915) [extracted already ✅]
└── tests (918-973) [56 lines]
```

### **Smart Refactoring Plan**

```
discovery_mechanism/
├── mod.rs (NEW - 150 lines)
│   ├── Module documentation
│   ├── Re-exports
│   ├── Capability type
│   ├── ServiceInfo struct
│   └── DiscoveryMechanism trait
├── builder.rs (NEW - 90 lines)
│   ├── DiscoveryBuilder struct
│   ├── impl DiscoveryBuilder
│   └── Auto-detection logic
├── mdns.rs (NEW - 170 lines)
│   └── mDNS implementation
├── consul.rs (NEW - 270 lines)
│   └── Consul implementation
├── k8s.rs (NEW - 265 lines)
│   └── Kubernetes implementation
├── testing.rs (EXISTS - 300 lines)
│   └── Mock discovery for tests
└── tests.rs (NEW - 60 lines)
    └── Unit tests
```

### **Benefits**

**Before**:
- 973 lines in one file
- Hard to navigate
- Harder to test individual backends
- All backends loaded even if not used

**After**:
- 6 files, largest ~270 lines
- Clear module boundaries
- Easy to test each backend separately
- Better code organization

**API Preserved**:
```rust
// Still works exactly the same!
use nestgate_core::discovery_mechanism::{
    DiscoveryMechanism,
    DiscoveryBuilder,
    ServiceInfo,
};

let discovery = DiscoveryBuilder::new()
    .detect()
    .await?;
```

---

## 📝 **Refactoring Steps**

### **Step 1: Create Module Structure** ✅

```bash
mkdir -p code/crates/nestgate-core/src/discovery_mechanism
```

### **Step 2: Extract Core Types** ✅

- Move ServiceInfo, Capability, DiscoveryMechanism trait
- Add module documentation
- Add re-exports

### **Step 3: Extract Builder** ✅

- Move DiscoveryBuilder
- Keep auto-detection logic
- Import from mod.rs

### **Step 4: Extract Backends** ✅

- **mdns.rs**: Extract mdns module (lines 222-388)
- **consul.rs**: Extract consul module (lines 389-653)
- **k8s.rs**: Extract k8s module (lines 654-914)

### **Step 5: Extract Tests** ✅

- Move tests from bottom of file
- Update imports

### **Step 6: Update Imports** ✅

- Update `mod.rs` in parent directory
- Ensure all imports resolve correctly

### **Step 7: Test & Verify** ✅

- Run all tests (7/7 passed!)
- Verify compilation (clean build!)
- Check no functionality lost (API unchanged!)

---

## ✅ **Success Criteria**

**File Size**:
- ✅ No file >500 lines
- ✅ Clear module boundaries
- ✅ Logical organization

**Functionality**:
- ✅ All tests pass
- ✅ API unchanged (backward compatible)
- ✅ Zero performance impact

**Maintainability**:
- ✅ Easier to navigate
- ✅ Easier to test
- ✅ Easier to add new backends

---

## 🎯 **Next Files to Refactor**

After `discovery_mechanism.rs`:

1. **`semantic_router.rs`** (929 lines)
   - Extract routing logic into modules
   - Separate route registration from handling

2. **`zero_copy_networking.rs`** (961 lines)
   - Split into buffer management, network IO, protocol

3. **`auto_configurator.rs`** (917 lines)
   - Separate detection, configuration, application

---

## 📊 **Progress Tracking**

| File | Lines | Modules | Status |
|------|-------|---------|--------|
| `discovery_mechanism.rs` | 973→322 max | 7 | ✅ Complete |
| `semantic_router.rs` | 929 | TBD | ⏳ Pending |
| `zero_copy_networking.rs` | 961 | TBD | ⏳ Pending |

**Goal**: Refactor 3-5 large files in Phase 2

---

**Created**: January 30, 2026  
**Status**: Active refactoring in progress  
**Target**: All files <500 lines
