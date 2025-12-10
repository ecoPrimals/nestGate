# 🌍 CAPABILITY-BASED CONFIGURATION MIGRATION GUIDE

**Purpose**: Replace hardcoded values with agnostic, capability-based discovery  
**Philosophy**: Each primal knows ONLY itself, discovers others by capability  
**Status**: ✅ **Infrastructure Complete** - Ready for migration

---

## 🎯 PHILOSOPHY

### The Problem with Hardcoding

```rust
// ❌ BAD: Hardcoded values create tight coupling
const API_PORT: u16 = 8080;
const METRICS_PORT: u16 = 9090;
const BEARDOG_ADDRESS: &str = "localhost:3000";

let api_addr = SocketAddr::new("127.0.0.1".parse()?, API_PORT);
let security = connect_to("beardog", BEARDOG_ADDRESS)?; // Hardcoded primal name!
```

**Problems**:
- Cannot move services to different ports
- Cannot scale horizontally
- Tight coupling between primals
- Hardcoded primal names violate sovereignty
- Impossible to test with different configurations

### The Solution: Capability-Based Discovery

```rust
// ✅ GOOD: Capability-based, no hardcoding
use nestgate_core::capability_config::*;

let config = CapabilityConfig::from_env()?;

// Discover by capability, NOT by name
let api_addr = config.get_endpoint("api")?;
let security_provider = discovery.find_capability("authentication").await?;
// NO hardcoded "beardog"!
```

**Benefits**:
- ✅ Configuration-driven (environment variables)
- ✅ Services can move freely
- ✅ Horizontal scaling supported
- ✅ True primal sovereignty (no hardcoded names)
- ✅ Easy testing with different configs

---

## 📋 MIGRATION PATTERNS

### Pattern 1: Hardcoded Ports → Environment Configuration

#### Before (Hardcoded):
```rust
// constants/hardcoding.rs
pub const API_PORT: u16 = 8080;
pub const METRICS_PORT: u16 = 9090;
pub const HEALTH_PORT: u16 = 8081;

// Usage:
let api_addr = SocketAddr::new("0.0.0.0".parse()?, API_PORT);
```

#### After (Capability-Based):
```rust
// Configuration (environment):
// NESTGATE_CAPABILITIES=api,metrics,health
// NESTGATE_API_ENDPOINT=0.0.0.0:8080
// NESTGATE_METRICS_ENDPOINT=0.0.0.0:9090
// NESTGATE_HEALTH_ENDPOINT=0.0.0.0:8081

// Usage:
use nestgate_core::capability_config::*;

let config = CapabilityConfig::from_env()?;
let api_addr = config.get_endpoint("api")?;
let metrics_addr = config.get_endpoint("metrics")?;
let health_addr = config.get_endpoint("health")?;
```

---

### Pattern 2: Hardcoded Primal Names → Capability Discovery

#### Before (Hardcoded):
```rust
// ❌ Hardcoded primal names violate sovereignty
const BEARDOG_ENDPOINT: &str = "localhost:3000";
const SONGBIRD_ENDPOINT: &str = "localhost:4000";

let security = connect_to_beardog(BEARDOG_ENDPOINT)?;
let network = connect_to_songbird(SONGBIRD_ENDPOINT)?;
```

#### After (Capability-Based):
```rust
// ✅ Discover by capability, respect sovereignty
use nestgate_core::self_knowledge::*;

// Initialize discovery (NO hardcoded primal names!)
let discovery = PrimalDiscovery::new(self_knowledge).await?;

// Discover by what they DO, not who they ARE
let security_providers = discovery.find_capability("authentication").await?;
let network_providers = discovery.find_capability("network-coordination").await?;

// Pick best available (load balancing, failover)
let security = security_providers.first()
    .ok_or(Error::NoSecurityProvider)?;
let network = network_providers.first()
    .ok_or(Error::NoNetworkProvider)?;
```

---

### Pattern 3: Hardcoded IPs → Dynamic Discovery

#### Before (Hardcoded):
```rust
// ❌ Hardcoded localhost
let addr: SocketAddr = "127.0.0.1:8080".parse()?;

// ❌ Hardcoded 0.0.0.0
let bind_addr: SocketAddr = "0.0.0.0:8080".parse()?;
```

#### After (Environment-Driven):
```rust
// ✅ Configuration-driven
let config = CapabilityConfig::from_env()?;
let addr = config.get_endpoint("api")?;

// Fallback to defaults only for development
let config = CapabilityConfig::from_env()?
    .with_fallback(CapabilityDefaults::development())?;
```

---

### Pattern 4: Hardcoded Timeouts → Configuration

#### Before (Hardcoded):
```rust
const CONNECTION_TIMEOUT_MS: u64 = 3000;
const REQUEST_TIMEOUT_MS: u64 = 10000;

let timeout = Duration::from_millis(CONNECTION_TIMEOUT_MS);
```

#### After (Environment-Driven):
```rust
// Environment:
// NESTGATE_CONNECTION_TIMEOUT=30s
// NESTGATE_REQUEST_TIMEOUT=60s

let connection_timeout = env::var("NESTGATE_CONNECTION_TIMEOUT")
    .ok()
    .and_then(|s| parse_duration(&s).ok())
    .unwrap_or(Duration::from_secs(30));

let request_timeout = env::var("NESTGATE_REQUEST_TIMEOUT")
    .ok()
    .and_then(|s| parse_duration(&s).ok())
    .unwrap_or(Duration::from_secs(60));
```

---

## 🔄 SYSTEMATIC MIGRATION PROCESS

### Phase 1: Infrastructure (✅ Complete)
- [x] Create `capability_config` module
- [x] Define `CapabilityConfig` types
- [x] Implement environment variable parsing
- [x] Add discovery backend support

### Phase 2: Module-by-Module Migration (In Progress)

#### Priority 1: Network Constants
```bash
# Files to migrate:
- constants/hardcoding.rs (ports module)
- constants/network_hardcoded.rs
- constants/network_defaults.rs
```

**Migration Script**:
```rust
// For each constant:
// 1. Document as deprecated
// 2. Add environment variable alternative
// 3. Update callers
// 4. Remove constant in next major version
```

#### Priority 2: Primal Discovery
```bash
# Files to migrate:
- universal_primal_discovery/stubs.rs
- dev_stubs/primal_discovery.rs
```

**Strategy**:
1. Replace hardcoded primal names with capability queries
2. Use `self_knowledge` pattern throughout
3. Remove all references to specific primal names

#### Priority 3: API Handlers
```bash
# Files to update:
- api/handlers/**/*.rs
```

**Pattern**:
```rust
// OLD:
const PORT: u16 = 8080;

// NEW:
let port = config.get_endpoint("api")?.port();
```

### Phase 3: Testing & Validation
- [ ] Update all tests to use configuration
- [ ] Add integration tests with different configs
- [ ] Validate with real primals (ToadStool, etc.)

### Phase 4: Deprecation & Cleanup
- [ ] Mark old constants as deprecated
- [ ] Update documentation
- [ ] Remove hardcoded values in v1.0.0

---

## 📝 ENVIRONMENT VARIABLE NAMING CONVENTION

### Standard Format:
```bash
NESTGATE_{COMPONENT}_{SETTING}
```

### Examples:
```bash
# Capabilities
NESTGATE_CAPABILITIES=api,metrics,storage,zfs-management

# Endpoints (one per capability)
NESTGATE_API_ENDPOINT=0.0.0.0:8080
NESTGATE_METRICS_ENDPOINT=0.0.0.0:9090
NESTGATE_STORAGE_ENDPOINT=0.0.0.0:7000

# Discovery
NESTGATE_DISCOVERY_BACKENDS=dns-srv,mdns,consul
NESTGATE_DNS_DOMAIN=nestgate.local
NESTGATE_MDNS_SERVICE=_nestgate._tcp

# Timeouts
NESTGATE_CONNECTION_TIMEOUT=30s
NESTGATE_REQUEST_TIMEOUT=60s
NESTGATE_HEALTH_CHECK_INTERVAL=10s

# Resources
NESTGATE_MAX_CONNECTIONS=1000
NESTGATE_MAX_MEMORY=2GB
NESTGATE_CPU_LIMIT=4
```

---

## 🧪 TESTING STRATEGY

### Unit Tests
```rust
#[test]
fn test_capability_from_env() {
    // Set environment variables
    temp_env::with_vars(
        vec![
            ("NESTGATE_CAPABILITIES", Some("api,metrics")),
            ("NESTGATE_API_ENDPOINT", Some("127.0.0.1:8080")),
        ],
        || {
            let config = CapabilityConfig::from_env().unwrap();
            assert!(config.capabilities.contains_key("api"));
            
            let addr = config.get_endpoint("api").unwrap();
            assert_eq!(addr.port(), 8080);
        },
    );
}
```

### Integration Tests
```rust
#[tokio::test]
async fn test_primal_discovery_without_hardcoding() {
    // NO hardcoded primal names!
    let discovery = PrimalDiscovery::new(self_knowledge).await?;
    
    // Discover by capability
    let providers = discovery.find_capability("authentication").await?;
    
    // Should work with ANY authentication provider
    // (beardog, custom impl, etc.)
    assert!(!providers.is_empty());
}
```

---

## 📊 MIGRATION PROGRESS TRACKING

| Module | Hardcoded Values | Migrated | Status |
|--------|-----------------|----------|--------|
| **constants/hardcoding.rs** | 47 | 0 | ⏳ Todo |
| **constants/network_hardcoded.rs** | 21 | 0 | ⏳ Todo |
| **constants/network_defaults.rs** | 59 | 0 | ⏳ Todo |
| **universal_primal_discovery/stubs.rs** | 6 | 0 | ⏳ Todo |
| **dev_stubs/primal_discovery.rs** | 6 | 0 | ⏳ Todo |
| **API handlers** | ~200 | 0 | ⏳ Todo |
| **Test files** | ~1,300 | 0 | 📝 Keep |

**Total**: ~1,687 hardcoded values  
**Target**: Migrate ~60% in Phase 2 (production code)  
**Keep**: Test hardcoding (acceptable for test clarity)

---

## 🎯 SUCCESS CRITERIA

### Phase 2 Complete When:
- [ ] All production constants use environment variables
- [ ] Zero hardcoded primal names in production code
- [ ] All API handlers use capability config
- [ ] Integration tests pass with different configs
- [ ] Documentation updated

### Expected Impact:
- **Flexibility**: ⬆️ Can reconfigure without code changes
- **Scalability**: ⬆️ Horizontal scaling supported
- **Testability**: ⬆️ Easy to test different scenarios
- **Sovereignty**: ⬆️ True primal autonomy
- **Grade**: ⬆️ +2 points (A- → A)

---

## 🚀 NEXT STEPS

1. **This Week**: Migrate network constants (Priority 1)
2. **Next Week**: Update primal discovery (Priority 2)
3. **Week 3**: Update API handlers (Priority 3)
4. **Week 4**: Testing & validation
5. **Week 5-6**: Cleanup & documentation

---

**Status**: ✅ **Infrastructure Ready** - Begin migration  
**Timeline**: 4-6 weeks to complete  
**Impact**: +2 grade points, true primal sovereignty

*No hardcoding. No coupling. Pure capability discovery.* 🌍

