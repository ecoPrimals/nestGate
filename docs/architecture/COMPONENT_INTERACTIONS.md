# 🏗️ Component Interaction Architecture

**Last Updated**: January 30, 2026  
**Version**: 3.3.0  
**Status**: Production Architecture

---

## 🎯 System Overview

NestGate is a **storage and discovery primal** in the ecoPrimals ecosystem, following the **Primal Sovereignty** architecture where each primal has self-knowledge and discovers others at runtime.

---

## 📊 High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                         NESTGATE PRIMAL                              │
│                  (Storage · Discovery · Metadata)                    │
├─────────────────────────────────────────────────────────────────────┤
│                                                                       │
│  ┌──────────────┐      ┌───────────────┐      ┌─────────────────┐  │
│  │   HTTP API   │      │  Unix Socket  │      │  Capability     │  │
│  │   :8080      │◄────►│  JSON-RPC     │◄────►│  Discovery      │  │
│  └──────┬───────┘      └───────┬───────┘      └────────┬────────┘  │
│         │                      │                       │            │
│         └──────────┬───────────┴───────────────────────┘            │
│                    │                                                 │
│         ┌──────────▼──────────────────────────┐                     │
│         │    STORAGE MANAGER SERVICE          │                     │
│         │  ┌─────────────┐  ┌──────────────┐  │                     │
│         │  │  Datasets   │  │   Objects    │  │                     │
│         │  │  (Create/   │  │   (Store/    │  │                     │
│         │  │   List/     │  │    Retrieve/ │  │                     │
│         │  │   Delete)   │  │    Delete)   │  │                     │
│         │  └──────┬──────┘  └──────┬───────┘  │                     │
│         │         └────────┬────────┘          │                     │
│         │                  │                   │                     │
│         │         ┌────────▼────────┐          │                     │
│         │         │  ZFS Backend    │          │                     │
│         │         │  (Filesystem)   │          │                     │
│         │         └────────┬────────┘          │                     │
│         └──────────────────┼──────────────────-┘                     │
│                            │                                          │
│                   ┌────────▼────────┐                                │
│                   │  XDG-Compliant  │                                │
│                   │  Storage Paths  │                                │
│                   │  (Phase 4)      │                                │
│                   └─────────────────┘                                │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 🔄 Component Interaction Flow

### **1. Client Request → API → Storage**

```
Client                  HTTP API              Storage Service          ZFS Backend
  │                        │                        │                      │
  ├─► POST /datasets ──────┤                        │                      │
  │                        ├─► create_dataset() ───►│                      │
  │                        │                        ├─► create_dir()  ─────┤
  │                        │                        │◄─── success ─────────┤
  │                        │◄─── DatasetInfo ───────┤                      │
  │◄─── 201 Created ───────┤                        │                      │
  │                        │                        │                      │
  ├─► PUT /objects ────────┤                        │                      │
  │                        ├─► store_object() ──────┤                      │
  │                        │                        ├─► write_file() ──────┤
  │                        │                        ├─► calc_checksum() ───┤
  │                        │                        │◄─── success ─────────┤
  │                        │◄─── ObjectInfo ────────┤                      │
  │◄─── 200 OK ────────────┤                        │                      │
```

### **2. Primal Discovery Flow**

```
NestGate (Self)       Discovery System      Other Primals      Service Registry
    │                      │                      │                   │
    ├─► self_knowledge ───►│                      │                   │
    │   (capabilities)     │                      │                   │
    │                      ├─► announce() ────────┼──────────────────►│
    │                      │                      │                   ├─► store metadata
    │                      │                      │                   │
    ├─► find_by_cap() ────►│                      │                   │
    │   ("security")       ├─► query() ───────────┼──────────────────►│
    │                      │◄─── services ────────┼───────────────────┤
    │                      │                      │                   │
    │                      ├─► connect(BearDog)  ►│                   │
    │◄─── connection ──────┤◄─────────────────────┤                   │
    │                      │                      │                   │
```

### **3. Unix Socket IPC Flow**

```
Client Process         Unix Socket           NestGate Server        Storage
      │                     │                       │                  │
      ├─► connect() ────────┤                       │                  │
      │   /run/.../sock     │                       │                  │
      │                     ├─► accept() ───────────┤                  │
      │◄─── connected ──────┤◄──────────────────────┤                  │
      │                     │                       │                  │
      ├─► JSON-RPC ─────────┤                       │                  │
      │   request           ├─► parse() ────────────┤                  │
      │                     │                       ├─► execute() ─────┤
      │                     │                       │◄─── result ──────┤
      │◄─── JSON-RPC ───────┤◄─── response ─────────┤                  │
      │   response          │                       │                  │
```

---

## 🧩 Core Components

### **1. HTTP API Layer** (`rpc/mod.rs`)

**Responsibility**: External HTTP/REST interface

**Features**:
- RESTful API for datasets/objects
- Health check endpoints
- Metrics endpoints
- JSON request/response

**Endpoints**:
- `GET /health` - Health check
- `POST /api/datasets` - Create dataset
- `GET /api/datasets` - List datasets
- `PUT /api/datasets/{name}/objects/{key}` - Store object
- `GET /api/datasets/{name}/objects/{key}` - Retrieve object
- `DELETE /api/datasets/{name}/objects/{key}` - Delete object

### **2. Unix Socket RPC Layer** (`rpc/unix_socket_server.rs`)

**Responsibility**: Inter-primal communication

**Features**:
- JSON-RPC over Unix sockets
- Low-latency IPC
- Capability-based discovery
- XDG-compliant socket paths

**Location**: `/run/user/{UID}/nestgate/nestgate.sock` (Linux)

### **3. Storage Manager** (`services/storage/service.rs`)

**Responsibility**: Dataset and object management

**Features**:
- Dataset creation/deletion
- Object CRUD operations
- SHA-256 checksums
- ZFS backend integration
- Quota management

**Operations**:
- Datasets: Create, list, delete
- Objects: Store, retrieve, delete
- Checksums: SHA-256 integrity verification

### **4. Discovery System** (`primal_discovery/runtime_discovery.rs`)

**Responsibility**: Capability-based service discovery

**Features**:
- Zero-knowledge startup
- Auto-detect discovery mechanism
- mDNS for bare metal
- Consul for cloud
- Capability queries (not names!)

**Flow**:
1. Announce own capabilities
2. Discover others by capability
3. Establish connections
4. Maintain service registry

### **5. Configuration System** (`config/environment.rs`)

**Responsibility**: Environment-driven configuration

**Features**:
- XDG Base Directory compliance
- 4-tier fallback (env > XDG > HOME > /var)
- 60+ environment variables
- Type-safe Port/Host validation

**Domains**:
- Network (ports, timeouts)
- Storage (paths, ZFS)
- Discovery (intervals, caching)
- Monitoring (metrics, logging)
- Security (TLS, auth)

---

## 🔌 Integration Points

### **With Songbird (Universal IPC)**:

```
NestGate                      Songbird
   │                              │
   ├─► register("nestgate") ──────►│
   │   capabilities: [storage]    ├─► announce to mesh
   │                              │
   │◄─── endpoint ────────────────┤
   │                              │
   ├─► listen(endpoint) ──────────►│
   │                              ├─► route requests
   │◄─── incoming requests ────────┤
```

### **With BearDog (Security)**:

```
NestGate                      BearDog
   │                              │
   ├─► find_by_cap("security") ───►│
   │◄─── connection ───────────────┤
   │                              │
   ├─► encrypt(data) ─────────────►│
   │                              ├─► encrypt with keys
   │◄─── encrypted_data ───────────┤
   │                              │
   ├─► verify_token(token) ───────►│
   │                              ├─► validate JWT
   │◄─── validation_result ────────┤
```

### **With Orchestrator**:

```
NestGate                   Orchestrator
   │                              │
   ├─► register() ────────────────►│
   │   metadata + capabilities    ├─► add to registry
   │                              │
   │◄─── heartbeat_request ────────┤
   ├─► health_status() ───────────►│
   │                              │
   │◄─── task_assignment ──────────┤
   ├─► execute_task() ────────────►│
   │◄─── task_complete ────────────┤
```

---

## 🎭 Runtime Behavior

### **Startup Sequence**:

```
1. Load Environment Config
   ├─► Parse NESTGATE_* variables
   ├─► Apply XDG fallbacks
   └─► Validate configuration

2. Initialize Storage
   ├─► Create XDG-compliant paths
   ├─► Check ZFS availability
   ├─► Discover existing pools
   └─► Initialize quota system

3. Start Discovery
   ├─► Auto-detect mechanism (mDNS/Consul/K8s)
   ├─► Announce own capabilities
   ├─► Build self-knowledge
   └─► Start background discovery loop

4. Launch Servers
   ├─► Bind Unix socket (XDG runtime dir)
   ├─► Start HTTP server (configured port)
   ├─► Enable health checks
   └─► Start monitoring

5. Runtime Operation
   ├─► Process incoming requests
   ├─► Maintain discovery cache
   ├─► Send heartbeats
   └─► Update metrics
```

### **Request Processing**:

```
Incoming Request
    │
    ├─► Parse & Validate
    │   ├─► Check auth (if enabled)
    │   ├─► Validate parameters
    │   └─► Rate limit check
    │
    ├─► Route to Handler
    │   ├─► Dataset operations
    │   ├─► Object operations
    │   └─── Service queries
    │
    ├─► Execute Operation
    │   ├─► Call storage service
    │   ├─► Interact with ZFS
    │   ├─► Calculate checksums
    │   └─► Update statistics
    │
    └─► Return Response
        ├─► Format JSON
        ├─► Add headers
        ├─► Log operation
        └─► Send to client
```

---

## 🧬 Primal Sovereignty Pattern

### **Self-Knowledge First**:

```rust
// NestGate knows ONLY about itself
let self_knowledge = SelfKnowledge::builder()
    .with_name("nestgate")
    .with_capabilities(vec![
        Capability::Storage,
        Capability::ZfsManagement,
        Capability::ServiceRegistry,
    ])
    .build();
```

### **Discover Others at Runtime**:

```rust
// Find primals by what they CAN DO, not what they ARE
let security_primal = discovery
    .find_by_capability(Capability::Security)
    .await?;

let orchestrator = discovery
    .find_by_capability(Capability::Orchestration)
    .await?;
```

### **Zero Hardcoding**:

```rust
// ❌ NEVER:
let beardog_url = "http://beardog:8443";

// ✅ ALWAYS:
let security_primal = runtime_discovery
    .find_security_primal()
    .await?;
let connection = security_primal.endpoint;
```

---

## 📡 Communication Patterns

### **Pattern 1: Synchronous Request/Response**

```
Client ──request──► NestGate ──query──► Storage ──result──► NestGate ──response──► Client
```

**Use Cases**: CRUD operations, health checks, queries

### **Pattern 2: Asynchronous Background Tasks**

```
NestGate ──periodic──► Discovery
    │                      │
    ├──background──► Quota Monitoring
    │                      │
    └──background──► Pool Discovery
```

**Use Cases**: Service discovery, monitoring, maintenance

### **Pattern 3: Event-Driven**

```
Storage Event ──►┌─────────────┐
                 │ Event Queue │
ZFS Event ───────►│             │──► Listeners ──► Actions
                 │             │
Discovery Event ─►└─────────────┘
```

**Use Cases**: Pool changes, service joins/leaves, failures

---

## 🔐 Security Integration

### **Authentication Flow**:

```
1. Client Request
   │
   ├─► Extract Token/API Key
   │
2. Validate with BearDog
   │
   ├─► discovery.find_security_primal()
   ├─► security.verify_token(token)
   │
3. Check Authorization
   │
   ├─► Verify capability permissions
   ├─► Check resource access
   │
4. Execute if Authorized
   │
   └─► Process request
```

### **Encryption Flow**:

```
Data ──►┌──────────┐──► BearDog ──►┌───────────┐──► Encrypted ──► Storage
        │ NestGate │               │  Encrypt  │      Data
        └──────────┘               └───────────┘
```

---

## 📊 Data Flow Examples

### **Example 1: Store Object with Encryption**

```
1. Client: PUT /api/datasets/photos/vacation.jpg
   │
2. NestGate receives request
   ├─► Validate request
   ├─► Check authentication
   │
3. Find security primal
   ├─► discovery.find_by_capability("security")
   ├─► connect to BearDog
   │
4. Encrypt data
   ├─► beardog.encrypt(image_data)
   ├─► receive encrypted_data
   │
5. Store encrypted data
   ├─► storage_service.store_object("photos", "vacation.jpg", encrypted_data)
   ├─► calculate SHA-256 checksum
   ├─► write to ZFS filesystem
   │
6. Return success
   └─► 200 OK + ObjectInfo (with checksum)
```

### **Example 2: Discover and Connect to Orchestrator**

```
1. NestGate startup
   │
2. Initialize discovery
   ├─► Auto-detect mechanism (mDNS/Consul/K8s)
   ├─► Create RuntimeDiscovery
   │
3. Announce self
   ├─► discovery.announce(self_knowledge)
   ├─► Broadcast capabilities: [Storage, ZFS, Registry]
   │
4. Find orchestrator
   ├─► discovery.find_by_capability(Capability::Orchestration)
   ├─► Query service registry
   ├─► Receive orchestrator endpoints
   │
5. Establish connection
   ├─► Connect via Unix socket (preferred)
   ├─► Or connect via HTTP (fallback)
   │
6. Register with orchestrator
   ├─► orchestrator.register_service(metadata)
   ├─► Start sending heartbeats
   │
7. Receive tasks
   └─► Listen for orchestration commands
```

---

## 🏭 Production Deployment Patterns

### **Pattern 1: Single Instance** (Development/Small Production)

```
┌─────────────────┐
│   Application   │
│   Container     │
├─────────────────┤
│   NestGate      │
│   :8080         │
│   + ZFS         │
└─────────────────┘
```

**Use Case**: Development, small deployments, embedded systems

### **Pattern 2: Multi-Instance with Load Balancer** (High Availability)

```
                  ┌──────────────┐
Client ──────────►│Load Balancer │
                  └──────┬───────┘
                         │
        ┌────────────────┼────────────────┐
        │                │                │
  ┌─────▼─────┐   ┌─────▼─────┐   ┌─────▼─────┐
  │ NestGate  │   │ NestGate  │   │ NestGate  │
  │ Instance1 │   │ Instance2 │   │ Instance3 │
  │ (Leader)  │   │(Follower) │   │(Follower) │
  └─────┬─────┘   └─────┬─────┘   └─────┬─────┘
        │                │                │
        └────────────────┼────────────────┘
                         │
                  ┌──────▼───────┐
                  │  Shared ZFS  │
                  │     Pool     │
                  └──────────────┘
```

**Use Case**: High availability, load distribution, enterprise

### **Pattern 3: Microservices Mesh** (ecoPrimals Ecosystem)

```
┌─────────────────────────────────────────────────────────────┐
│                    Service Mesh                              │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌──────────┐    ┌──────────┐    ┌──────────┐              │
│  │Orchestr. │◄──►│ NestGate │◄──►│ BearDog  │              │
│  │(Control) │    │(Storage) │    │(Security)│              │
│  └──────────┘    └────┬─────┘    └──────────┘              │
│                       │                                       │
│                       ▼                                       │
│  ┌──────────┐    ┌──────────┐    ┌──────────┐              │
│  │ Songbird │◄──►│   ZFS    │◄──►│  mDNS    │              │
│  │  (IPC)   │    │ Backend  │    │Discovery │              │
│  └──────────┘    └──────────┘    └──────────┘              │
│                                                               │
└───────────────────────────────────────────────────────────-──┘
```

**Use Case**: Full ecoPrimals ecosystem, primal coordination

---

## 🔄 State Management

### **Component States**:

```
┌─────────────────────────────────────────────────┐
│ NestGate Service States                         │
├─────────────────────────────────────────────────┤
│                                                  │
│  Initializing                                   │
│      │                                           │
│      ├─► Load config                            │
│      ├─► Initialize storage                     │
│      ├─► Start discovery                        │
│      │                                           │
│      ▼                                           │
│  Running ◄──┐                                    │
│      │      │                                    │
│      ├─► Process requests                       │
│      ├─► Maintain discovery                     │
│      ├─► Send heartbeats                        │
│      │      │                                    │
│      ▼      │                                    │
│  Degraded ─┘ (recoverable)                      │
│      │                                           │
│      ├─► Reduce capacity                        │
│      ├─► Alert monitoring                       │
│      │                                           │
│      ▼                                           │
│  Shutting Down                                  │
│      │                                           │
│      ├─► Drain connections                      │
│      ├─► Flush buffers                          │
│      ├─► Deregister from discovery              │
│      │                                           │
│      ▼                                           │
│  Stopped                                        │
│                                                  │
└─────────────────────────────────────────────────┘
```

---

## 📈 Performance Characteristics

### **Throughput**:
- **HTTP API**: ~10,000 req/sec (single instance)
- **Unix Socket**: ~50,000 req/sec (IPC)
- **Storage**: Limited by ZFS backend performance

### **Latency** (p99):
- **Health Check**: <1ms
- **Object Store**: <10ms (small objects)
- **Discovery Query**: <5ms (cached)

### **Resource Usage**:
- **Memory**: ~50MB baseline
- **CPU**: <5% idle, scales with load
- **Disk**: Dependent on stored data + ZFS ARC cache

---

## 🎯 Design Principles

### **1. Primal Sovereignty** ✅
Each primal is autonomous with self-knowledge

### **2. Capability-Based Discovery** ✅
Find services by what they CAN DO, not what they ARE

### **3. Zero Hardcoding** ✅
All configuration from environment, discovery at runtime

### **4. XDG Compliance** ✅
Follow standards for paths, sockets, configuration

### **5. Fail-Safe Defaults** ✅
Sensible defaults, graceful degradation

---

**For detailed component documentation, see**:
- Storage: `docs/guides/STORAGE_GUIDE.md`
- Discovery: `docs/INFANT_DISCOVERY_ARCHITECTURE.md`
- API: `docs/api/REST_API.md`
- Configuration: `docs/guides/ENVIRONMENT_VARIABLES.md`

🦀 **NestGate · Storage · Discovery · Pure Rust** 🦀
