# API Config Migration Plan
**Date**: November 7, 2025  
**Status**: 🟢 **READY TO EXECUTE**  
**Pattern**: Replicate ZFS Success (4.25h actual vs 7h estimated)

---

## 🎯 OBJECTIVE

Consolidate **3+ fragmented API config structures** into a single canonical API configuration in `nestgate-core::config::canonical_primary`, following the proven ZFS migration pattern.

---

## 📊 CURRENT STATE ANALYSIS

### Fragmented API Configs Identified

| File | Config Name | Fields | Status |
|------|-------------|--------|--------|
| `canonical_primary/api_config.rs` | `ApiConfig` | **3** (minimal stub) | ⚠️ Incomplete |
| `canonical_primary/domains/network/api.rs` | `NetworkApiConfig` | **9** (network-focused) | ✅ Comprehensive |
| `nestgate-api/config/unified_api_config.rs` | `UnifiedApiConfig` | **4** (server/security/perf/monitoring) | ✅ Well-designed |
| `unified_api_config/api_core.rs` | `UnifiedApiConfig` | **2** (uses CanonicalModernizedConfig) | ⚠️ Legacy pattern |
| `unified_api_config/handlers.rs` | `UnifiedApiHandlerConfig` | Complex handler extensions | ⚠️ Over-engineered |
| `unified_fuzz_config.rs` | `FuzzApiConfigData` | **5** (testing-specific) | ⚠️ Fragmented |

### Total Fragmentation

- **6 different API config structures**
- **~40 total configuration fields** scattered across files
- **3 different import paths** for "API config"
- **2 conflicting naming patterns** (ApiConfig vs NetworkApiConfig)

---

## 🎯 CANONICAL TARGET

### Primary File
**`code/crates/nestgate-core/src/config/canonical_primary/domains/network/api.rs`**

**Why this location**:
- ✅ Already has comprehensive `NetworkApiConfig` (9 fields)
- ✅ Properly located in `domains/network` (matches architecture)
- ✅ Has TLS + RateLimiting sub-configs
- ✅ Has `development_optimized()` and `production_hardened()` methods
- ✅ Has validation logic
- ✅ Uses canonical constants

**Decision**: Extend `NetworkApiConfig` → Rename to `ApiConfig` for simplicity

---

## 📋 MIGRATION STEPS (ZFS Pattern)

### Step 1: Extend Canonical Config ✅ (2 hours)

**Target**: `domains/network/api.rs::NetworkApiConfig`

**Additions Needed**:
```rust
// Add from unified_api_config.rs
pub struct ApiConfig {  // Rename from NetworkApiConfig
    // Existing network fields (9)
    pub bind_address: IpAddr,
    pub port: u16,
    pub max_connections: u32,
    pub request_timeout: Duration,
    pub connection_timeout: Duration,
    pub tls_enabled: bool,
    pub tls: TlsConfig,
    pub rate_limiting: RateLimitingConfig,
    pub port_range_start: u16,
    pub port_range_end: u16,
    
    // NEW: Add server metadata
    pub version: String,           // from api_config.rs
    pub enabled: bool,             // from api_config.rs
    
    // NEW: Add security
    pub security: ApiSecurityConfig,  // from unified_api_config.rs
    
    // NEW: Add performance
    pub performance: ApiPerformanceConfig,  // from unified_api_config.rs
    
    // NEW: Add monitoring  
    pub monitoring: ApiMonitoringConfig,  // from unified_api_config.rs
    
    // NEW: Add handler extensions (optional)
    pub handlers: Option<ApiHandlerExtensions>,  // from handlers.rs
}

// Import sub-configs
pub struct ApiSecurityConfig { /* from unified_api_config.rs */ }
pub struct ApiPerformanceConfig { /* from unified_api_config.rs */ }
pub struct ApiMonitoringConfig { /* from unified_api_config.rs */ }
```

**Total New Fields**: ~25 fields (including sub-structs)

---

### Step 2: Update Exports ✅ (30 minutes)

**Files to Update**:
1. `domains/network/mod.rs` - Export all API types
2. `domains/mod.rs` - Re-export API types at domain level
3. `canonical_primary/mod.rs` - Ensure visible at top level

```rust
// domains/network/mod.rs
pub use api::{
    ApiConfig,
    ApiSecurityConfig,
    ApiPerformanceConfig,
    ApiMonitoringConfig,
    TlsConfig,
    RateLimitingConfig,
};

// domains/mod.rs  
pub use network::{
    ApiConfig,
    ApiSecurityConfig,
    /* ... */
};
```

---

### Step 3: Create Type Aliases ✅ (45 minutes)

**File**: `code/crates/nestgate-api/src/types.rs`

```rust
// Re-export canonical API configuration
pub use nestgate_core::config::canonical_primary::domains::network::{
    ApiConfig as CanonicalApiConfig,
    ApiSecurityConfig,
    ApiPerformanceConfig,
    ApiMonitoringConfig,
    TlsConfig,
    RateLimitingConfig,
};

// Backward compatibility
pub use CanonicalApiConfig as UnifiedApiConfig;
pub use CanonicalApiConfig as NetworkApiConfig;
```

---

### Step 4: Mark Old Files as Deprecated ✅ (1 hour)

**Files to Deprecate**:

1. ✅ `canonical_primary/api_config.rs`
```rust
#[deprecated(since = "0.2.0", note = "Use domains::network::ApiConfig")]
pub struct ApiConfig { /* ... */ }
```

2. ✅ `nestgate-api/config/unified_api_config.rs`
```rust
//! **⚠️ DEPRECATED - USE CANONICAL INSTEAD**
//! Use `nestgate_core::config::canonical_primary::domains::network::ApiConfig`
#[deprecated(since = "0.2.0", note = "Use crate::types::CanonicalApiConfig")]
pub struct UnifiedApiConfig { /* ... */ }
```

3. ✅ `unified_api_config/api_core.rs`
```rust
#[deprecated(since = "0.2.0", note = "Use nestgate_core::config::canonical_primary::domains::network::ApiConfig")]
pub struct UnifiedApiConfig { /* ... */ }
```

---

### Step 5: Update Tests (OPTIONAL) ⏳ (Skip if backward compat works)

**Strategy**: Let backward compatibility handle it initially

---

### Step 6: Remove Old Files ⏳ (30 minutes)

**Files to Remove** (after deprecation period):
1. `canonical_primary/api_config.rs` (26 lines - stub)
2. `unified_api_config/api_core.rs` (97 lines - legacy)
3. `nestgate-api/config/unified_api_config.rs` (202 lines - duplicate)

**Total cleanup**: ~325 lines removed

---

### Step 7: Final Validation ⏳ (30 minutes)

- ✅ Workspace builds cleanly
- ✅ All tests pass
- ✅ No breaking changes
- ✅ Documentation updated

---

## 🎯 SUCCESS CRITERIA

| Criterion | Target | Expected |
|-----------|--------|----------|
| **Canonical config complete** | 100% | ✅ All fields consolidated |
| **Exports working** | 100% | ✅ Clean export hierarchy |
| **Build clean** | 0 errors | ✅ Zero errors |
| **Tests passing** | 100% | ✅ All passing |
| **Breaking changes** | 0 | ✅ Backward compatible |
| **Deprecation warnings** | Strategic | ✅ Guide migration |

---

## 📊 ESTIMATED TIMELINE

| Step | Task | Time | Running Total |
|------|------|------|---------------|
| **1** | Extend canonical config | 2h | 2h |
| **2** | Update exports | 30min | 2.5h |
| **3** | Create type aliases | 45min | 3.25h |
| **4** | Mark old files deprecated | 1h | 4.25h |
| **5** | Update tests (skip) | - | 4.25h |
| **6** | Remove old files (later) | - | 4.25h |
| **7** | Final validation | 30min | 4.75h |

**Total Estimate**: **4.75 hours** (vs ZFS: 4.25h actual)

---

## 🚀 EXECUTION STRATEGY

### Phase 1: Extend Canonical (NOW)
1. Start with `domains/network/api.rs`
2. Rename `NetworkApiConfig` → `ApiConfig`
3. Add security/performance/monitoring sub-configs
4. Update Default implementations
5. **Validate**: `cargo check -p nestgate-core`

### Phase 2: Connect Exports (NOW)
1. Update `domains/network/mod.rs`
2. Update `domains/mod.rs`
3. **Validate**: `cargo check -p nestgate-core`

### Phase 3: Create Aliases (NOW)
1. Add to `nestgate-api/src/types.rs`
2. **Validate**: `cargo check -p nestgate-api`

### Phase 4: Deprecate Old (NOW)
1. Mark `api_config.rs` deprecated
2. Mark `unified_api_config.rs` deprecated
3. Mark `api_core.rs` deprecated
4. **Validate**: `cargo check --workspace`

### Phase 5: Polish (LATER)
1. Fix any example files
2. Update documentation
3. Remove deprecated files in next major version

---

## 🎯 CONFIDENCE LEVEL

**Ready to Execute**: ⭐⭐⭐⭐⭐ (VERY HIGH)

**Reasons**:
1. ✅ Pattern proven successful (ZFS: 0 errors, ahead of schedule)
2. ✅ Target file already comprehensive (NetworkApiConfig has 9 fields)
3. ✅ Clear source files identified
4. ✅ Backward compatibility strategy defined
5. ✅ Incremental validation at each step

**Risk Level**: **VERY LOW**

---

## 📚 REFERENCES

### Successful Pattern
- **ZFS Migration**: 4.25h actual, 0 errors, 248/248 tests passing
- **Key Success**: Backward-compatible re-exports

### Files to Reference
1. `domains/network/api.rs` - Primary target
2. `nestgate-api/config/unified_api_config.rs` - Fields to add
3. `canonical_primary/api_config.rs` - Stub to replace

---

**Status**: ✅ **READY TO EXECUTE**  
**Next Action**: Execute Step 1 (Extend Canonical Config)  
**Expected Duration**: 4.75 hours  
**Pattern**: ZFS Success (proven)

