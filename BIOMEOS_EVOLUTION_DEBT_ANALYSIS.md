# 🗄️ NestGate Evolution Debt - biomeOS Integration Analysis

**Date**: January 10, 2026  
**Status**: ⚠️ **CRITICAL GAP IDENTIFIED**  
**Priority**: **HIGH** - Blocks biomeOS native IPC integration

---

## 🎯 **EXECUTIVE SUMMARY**

**biomeOS Integration Assessment**:
- **Grade**: B+ (85/100) from biomeOS perspective
- **Status**: ✅ **Binary Harvested** (v2.0.0, 3.4 MB)
- **Critical Gap**: ⚠️ **JSON-RPC Server Mode Missing**
- **Client**: ✅ Ready (7 methods implemented in biomeOS)
- **Timeline**: 2-3 weeks to implement server mode

---

## 📊 **CURRENT STATUS - NESTGATE**

### **What We Have** ✅
```
Production Ready:
✅ Grade: A- (92/100) from our perspective
✅ Tests: 1,224+ passing (100%)
✅ Warnings: 3 (down from 25)
✅ Build: PASSING
✅ Architecture: Modern, capability-based
✅ File size: 100% compliant
✅ Technical debt: Minimal
✅ Philosophy: 98/100 validated
```

### **What biomeOS Expects** ⚠️
```
Critical Requirements:
⚠️ JSON-RPC 2.0 Server (Unix socket)
⚠️ Socket path: /run/user/{uid}/nestgate-{family_id}.sock
⚠️ Environment: $NESTGATE_FAMILY_ID
⚠️ 7 API methods (storage.*)
⚠️ Songbird auto-registration
⚠️ Capability registration
```

---

## 🔍 **GAP ANALYSIS**

### **1. JSON-RPC Server Mode** ⚠️ **MISSING**

**What biomeOS Needs**:
```rust
// Unix socket JSON-RPC 2.0 server
Socket Path: /run/user/{uid}/nestgate-{family_id}.sock
Environment: $NESTGATE_FAMILY_ID

Methods (7 required):
1. storage.store(key, data, family_id) -> StorageResult
2. storage.retrieve(key, family_id) -> Value
3. storage.delete(key, family_id) -> ()
4. storage.list(family_id, prefix?) -> Vec<String>
5. storage.stats(family_id) -> StorageStats
6. storage.store_blob(key, blob_base64, family_id) -> StorageResult
7. storage.retrieve_blob(key, family_id) -> blob_base64
```

**What We Currently Have**:
```rust
// We have JSON-RPC types and client, but NO server implementation
// File: code/crates/nestgate-core/src/rpc/jsonrpc_server.rs exists
// But it's HTTP-based, not Unix socket based

Current: HTTP JSON-RPC (not suitable for IPC)
Needed: Unix socket JSON-RPC (biomeOS standard)
```

**Gap**: Unix socket listener + JSON-RPC 2.0 handler not implemented

### **2. API Method Alignment** ⚠️ **NEEDS VERIFICATION**

**biomeOS Client Methods**:
1. ✅ `store(key, data)` - Store key-value
2. ✅ `retrieve(key)` - Retrieve data
3. ✅ `delete(key)` - Delete data
4. ✅ `list_keys(prefix)` - List keys
5. ✅ `get_stats()` - Get statistics
6. ✅ `store_blob(id, data)` - Store blob
7. ✅ `retrieve_blob(id)` - Retrieve blob

**Our Current RPC Methods** (from tarpc_types.rs):
```rust
// We have dataset-based methods, not storage.*
- create_dataset
- delete_dataset
- store_object
- retrieve_object
- list_objects
- get_metrics
```

**Gap**: Method names and signatures don't match biomeOS expectations

### **3. Socket Path Logic** ⚠️ **MISSING**

**Required**:
```rust
// Socket path pattern
/run/user/{uid}/nestgate-{family_id}.sock

// Where family_id comes from:
std::env::var("NESTGATE_FAMILY_ID")
    .context("NESTGATE_FAMILY_ID not set")
```

**Current**: No Unix socket server implementation at all

### **4. Songbird Auto-Registration** ⚠️ **PARTIAL**

**Required**:
```rust
// On startup:
1. Discover Songbird via $SONGBIRD_FAMILY_ID
2. Register capabilities:
   - "storage"
   - "persistence"
   - "key-value"
   - "blob-storage"
3. Report health periodically
```

**Current**: We have capability discovery, but no auto-registration on startup

---

## 🏗️ **IMPLEMENTATION PLAN**

### **Phase 1: JSON-RPC Unix Socket Server** (Week 1-2)

**Task 1.1: Unix Socket Listener**
```rust
// New file: code/crates/nestgate-core/src/rpc/jsonrpc_unix_server.rs

use tokio::net::UnixListener;
use std::path::PathBuf;

pub struct JsonRpcUnixServer {
    socket_path: PathBuf,
    service: NestGateStorageService,
}

impl JsonRpcUnixServer {
    pub async fn new(family_id: &str) -> Result<Self> {
        let uid = users::get_current_uid();
        let socket_path = PathBuf::from(format!(
            "/run/user/{}/nestgate-{}.sock",
            uid, family_id
        ));
        
        // Create socket listener
        let listener = UnixListener::bind(&socket_path)?;
        
        Ok(Self {
            socket_path,
            service: NestGateStorageService::new(),
        })
    }
    
    pub async fn serve(self) -> Result<()> {
        // Accept connections and handle JSON-RPC 2.0 requests
        loop {
            let (stream, _) = self.listener.accept().await?;
            // Handle JSON-RPC messages
        }
    }
}
```

**Task 1.2: Storage Service Implementation**
```rust
// Implement the 7 storage.* methods
pub struct NestGateStorageService {
    // Internal storage backend
}

impl NestGateStorageService {
    async fn storage_store(&self, key: String, data: Value, family_id: String) 
        -> Result<StorageResult> {
        // Implementation
    }
    
    async fn storage_retrieve(&self, key: String, family_id: String) 
        -> Result<Value> {
        // Implementation
    }
    
    // ... 5 more methods
}
```

**Task 1.3: JSON-RPC 2.0 Handler**
```rust
// Handle JSON-RPC 2.0 requests
async fn handle_jsonrpc_request(
    request: JsonRpcRequest,
    service: &NestGateStorageService
) -> JsonRpcResponse {
    match request.method.as_str() {
        "storage.store" => /* ... */,
        "storage.retrieve" => /* ... */,
        // ... handle all 7 methods
    }
}
```

### **Phase 2: Songbird Integration** (Week 2-3)

**Task 2.1: Auto-Registration**
```rust
// On startup, register with Songbird
async fn register_with_songbird(family_id: &str) -> Result<()> {
    let songbird_client = SongbirdClient::discover(family_id).await?;
    
    songbird_client.register_service(ServiceInfo {
        name: "nestgate",
        version: env!("CARGO_PKG_VERSION"),
        capabilities: vec![
            "storage",
            "persistence",
            "key-value",
            "blob-storage",
        ],
        socket_path: format!("/run/user/{}/nestgate-{}.sock", uid, family_id),
    }).await?;
    
    Ok(())
}
```

**Task 2.2: Health Reporting**
```rust
// Periodic health updates
tokio::spawn(async move {
    loop {
        songbird_client.report_health(HealthStatus {
            status: "healthy",
            uptime: start_time.elapsed(),
        }).await?;
        
        tokio::time::sleep(Duration::from_secs(30)).await;
    }
});
```

### **Phase 3: Testing & Integration** (Week 3)

**Task 3.1: Unit Tests**
```rust
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_storage_store_retrieve() {
        let server = JsonRpcUnixServer::new("test").await.unwrap();
        // Test all 7 methods
    }
}
```

**Task 3.2: Integration Tests with biomeOS**
```rust
// Test with actual biomeOS NestGateClient
#[tokio::test]
async fn test_biomeos_client_integration() {
    // Start our server
    let server = JsonRpcUnixServer::new("test").await.unwrap();
    tokio::spawn(server.serve());
    
    // Use biomeOS client
    let client = biomeos_core::clients::NestGateClient::discover("test").await.unwrap();
    
    // Test all operations
    client.store("key", &json!({"value": "test"})).await.unwrap();
    let data = client.retrieve("key").await.unwrap();
    assert_eq!(data["value"], "test");
}
```

---

## 📋 **EVOLUTION DEBT CHECKLIST**

### **Critical (Blocks biomeOS)** ⚠️
- [ ] Implement Unix socket JSON-RPC 2.0 server
- [ ] Implement 7 storage.* methods
- [ ] Add socket path logic ($NESTGATE_FAMILY_ID)
- [ ] Add Songbird auto-registration
- [ ] Test with biomeOS NestGateClient

### **High Priority**
- [ ] Fix mdns-discovery warning
- [ ] Align API method names with biomeOS expectations
- [ ] Add capability registration
- [ ] Implement health reporting

### **Medium Priority** (Already tracked)
- [ ] Continue test coverage expansion (69.7% → 75%)
- [ ] Performance optimization opportunities
- [ ] Study more BearDog patterns

### **Low Priority**
- [ ] Unwrap cleanup (~200 instances in tests)
- [ ] Code documentation expansion
- [ ] Performance benchmarks

---

## 🎯 **COMPARISON: US VS BEARDOG**

### **NestGate Status**:
```
Grade: A- (92/100) - Our perspective
       B+ (85/100) - biomeOS perspective
Tests: 1,224+ passing
biomeOS Integration: ⚠️ BLOCKED (needs server mode)
```

### **BearDog Status** (Reference):
```
Grade: A+ (100/100)
Tests: 16,135 passing
Coverage: 97.4%
biomeOS Integration: ✅ COMPLETE (JSON-RPC server working)
```

### **What BearDog Has That We Need**:
1. ✅ JSON-RPC Unix socket server
2. ✅ Songbird auto-registration
3. ✅ biomeOS client compatibility
4. ✅ Comprehensive testing

---

## 📈 **TIMELINE & EFFORT**

### **JSON-RPC Server Implementation**:
- **Complexity**: Medium
- **Effort**: 2-3 weeks (1 developer)
- **Dependencies**: None (Songbird optional)
- **Blockers**: None

### **Breakdown**:
```
Week 1:
- Days 1-2: Unix socket listener + JSON-RPC handler
- Days 3-4: Implement 7 storage.* methods
- Day 5: Unit tests

Week 2:
- Days 1-2: Songbird integration
- Days 3-4: Integration testing with biomeOS
- Day 5: Bug fixes

Week 3 (optional):
- Performance testing
- Load testing
- Production hardening
```

### **Comparison to Other Primals**:
- **Songbird**: ✅ Done (has JSON-RPC server)
- **BearDog**: ✅ Done (has JSON-RPC server)
- **Squirrel**: ✅ Done (has JSON-RPC server)
- **NestGate**: ⚠️ **2-3 weeks** (THIS TASK)
- **ToadStool**: ⚠️ Similar effort needed
- **petalTongue**: ⚠️ Just needs wiring

---

## 🚀 **IMMEDIATE NEXT STEPS**

### **For NestGate Team** (Us):
1. **Create JSON-RPC Unix server** (Priority 1)
   - New file: `jsonrpc_unix_server.rs`
   - Unix socket listener
   - JSON-RPC 2.0 handler

2. **Implement storage.* methods** (Priority 1)
   - Align with biomeOS API expectations
   - 7 methods matching client

3. **Add Songbird integration** (Priority 2)
   - Auto-registration on startup
   - Capability registration
   - Health reporting

4. **Test with biomeOS** (Priority 1)
   - Integration tests
   - Verify all 7 methods work
   - Load testing

### **For biomeOS** (Already Done):
- ✅ Client implemented (NestGateClient)
- ✅ 7 methods defined
- ✅ Transport layer ready
- ✅ Integration tests scaffolded
- ⏳ Waiting for our server implementation

---

## 📊 **SUCCESS CRITERIA**

### **Definition of Done**:
1. ✅ Unix socket JSON-RPC server running
2. ✅ All 7 storage.* methods implemented
3. ✅ biomeOS NestGateClient can connect
4. ✅ All integration tests passing
5. ✅ Songbird registration working
6. ✅ Health reporting functional
7. ✅ Grade improves to A (95/100) from biomeOS

### **Validation**:
```bash
# Test 1: Server starts
$ NESTGATE_FAMILY_ID=test ./target/release/nestgate
✅ NestGate v2.0.0 starting...
✅ Unix socket: /run/user/1000/nestgate-test.sock
✅ Registered with Songbird
✅ Ready for connections

# Test 2: biomeOS client connects
$ cargo test --package biomeos-core -- nestgate_integration
✅ test nestgate_store_retrieve ... ok
✅ test nestgate_delete ... ok
✅ test nestgate_list_keys ... ok
✅ test nestgate_stats ... ok
✅ test nestgate_blob_storage ... ok
```

---

## 🎊 **CONCLUSION**

### **Current State**:
- **Our Perspective**: A- (92/100) - Production ready
- **biomeOS Perspective**: B+ (85/100) - Needs JSON-RPC server

### **Critical Gap**:
- ⚠️ **JSON-RPC Unix socket server** (2-3 weeks)

### **Path Forward**:
1. Implement JSON-RPC Unix server (Week 1-2)
2. Integrate with Songbird (Week 2-3)
3. Test with biomeOS (Week 3)
4. Deploy to production

### **Impact**:
- **HIGH**: Enables biomeOS native IPC
- **HIGH**: Unlocks persistence layer for ecosystem
- **CRITICAL**: Required for 7-primal ecosystem completion

---

**Status**: ⚠️ **EVOLUTION DEBT IDENTIFIED**  
**Priority**: **HIGH** (blocks biomeOS integration)  
**Timeline**: 2-3 weeks  
**Next**: Implement JSON-RPC Unix socket server

🗄️ **NestGate: Ready to Evolve!** 🗄️
