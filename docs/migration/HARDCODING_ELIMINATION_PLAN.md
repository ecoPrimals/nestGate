# 🎯 Hardcoding Elimination Plan - January 13, 2026

**Mission**: Achieve **infant discovery pattern** - Start with zero knowledge, discover everything at runtime

---

## 📊 **Current State**

### **Hardcoding Audit Results**
| Category | Count | Status |
|----------|-------|--------|
| **Vendor Names** | 326 matches | 🔴 Needs work |
| **Primal Names** | 175 matches | 🔴 Needs work |
| **Port Numbers** | 652 matches | 🟡 Infrastructure exists |

### **Existing Infrastructure** ✅
- ✅ Self-knowledge pattern (`self_knowledge/mod.rs`)
- ✅ Capability-based discovery
- ✅ Port defaults with env vars
- ✅ Universal adapter
- ✅ Infant discovery framework

---

## 🎯 **Philosophy: Infant Discovery**

### **Core Principles**

1. **Self-Awareness Only**
   - Primal knows ONLY itself
   - No compile-time knowledge of other primals
   - No hardcoded service names

2. **Runtime Discovery**
   - Discover by capability, not name
   - Use universal adapter for all connections
   - Network effects through discovery, not hardcoding

3. **Vendor Agnostic**
   - No hardcoded infrastructure (k8s, consul, etc.)
   - Discovery mechanism is pluggable
   - Works on bare metal, cloud, or k8s

4. **Sovereignty Preserving**
   - Each primal remains sovereign
   - No privileged knowledge of others
   - Configuration from capabilities, not assumptions

---

## 📋 **Elimination Strategy**

### **Phase 1: Vendor Hardcoding** (326 matches)

#### **Target Vendors**
- kubernetes / k8s (infrastructure)
- consul / etcd (service discovery)
- redis / postgres / mysql (data stores)
- docker (containers)

#### **Pattern Evolution**

**OLD (Hardcoded)**:
```rust
// ❌ Vendor-specific, not portable
const CONSUL_URL: &str = "http://consul:8500";
client.register_service(CONSUL_URL).await?;

// ❌ K8s-specific config
let k8s_config = load_k8s_config()?;
```

**NEW (Agnostic)**:
```rust
// ✅ Discovery-mechanism agnostic
let discovery = DiscoveryMechanism::detect().await?;
discovery.register_service(self_knowledge).await?;

// ✅ Platform-agnostic config
let config = PlatformConfig::detect().await?;
```

#### **Implementation Steps**
1. Create `DiscoveryMechanism` trait
2. Implement adapters for each vendor (consul, k8s, mdns, etc.)
3. Auto-detect available mechanism at runtime
4. Migrate hardcoded calls to trait
5. Make vendor-specific code feature-gated

---

### **Phase 2: Primal Name Hardcoding** (175 matches)

#### **Target Primal Names**
- songbird (orchestration)
- beardog (security)
- squirrel (encryption)
- toadstool (compute)
- biomeos (OS integration)

#### **Pattern Evolution**

**OLD (Name-Based)**:
```rust
// ❌ Hardcoded primal name - creates coupling
const ORCHESTRATOR: &str = "songbird";
let endpoint = format!("http://{}:8080", ORCHESTRATOR);

// ❌ Hardcoded service name
let auth_service = connect_to("beardog").await?;
```

**NEW (Capability-Based)**:
```rust
// ✅ Capability-based - any primal can provide
use Capability::Orchestration;
let orchestrators = discovery.find_by_capability(Orchestration).await?;
let orch = orchestrators.first().context("No orchestrator available")?;

// ✅ Capability-based authentication
use Capability::Authentication;
let auth = discovery.find_by_capability(Authentication).await?;
```

#### **Allowed Self-Knowledge**
```rust
// ✅ ONLY self-awareness allowed
const SELF_NAME: &str = "nestgate";
let self_knowledge = SelfKnowledge::builder()
    .with_name(SELF_NAME)
    .with_capability(Capability::Storage)
    .build();
```

#### **Implementation Steps**
1. Audit all primal name references
2. Categorize: self-awareness vs discovery vs documentation
3. Replace discovery code with capability queries
4. Update tests to use capability mocks
5. Preserve only self-awareness and docs

---

### **Phase 3: Numeric Port Hardcoding** (652 matches)

#### **Current Status**
- ✅ Infrastructure exists: `port_defaults.rs`
- ✅ Environment variables supported
- 🔴 Not consistently used

#### **Pattern Evolution**

**OLD (Hardcoded)**:
```rust
// ❌ Magic numbers scattered everywhere
let api_server = Server::bind("0.0.0.0:8080").await?;
let metrics = Server::bind("0.0.0.0:9090").await?;
let redis = connect("redis://localhost:6379")?;
```

**NEW (Centralized)**:
```rust
// ✅ Centralized with env var overrides
use nestgate_core::constants::port_defaults;

let api_port = port_defaults::get_api_port();  // NESTGATE_API_PORT or 8080
let api_server = Server::bind(format!("0.0.0.0:{}", api_port)).await?;

// ✅ Or use builder pattern
let config = ServerConfig::from_env()?;
let server = Server::bind(config.api_endpoint()).await?;
```

#### **Implementation Steps**
1. Scan for `:NNNN` patterns
2. Categorize: API, metrics, data stores, etc.
3. Ensure all have constants in `port_defaults.rs`
4. Replace magic numbers with constants
5. Add env var documentation

---

## 🎯 **Target State: Pure Infant Discovery**

### **Startup Sequence (Zero Knowledge → Full Discovery)**

```rust
// 1. SELF-AWARENESS (Birth)
let self_knowledge = SelfKnowledge::builder()
    .with_name("nestgate")
    .with_capabilities(vec![
        Capability::Storage,
        Capability::ZfsManagement,
    ])
    .with_endpoints(Endpoints::from_env()?)  // ← env vars, not hardcoded
    .build()?;

// 2. DISCOVERY MECHANISM (Learning to see)
let discovery = DiscoveryMechanism::detect().await?
    .unwrap_or(DiscoveryMechanism::mdns_default());  // Graceful fallback

// 3. ANNOUNCE SELF (Hello world!)
discovery.announce(self_knowledge.clone()).await?;

// 4. DISCOVER OTHERS (Finding siblings)
// ✅ Capability-based, not name-based
let orchestrators = discovery
    .find_by_capability(Capability::Orchestration)
    .await?;

let auth_services = discovery
    .find_by_capability(Capability::Authentication)
    .await?;

// 5. ESTABLISH CONNECTIONS (Building relationships)
for orch in orchestrators {
    // Universal adapter handles protocol negotiation
    let connection = UniversalAdapter::connect(orch).await?;
    // Use connection...
}

// 6. OPERATE (Living and growing)
// All connections discovered, no hardcoded dependencies!
```

---

## 📊 **Categorization Matrix**

### **Vendor Hardcoding (326 matches)**
| Vendor | Occurrences | Category | Priority |
|--------|-------------|----------|----------|
| kubernetes/k8s | ~150 | Infrastructure | HIGH |
| consul | ~50 | Service Discovery | HIGH |
| redis | ~40 | Data Store | MEDIUM |
| postgres | ~30 | Data Store | MEDIUM |
| docker | ~30 | Container | LOW |
| etcd | ~15 | Service Discovery | MEDIUM |
| mysql | ~11 | Data Store | LOW |

**Strategy**:
- HIGH: Create abstraction trait, implement adapters
- MEDIUM: Use trait if exists, else feature-gate
- LOW: Feature-gate only

### **Primal Hardcoding (175 matches)**
| Context | Count | Action |
|---------|-------|--------|
| Self-awareness | ~20 | ✅ Keep (legitimate) |
| Documentation/examples | ~50 | ✅ Keep (educational) |
| Test mocks | ~30 | 🔄 Convert to capability mocks |
| Production discovery | ~40 | 🔴 Migrate to capability-based |
| Comments/strings | ~35 | ✅ Keep (clarity) |

### **Port Hardcoding (652 matches)**
| Context | Count | Action |
|---------|-------|--------|
| Already using constants | ~300 | ✅ Good |
| Magic numbers | ~200 | 🔴 Migrate to constants |
| Test fixtures | ~100 | 🟡 Use test helpers |
| Documentation | ~52 | ✅ Keep (examples) |

---

## 🚀 **Execution Plan**

### **Week 1: Vendor Abstraction** (HIGH priority)
**Target**: 326 → <50 matches

**Day 1-2: Discovery Mechanism Trait**
- [ ] Create `DiscoveryMechanism` trait
- [ ] Implement `MdnsDiscovery`
- [ ] Implement `ConsulDiscovery`
- [ ] Implement `K8sDiscovery`
- [ ] Add auto-detection

**Day 3-4: Migrate Core Code**
- [ ] Replace consul-specific calls
- [ ] Replace k8s-specific calls
- [ ] Update service registration
- [ ] Update service lookup

**Day 5: Testing & Documentation**
- [ ] Integration tests for each adapter
- [ ] Documentation on adding adapters
- [ ] Migration guide

### **Week 2: Primal Name Elimination** (HIGH priority)
**Target**: 175 → <50 matches (only self/docs)

**Day 1-2: Audit & Categorize**
- [ ] Identify all primal name references
- [ ] Categorize: self, docs, tests, production
- [ ] Create migration matrix

**Day 3-4: Migrate Production Code**
- [ ] Replace name-based discovery with capability
- [ ] Update connection logic
- [ ] Migrate tests to capability mocks

**Day 5: Cleanup**
- [ ] Remove unused name constants
- [ ] Update documentation
- [ ] Verify self-awareness only

### **Week 3: Port Number Cleanup** (MEDIUM priority)
**Target**: 652 → <100 magic numbers

**Day 1-2: Audit & Extend Constants**
- [ ] Identify all port usages
- [ ] Ensure constants exist for all
- [ ] Add missing env var support

**Day 3-4: Migrate Code**
- [ ] Replace magic numbers
- [ ] Update tests to use helpers
- [ ] Document all port env vars

**Day 5: Validation**
- [ ] Verify all ports configurable
- [ ] Integration tests with custom ports
- [ ] Documentation complete

---

## 📈 **Success Metrics**

### **Quantitative**
- [ ] Vendor references: <50 (from 326)
- [ ] Primal name hardcoding: <50 (from 175)
- [ ] Magic port numbers: <100 (from 652)
- [ ] Discovery coverage: 100% capability-based
- [ ] All tests pass with custom ports
- [ ] All tests pass with different discovery mechanisms

### **Qualitative**
- [ ] Code starts with zero external knowledge
- [ ] All primals discovered via capabilities
- [ ] No vendor lock-in
- [ ] Portable across infrastructure (bare metal, cloud, k8s)
- [ ] Documentation reflects infant discovery pattern
- [ ] New developers understand sovereignty principles

---

## 🎯 **Infant Discovery Pattern Checklist**

### **Birth (Self-Awareness)**
- [ ] Primal knows only its name
- [ ] Primal knows its capabilities
- [ ] Primal knows its endpoints (from env)
- [ ] Primal knows its resources

### **Discovery (Learning)**
- [ ] Auto-detects discovery mechanism
- [ ] Announces self to ecosystem
- [ ] Queries by capability (not name)
- [ ] Caches discovery results

### **Connection (Relationships)**
- [ ] Uses universal adapter
- [ ] Protocol negotiation automatic
- [ ] Graceful degradation on failure
- [ ] No hardcoded URLs/addresses

### **Operation (Living)**
- [ ] All connections discovered
- [ ] Re-discovery on failure
- [ ] Health monitoring
- [ ] Sovereignty preserved

---

## 🔍 **Migration Examples**

### **Example 1: Orchestrator Connection**

**Before (Hardcoded)**:
```rust
// ❌ Violates sovereignty - knows primal name and location
const SONGBIRD_URL: &str = "http://songbird:8080";

async fn connect_orchestrator() -> Result<OrchestratorClient> {
    let client = reqwest::Client::new();
    Ok(OrchestratorClient::new(client, SONGBIRD_URL))
}
```

**After (Infant Discovery)**:
```rust
// ✅ Pure capability-based - no knowledge of primal names
use Capability::Orchestration;

async fn connect_orchestrator(
    discovery: &DiscoveryMechanism
) -> Result<OrchestratorClient> {
    let orchestrators = discovery
        .find_by_capability(Orchestration)
        .await
        .context("No orchestrator discovered")?;
    
    let orch = orchestrators
        .first()
        .context("No orchestrator available")?;
    
    let adapter = UniversalAdapter::connect(orch).await?;
    Ok(OrchestratorClient::from_adapter(adapter))
}
```

### **Example 2: Service Registration**

**Before (Vendor-Specific)**:
```rust
// ❌ Consul-specific - not portable
let consul = consul::Client::new("http://consul:8500")?;
consul.register_service("nestgate", 8080).await?;
```

**After (Vendor-Agnostic)**:
```rust
// ✅ Works with consul, k8s, mdns, or any discovery mechanism
let discovery = DiscoveryMechanism::detect().await?;
let self_knowledge = SelfKnowledge::load()?;
discovery.announce(self_knowledge).await?;
```

### **Example 3: Port Configuration**

**Before (Magic Numbers)**:
```rust
// ❌ Hardcoded port - not configurable
let server = Server::bind("0.0.0.0:8080").await?;
let metrics = Server::bind("0.0.0.0:9090").await?;
```

**After (Env-Configurable)**:
```rust
// ✅ Environment variable override
use nestgate_core::constants::port_defaults;

let api_port = port_defaults::get_api_port();
let metrics_port = port_defaults::get_metrics_port();

let server = Server::bind(format!("0.0.0.0:{}", api_port)).await?;
let metrics = Server::bind(format!("0.0.0.0:{}", metrics_port)).await?;
```

---

## 📚 **Reference: Sibling Primal Patterns**

### **biomeOS**
- ✅ Excellent capability taxonomy (50+ capabilities)
- ✅ Deep debt evolution plan
- ✅ SystemPaths for XDG compliance
- **Learn from**: Capability taxonomy structure

### **Songbird**
- (Check patterns for orchestration discovery)

### **BearDog**
- (Check patterns for security capability)

### **Squirrel & Toadstool**
- (Check patterns for specialized services)

---

## 🎯 **Phase 1 Immediate Actions** (This Session)

### **Quick Wins** (1-2 hours)
1. [ ] Create vendor hardcoding analysis
2. [ ] Create primal name hardcoding analysis
3. [ ] Document current self_knowledge usage
4. [ ] Identify quick migration candidates

### **Foundation** (2-3 hours)
1. [ ] Design `DiscoveryMechanism` trait
2. [ ] Implement mdns adapter (default)
3. [ ] Create capability-based mock for tests
4. [ ] Update 5-10 high-value files

### **Validation** (1 hour)
1. [ ] Run tests
2. [ ] Verify build
3. [ ] Document changes

---

## 🏆 **Success Criteria**

### **Phase 1 Complete When:**
- [ ] Vendor abstraction trait defined
- [ ] At least 2 adapters implemented
- [ ] 50+ vendor references migrated
- [ ] All tests passing
- [ ] Documentation updated

### **Project Complete When:**
- [ ] <50 vendor references (from 326)
- [ ] <50 primal hardcoding (self/docs only)
- [ ] <100 magic ports (from 652)
- [ ] Infant discovery fully demonstrated
- [ ] Portable across all infrastructure

---

**Status**: Ready to execute!  
**First Target**: Vendor hardcoding analysis and trait design  
**Timeline**: 3 weeks for complete elimination  
**Impact**: ✅ **TRUE INFANT DISCOVERY PATTERN**
