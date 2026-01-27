# 🔧 Port Migration Batch 1 - Execution Plan

**Date**: January 27, 2026 15:00 UTC  
**Target**: Network Service Ports in Production Code  
**Scope**: nestgate-core/src/rpc + related network files  
**Estimated Impact**: ~30-50 hardcoded references  
**Time Estimate**: 1-2 hours

---

## 📊 CURRENT STATE

**Total Hardcoded Port References**: 1,303 across codebase  
**Focus Area**: `code/crates/nestgate-core/src/rpc/`

**Files Identified** (15 files with hardcoded socket addresses):
1. `rpc/mod.rs` - Documentation examples
2. `rpc/tarpc_server.rs` - **PRODUCTION CODE** ⚠️
3. `rpc/tarpc_client.rs` - Client connection code
4. `discovery_mechanism.rs` - Service discovery
5. `network/client/pool.rs` - Connection pooling
6. `config/runtime/network.rs` - Network configuration
7. Others (utilities, tests)

---

## 🎯 MIGRATION STRATEGY

### **Phase 1: Production Code** (Priority: CRITICAL)

**Target**: `rpc/tarpc_server.rs` lines 542, 549

**Current Code**:
```rust
// In supported_protocols() method:
ProtocolInfo {
    protocol: "jsonrpc".to_string(),
    version: "2.0".to_string(),
    endpoint: "http://0.0.0.0:8080/rpc".to_string(),  // ❌ HARDCODED
    priority: 2,
    enabled: false,
},
ProtocolInfo {
    protocol: "http".to_string(),
    version: "1.1".to_string(),
    endpoint: "http://0.0.0.0:8080".to_string(),  // ❌ HARDCODED
    priority: 3,
    enabled: false,
},
```

**Migration Pattern**:
```rust
use crate::constants::ports;
use crate::config::environment::NetworkConfig;

// Option A: Use constants module (simple)
let host = std::env::var("NESTGATE_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
let port = std::env::var("NESTGATE_PORT")
    .ok()
    .and_then(|s| s.parse().ok())
    .unwrap_or(ports::API_SERVER_DEFAULT);
let endpoint = format!("http://{}:{}/rpc", host, port);

// Option B: Use modern EnvironmentConfig (preferred)
let config = NetworkConfig::from_env_or_default();
let endpoint = format!("http://{}:{}/rpc", config.host(), config.port());
```

**Decision**: Use **Option A** for this batch (minimal refactoring).  
**Rationale**: Quick win, low risk, maintains current structure.

---

### **Phase 2: Documentation Examples**

**Target**: Doc comments in `rpc/mod.rs`, `rpc/tarpc_server.rs`

**Current Code**:
```rust
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let addr: SocketAddr = "0.0.0.0:8091".parse()?;  // ❌ HARDCODED
//! serve_tarpc(addr, service).await?;
//! # Ok(())
//! # }
```

**Migration Pattern**:
```rust
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! use nestgate_core::constants::ports;
//! let host = std::env::var("NESTGATE_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
//! let port = std::env::var("NESTGATE_RPC_PORT")
//!     .ok()
//!     .and_then(|s| s.parse().ok())
//!     .unwrap_or(8091);
//! let addr: SocketAddr = format!("{}:{}", host, port).parse()?;
//! serve_tarpc(addr, service).await?;
//! # Ok(())
//! # }
```

---

### **Phase 3: Test Code**

**Strategy**: Tests can keep hardcoded values BUT should use constants module.

**Pattern**:
```rust
// ❌ OLD: Direct hardcode
let addr = "127.0.0.1:8080".parse().unwrap();

// ✅ NEW: Use test-specific constants
use crate::constants::ports;
let addr = format!("127.0.0.1:{}", ports::API_SERVER_DEFAULT).parse().unwrap();

// ⭐ BETTER: Use test helper
let addr = test_socket_addr(ports::API_SERVER_DEFAULT);
```

---

## 📋 EXECUTION CHECKLIST

### **Step 1: Create Helper Functions** (10 min)

Add to `constants/ports.rs`:

```rust
/// Get API server address (host:port) from environment
///
/// **Environment Variables**:
/// - `NESTGATE_HOST`: Bind host (default: "0.0.0.0")
/// - `NESTGATE_PORT`: Bind port (default: 8080)
///
/// # Returns
/// Formatted address string like "0.0.0.0:8080"
pub fn get_api_server_addr() -> String {
    let host = std::env::var("NESTGATE_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("NESTGATE_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(API_SERVER_DEFAULT);
    format!("{}:{}", host, port)
}

/// Get RPC server address (host:port) from environment
///
/// **Environment Variables**:
/// - `NESTGATE_RPC_HOST`: Bind host (default: "0.0.0.0")
/// - `NESTGATE_RPC_PORT`: Bind port (default: 8091)
pub fn get_rpc_server_addr() -> String {
    let host = std::env::var("NESTGATE_RPC_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("NESTGATE_RPC_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8091);
    format!("{}:{}", host, port)
}
```

---

### **Step 2: Migrate Production Code** (20-30 min)

**File**: `rpc/tarpc_server.rs`

**Changes**:
1. Add import: `use crate::constants::ports;`
2. Update `supported_protocols()` method (lines 540-554)
3. Replace hardcoded endpoints with dynamic generation
4. Verify tests still pass

**Expected Diff**:
```diff
+ use crate::constants::ports;

  fn supported_protocols(&self) -> Vec<ProtocolInfo> {
+     let api_addr = ports::get_api_server_addr();
      vec![
          // ... tarpc protocol ...
          ProtocolInfo {
              protocol: "jsonrpc".to_string(),
              version: "2.0".to_string(),
-             endpoint: "http://0.0.0.0:8080/rpc".to_string(),
+             endpoint: format!("http://{}/rpc", api_addr),
              priority: 2,
              enabled: false,
          },
          ProtocolInfo {
              protocol: "http".to_string(),
              version: "1.1".to_string(),
-             endpoint: "http://0.0.0.0:8080".to_string(),
+             endpoint: format!("http://{}", api_addr),
              priority: 3,
              enabled: false,
          },
      ]
  }
```

---

### **Step 3: Update Documentation Examples** (15-20 min)

**Files**: `rpc/mod.rs`, `rpc/tarpc_server.rs`

**Pattern**: Show environment-driven configuration in examples

---

### **Step 4: Verify** (15 min)

```bash
# Build
cargo build --package nestgate-core --lib

# Test
cargo test --package nestgate-core --lib

# Clippy
cargo clippy --package nestgate-core --lib -- -D warnings

# Verify no new hardcoded references introduced
rg ":8080|:8091|:3030|:9090" code/crates/nestgate-core/src/rpc/
```

---

### **Step 5: Document** (10 min)

Update `PHASE_2_EXECUTION_LOG_JAN_27_2026.md`:

```markdown
## Batch 2: Port Migration - RPC Server Endpoints

**Status**: ✅ COMPLETE  
**Time**: 1.5 hours  
**Files Modified**: 3

**Changes**:
- ✅ Added `get_api_server_addr()` helper to `constants/ports.rs`
- ✅ Added `get_rpc_server_addr()` helper to `constants/ports.rs`
- ✅ Migrated `rpc/tarpc_server.rs` production code (2 hardcoded refs)
- ✅ Updated documentation examples (4 refs)
- ✅ All tests passing

**Impact**:
- Hardcoded port refs: 306 → 300 (-6)
- Production hardcoded endpoints: 2 → 0
- Environment-driven: API server, RPC server
- Grade: A- (90.5) → A- (90.6)

**Pattern Established**: Helper functions in constants module
```

---

## 🎯 SUCCESS METRICS

**Before**:
- Production hardcoded endpoints: 2+ in tarpc_server.rs
- Environment support: None for protocol endpoints
- Flexibility: Zero (hardcoded 0.0.0.0:8080)

**After**:
- Production hardcoded endpoints: 0 ✅
- Environment support: Full ($NESTGATE_HOST, $NESTGATE_PORT)
- Flexibility: Complete (any host/port via env)

**Grade Impact**: +0.1 points (small but important win)

---

## 🔄 NEXT BATCHES

### **Batch 2: Discovery Mechanism**

**Target**: `discovery_mechanism.rs` (2 refs)  
**Time**: 30 min  
**Pattern**: Same as Batch 1

### **Batch 3: Network Client Pool**

**Target**: `network/client/pool.rs` (3 refs)  
**Time**: 30 min  
**Pattern**: Connection pool configuration

### **Batch 4: Configuration Files**

**Target**: `config/runtime/network.rs`, `config/capability_discovery.rs`  
**Time**: 45 min  
**Pattern**: Config system integration

### **Batch 5-10: Systematic Cleanup**

**Target**: Remaining 280+ references  
**Time**: 8-10 hours  
**Pattern**: Apply established patterns

---

## 💡 PATTERNS ESTABLISHED

### **Pattern 1: Helper Functions**

**Location**: `constants/ports.rs`  
**Format**: `get_<service>_addr() -> String`  
**Env Vars**: `NESTGATE_<SERVICE>_HOST`, `NESTGATE_<SERVICE>_PORT`  
**Defaults**: Constants from same module

### **Pattern 2: Production Code Migration**

```rust
// ❌ BEFORE
let endpoint = "http://0.0.0.0:8080/rpc".to_string();

// ✅ AFTER
use crate::constants::ports;
let endpoint = format!("http://{}/rpc", ports::get_api_server_addr());
```

### **Pattern 3: Documentation Examples**

```rust
//! # Example
//! ```no_run
//! use nestgate_core::constants::ports;
//! let addr = ports::get_api_server_addr();
//! ```
```

---

## 🚀 EXECUTION READINESS

**Prerequisites**: ✅ All met
- ✅ Constants module exists
- ✅ Patterns identified
- ✅ Files located
- ✅ Tests baseline established

**Confidence**: **VERY HIGH** 💪
- Clear scope (15 files, ~50 refs)
- Established pattern
- Low risk (mostly disabled protocol stubs)
- Easy verification

**Expected Outcome**: Clean execution, pattern validated, momentum maintained

---

**Status**: 📋 **READY TO EXECUTE**  
**Next Action**: Create helper functions in `constants/ports.rs`  
**Time to Start**: **NOW** 🚀

---

*Systematic execution · Deep debt solutions · Environment-driven excellence*

**🦀 Foundation ready. Pattern clear. Execute with confidence. 🚀**
