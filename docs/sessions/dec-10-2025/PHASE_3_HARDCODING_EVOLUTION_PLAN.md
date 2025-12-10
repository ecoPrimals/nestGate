# Phase 3: Hardcoding Evolution Plan

**Status**: IN PROGRESS  
**Goal**: Evolve 814 hardcoded values to capability-based discovery  
**Timeline**: 20-30 hours (2 weeks)

---

## STRATEGY

### Core Principle
**Primal Sovereignty**: Each primal only knows itself, discovers others at runtime

### Current State
- ✅ **Foundation exists**: `universal_primal_discovery` module
- ✅ **Discovery working**: mDNS backend operational
- ⚠️ **Hardcoding remains**: 814 instances across codebase
- ⚠️ **Mixed usage**: Some code uses discovery, some uses constants

### Evolution Path
```
Hardcoded Constants → Environment Config → Capability Discovery
```

---

## 814 HARDCODED VALUES BREAKDOWN

### Network (316 instances)
- **Addresses**: `127.0.0.1`, `0.0.0.0`, `localhost`
- **Status**: Centralized in `network_hardcoded.rs`
- **Action**: Keep for testing, add env override, use discovery for prod

### Ports (133 instances)
- **Values**: 8080, 3000, 5000, 9090, 9091, 9092
- **Status**: Centralized in `ports.rs`, deprecated helpers
- **Action**: ✅ **GOOD** - already using `EnvironmentConfig`

### Primal Services (80+ instances)
- **Pattern**: Direct service address assumptions
- **Example**: `http://localhost:9091` for networking service
- **Status**: ⚠️ **BLOCKS SOVEREIGNTY**
- **Action**: **CRITICAL** - Replace with capability discovery

### Storage Paths (100+ instances)
- **Pattern**: `/var/lib/nestgate`, `/tmp/nestgate`
- **Status**: Some use `StorageConfig`, some hardcoded
- **Action**: Migrate all to `EnvironmentConfig::storage`

### Timeouts (40+ instances)
- **Values**: 5000ms, 30000ms, 300000ms
- **Status**: Centralized in `network_hardcoded::timeouts`
- **Action**: Keep for defaults, allow env override

### Other Constants (145+ instances)
- **Mix**: Buffer sizes, retry counts, limits
- **Status**: Various locations
- **Action**: Audit case-by-case

---

## EVOLUTION PRIORITY

### P0: CRITICAL (Week 1) - Primal Discovery
**Goal**: Remove all hardcoded primal service addresses

**Files to Evolve**:
1. `universal_adapter/capability_system.rs` (line 459)
   - Replace `build_api_url()` with capability lookup
2. `universal_adapter/config.rs`
   - Remove hardcoded endpoints from `AdapterConfig::new()`
3. `config/discovery_config.rs`
   - Replace hardcoded discovery endpoints

**Pattern Before**:
```rust
let endpoint = "http://localhost:9091".to_string();
```

**Pattern After**:
```rust
// Discover service by capability
let service = self.discovery
    .find_by_capability(&PrimalCapability::Networking)
    .await?;
let endpoint = service.endpoint;
```

### P1: HIGH (Week 1-2) - Environment Config
**Goal**: All runtime values from environment or discovery

**Files to Evolve**:
1. API server bindings
2. Metrics endpoints
3. Storage paths
4. Service URLs

**Pattern**:
- Use `EnvironmentConfig::from_env()`
- Fall back to discovery if not in env
- Constants only for testing

### P2: MEDIUM (Week 2) - Test Isolation
**Goal**: Hardcoded values only in test code

**Pattern**:
```rust
#[cfg(test)]
const TEST_PORT: u16 = 9999;

#[cfg(not(test))]
fn get_port() -> Result<u16> {
    EnvironmentConfig::from_env()?.network.port.get()
}
```

---

## IMPLEMENTATION PHASES

### Phase 3.1: Discovery Integration (8-10 hours)
**Expand universal_primal_discovery usage**

1. ✅ **Foundation exists** (`capability_based_discovery.rs`)
2. **Add**: Service registry helper
   ```rust
   pub struct ServiceRegistry {
       discovery: Arc<CapabilityBasedDiscovery>,
   }
   
   impl ServiceRegistry {
       pub async fn find_networking_service(&self) -> Result<ServiceEndpoint> {
           self.discovery
               .find_by_capability(&PrimalCapability::Networking)
               .await
       }
       
       pub async fn find_security_service(&self) -> Result<ServiceEndpoint> {
           self.discovery
               .find_by_capability(&PrimalCapability::Security)
               .await
       }
   }
   ```

3. **Integrate**: Use in universal_adapter
   - Replace `build_api_url()` calls
   - Replace hardcoded endpoints
   - Add fallback chain: discovery → env → error

4. **Test**: Discovery works end-to-end
   - Unit tests (mock discovery)
   - Integration tests (real mDNS)
   - Fallback tests (missing services)

### Phase 3.2: Universal Adapter Evolution (6-8 hours)
**Remove hardcoded primal connections**

**Files**:
- `universal_adapter/mod.rs`
- `universal_adapter/config.rs`
- `universal_adapter/capability_system.rs`

**Changes**:
1. Remove hardcoded endpoint lists
2. Use discovery for all service lookups
3. Cache discovered services
4. Handle discovery failures gracefully

**Testing**:
- Adapter finds services dynamically
- Cache works correctly
- Fallback to env vars
- Error handling

### Phase 3.3: Configuration Migration (4-6 hours)
**All config from environment or discovery**

**Files**:
- `config/runtime/*.rs`
- `api_server_config.rs`
- Service initialization code

**Pattern**:
```rust
// OLD
const API_URL: &str = "http://localhost:8080";

// NEW
pub fn api_url() -> Result<String> {
    EnvironmentConfig::from_env()?
        .network
        .bind_address()
        .map(|addr| format!("http://{}", addr))
}
```

### Phase 3.4: Storage Path Migration (2-4 hours)
**No hardcoded file paths**

**Files**:
- Storage initialization
- Log file paths
- Cache directories
- Data directories

**Pattern**:
```rust
// OLD
const DATA_DIR: &str = "/var/lib/nestgate";

// NEW
pub fn data_dir() -> Result<PathBuf> {
    EnvironmentConfig::from_env()?
        .storage
        .data_directory
}
```

---

## TESTING STRATEGY

### Unit Tests
- Discovery returns expected services
- Fallback chain works
- Environment overrides work
- Caching works correctly

### Integration Tests
- Services discover each other
- Communication succeeds
- Multiple primals coexist
- Discovery failures handled

### E2E Tests
- Full system with dynamic discovery
- No hardcoded values used
- Services join/leave dynamically
- Config from environment only

---

## SUCCESS CRITERIA

### Code Quality
- [ ] No hardcoded service URLs in production code
- [ ] All ports from environment or discovery
- [ ] All paths from environment
- [ ] Constants only in test code or as fallback defaults

### Functionality
- [ ] Services discover each other automatically
- [ ] Environment config works end-to-end
- [ ] Fallback chain: discovery → env → default
- [ ] Multiple instances coexist

### Testing
- [ ] Unit tests for discovery
- [ ] Integration tests for adapter
- [ ] E2E tests for full system
- [ ] Chaos tests (discovery failures)

### Documentation
- [ ] Architecture updated
- [ ] Migration guide for users
- [ ] Examples of capability discovery
- [ ] Troubleshooting guide

---

## METRICS

### Before
- Hardcoded values: 814
- Services using discovery: ~20%
- Environment config: ~40%
- Pure constants: ~40%

### After (Target)
- Hardcoded values: 0 (production), <100 (tests)
- Services using discovery: 80%
- Environment config: 20%
- Pure constants: 0% (production)

### Intermediate (Week 1)
- Hardcoded values: ~400 (50% reduction)
- Discovery: 50%
- Environment: 40%
- Constants: 10%

---

## NEXT STEPS (This Session)

### Immediate (Next 2-4 hours)
1. **Create** `ServiceRegistry` helper
2. **Integrate** into `UniversalAdapter`
3. **Replace** first batch of hardcoded URLs
4. **Test** discovery chain works

### This Week (20-30 hours)
1. Complete Phase 3.1 (Discovery Integration)
2. Start Phase 3.2 (Universal Adapter Evolution)
3. Measure progress: 814 → ~400 hardcoded values

---

**Status**: PLAN COMPLETE, READY TO EXECUTE  
**First Task**: Create ServiceRegistry helper  
**Estimated Time**: 2-4 hours  
**Impact**: Remove ~80 hardcoded primal service URLs

Let's begin! 🚀

