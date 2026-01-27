# 🌍 NestGate Universal IPC Evolution Plan - January 19, 2026

**From**: NestGate Team  
**To**: biomeOS Architecture, Songbird Team, ToadStool Team  
**Subject**: NestGate Evolution to Support Universal IPC Architecture  
**Status**: 🔄 IN PROGRESS

---

## 🎯 EXECUTIVE SUMMARY

**Mission**: Evolve NestGate to its proper role in the Three-Primal Universal IPC Architecture

**Role**: Persistent metadata storage (NOT connection logic!)  
**Partner**: Songbird (owns ALL communication)  
**Result**: True separation of concerns

**Timeline**: 2-3 weeks (5-8 hours total)

---

## 📋 NESTGATE'S ROLE IN UNIVERSAL IPC

### ✅ What NestGate Owns

1. **Persistent Service Registry**
   - Store service metadata (name, version, capabilities, endpoint)
   - Index by capability for discovery
   - Survive restarts (persistent storage)

2. **Capability-Based Discovery**
   - Find services by capability
   - List all services
   - Service health status

3. **Metadata Storage**
   - Platform information
   - Version tracking
   - Custom metadata

### ❌ What NestGate Does NOT Own

1. **IPC Connections** → Songbird's domain
2. **Socket Creation** → Songbird's domain
3. **Platform-Specific Logic** → Songbird's domain
4. **Communication Protocols** → Songbird's domain

**Clean Separation**: Storage ≠ Connection!

---

## 🏗️ ARCHITECTURE

### Current State (Mixed Concerns)

```
❌ PROBLEM: NestGate handles both storage AND connections

NestGate:
  - Unix socket server (should be Songbird!)
  - Service registry storage (correct!)
  - JSON-RPC over Unix sockets (should be Songbird!)
  - Capability discovery (correct!)
```

### Target State (Clean Separation)

```
✅ SOLUTION: Clean delegation

Songbird (Communication Service):
  - Provides IPC discovery SERVICE (NOT library!)
  - Exposes /primal/songbird socket with JSON-RPC
  - Creates platform-specific endpoints
  - Handles ALL connections (remote + local IPC)
  - Abstracts platform differences
  
NestGate (Storage Layer):
  - Stores service metadata (persistent)
  - Enables capability-based discovery
  - Provides persistent registry
  - Calls Songbird SERVICE for runtime resolution

Application Primals:
  - Call Songbird SERVICE for connections (JSON-RPC)
  - Call NestGate for persistent metadata (optional)
  - NO cross-embedding (service calls only!)
```

---

## 🦅 SONGBIRD INTEGRATION (SERVICE-BASED)

### **CRITICAL**: Songbird is a SERVICE, NOT a Library!

**❌ WRONG (Cross-Embedding)**:
```rust
// DO NOT DO THIS!
use songbird_universal_ipc::ipc;  // ❌ Embeds Songbird code!
let stream = ipc::connect("/primal/beardog").await?;
```

**✅ CORRECT (Service-Based)**:
```rust
// Use Songbird as a SERVICE
use tokio::net::UnixStream;

// 1. Connect to Songbird service
let mut songbird = UnixStream::connect("/primal/songbird").await?;

// 2. Ask Songbird via JSON-RPC
let request = json!({
    "jsonrpc": "2.0",
    "method": "ipc.resolve",
    "params": { "primal": "beardog" },
    "id": 1
});
write_json(&mut songbird, &request).await?;

// 3. Get endpoint from response
let response: JsonRpcResponse = read_json(&mut songbird).await?;
let endpoint = response.result.endpoint;

// 4. Connect directly to target
let stream = UnixStream::connect(&endpoint).await?;
```

### **Songbird JSON-RPC Protocol**

**Endpoint**: `/primal/songbird` (Unix socket)

**Methods**:
1. **`ipc.register`** - Register this primal
   ```json
   {
     "jsonrpc": "2.0",
     "method": "ipc.register",
     "params": {
       "primal": "nestgate",
       "capabilities": ["storage", "discovery"],
       "endpoint": "/primal/nestgate"
     },
     "id": 1
   }
   ```

2. **`ipc.resolve`** - Find a primal's endpoint
   ```json
   {
     "jsonrpc": "2.0",
     "method": "ipc.resolve",
     "params": { "primal": "beardog" },
     "id": 1
   }
   ```

3. **`ipc.capabilities`** - Find primals by capability
   ```json
   {
     "jsonrpc": "2.0",
     "method": "ipc.capabilities",
     "params": { "capability": "crypto" },
     "id": 1
   }
   ```

4. **`ipc.list`** - List all registered primals
   ```json
   {
     "jsonrpc": "2.0",
     "method": "ipc.list",
     "params": {},
     "id": 1
   }
   ```

### **NestGate's Integration with Songbird**

**On Startup** (register):
```rust
async fn register_with_songbird() -> Result<()> {
    let mut songbird = UnixStream::connect("/primal/songbird").await?;
    
    let request = json!({
        "jsonrpc": "2.0",
        "method": "ipc.register",
        "params": {
            "primal": "nestgate",
            "capabilities": ["storage", "discovery", "metadata"],
            "endpoint": "/primal/nestgate"
        },
        "id": 1
    });
    
    write_json(&mut songbird, &request).await?;
    let _response: JsonRpcResponse = read_json(&mut songbird).await?;
    Ok(())
}
```

**For Discovery** (resolve):
```rust
async fn resolve_primal(name: &str) -> Result<String> {
    // 1. Check our metadata cache first (fast path)
    if let Ok(meta) = service_store.get_service(name).await {
        return Ok(meta.native_endpoint);
    }
    
    // 2. Cache miss - ask Songbird (service call)
    let mut songbird = UnixStream::connect("/primal/songbird").await?;
    
    let request = json!({
        "jsonrpc": "2.0",
        "method": "ipc.resolve",
        "params": { "primal": name },
        "id": 1
    });
    
    write_json(&mut songbird, &request).await?;
    let response: JsonRpcResponse = read_json(&mut songbird).await?;
    
    let endpoint = response.result.endpoint;
    
    // 3. Cache in our metadata store
    service_store.store_service(ServiceMetadata {
        name: name.to_string(),
        native_endpoint: endpoint.clone(),
        // ... other metadata
    }).await?;
    
    Ok(endpoint)
}
```

**Key Principles**:
- ✅ **Service calls ONLY** (no imports)
- ✅ **Standard JSON-RPC** (protocol-based)
- ✅ **Cache + fallback** (fast path + discovery)
- ✅ **Zero cross-embedding** (primal autonomy)

---

## 📊 CURRENT STATE ANALYSIS

### Files with Mixed Concerns (Need Evolution)

| File | Current Role | Should Be |
|------|-------------|-----------|
| `rpc/unix_socket_server.rs` | Creates Unix sockets | → Songbird |
| `rpc/socket_config.rs` | Socket configuration | → Songbird |
| `api/transport/unix_socket.rs` | Unix socket listener | → Songbird |
| `api/transport/server.rs` | Connection handling | → Songbird |

### Files with Correct Concerns (Keep & Enhance)

| File | Current Role | Evolution |
|------|-------------|-----------|
| `capabilities/discovery/registry.rs` | Service capability registry | ✅ Enhance |
| `service_discovery/registry.rs` | Service registration | ✅ Enhance |
| `service_metadata/mod.rs` | **NEW** Service metadata | ✅ Use |

---

## 🚀 IMPLEMENTATION PLAN

### Phase 1: Foundation (Week 1) ✅ COMPLETE

**Goal**: Create clean service metadata storage API

- [x] Create `service_metadata` module
- [x] Define `ServiceMetadata` struct
- [x] Implement `ServiceMetadataStore` (lock-free with DashMap)
- [x] Capability-based indexing
- [x] Comprehensive tests
- [x] Integration into `lib.rs`

**Deliverable**: Clean API for Songbird integration

**Files**:
- ✅ `code/crates/nestgate-core/src/service_metadata/mod.rs`

---

### Phase 2: Deprecation Markers (Week 1-2)

**Goal**: Mark connection code for delegation to Songbird

**Tasks**:

1. **Add deprecation notices to Unix socket code**
   ```rust
   #[deprecated(
       since = "2.3.0",
       note = "Connection logic moved to Songbird (Universal IPC). Use songbird::ipc instead."
   )]
   pub struct JsonRpcUnixServer { ... }
   ```

2. **Update documentation**
   - Add migration guides
   - Reference Universal IPC architecture
   - Explain Songbird delegation

3. **Create compatibility layer**
   - Keep existing APIs working (backward compatibility)
   - Add deprecation warnings
   - Guide users to Songbird

**Files to Update**:
- `code/crates/nestgate-core/src/rpc/unix_socket_server.rs`
- `code/crates/nestgate-api/src/transport/unix_socket.rs`
- `code/crates/nestgate-api/src/transport/server.rs`
- `code/crates/nestgate-bin/src/commands/service.rs`

**Deliverable**: Clear migration path for users

---

### Phase 3: Songbird Integration API (Week 2)

**Goal**: Create clean API for Songbird to store metadata

**Tasks**:

1. **Create Songbird integration module**
   ```rust
   // code/crates/nestgate-core/src/service_metadata/songbird_integration.rs
   
   pub struct SongbirdIntegration {
       store: ServiceMetadataStore,
   }
   
   impl SongbirdIntegration {
       /// Called by Songbird when registering a service
       pub async fn register_service(
           &self,
           name: &str,
           capabilities: Vec<String>,
           virtual_endpoint: String,
           native_endpoint: String,
       ) -> Result<()> {
           let meta = ServiceMetadata {
               name: name.to_string(),
               version: "1.0.0".to_string(), // TODO: From service
               capabilities,
               virtual_endpoint,
               registered_at: SystemTime::now(),
               last_seen: SystemTime::now(),
               platform: std::env::consts::OS.to_string(),
               native_endpoint,
               metadata: std::collections::HashMap::new(),
           };
           
           self.store.store_service(meta).await
       }
   }
   ```

2. **Create discovery API for application primals**
   ```rust
   // Application primals can still use NestGate for discovery
   let services = nestgate::service_metadata::find_by_capability("crypto").await?;
   
   // But connect via Songbird (not NestGate!)
   for service in services {
       let stream = songbird::ipc::connect(&service.virtual_endpoint).await?;
       // Use stream...
   }
   ```

**Files to Create**:
- `code/crates/nestgate-core/src/service_metadata/songbird_integration.rs`
- `code/crates/nestgate-core/src/service_metadata/discovery_api.rs`

**Deliverable**: Clean integration with Songbird

---

### Phase 4: Persistent Storage (Week 2-3)

**Goal**: Back metadata store with persistent storage

**Tasks**:

1. **Integrate with NestGate's key-value store**
   ```rust
   impl ServiceMetadataStore {
       /// Store service metadata (persistent!)
       pub async fn store_service(&self, meta: ServiceMetadata) -> Result<()> {
           // In-memory (fast access)
           self.services.insert(meta.name.clone(), meta.clone());
           
           // Persistent storage (survives restarts)
           let key = format!("services/{}", meta.name);
           self.kv_store.put(&key, &meta).await?;
           
           // ... rest of implementation
       }
   }
   ```

2. **Implement recovery on startup**
   - Load metadata from persistent storage
   - Rebuild in-memory indices
   - Validate stale entries

3. **Add cleanup/garbage collection**
   - Remove services not seen in N hours
   - Prune old metadata
   - Compact storage

**Files to Update**:
- `code/crates/nestgate-core/src/service_metadata/mod.rs`
- `code/crates/nestgate-core/src/service_metadata/persistence.rs` (new)

**Deliverable**: Metadata survives restarts

---

### Phase 5: Testing & Documentation (Week 3)

**Goal**: Comprehensive testing and documentation

**Tasks**:

1. **Integration tests with Songbird**
   - Mock Songbird registering services
   - Test discovery flow
   - Test persistence
   - Test capability indexing

2. **Documentation**
   - API documentation (rustdoc)
   - Integration guide for Songbird
   - Migration guide for application primals
   - Architecture diagrams

3. **Examples**
   - Simple registration example
   - Discovery example
   - Full integration example

**Files to Create**:
- `code/crates/nestgate-core/src/service_metadata/tests/integration_tests.rs`
- `docs/integration/universal_ipc/NESTGATE_ROLE.md`
- `docs/integration/universal_ipc/SONGBIRD_INTEGRATION_GUIDE.md`
- `docs/integration/universal_ipc/MIGRATION_GUIDE.md`
- `examples/service_metadata_integration.rs`

**Deliverable**: Production-ready documentation

---

### Phase 6: Release (Week 3)

**Goal**: Release updated NestGate with clean separation

**Tasks**:

1. **Version bump**
   - NestGate v2.3.0
   - Breaking change: Deprecation notices
   - Feature: Service metadata storage

2. **Changelog**
   - New: `service_metadata` module
   - Deprecated: Unix socket server (use Songbird)
   - Enhanced: Capability-based discovery
   - Fixed: Clean separation of concerns

3. **Coordination**
   - Notify Songbird team
   - Notify biomeOS team
   - Update wateringHole docs

**Deliverable**: v2.3.0 release

---

## 📝 DETAILED TASK BREAKDOWN

### Week 1: Foundation & Deprecation

**Day 1-2**: Service Metadata Module ✅
- [x] Create module structure
- [x] Implement `ServiceMetadata` struct
- [x] Implement `ServiceMetadataStore`
- [x] Write tests

**Day 3-4**: Deprecation Markers
- [ ] Add deprecation notices to connection code
- [ ] Update documentation
- [ ] Create compatibility layer

**Day 5**: Testing & Review
- [ ] Run full test suite
- [ ] Code review
- [ ] Fix any issues

---

### Week 2: Integration & Persistence

**Day 1-2**: Songbird Integration
- [ ] Create integration module
- [ ] Implement registration API
- [ ] Implement discovery API

**Day 3-4**: Persistent Storage
- [ ] Integrate with key-value store
- [ ] Implement recovery
- [ ] Add cleanup/GC

**Day 5**: Testing
- [ ] Integration tests
- [ ] Performance tests
- [ ] Edge case tests

---

### Week 3: Documentation & Release

**Day 1-2**: Documentation
- [ ] API documentation
- [ ] Integration guides
- [ ] Migration guides
- [ ] Examples

**Day 3-4**: Final Testing
- [ ] Full integration test
- [ ] Cross-platform validation
- [ ] Performance benchmarks

**Day 5**: Release
- [ ] Version bump
- [ ] Changelog
- [ ] Release notes
- [ ] Coordination with teams

---

## 🎯 SUCCESS CRITERIA

### After Phase 1 (Current) ✅

- [x] `service_metadata` module exists
- [x] Clean API for metadata storage
- [x] Capability-based indexing
- [x] Comprehensive tests

### After Phase 2

- [ ] Connection code marked as deprecated
- [ ] Clear migration path documented
- [ ] Backward compatibility maintained

### After Phase 3

- [ ] Clean API for Songbird integration
- [ ] Discovery API for application primals
- [ ] Integration tests passing

### After Phase 4

- [ ] Metadata persists across restarts
- [ ] Recovery on startup works
- [ ] Cleanup/GC implemented

### After Phase 5

- [ ] Comprehensive documentation
- [ ] Integration guides complete
- [ ] Examples working

### After Phase 6

- [ ] v2.3.0 released
- [ ] Teams coordinated
- [ ] Production ready

---

## 🔗 INTEGRATION WITH SONGBIRD

### Songbird's Responsibilities

```rust
// Songbird handles ALL communication

// 1. Register a primal (Songbird creates endpoint)
let endpoint = songbird::ipc::register("beardog").await?;

// 2. Store metadata in NestGate
nestgate::service_metadata::register_service(
    "beardog",
    vec!["crypto", "btsp"],
    endpoint.virtual_path(),
    endpoint.native_path(),
).await?;

// 3. Listen for connections (Songbird handles protocol)
songbird::ipc::listen(endpoint).await?;
```

### NestGate's Responsibilities

```rust
// NestGate provides discovery only

// 1. Find services by capability
let services = nestgate::service_metadata::find_by_capability("crypto").await?;

// 2. Get service details
for service in services {
    println!("Service: {}", service.name);
    println!("Endpoint: {}", service.virtual_endpoint);
    println!("Capabilities: {:?}", service.capabilities);
}

// 3. Connect via Songbird (NOT NestGate!)
let stream = songbird::ipc::connect(&services[0].virtual_endpoint).await?;
```

### Application Primal Usage

```rust
// Application primals use BOTH (for different purposes)

// Discovery: NestGate
let crypto_providers = nestgate::find_by_capability("crypto").await?;

// Connection: Songbird
let stream = songbird::ipc::connect(&crypto_providers[0].virtual_endpoint).await?;

// No platform-specific code needed! ✅
```

---

## 📊 MIGRATION GUIDE FOR USERS

### Before (Mixed Concerns)

```rust
// Application code had to deal with Unix sockets directly
use nestgate::rpc::JsonRpcUnixServer;
use tokio::net::UnixStream;

let server = JsonRpcUnixServer::new("myservice").await?;
server.serve().await?;

let stream = UnixStream::connect("/tmp/primal-beardog.sock").await?;
// Use stream...
```

### After (Clean Separation)

```rust
// Application code is platform-agnostic!
use songbird::ipc;
use nestgate::service_metadata;

// Register via Songbird
let endpoint = ipc::register("myservice").await?;
ipc::listen(endpoint).await?;

// Discover via NestGate
let services = service_metadata::find_by_capability("crypto").await?;

// Connect via Songbird (works on ALL platforms!)
let stream = ipc::connect(&services[0].virtual_endpoint).await?;
// Use stream...
```

**Benefits**:
- ✅ No platform-specific code (#[cfg] gone!)
- ✅ Works on Linux, macOS, Windows, RISC-V, etc.
- ✅ Cleaner separation of concerns
- ✅ Easier to test and maintain

---

## 🌟 BENEFITS OF THIS EVOLUTION

### For NestGate

- ✅ **Focused Role**: Storage only (not connection)
- ✅ **Simpler Code**: Delegate platform logic to Songbird
- ✅ **Better Tests**: Storage easier to test than connections
- ✅ **Persistent Registry**: Metadata survives restarts

### For Songbird

- ✅ **Natural Extension**: Communication = networking + IPC
- ✅ **Centralized Platform Logic**: One place for all platforms
- ✅ **Enhanced Tower Atomic**: Universal IPC support

### For Application Primals

- ✅ **Zero Platform-Specific Code**: No #[cfg] needed!
- ✅ **Simple API**: Register, discover, connect
- ✅ **Universal**: Works everywhere Rust runs

### For Ecosystem

- ✅ **Clean Architecture**: Each primal owns its domain
- ✅ **True Universality**: All platforms supported
- ✅ **Easy Maintenance**: Change once, works everywhere

---

## 📚 REFERENCES

**Guidance Document**:
- `UNIVERSAL_IPC_ARCHITECTURE_HANDOFF_JAN_19_2026.md`

**Standards**:
- `ecoPrimals/wateringHole/UNIBIN_ARCHITECTURE_STANDARD.md`
- `ecoPrimals/wateringHole/ECOBIN_ARCHITECTURE_STANDARD.md`
- `ecoPrimals/wateringHole/GENOMEBIN_ARCHITECTURE_STANDARD.md`

**Related NestGate Docs**:
- `code/crates/nestgate-core/src/service_metadata/mod.rs`
- `docs/integration/biomeos/BIOMEOS_UNIX_SOCKET_INTEGRATION_COMPLETE.md`

---

## 🎊 SUMMARY

**Timeline**: 2-3 weeks (5-8 hours total)

**Current Status**: Phase 1 ✅ COMPLETE (Service metadata module created)

**Next Steps**:
1. Phase 2: Add deprecation markers (Week 1-2)
2. Phase 3: Songbird integration API (Week 2)
3. Phase 4: Persistent storage (Week 2-3)
4. Phase 5: Documentation (Week 3)
5. Phase 6: Release v2.3.0 (Week 3)

**Coordination**:
- Songbird team: Ready for handoff (Week 2)
- biomeOS team: Informed of evolution
- Application primals: Migration guide ready (Week 3)

**Result**:
- ✅ NestGate focuses on storage
- ✅ Songbird owns ALL communication
- ✅ Clean separation of concerns
- ✅ True universal IPC architecture

---

**Ready to complete the evolution to TRUE universality!** 🌍🦀✨

---

**Document**: UNIVERSAL_IPC_EVOLUTION_PLAN_JAN_19_2026.md  
**Date**: January 19, 2026  
**Status**: Phase 1 Complete, Ready for Phase 2  
**Team**: NestGate (with Songbird & biomeOS coordination)

🏰🐦 **Two primals, one universal architecture!** ✨
