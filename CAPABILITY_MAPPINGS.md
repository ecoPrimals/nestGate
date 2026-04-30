# NestGate Capability Mappings

**Purpose**: Document NestGate's provided and required capabilities for primal compliance  
**Standard**: wateringHole/SEMANTIC_METHOD_NAMING_STANDARD.md v2.0  
**Last Updated**: April 30, 2026 (Session 50)

---

## Overview

NestGate operates as a **storage & discovery primal** within the ecoPrimals ecosystem. This document maps:

1. **Capabilities Provided** - What NestGate offers to other primals
2. **Capabilities Required** - What NestGate needs from other primals  
3. **Semantic Method Mappings** - Internal methods → semantic names
4. **Neural API Translation** - How biomeOS routes requests

---

## Wire Standard Compliance

**Level**: 3 (Composable) — full compliance as of April 8, 2026

`capabilities.list` returns the standard envelope:

```json
{
  "primal": "nestgate",
  "version": "<semver>",
  "methods": ["<51 UDS methods — `UNIX_SOCKET_SUPPORTED_METHODS`>"],
  "provided_capabilities": [
    {"type": "storage", "methods": ["store", "retrieve", "exists", "delete", "list", "stats", "store_blob", "retrieve_blob", "retrieve_range", "object.size", "namespaces.list", "fetch_external", "store_stream", "store_stream_chunk", "retrieve_stream", "retrieve_stream_chunk"]},
    {"type": "model", "methods": ["register", "exists", "locate", "metadata"]},
    {"type": "templates", "methods": ["store", "retrieve", "list", "community_top"]},
    {"type": "session", "methods": ["save", "load"]},
    {"type": "audit", "methods": ["store_execution"]},
    {"type": "nat", "methods": ["store_traversal_info", "retrieve_traversal_info"]},
    {"type": "beacon", "methods": ["store", "retrieve", "list", "delete"]},
    {"type": "bonding", "methods": ["ledger.store", "ledger.retrieve", "ledger.list"]},
    {"type": "zfs", "methods": ["pool.list", "pool.get", "pool.health", "dataset.list", "dataset.get", "snapshot.list", "health"]}
  ],
  "consumed_capabilities": [
    {"type": "security", "methods": ["verify_token", "encrypt", "decrypt"]},
    {"type": "crypto", "methods": ["sign", "verify"]},
    {"type": "network", "methods": ["resolve", "connect"]}
  ],
  "protocol": "jsonrpc-2.0",
  "transport": ["uds", "http", "tcp"]
}
```

`capability_registry.toml` is the full **12**-group / **45**-method inventory (excluding `data.*`, which is wildcard-only). The `capabilities.list` `provided_capabilities` block in `model_cache_handlers.rs` groups **9** durability domains (storage through zfs); health, identity, and discovery appear in the top-level `methods` array and related handlers instead. `data.*` is not a NestGate-implemented feed surface.

`identity.get` returns `{primal, version, domain: "storage", license: "AGPL-3.0-or-later", family_id}`.

See `wateringHole/CAPABILITY_WIRE_STANDARD.md` for the ecosystem-wide standard.

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
  "storage.quota": "Get/set storage quotas",
  "storage.fetch_external": "Fetch content from external HTTPS URLs (TLS terminated in NestGate)"
}
```

#### **Internal Implementation**:

```rust
// Current internal methods:
pub async fn store_object(...)  // → storage.put
pub async fn retrieve_object(...)  // → storage.get
pub async fn delete_object(...)  // → storage.delete
pub async fn list_objects(...)  // → storage.list
// fetch_external handler  // → storage.fetch_external (HTTPS/TLS in NestGate)
```

**Evolution Plan**: Rename internal methods to match semantic format (8-12 hours)

---

### **1b. ZFS Capability (GAP-MATRIX-04 Resolution)**

**Domain**: `zfs.*`

ZFS pool, dataset, and snapshot management exposed over JSON-RPC/UDS. Resolves
GAP-MATRIX-04 by making the ZFS surface reachable through the same
`socat`/`PrimalClient` tooling every other primal uses.

#### **Methods Provided**:

```json
{
  "zfs.pool.list": "List all ZFS pools with size/allocation/health",
  "zfs.pool.get": "Get status for a single pool (params: {pool})",
  "zfs.pool.health": "Health summary across all pools, flags unhealthy",
  "zfs.dataset.list": "List ZFS datasets, optionally scoped to a pool",
  "zfs.dataset.get": "Get a single dataset by name (params: {dataset})",
  "zfs.snapshot.list": "List snapshots, optionally scoped to a dataset",
  "zfs.health": "ZFS/zpool userland availability and version"
}
```

**Transport**: JSON-RPC over UDS (ecosystem standard) **and** HTTP `/jsonrpc`

**Status**: Implemented — subprocess-backed handlers in `nestgate-rpc` (UDS)
and `nestgate-api` (HTTP JSON-RPC). Gracefully returns structured errors when
ZFS userland is unavailable.

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

**Status**: Implemented, needs semantic method naming

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

**Status**: Implemented, needs semantic naming

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

**Status**: Implemented

---

## Capabilities Required by NestGate

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

**Status**: Using capability discovery pattern

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

### **3. External HTTPS fetch (NestGate-owned TLS boundary)**

**Capability**: `storage.fetch_external` (storage domain)  
**Provider**: NestGate (TLS and HTTP client for ecosystem external fetch)  
**Usage**: Fetching remote content over HTTPS for storage workflows — NestGate owns the TLS boundary; this is **not** delegated to Songbird.

#### **Methods Used**:

```json
{
  "storage.fetch_external": "HTTPS GET to external URLs with content-addressed result"
}
```

#### **Current Usage**:

```rust
// External HTTPS fetch is implemented in NestGate (e.g. storage handler → fetch_external):
// reqwest + rustls; content addressing via blake3; no Songbird http.get hop for this path.
```

**Status**: Implemented — NestGate terminates TLS for this fetch path

#### **Optional / future (Songbird or other primals)**

Legacy documentation referred to Songbird for generic `http.get` / `http.post` and `tls.derive_secrets`. Those remain **optional** for orchestration or other capabilities; they are **not** the path for `storage.fetch_external`.

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

**Status**: Planned (not yet integrated)

---

## 🔄 SEMANTIC METHOD NAMING EVOLUTION

### **Current State**

**External (JSON-RPC)**:
- Uses semantic names (`crypto.generate_keypair`)
- Follows wateringHole standard
- Compatible with Neural API

**Internal (Rust methods)**:
- ⚠️ Uses descriptive names (`store_object`, `retrieve_object`)
- ⚠️ Not fully semantic (`storage.put` format)
- ⚠️ Needs refactoring for consistency

---

### **Evolution Plan**

#### **Phase 1: Internal Method Refactoring** (8-12 hours)

**Pattern**:
```rust
// OLD: Domain-specific method names
impl StorageService {
    pub async fn store_object(...) -> Result<()>
    pub async fn retrieve_object(...) -> Result<()>
    pub async fn delete_object(...) -> Result<()>
}

// NEW: Semantic method routing
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

## Neural API Translation

### **How biomeOS Routes NestGate Requests**

Authoritative capability and method inventory: [`capability_registry.toml`](capability_registry.toml) at the NestGate repository root (not under `graphs/`). The `graphs/` directory in this repo holds other TOML assets (for example `nestgate_standalone.toml`); it does not define a separate `nestgate_capabilities.toml`.

**Illustrative biomeOS graph fragment** (conceptual routing example):

```toml
# Canonical semantic names: see capability_registry.toml (NestGate repo root).

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

## Capability Matrix

### **NestGate Capability Summary**

| Domain | Methods | Status | Priority |
|--------|---------|--------|----------|
| **storage** | 10 methods | Implemented | P0 (core) |
| **zfs** | 7 methods | Implemented (GAP-MATRIX-04) | P0 (core) |
| **discovery** | 5 methods | Implemented | P0 (core) |
| **metadata** | 4 methods | Implemented | P1 (important) |
| **health** | 4 methods | Implemented | P1 (important) |

### **Required From Other Primals**

| Capability | Provider | Methods Used | Status |
|------------|----------|--------------|--------|
| **ipc** | IPC capability provider | 5 methods | Integrated |
| **crypto** | Security capability provider | 6 methods | Evolved (delegation) |
| **networking** | Network capability provider (optional) | 3 methods | Planned (not used for `storage.fetch_external`) |
| **compute** | Compute capability provider | 3 methods | Future |

---

## Integration Examples

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

**Status**: Semantic naming complete. Streaming storage methods wired. TCP transport active.

**Next Steps**:
1. Push coverage 84.12% → 90%
2. Test cross-primal integration with streaming tensors
3. Track vendored `rustls-rustcrypto` + `rustls-webpki` upstream for drop opportunity

---

**Last Updated**: April 30, 2026 (Session 50)
