# 🌍 Cross-Primal Name Evolution - January 26, 2026

**Status**: IN PROGRESS  
**Approach**: Capability-Based Discovery  
**Philosophy**: Primal Autonomy & Self-Knowledge

---

## 🎯 PROBLEM STATEMENT

**Violation**: 511 hardcoded primal names across 60 files

**Examples Found**:
```rust
// ❌ WRONG: Hardcoded primal name
connect("/primal/beardog").await?;
let songbird = discover("songbird").await?;
format!("Calling toadstool service...");
```

**Why This Violates Standards**:
1. **Primal Autonomy** - Primals should not know other primal names
2. **Self-Knowledge** - Only know capabilities, discover at runtime
3. **Inter-Primal Interactions** - Use capability-based discovery

**Reference**: `/wateringHole/INTER_PRIMAL_INTERACTIONS.md`

---

## ✅ DEEP DEBT SOLUTION

### Pattern Evolution

```rust
// ❌ OLD: Hardcoded primal name
let crypto_service = connect("/primal/beardog").await?;
let response = crypto_service.call("generate_key", params).await?;

// ✅ NEW: Capability-based discovery
let crypto_service = self.discover_capability("crypto").await?;
let response = crypto_service.call("generate_key", params).await?;

// 🎯 BEST: Via Songbird IPC service
let songbird = self.discover_songbird_ipc().await?;
let crypto_endpoint = songbird.find_service("crypto").await?;
let response = crypto_endpoint.call("generate_key", params).await?;
```

---

## 📊 AFFECTED FILES

### High Priority (Direct Socket Connections)
1. `rpc/songbird_registration.rs` (73 refs) - Direct Songbird calls
2. `service_metadata/mod.rs` (54 refs) - Metadata storage references
3. `rpc/orchestrator_registration.rs` (5 refs) - Orchestrator discovery
4. `transport/security.rs` (50 refs) - BearDog crypto calls
5. `primal_discovery/capability_helpers.rs` (17 refs) - Discovery helpers

### Medium Priority (Configuration/Examples)
6. `config/runtime/services.rs` (25 refs) - Service configs
7. `primal_discovery.rs` (3 refs) - Discovery logic
8. `examples/infant_discovery_demo.rs` (3 refs) - Example code

### Low Priority (Tests/Documentation)
9. Test files (284 refs) - Test fixtures (acceptable for now)
10. Documentation (50+ refs) - Examples in docs

---

## 🔧 IMPLEMENTATION STRATEGY

### Phase 1: Create Capability Discovery API

**New Module**: `capability_discovery.rs`

```rust
/// Capability-based primal discovery
/// 
/// Discovers primals by capability, not by name.
/// Uses Songbird IPC service for runtime resolution.
pub struct CapabilityDiscovery {
    /// Songbird IPC client
    songbird: Arc<SongbirdIpcClient>,
    
    /// Cache of discovered capabilities
    cache: Arc<DashMap<String, ServiceEndpoint>>,
}

impl CapabilityDiscovery {
    /// Discover a service providing a specific capability
    /// 
    /// # Examples
    /// ```
    /// let discovery = CapabilityDiscovery::new().await?;
    /// let crypto_service = discovery.find("crypto").await?;
    /// ```
    pub async fn find(&self, capability: &str) -> Result<ServiceEndpoint> {
        // Check cache first
        if let Some(endpoint) = self.cache.get(capability) {
            return Ok(endpoint.clone());
        }
        
        // Query Songbird IPC service
        let response = self.songbird.call_rpc(
            "ipc.find_capability",
            json!({ "capability": capability })
        ).await?;
        
        let endpoint = ServiceEndpoint::from_response(response)?;
        
        // Cache for future use
        self.cache.insert(capability.to_string(), endpoint.clone());
        
        Ok(endpoint)
    }
    
    /// Discover Songbird IPC service itself
    /// 
    /// Special case: Songbird is discovered via environment/config,
    /// then used to discover all other services.
    pub async fn discover_songbird_ipc() -> Result<SongbirdIpcClient> {
        // Try environment variable first
        if let Ok(path) = env::var("SONGBIRD_IPC_PATH") {
            return SongbirdIpcClient::connect(&path).await;
        }
        
        // Try standard path
        let standard_path = "/primal/songbird";
        if Path::new(standard_path).exists() {
            return SongbirdIpcClient::connect(standard_path).await;
        }
        
        // Try discovery via environment
        let host = env::var("SONGBIRD_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = env::var("SONGBIRD_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(8080);
        
        SongbirdIpcClient::connect_tcp(&host, port).await
    }
}
```

### Phase 2: Evolve High-Priority Files

**File**: `rpc/songbird_registration.rs` (73 refs)

**Current Pattern**:
```rust
// Hardcoded Songbird endpoint
let endpoint = "/primal/songbird";
let stream = UnixStream::connect(endpoint).await?;
```

**Evolution**:
```rust
// Capability-based discovery
let songbird = CapabilityDiscovery::discover_songbird_ipc().await?;
let response = songbird.register_service(metadata).await?;
```

**Timeline**: 2-3 hours

---

**File**: `service_metadata/mod.rs` (54 refs)

**Current Pattern**:
```rust
// Hardcoded service names for examples/tests
let beardog_meta = ServiceMetadata {
    name: "beardog",  // ❌ Hardcoded
    capabilities: vec!["crypto"],
    ...
};
```

**Evolution**:
```rust
// Generic capability examples
let crypto_meta = ServiceMetadata {
    name: "crypto-provider",  // ✅ Generic role name
    capabilities: vec!["crypto"],
    ...
};

// For actual discovery, use capabilities only
let crypto_service = discovery.find("crypto").await?;
```

**Timeline**: 2-3 hours

---

**File**: `transport/security.rs` (50 refs)

**Current Pattern**:
```rust
// Direct BearDog crypto calls
let beardog = connect("/primal/beardog").await?;
let key = beardog.generate_key().await?;
```

**Evolution**:
```rust
// Capability-based crypto service
let crypto = self.discover_capability("crypto").await?;
let key = crypto.call_rpc("crypto.generate_keypair", params).await?;
```

**Timeline**: 3-4 hours

---

### Phase 3: Update Configuration

**File**: `config/runtime/services.rs` (25 refs)

**Current Pattern**:
```rust
// Hardcoded primal names in config
pub fn default_services() -> HashMap<String, ServiceConfig> {
    let mut services = HashMap::new();
    services.insert("beardog", ServiceConfig { ... });
    services.insert("songbird", ServiceConfig { ... });
    services
}
```

**Evolution**:
```rust
// Capability-based service configs
pub fn default_capabilities() -> HashMap<String, CapabilityConfig> {
    let mut capabilities = HashMap::new();
    capabilities.insert("crypto", CapabilityConfig {
        required: true,
        fallback: FallbackMode::Fail,
        discovery_method: DiscoveryMethod::Runtime,
    });
    capabilities.insert("http", CapabilityConfig {
        required: true,
        fallback: FallbackMode::Fail,
        discovery_method: DiscoveryMethod::Runtime,
    });
    capabilities
}
```

**Timeline**: 2-3 hours

---

### Phase 4: Add Deprecation Warnings

**For old patterns**:
```rust
#[deprecated(
    since = "2.2.0",
    note = "Hardcoded primal names violate autonomy. Use capability_discovery::find() instead."
)]
pub async fn connect_to_beardog() -> Result<Connection> {
    // Legacy implementation with warning
    warn!("DEPRECATED: Hardcoded primal name 'beardog'. Use capability discovery instead.");
    // ...
}
```

**Timeline**: 1-2 hours

---

## 📋 EXECUTION CHECKLIST

### Batch 1: Foundation (4-5 hours)
- [ ] Create `capability_discovery.rs` module
- [ ] Implement `CapabilityDiscovery` struct
- [ ] Implement `discover_songbird_ipc()` function
- [ ] Add comprehensive tests
- [ ] Document usage patterns

### Batch 2: High Priority (8-10 hours)
- [ ] Evolve `rpc/songbird_registration.rs` (73 refs)
- [ ] Evolve `service_metadata/mod.rs` (54 refs)
- [ ] Evolve `transport/security.rs` (50 refs)
- [ ] Add deprecation warnings
- [ ] Update tests

### Batch 3: Medium Priority (5-7 hours)
- [ ] Evolve `config/runtime/services.rs` (25 refs)
- [ ] Evolve `primal_discovery/capability_helpers.rs` (17 refs)
- [ ] Evolve `primal_discovery.rs` (3 refs)
- [ ] Update configuration patterns
- [ ] Update examples

### Batch 4: Documentation (2-3 hours)
- [ ] Update all documentation examples
- [ ] Create capability discovery guide
- [ ] Document migration path
- [ ] Update ecosystem integration docs

### Batch 5: Verification (2-3 hours)
- [ ] Run full test suite
- [ ] Verify no hardcoded names remain in production code
- [ ] Performance testing
- [ ] Integration testing

**Total Timeline**: 21-28 hours (3-4 weeks with other tasks)

---

## 🎯 SUCCESS CRITERIA

### Technical
- [ ] Zero hardcoded primal names in production code
- [ ] All discovery via capability queries
- [ ] Songbird IPC integration complete
- [ ] Tests pass with new patterns
- [ ] Performance acceptable (cached discovery)

### Architectural
- [ ] Primal autonomy restored
- [ ] Self-knowledge only (no cross-knowledge)
- [ ] Runtime discovery throughout
- [ ] Capability-based configuration

### Ecosystem Compliance
- [ ] Inter-Primal Interactions standard: ✅ COMPLIANT
- [ ] Primal IPC Protocol standard: ✅ COMPLIANT
- [ ] Semantic Method Naming: ✅ COMPLIANT

---

## 💡 PATTERNS & EXAMPLES

### Pattern 1: Discover by Capability
```rust
// Application code discovers services by capability
let crypto = discovery.find("crypto").await?;
let http = discovery.find("http").await?;
let storage = discovery.find("storage").await?;
```

### Pattern 2: Songbird Bootstrap
```rust
// Bootstrap: Discover Songbird first
let songbird = CapabilityDiscovery::discover_songbird_ipc().await?;

// Then use Songbird to discover everything else
let discovery = CapabilityDiscovery::new(songbird);
let crypto = discovery.find("crypto").await?;
```

### Pattern 3: Cached Discovery
```rust
// Discovery is cached for performance
let crypto1 = discovery.find("crypto").await?;  // Network call
let crypto2 = discovery.find("crypto").await?;  // Cached!
```

### Pattern 4: Fallback Configuration
```rust
// Configure fallback behavior per capability
let config = CapabilityConfig {
    capability: "crypto",
    required: true,
    fallback: FallbackMode::Fail,  // No fallback for crypto
};
```

---

## 🚀 NEXT ACTIONS

### This Session
1. Create `capability_discovery.rs` module
2. Implement core discovery logic
3. Add comprehensive tests
4. Document usage patterns

### Next Session
1. Start Batch 2 (high priority files)
2. Evolve `rpc/songbird_registration.rs`
3. Evolve `service_metadata/mod.rs`
4. Add deprecation warnings

### This Week
1. Complete Batches 1-2
2. Begin Batch 3
3. Update documentation

---

## 📊 PROGRESS TRACKING

| Batch | Status | Files | Refs | Time Spent | Time Remaining |
|-------|--------|-------|------|------------|----------------|
| **1. Foundation** | 🔄 Starting | 1 | 0 | 0 | 4-5 hours |
| **2. High Priority** | ⏳ Planned | 3 | 177 | 0 | 8-10 hours |
| **3. Medium Priority** | ⏳ Planned | 3 | 45 | 0 | 5-7 hours |
| **4. Documentation** | ⏳ Planned | - | 50+ | 0 | 2-3 hours |
| **5. Verification** | ⏳ Planned | - | - | 0 | 2-3 hours |
| **TOTAL** | 🔄 In Progress | 60 | 511 | 0 | 21-28 hours |

---

## 🏆 BENEFITS

### Architectural
- ✅ Primal autonomy restored
- ✅ Self-knowledge pattern enforced
- ✅ Runtime discovery throughout
- ✅ Ecosystem standards compliant

### Maintainability
- ✅ No brittle name dependencies
- ✅ Services can be renamed freely
- ✅ Easy to add new service providers
- ✅ Configuration-driven behavior

### Flexibility
- ✅ Multiple providers per capability
- ✅ Failover to alternative providers
- ✅ Development vs production configs
- ✅ Testing with mock providers

---

**Status**: Foundation phase starting  
**Next**: Create `capability_discovery.rs` module  
**Timeline**: 21-28 hours total, 4-5 hours for foundation

🌍 **Evolving to TRUE primal autonomy!** ✨
