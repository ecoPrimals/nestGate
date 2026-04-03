# NestGate Capability Mappings

**Purpose**: Document NestGate's provided and required capabilities for primal compliance  
**Standard**: wateringHole/SEMANTIC_METHOD_NAMING_STANDARD.md v2.0  
**Last Updated**: April 3, 2026

---

## 📊 OVERVIEW

NestGate operates as a **storage & discovery primal** within the ecoPrimals ecosystem. This document maps:

1. **Capabilities Provided** - What NestGate offers to other primals
2. **Capabilities Required** - What NestGate needs from other primals  
3. **Semantic Method Mappings** - Internal methods → semantic names
4. **Neural API Translation** - How biomeOS routes requests

---

## 🎁 CAPABILITIES PROVIDED BY NESTGATE

### **1. Storage Capability**

**Domain**: `storage.*`

#### **Methods Provided**:

```json
{
  "storage.put": "Store object/blob/dataset",
  "storage.get": "Retrieve object/blob/dataset",
  "storage.delete": "Remove object/blob/dataset",
  "storage.list": "List objects/blobs/datasets",
  "storage.exists": "Check if object exists",
  "storage.metadata": "Get object metadata",
  "storage.copy": "Copy object",
  "storage.move": "Move object",
  "storage.quota": "Get/set storage quotas"
}
```

#### **Internal Implementation**:

```rust
// Current internal methods:
pub async fn store_object(...)  // → storage.put
pub async fn retrieve_object(...)  // → storage.get
pub async fn delete_object(...)  // → storage.delete
pub async fn list_objects(...)  // → storage.list
```

**Evolution Plan**: Rename internal methods to match semantic format (8-12 hours)

---

### **2. Discovery Capability**

**Domain**: `discovery.*`

#### **Methods Provided**:

```json
{
  "discovery.announce": "Register service with NestGate registry",
  "discovery.query": "Find services by capability",
  "discovery.list": "List all registered services",
  "discovery.metadata": "Get service metadata",
  "discovery.capabilities": "Get service capabilities"
}
```

#### **Internal Implementation**:

```rust
// Current (CapabilityDiscovery module):
pub async fn find(capability: &str)  // → discovery.query
pub async fn list_services()  // → discovery.list
pub async fn get_service_metadata(name: &str)  // → discovery.metadata
```

**Status**: ✅ Well-architected, needs semantic method naming

---

### **3. Metadata Capability**

**Domain**: `metadata.*`

#### **Methods Provided**:

```json
{
  "metadata.store": "Store service metadata",
  "metadata.retrieve": "Get service metadata",
  "metadata.update": "Update service metadata",
  "metadata.search": "Search metadata by attributes"
}
```

#### **Internal Implementation**:

```rust
// Current (ServiceMetadataStore):
pub async fn store_service(meta: ServiceMetadata)  // → metadata.store
pub async fn get_service(name: &str)  // → metadata.retrieve
pub async fn find_by_capability(cap: &str)  // → metadata.search
```

**Status**: ✅ Implemented, needs semantic naming

---

### **4. Health Capability**

**Domain**: `health.*`

#### **Methods Provided**:

```json
{
  "health.check": "Get service health status",
  "health.metrics": "Get performance metrics",
  "health.readiness": "Readiness check",
  "health.liveness": "Liveness check"
}
```

#### **Internal Implementation**:

```rust
// Current (RPC health endpoints):
pub async fn health_check()  // → health.check
pub async fn readiness_check()  // → health.readiness
pub async fn liveness_check()  // → health.liveness
```

**Status**: ✅ Implemented

---

## 🔍 CAPABILITIES REQUIRED BY NESTGATE

### **1. IPC/Orchestration (from Songbird)**

**Capability**: `ipc` or `orchestration`  
**Provider**: Songbird primal

#### **Methods Used**:

```json
{
  "ipc.register": "Register NestGate service with orchestrator",
  "ipc.resolve": "Resolve other primal endpoints",
  "ipc.find_capability": "Find services by capability",
  "ipc.list": "List all services",
  "ipc.heartbeat": "Send health heartbeat"
}
```

#### **Current Usage**:

```rust
// Via CapabilityDiscovery module:
let songbird = CapabilityDiscovery::discover_songbird_ipc().await?;
let discovery = CapabilityDiscovery::new(songbird);
let crypto_service = discovery.find("crypto").await?;
```

**Status**: ✅ Using capability discovery pattern

---

### **2. Security/Crypto (from BearDog)**

**Capability**: `crypto` or `security`  
**Provider**: BearDog primal  
**Usage**: Optional (for encrypted storage)

#### **Methods Used**:

```json
{
  "crypto.generate_keypair": "Generate encryption keys",
  "crypto.encrypt": "Encrypt data",
  "crypto.decrypt": "Decrypt data",
  "crypto.hash": "Hash data",
  "crypto.sign": "Sign data",
  "crypto.verify": "Verify signature"
}
```

#### **Current Usage**:

```rust
// Future integration (when encryption enabled):
let crypto = discovery.find("crypto").await?;
let response = crypto.call_rpc("crypto.encrypt", params).await?;
```

**Status**: Evolved — `crypto/delegate.rs` provides capability-based delegation via JSON-RPC. Discovers crypto provider at runtime (env var -> capability discovery -> socket scan).

---

### **3. TLS/Networking (from Songbird)**

**Capability**: `tls` or `networking`  
**Provider**: Songbird primal  
**Usage**: For external HTTP/HTTPS (concentrated gap)

#### **Methods Used**:

```json
{
  "tls.derive_secrets": "TLS key derivation",
  "http.get": "HTTP GET request",
  "http.post": "HTTP POST request"
}
```

#### **Current Usage**:

```rust
// All external HTTP goes through Songbird:
let songbird = discovery.find("networking").await?;
let response = songbird.call_rpc("http.get", params).await?;
```

**Status**: ✅ Architecture established, not yet implemented

---

### **4. Compute (from ToadStool)**

**Capability**: `compute`  
**Provider**: ToadStool primal  
**Usage**: Optional (for storage optimization)

#### **Methods Used**:

```json
{
  "compute.optimize": "Optimize storage layout",
  "compute.analyze": "Analyze storage patterns",
  "compute.compress": "Compress data"
}
```

**Status**: 📋 Planned (not yet integrated)

---

## 🔄 SEMANTIC METHOD NAMING EVOLUTION

### **Current State**

**External (JSON-RPC)**:
- ✅ Uses semantic names (`crypto.generate_keypair`)
- ✅ Follows wateringHole standard
- ✅ Compatible with Neural API

**Internal (Rust methods)**:
- ⚠️ Uses descriptive names (`store_object`, `retrieve_object`)
- ⚠️ Not fully semantic (`storage.put` format)
- ⚠️ Needs refactoring for consistency

---

### **Evolution Plan**

#### **Phase 1: Internal Method Refactoring** (8-12 hours)

**Pattern**:
```rust
// ❌ OLD: Domain-specific method names
impl StorageService {
    pub async fn store_object(...) -> Result<()>
    pub async fn retrieve_object(...) -> Result<()>
    pub async fn delete_object(...) -> Result<()>
}

// ✅ NEW: Semantic method routing
impl StorageService {
    pub async fn call_method(method: &str, params: Value) -> Result<Value> {
        match method {
            "storage.put" => self.storage_put(params).await,
            "storage.get" => self.storage_get(params).await,
            "storage.delete" => self.storage_delete(params).await,
            _ => Err(NestGateError::method_not_found(method)),
        }
    }
    
    async fn storage_put(&self, params: Value) -> Result<Value> {
        // Implementation
    }
}
```

#### **Phase 2: Documentation** (2-3 hours)

- Create capability registry
- Document all provided methods
- Document required methods
- Update integration guides

#### **Phase 3: Neural API Integration** (2-3 hours)

- Add capability mappings to biomeOS graphs
- Configure translation layer
- Test cross-primal method calls

---

## 🌐 NEURAL API TRANSLATION

### **How biomeOS Routes NestGate Requests**

**Graph Configuration** (in biomeOS):

```toml
# graphs/nestgate_capabilities.toml

[nodes.nestgate]
primal_name = "nestgate"
binary_path = "plasmidBin/primals/nestgate"
socket_path = "/primal/nestgate"

[nodes.nestgate.capabilities_provided]
# Semantic Name → Actual Method (what NestGate implements)
"storage.put" = "store_object"
"storage.get" = "retrieve_object"
"storage.delete" = "delete_object"
"storage.list" = "list_objects"
"discovery.query" = "find_by_capability"
"discovery.list" = "list_services"
"metadata.store" = "store_service"
"metadata.retrieve" = "get_service"

[nodes.nestgate.capabilities_required]
# What NestGate needs from other primals
required = ["ipc", "orchestration"]
optional = ["crypto", "compute"]
```

### **Request Flow**

```
1. External primal calls Neural API:
   {"method": "storage.put", "params": {...}}

2. Neural API looks up translation:
   "storage.put" → "store_object" (NestGate implementation)

3. Neural API routes to NestGate:
   {"method": "store_object", "params": {...}}

4. NestGate executes and returns result

5. Neural API returns to caller
```

---

## 📊 CAPABILITY MATRIX

### **NestGate Capability Summary**

| Domain | Methods | Status | Priority |
|--------|---------|--------|----------|
| **storage** | 9 methods | ✅ Implemented | P0 (core) |
| **discovery** | 5 methods | ✅ Implemented | P0 (core) |
| **metadata** | 4 methods | ✅ Implemented | P1 (important) |
| **health** | 4 methods | ✅ Implemented | P1 (important) |

### **Required From Other Primals**

| Capability | Provider | Methods Used | Status |
|------------|----------|--------------|--------|
| **ipc** | Songbird | 5 methods | ✅ Integrated |
| **crypto** | BearDog | 6 methods | Evolved (delegation) |
| **networking** | Songbird | 3 methods | 📋 Planned |
| **compute** | ToadStool | 3 methods | 📋 Future |

---

## 🎯 INTEGRATION EXAMPLES

### **Example 1: Another Primal Uses NestGate Storage**

```rust
// In BearDog (or any primal):
use nestgate_core::capability_discovery::CapabilityDiscovery;

// Discover storage capability
let discovery = CapabilityDiscovery::discover_songbird_ipc().await?;
let storage = discovery.find("storage").await?;

// Use semantic method name
let response = storage.call_rpc("storage.put", json!({
    "key": "my-key",
    "data": "my-encrypted-data",
    "metadata": {"encrypted": true}
})).await?;
```

### **Example 2: NestGate Uses BearDog Crypto**

```rust
// In NestGate:
let discovery = CapabilityDiscovery::discover_songbird_ipc().await?;
let crypto = discovery.find("crypto").await?;

// Encrypt data before storing
let response = crypto.call_rpc("crypto.encrypt", json!({
    "data": data,
    "algorithm": "aes-256-gcm"
})).await?;

// Store encrypted data
self.call_method("storage.put", json!({
    "key": key,
    "data": response["encrypted_data"]
})).await?;
```

---

## 📚 REFERENCES

- wateringHole/SEMANTIC_METHOD_NAMING_STANDARD.md v2.0
- wateringHole/PRIMAL_IPC_PROTOCOL.md v1.0
- UNIVERSAL_IPC_EVOLUTION_PLAN_JAN_19_2026.md
- biomeOS/NEURAL_API_ROUTING_SPECIFICATION.md (when available)

---

**Status**: Partial semantic naming implemented. Internal method refactoring ongoing.

**Next Steps**:
1. Complete internal method semantic routing
2. Create biomeOS graph configuration
3. Test cross-primal integration

---

**Last Updated**: April 3, 2026
