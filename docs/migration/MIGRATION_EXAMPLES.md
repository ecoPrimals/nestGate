# Migration Examples - Infant Discovery Pattern

**Date**: January 13, 2026  
**Status**: Ready for Production Use

---

## 📚 **MIGRATION GUIDE**

This document provides concrete examples of migrating from hardcoded primal names
to the infant discovery pattern.

---

## 🎯 **BEFORE vs AFTER**

### **Example 1: Songbird Registration → Orchestrator Registration**

#### **BEFORE** (Hardcoded "Songbird")
```rust
use nestgate_core::rpc::songbird_registration::SongbirdRegistration;

// ❌ Hardcoded primal name "songbird"
let family_id = std::env::var("NESTGATE_FAMILY_ID")?;
let registration = SongbirdRegistration::new(&family_id).await?;
registration.register().await?;
```

#### **AFTER** (Capability-Based)
```rust
use nestgate_core::rpc::orchestrator_registration::OrchestratorRegistration;
use nestgate_core::self_knowledge::SelfKnowledge;

// ✅ Capability-based: finds ANY orchestrator
let self_knowledge = SelfKnowledge::builder()
    .with_id("nestgate-001")
    .with_name("nestgate")
    .with_capability("storage")
    .with_endpoint("api", "0.0.0.0:8080".parse()?)
    .build()?;

let registration = OrchestratorRegistration::new(self_knowledge).await?;
registration.register().await?;  // Discovers orchestration capability
```

**Benefits**:
- ✅ Works with ANY orchestrator (Songbird, custom, future)
- ✅ No primal name hardcoding
- ✅ Infrastructure agnostic (mDNS, Consul, k8s)
- ✅ Graceful fallback if no orchestrator
- ✅ Self-knowledge based

---

### **Example 2: Direct Service Discovery**

#### **BEFORE** (Hardcoded Names)
```rust
// ❌ Hardcoded primal names
let songbird_url = format!("http://songbird:8080");
let beardog_url = format!("http://beardog:9090");
let toadstool_url = format!("http://toadstool:7070");
```

#### **AFTER** (Capability-Based)
```rust
use nestgate_core::discovery_mechanism::DiscoveryBuilder;

// ✅ Capability-based discovery
let discovery = DiscoveryBuilder::new().detect().await?;

// Find by capability, not name
let orchestrators = discovery.find_by_capability("orchestration".to_string()).await?;
let security_services = discovery.find_by_capability("security".to_string()).await?;
let compute_services = discovery.find_by_capability("compute".to_string()).await?;

// Use discovered services
for service in orchestrators {
    println!("Found orchestrator: {} at {}", service.name, service.endpoint);
}
```

**Benefits**:
- ✅ Zero primal name hardcoding
- ✅ Discovers actual available services
- ✅ Works with multiple providers
- ✅ Auto-adapts to infrastructure

---

### **Example 3: Network Communication**

#### **BEFORE** (Hardcoded Ports & Names)
```rust
// ❌ Hardcoded ports and names
async fn connect_to_songbird() -> Result<Client> {
    let client = Client::connect("songbird:8080").await?;
    Ok(client)
}
```

#### **AFTER** (Discovered Endpoints)
```rust
use nestgate_core::discovery_mechanism::DiscoveryBuilder;

async fn connect_to_orchestrator() -> Result<Client> {
    // ✅ Discover orchestrator endpoint
    let discovery = DiscoveryBuilder::new().detect().await?;
    
    let orchestrators = discovery
        .find_by_capability("orchestration".to_string())
        .await?;
    
    if let Some(orch) = orchestrators.first() {
        let client = Client::connect(&orch.endpoint).await?;
        Ok(client)
    } else {
        Err(Error::NoOrchestratorFound)
    }
}
```

**Benefits**:
- ✅ No port hardcoding
- ✅ No name hardcoding
- ✅ Discovers actual endpoint
- ✅ Works across environments

---

### **Example 4: Configuration**

#### **BEFORE** (Hardcoded in Config)
```toml
# ❌ config.toml with hardcoded services
[services]
songbird_url = "http://songbird:8080"
beardog_url = "http://beardog:9090"
toadstool_url = "http://toadstool:7070"
```

#### **AFTER** (Runtime Discovery)
```toml
# ✅ config.toml with capability requirements
[discovery]
mechanism = "auto"  # auto-detect: k8s → consul → mdns
timeout_sec = 5

[required_capabilities]
orchestration = true
security = false  # optional
compute = false  # optional
```

```rust
// Runtime discovery based on config
let discovery = DiscoveryBuilder::new()
    .with_timeout(Duration::from_secs(config.discovery.timeout_sec))
    .detect()
    .await?;

// Discover required capabilities
if config.required_capabilities.orchestration {
    let orch = discovery.find_by_capability("orchestration".to_string()).await?;
    ensure!(!orch.is_empty(), "Orchestration capability required");
}
```

**Benefits**:
- ✅ Config declares requirements, not specifics
- ✅ Runtime adapts to environment
- ✅ Works in dev, staging, production
- ✅ No hardcoded URLs

---

## 🧪 **TESTING MIGRATION**

### **BEFORE** (Mock Specific Services)
```rust
#[tokio::test]
async fn test_with_songbird() {
    // ❌ Must mock specific service
    let mock_songbird = MockSongbird::new();
    mock_songbird.start().await;
    
    // Test depends on "songbird" being available
    let client = connect_to_songbird().await.unwrap();
    assert!(client.is_connected());
}
```

### **AFTER** (Mock Capabilities)
```rust
use nestgate_core::discovery_mechanism::testing::{MockDiscovery, TestServiceBuilder};

#[tokio::test]
async fn test_with_orchestrator() {
    // ✅ Mock ANY service with orchestration capability
    let mut mock_discovery = MockDiscovery::new();
    
    let orchestrator = TestServiceBuilder::new("test-orch")
        .name("TestOrchestrator")
        .capability("orchestration")
        .endpoint("/tmp/test.sock")
        .build();
    
    mock_discovery.add_service(orchestrator);
    
    // Test works with ANY orchestrator
    let registration = OrchestratorRegistration::new_with_discovery(
        self_knowledge,
        Arc::new(mock_discovery)
    ).await.unwrap();
    
    assert!(registration.orchestrator().is_some());
}
```

**Benefits**:
- ✅ Test capabilities, not implementations
- ✅ Easy to create test services
- ✅ No specific mock infrastructure
- ✅ Tests are vendor-agnostic

---

## 📋 **MIGRATION CHECKLIST**

For each file being migrated:

- [ ] **Audit**: Identify hardcoded primal names (songbird, beardog, etc.)
- [ ] **Audit**: Identify hardcoded vendor names (k8s, consul, etc.)
- [ ] **Audit**: Identify hardcoded ports
- [ ] **Replace**: Primal names with capability queries
- [ ] **Replace**: Vendor-specific code with `DiscoveryMechanism` trait
- [ ] **Replace**: Hardcoded ports with discovered endpoints
- [ ] **Update**: Tests to use `MockDiscovery`
- [ ] **Test**: Verify functionality with discovery
- [ ] **Document**: Add migration notes to commit message

---

## 🎯 **PRIORITY FILES FOR MIGRATION**

Based on hardcoding audit, prioritize migration in this order:

### **Week 1** (High Impact)
1. `rpc/songbird_registration.rs` → `rpc/orchestrator_registration.rs` ✅ **DONE**
2. `universal_adapter/mod.rs` - Central adapter
3. `capability_resolver.rs` - Already capability-based, ensure uses new trait
4. `primal_discovery.rs` - Integrate new DiscoveryMechanism
5. `config/external/services_config.rs` - Service config

### **Week 2** (Medium Impact)
6. `universal_primal_discovery/network.rs` - Network discovery
7. `universal_primal_discovery/registry.rs` - Registry
8. `ecosystem_integration/mod.rs` - Ecosystem integration
9. `capabilities/mod.rs` - Capabilities system
10. `zero_cost_security_provider/capability_auth.rs` - Security

### **Week 3** (Low Impact, Cleanup)
11. Config files - Remove hardcoded defaults
12. Documentation - Update examples
13. Tests - Migrate to `MockDiscovery`
14. Examples - Update to infant discovery
15. Deprecated code - Remove or mark legacy

---

## 💡 **TIPS FOR MIGRATION**

### **1. Start with Tests**
```rust
// Create mock first, then implement
let mut mock = MockDiscovery::new();
mock.add_service(/* ... */);
// Now write code that uses discovery
```

### **2. Use Builder Patterns**
```rust
// Easy to read, easy to maintain
let service = TestServiceBuilder::new("id")
    .capability("storage")
    .capability("zfs")
    .endpoint("localhost:8080")
    .metadata("version", "1.0.0")
    .build();
```

### **3. Graceful Fallback**
```rust
// Always handle missing services gracefully
let services = discovery.find_by_capability("orchestration".to_string()).await?;
if services.is_empty() {
    warn!("No orchestrator found - continuing without orchestration");
    return Ok(());  // Graceful fallback
}
```

### **4. Log Discovery**
```rust
// Always log what was discovered
info!("🔍 Using discovery mechanism: {}", discovery.mechanism_name());
info!("✅ Discovered orchestrator: {} at {}", service.name, service.endpoint);
```

---

## 🎉 **SUCCESS CRITERIA**

After migration, the file should:

- ✅ **Zero primal names**: No "songbird", "beardog", etc.
- ✅ **Zero vendor names**: No "k8s", "consul", etc. in application code
- ✅ **Zero magic ports**: All ports from discovery or config
- ✅ **Capability-based**: Query by capability, not name
- ✅ **Infrastructure agnostic**: Works with mDNS, Consul, k8s
- ✅ **Testable**: Uses `MockDiscovery` in tests
- ✅ **Documented**: Clear comments explaining discovery
- ✅ **Logged**: Discovery steps logged for debugging

---

## 📚 **RELATED DOCUMENTATION**

- `HARDCODING_ELIMINATION_PLAN_JAN_13_2026.md` - Full 3-week plan
- `code/crates/nestgate-core/src/discovery_mechanism.rs` - Core trait
- `code/crates/nestgate-core/src/discovery_mechanism/testing.rs` - Test utilities
- `code/crates/nestgate-core/examples/infant_discovery_demo.rs` - Working example

---

**🎉 Happy migrating! The infant discovery pattern is here to stay!** 🎉
