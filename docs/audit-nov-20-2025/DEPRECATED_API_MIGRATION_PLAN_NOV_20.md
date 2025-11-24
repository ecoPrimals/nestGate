# Deprecated API Migration Plan - November 20, 2025

## 🎯 TASK
Migrate 13 deprecated `ServerConfig::bind_endpoint` usages to canonical configuration

## 📍 LOCATION
**File**: `code/crates/nestgate-api/src/bin/nestgate-api-server.rs`

## 🔍 CURRENT STATE

### Deprecated Struct
```rust
#[deprecated(since = "0.11.0")]
pub struct ServerConfig {
    pub bind_endpoint: SocketAddr,  // ⚠️ DEPRECATED
    pub enable_cors: bool,
    // ... other fields
}
```

### 13 Usages of `bind_endpoint`
1. Line 70: Struct field definition
2. Line 94: Default constructor assignment
3. Line 122: Info log - `config.bind_endpoint`
4. Line 156: Info log - `config.bind_endpoint`
5. Line 159: `TcpListener::bind(config.bind_endpoint)`
6. Line 176: Override assignment
7. Line 354: Port access - `config.bind_endpoint.port()`
8. Line 367: Port access - `config.bind_endpoint.port()`
9. Line 372: Port access - `config.bind_endpoint.port()`
10. Line 377: Port access - `config.bind_endpoint.port()`
11. Line 384: Port access - `config.bind_endpoint.port()`
12. Line 393: Port access - `config.bind_endpoint.port()`
13. Line 404: Port access - `config.bind_endpoint.port()`

## 🎯 MIGRATION TARGET

### Canonical Configuration
```rust
use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Access via:
// config.api.bind_address  (IpAddr)
// config.api.port  (u16)

// Construct SocketAddr:
// SocketAddr::new(config.api.bind_address, config.api.port)
```

## 📋 MIGRATION STEPS

### Step 1: Update Imports
- Add: `use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;`
- Potentially use `use nestgate_core::config::runtime::get_config;` (already present)

### Step 2: Replace ServerConfig Struct
- Remove deprecated `ServerConfig` struct  
- Use `CanonicalNetworkConfig` or runtime config helper

### Step 3: Update all 13 usages
Replace:
- `config.bind_endpoint` → `SocketAddr::new(config.api.bind_address, config.api.port)`
- `config.bind_endpoint.port()` → `config.api.port`

### Step 4: Test
- Build: `cargo build --bin nestgate-api-server`
- Verify no deprecation warnings
- Run integration tests

## ⚙️ IMPLEMENTATION APPROACH

### Option A: Direct Replacement (Recommended)
Replace `ServerConfig` entirely with canonical config access:
```rust
// Get canonical config
let runtime_config = nestgate_core::config::runtime::get_config();
let bind_addr = SocketAddr::new(
    runtime_config.network.api.bind_address,
    runtime_config.network.api.port
);
```

### Option B: Wrapper/Adapter
Keep `ServerConfig` structure but populate from canonical config:
```rust
impl ServerConfig {
    fn from_canonical(canonical: &CanonicalNetworkConfig) -> Self {
        Self {
            bind_endpoint: SocketAddr::new(
                canonical.api.bind_address,
                canonical.api.port
            ),
            // ... map other fields
        }
    }
}
```

## 📊 IMPACT ASSESSMENT

### Risk Level: LOW
- Single file affected
- No external API surface changes
- Well-documented migration path
- Existing #[allow(deprecated)] shows awareness

### Testing Required
- ✅ Build verification
- ✅ Unit tests (if any for binary)
- ✅ Integration tests
- ✅ Manual smoke test: Start server and verify binding

### Timeline
- **Estimated**: 15-30 minutes
- **Complexity**: Low-Medium
- **Priority**: P0 (deprecation cleanup)

## ✅ SUCCESS CRITERIA

1. ✅ No more `bind_endpoint` references
2. ✅ No deprecation warnings for this file
3. ✅ Server starts and binds correctly
4. ✅ All 13 usages migrated
5. ✅ Tests pass

## 🔄 ROLLBACK PLAN

If issues arise:
1. Git revert the changes
2. Keep `#[allow(deprecated)]` 
3. Document blockers
4. Reschedule migration

## 📝 NOTES

- The file already uses `nestgate_core::config::runtime::get_config()`  
- Migration comments already present in code
- This is part of broader consolidation to canonical config
- Timeline note: "maintained until v0.12.0 (May 2026)"

---

**Status**: Ready to execute  
**Estimated Time**: 15-30 minutes  
**Complexity**: Low-Medium  
**Priority**: P0
