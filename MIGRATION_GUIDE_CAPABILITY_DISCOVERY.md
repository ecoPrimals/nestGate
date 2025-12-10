# 🚀 Migration Guide: Capability-Based Discovery

**Status**: Migration in Progress (Phase 1 Complete)  
**Timeline**: Week 2 of Evolution Plan  
**Impact**: 31 files need migration

---

## 📋 Quick Summary

**From**: Environment-based discovery with hardcoded fallbacks  
**To**: Pure capability-based runtime discovery

**Benefits**:
- ✅ Zero hardcoding
- ✅ Runtime flexibility
- ✅ Sovereignty compliance
- ✅ Better error handling

---

## 🔄 Migration Pattern

### **Before** (Deprecated):
```rust
use crate::universal_primal_discovery::production_discovery::ProductionServiceDiscovery;

let discovery = ProductionServiceDiscovery::new(&config)?;
let port = discovery.discover_port("api")?;  // Falls back to 8080 if not found
let endpoint = discovery.discover_endpoint("api")?;
```

### **After** (Modern):
```rust
use crate::universal_primal_discovery::production_capability_bridge::CapabilityAwareDiscovery;
use crate::universal_primal_discovery::capability_based_discovery::PrimalCapability;

let discovery = CapabilityAwareDiscovery::initialize(&config).await?;

// Find by capability (not by hardcoded name!)
let services = discovery.find_capability(&PrimalCapability::ApiGateway).await?;
let best = services.first()
    .ok_or_else(|| NestGateError::not_found("No API gateway found"))?;
let port = best.binding.port;
let endpoint = best.endpoint.address;
```

---

## 📊 Files Requiring Migration

### 31 Files Found Using Old API:

**Priority 1 (Production Code)**: 15 files
1. `code/crates/nestgate-core/src/config/external/services.rs`
2. `code/crates/nestgate-core/src/monitoring/metrics.rs`
3. `code/crates/nestgate-core/src/network/native_async/production.rs`
4. `code/crates/nestgate-core/src/config/runtime_config.rs`
5. `code/crates/nestgate-core/src/security/universal_auth_adapter.rs`
... and 10 more

**Priority 2 (Config/Utils)**: 10 files
- Various config and adapter files

**Priority 3 (Tests)**: 6 files
- Test files (keep old API for compatibility tests)

---

## 🔧 Step-by-Step Migration

### Step 1: Update Imports

**Remove**:
```rust
use crate::universal_primal_discovery::production_discovery::{
    ProductionServiceDiscovery,
    ServiceDiscoveryConfig,
};
```

**Add**:
```rust
use crate::universal_primal_discovery::production_capability_bridge::CapabilityAwareDiscovery;
use crate::universal_primal_discovery::capability_based_discovery::{
    PrimalCapability,
    CapabilityCategory,
};
```

### Step 2: Update Initialization

**Before**:
```rust
let discovery = ProductionServiceDiscovery::new(&config)?;
```

**After**:
```rust
let discovery = CapabilityAwareDiscovery::initialize(&config).await?;
```

**Note**: Method is now `async` - ensure calling context supports async.

### Step 3: Update Discovery Calls

#### Port Discovery:
**Before**:
```rust
let port = discovery.discover_port("api")?;
```

**After**:
```rust
let services = discovery.find_service("api").await?;
let port = services.first()
    .ok_or_else(|| NestGateError::not_found("API service"))?
    .endpoint.address.port();
```

#### Endpoint Discovery:
**Before**:
```rust
let endpoint = discovery.discover_endpoint("api")?;
```

**After**:
```rust
let endpoint = discovery.get_best_endpoint("api").await?;
```

#### Capability-Based (New!):
```rust
// Find by capability instead of name
let security_services = discovery
    .find_capability(&PrimalCapability::Authentication)
    .await?;

let storage_services = discovery
    .find_capability(&PrimalCapability::ZfsStorage)
    .await?;
```

### Step 4: Error Handling

**Old**: Silent fallbacks to hardcoded defaults  
**New**: Explicit error handling

**Example**:
```rust
// OLD: Returns 8080 if service not found (silent fallback)
let port = discovery.discover_port("api")?;

// NEW: Returns error if service not found (explicit)
let port = match discovery.find_service("api").await {
    Ok(services) if !services.is_empty() => services[0].endpoint.address.port(),
    Ok(_) => return Err(NestGateError::not_found("API service not discovered")),
    Err(e) => return Err(e),
};

// Or use helper method:
let endpoint = discovery.get_best_endpoint("api").await?;
```

---

## 🎯 Migration Examples

### Example 1: Simple Port Discovery

**Before**:
```rust
fn get_api_port(config: &NestGateCanonicalConfig) -> Result<u16> {
    let discovery = ProductionServiceDiscovery::new(config)?;
    discovery.discover_port("api")
}
```

**After**:
```rust
async fn get_api_port(config: &NestGateCanonicalConfig) -> Result<u16> {
    let discovery = CapabilityAwareDiscovery::initialize(config).await?;
    let services = discovery.find_service("api").await?;
    services.first()
        .ok_or_else(|| NestGateError::not_found("API service"))
        .map(|s| s.endpoint.address.port())
}
```

### Example 2: Multiple Services

**Before**:
```rust
fn setup_services(config: &NestGateCanonicalConfig) -> Result<ServiceConfig> {
    let discovery = ProductionServiceDiscovery::new(config)?;
    
    Ok(ServiceConfig {
        api_port: discovery.discover_port("api")?,
        metrics_port: discovery.discover_port("metrics")?,
        health_port: discovery.discover_port("health")?,
    })
}
```

**After**:
```rust
async fn setup_services(config: &NestGateCanonicalConfig) -> Result<ServiceConfig> {
    let discovery = CapabilityAwareDiscovery::initialize(config).await?;
    
    let api = discovery.get_best_endpoint("api").await?;
    let metrics = discovery.get_best_endpoint("metrics").await?;
    let health = discovery.get_best_endpoint("health").await?;
    
    Ok(ServiceConfig {
        api_port: api.port(),
        metrics_port: metrics.port(),
        health_port: health.port(),
    })
}
```

### Example 3: Capability-Based Discovery (New Pattern)

```rust
async fn discover_security_services(
    config: &NestGateCanonicalConfig
) -> Result<Vec<SocketAddr>> {
    let discovery = CapabilityAwareDiscovery::initialize(config).await?;
    
    // Find all services with security capabilities
    let auth_services = discovery
        .find_capability(&PrimalCapability::Authentication)
        .await?;
    
    // Extract endpoints from healthy services only
    let endpoints: Vec<SocketAddr> = auth_services
        .into_iter()
        .filter(|s| matches!(s.health, HealthStatus::Healthy))
        .map(|s| s.endpoint.address)
        .collect();
    
    if endpoints.is_empty() {
        return Err(NestGateError::not_found(
            "No healthy authentication services found"
        ));
    }
    
    Ok(endpoints)
}
```

---

## 🚨 Common Pitfalls

### Pitfall 1: Forgetting Async
**Problem**: `CapabilityAwareDiscovery` methods are async  
**Solution**: Add `.await?` and ensure calling context is async

### Pitfall 2: Expecting Hardcoded Fallbacks
**Problem**: Old API silently fell back to defaults (8080, etc.)  
**Solution**: Handle "not found" errors explicitly or provide defaults

```rust
// Explicit default handling
let port = match discovery.find_service("api").await {
    Ok(services) if !services.is_empty() => services[0].endpoint.address.port(),
    _ => 8080, // Explicit default
};
```

### Pitfall 3: Synchronous Context
**Problem**: Can't use async methods in sync context  
**Solution**: Use compatibility methods or refactor to async

```rust
// If you MUST stay synchronous (not recommended):
#[allow(deprecated)]
let discovery = ProductionServiceDiscovery::new(&config)?;
let port = discovery.discover_port("api")?;

// Better: Refactor to async
async fn my_function() {
    let discovery = CapabilityAwareDiscovery::initialize(&config).await?;
    // ...
}
```

---

## ✅ Testing Migration

### Before Migration:
```bash
# Old tests should still pass
cargo test production_discovery
```

### After Migration:
```bash
# New tests verify capability-based discovery
cargo test capability_aware_discovery

# Integration tests
cargo test e2e_capability_discovery
```

---

## 📈 Migration Progress

### Week 1: Planning ✅
- [x] Identify all files using old API (31 found)
- [x] Create migration guide
- [x] Establish testing strategy

### Week 2: Execution (Current)
- [ ] Migrate Priority 1 files (15 production)
- [ ] Add migration tests
- [ ] Update documentation

### Week 3: Completion
- [ ] Migrate Priority 2 files (10 config/utils)
- [ ] Remove deprecated module
- [ ] Celebrate! 🎉

---

## 🎯 Benefits After Migration

1. **Zero Hardcoding** ✅
   - No more magic numbers (8080, etc.)
   - All values discovered at runtime

2. **Better Error Handling** ✅
   - Explicit errors instead of silent fallbacks
   - Clear failure modes

3. **Sovereignty Compliance** ✅
   - Capability-based discovery
   - No vendor-specific assumptions

4. **Runtime Flexibility** ✅
   - Services can be discovered dynamically
   - Supports multi-primal deployments

5. **Cleaner Code** ✅
   - Single source of truth
   - Modern async patterns
   - Better testability

---

## 📞 Getting Help

**Questions?** Check:
1. `production_capability_bridge.rs` - Full API documentation
2. `capability_based_discovery.rs` - Core types
3. `e2e_scenario_22_infant_discovery.rs` - Integration examples

**Need Support?** See `EVOLUTION_PLAN_DEC_5_2025.md` for roadmap

---

**Status**: Ready for Week 2 execution  
**Confidence**: 95%  
**Next**: Start migrating Priority 1 files

*Evolution to modern patterns in progress.* 🚀

