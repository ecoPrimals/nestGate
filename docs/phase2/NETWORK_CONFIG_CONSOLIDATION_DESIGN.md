# 🌐 Network Configuration Consolidation Design

**Phase 2 - Week 1 Days 2-5**  
**Date**: November 11, 2025  
**Target**: Consolidate 182 network configs → 1 canonical

---

## 📊 CURRENT STATE ANALYSIS

### Inventory Results

```
Total Network Config Definitions: 182
Existing Canonical Structure: ✅ Yes (CanonicalNetworkConfig)
Location: code/crates/nestgate-core/src/config/canonical_primary/domains/network/
```

### Major Config Categories Found

1. **Core Network Configs** (60+ definitions)
   - `NetworkConfig` (25+ variants across crates)
   - `NetworkConnectionConfig`  
   - `NetworkModuleConfig`
   - `Network*Config` (various aspects)

2. **Protocol Configs** (20+ definitions)
   - `ProtocolConfig` (3+ variants)
   - `HttpConfig`
   - `WebSocketConfig`
   - `GrpcConfig`
   - `NetworkProtocolConfig`

3. **Specialized Network Configs** (50+ definitions)
   - `NetworkTlsConfig`
   - `NetworkSecurityConfig`
   - `NetworkPerformanceConfig`
   - `NetworkMonitoringConfig`
   - `NetworkDiscoveryConfig`
   - etc.

4. **Feature-Specific Configs** (30+ definitions)
   - `NetworkPoolConfig`
   - `NetworkRetryConfig`
   - `NetworkCircuitBreakerConfig`
   - `NetworkCacheConfig`
   - `NetworkAuthConfig`
   - etc.

5. **Extension Configs** (22+ definitions)
   - `OrchestrationRetryConfig`
   - `NetworkVlanConfig`
   - `NetworkApiSettings`
   - etc.

---

## ✅ CANONICAL STRUCTURE (EXISTING)

The good news: **A well-designed canonical structure already exists!**

### Current Canonical Location

```rust
// File: code/crates/nestgate-core/src/config/canonical_primary/domains/network/mod.rs

pub struct CanonicalNetworkConfig {
    /// Core API server configuration
    pub api: ApiConfig,
    
    /// Network orchestration configuration
    pub orchestration: NetworkOrchestrationConfig,
    
    /// Protocol-specific configurations
    pub protocols: NetworkProtocolConfig,
    
    /// VLAN and network segmentation
    pub vlan: NetworkVlanConfig,
    
    /// Service discovery configuration
    pub discovery: NetworkDiscoveryConfig,
    
    /// Performance and optimization settings
    pub performance: NetworkPerformanceConfig,
    
    /// Security and authentication settings
    pub security: NetworkSecurityConfig,
    
    /// Monitoring and observability
    pub monitoring: NetworkMonitoringConfig,
    
    /// Environment-specific overrides
    pub environment: NetworkEnvironmentConfig,
}
```

### Sub-Config Structures (Already Exist)

Located in: `code/crates/nestgate-core/src/config/canonical_primary/domains/network/`

- `api.rs` - ApiConfig, TlsConfig, RateLimitingConfig, etc.
- `orchestration.rs` - NetworkOrchestrationConfig
- `protocols.rs` - NetworkProtocolConfig, HttpConfig, WebSocketConfig, GrpcConfig
- `vlan.rs` - NetworkVlanConfig
- `discovery.rs` - NetworkDiscoveryConfig
- `performance.rs` - NetworkPerformanceConfig
- `security.rs` - NetworkSecurityConfig
- `monitoring.rs` - NetworkMonitoringConfig
- `environment.rs` - NetworkEnvironmentConfig

**Status**: ✅ **Excellent canonical structure exists - we just need to migrate usage!**

---

## 🎯 CONSOLIDATION STRATEGY

### Phase 1: Type Alias Migration (Week 1)

**Goal**: Replace all 182 network config definitions with type aliases pointing to canonical

**Approach**: Non-breaking backward compatibility via type aliases

```rust
// BEFORE (scattered):
pub struct NetworkConfig {
    pub host: String,
    pub port: u16,
    // ... fields
}

// AFTER (type alias):
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig")]
pub type NetworkConfig = nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
```

### Phase 2: Import Migration (Week 1)

**Goal**: Update all imports to use canonical location

```rust
// OLD (deprecated):
use crate::network::config::NetworkConfig;
use nestgate_network::types::NetworkConfig;
use nestgate_canonical::types::NetworkConfig;

// NEW (canonical):
use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
// Or shorter:
use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig as NetworkConfig;
```

### Phase 3: Direct Usage (Week 2+)

**Goal**: Gradually migrate code to use canonical directly

```rust
// CURRENT (via alias):
let config: NetworkConfig = load_config();

// TARGET (direct canonical):
use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
let config: CanonicalNetworkConfig = load_config();
```

---

## 📝 MIGRATION PATTERNS

### Pattern 1: Simple Struct Replacement

For structs with matching fields:

```rust
// BEFORE:
pub struct NetworkConnectionConfig {
    pub host: String,
    pub port: u16,
    pub timeout: Duration,
}

// AFTER (if canonical has these fields):
pub type NetworkConnectionConfig = CanonicalNetworkConfig;

// Or if it's a sub-config:
pub use nestgate_core::config::canonical_primary::domains::network::api::ApiConfig as NetworkConnectionConfig;
```

### Pattern 2: Struct with Extra Fields

For structs with additional fields not in canonical:

```rust
// BEFORE:
pub struct NetworkPoolConfig {
    pub max_connections: usize,
    pub min_idle: usize,
    pub custom_field: String,  // Not in canonical
}

// OPTION A: Add to canonical (if generally useful)
// OPTION B: Create extension struct
pub struct NetworkPoolConfigExtensions {
    pub base: CanonicalNetworkConfig,
    pub custom_field: String,
}
```

### Pattern 3: Const Generic Configs

For configs using const generics:

```rust
// BEFORE:
pub struct ClientConfig<const DEFAULT_TIMEOUT_MS: u64 = 30000> {
    // ... fields
}

// AFTER: Use canonical with builder pattern
pub type ClientConfig = CanonicalNetworkConfig;

// Or create typed wrapper:
pub struct TypedClientConfig<const TIMEOUT: u64>(CanonicalNetworkConfig);
```

### Pattern 4: Empty/Marker Configs

For empty configs used as markers:

```rust
// BEFORE:
pub struct NetworkTraitsConfig {
    // Empty - just a marker
}

// AFTER: Type alias or remove entirely
pub type NetworkTraitsConfig = CanonicalNetworkConfig;
// Or: Just remove and use CanonicalNetworkConfig directly
```

---

## 🗺️ FILE-BY-FILE MIGRATION PLAN

### Priority 1: Core Network Crates (Days 2-3)

**Files to migrate**:
1. `nestgate-network/src/types.rs` ✅ (already uses CanonicalNetworkConfig!)
2. `nestgate-network/src/config.rs`
3. `nestgate-network/src/unified_network_config/*`
4. `nestgate-canonical/src/types.rs`

**Actions**:
- Add deprecation markers to old definitions
- Add type aliases pointing to canonical
- Update internal usage gradually

### Priority 2: Core Infrastructure (Days 3-4)

**Files to migrate**:
5. `nestgate-core/src/network/*.rs` (20+ files)
6. `nestgate-core/src/config_root/mod.rs`
7. `nestgate-core/src/canonical/types/config_registry.rs`

**Actions**:
- Replace struct definitions with type aliases
- Update imports across dependent code
- Run tests after each file

### Priority 3: API & Client Code (Day 4)

**Files to migrate**:
8. `nestgate-api/src/ecoprimal_sdk/config.rs`
9. `nestgate-api/src/*` (any NetworkConfig usage)

**Actions**:
- Update API handler configs
- Update client configurations
- Ensure backward compatibility

### Priority 4: Zero-Cost & Advanced (Day 5)

**Files to migrate**:
10. `nestgate-core/src/zero_cost/const_generic_config.rs`
11. `nestgate-core/src/canonical_modernization/unified_types.rs`
12. `nestgate-core/src/dev_stubs/*.rs`

**Actions**:
- Handle const generic configs specially
- Update modernization layer
- Clean up dev stubs

---

## 🔧 IMPLEMENTATION STEPS

### Step 1: Audit Existing Canonical (✅ Already Done)

Review canonical structure - **COMPLETE**, structure is excellent!

### Step 2: Create Type Alias Template Script

```bash
#!/bin/bash
# File: scripts/create_network_config_alias.sh

FILE=$1
CONFIG_NAME=$2

# Add deprecation and type alias
cat >> "$FILE" << 'EOF'

// ==================== MIGRATION TO CANONICAL ====================
// This config has been consolidated into canonical_primary

#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig")]
pub type $CONFIG_NAME = nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// For backward compatibility, this type alias will be removed in v0.12.0 (May 2026)
EOF
```

### Step 3: Systematic File Migration

For each file with NetworkConfig:
1. **Backup original** (already done via git)
2. **Add deprecation marker** to existing struct
3. **Add type alias** pointing to canonical
4. **Update imports** in same file
5. **Run tests** to verify
6. **Commit** the change

### Step 4: Validate Each Change

```bash
# After each file migration:
cargo test --package <affected-crate>
cargo check --workspace
```

---

## 📊 MIGRATION TRACKING

### Network Config Migration Checklist

#### Core Crates (25 configs)
- [ ] nestgate-network/src/types.rs: `NetworkConfig` (✅ already aliased!)
- [ ] nestgate-network/src/config.rs: `NetworkConfig`
- [ ] nestgate-network/src/protocols.rs: `ProtocolConfig` (2 instances)
- [ ] nestgate-network/src/vlan.rs: `VlanConfig`
- [ ] nestgate-network/src/unified_network_config/network_core.rs: `NetworkExtensions`
- [ ] nestgate-network/src/unified_network_extensions/orchestration.rs: `OrchestrationRetryConfig`
- [ ] nestgate-canonical/src/types.rs: `NetworkConfig`
- [ ] ... (20+ more)

#### nestgate-core/src/network/ (30+ configs)
- [ ] network/tracing.rs: `NetworkTracingConfig`
- [ ] network/middleware.rs: `NetworkMiddlewareConfig`
- [ ] network/request.rs: `NetworkRequestConfig`
- [ ] network/pool.rs: `NetworkPoolConfig`
- [ ] network/traits.rs: `NetworkTraitsConfig`
- [ ] network/auth.rs: `NetworkAuthConfig`
- [ ] network/connection.rs: `NetworkConnectionConfig`
- [ ] network/retry.rs: `NetworkRetryConfig`
- [ ] network/response.rs: `NetworkResponseConfig`
- [ ] network/client.rs: `ClientConfig<const>`
- [ ] network/config.rs: `NetworkModuleConfig`
- [ ] network/native_async/service.rs: `NetworkServiceConfig`
- [ ] network/tls.rs: `NetworkTlsConfig`
- [ ] network/timeout.rs: `NetworkTimeoutConfig`
- [ ] network/types.rs: `NetworkTypesConfig`
- [ ] network/security.rs: `NetworkSecurityConfig`
- [ ] network/compression.rs: `NetworkCompressionConfig`
- [ ] network/error.rs: `NetworkErrorConfig`
- [ ] network/cache.rs: `NetworkCacheConfig`
- [ ] network/metrics.rs: `NetworkMetricsConfig`
- [ ] network/circuit_breaker.rs: `NetworkCircuitBreakerConfig`
- [ ] ... (10+ more)

#### Other Locations (127+ configs)
- [ ] canonical_primary/domains/network/*.rs (45+ configs) ✅ (canonical location)
- [ ] config/performance/network.rs (3 configs)
- [ ] zero_cost/const_generic_config.rs (2 configs)
- [ ] universal_primal_discovery/network.rs (`NetworkDiscoveryConfig`)
- [ ] ... (75+ more)

**Total**: 182 network configs to consolidate

---

## ⚠️ RISK MITIGATION

### Risk 1: Breaking Changes

**Mitigation**:
- Use type aliases for 6-month deprecation period
- All existing code continues to work
- Gradual migration timeline

### Risk 2: Field Mismatches

**Mitigation**:
- Audit each config for unique fields
- Add extensions if needed
- Document migration path

### Risk 3: Const Generic Issues

**Mitigation**:
- Handle const generic configs specially
- Use builder pattern where appropriate
- Document type parameter migration

### Risk 4: Test Failures

**Mitigation**:
- Run tests after each file migration
- Fix issues immediately
- Rollback if needed (git branch)

---

## ✅ SUCCESS CRITERIA

### Week 1 Goals

1. **Type Aliases Added**: All 182 configs have type aliases pointing to canonical
2. **Zero Breaking Changes**: All existing code compiles and tests pass
3. **Documentation Updated**: Migration paths documented for each config
4. **Progress Tracked**: Checklist maintained, commits organized

### Metrics

```
Baseline: 182 NetworkConfig definitions
Target:   1 CanonicalNetworkConfig + 181 type aliases
Progress: Track via PHASE_2_PROGRESS.md
```

---

## 📚 REFERENCES

### Existing Canonical Structure

- **Location**: `code/crates/nestgate-core/src/config/canonical_primary/domains/network/`
- **Main struct**: `CanonicalNetworkConfig`
- **Sub-configs**: 9 modules (api, orchestration, protocols, vlan, discovery, performance, security, monitoring, environment)

### Parent Project Patterns

- **BearDog Phase 2**: Successfully consolidated network configs using same approach
- **Patterns used**: Type aliases, gradual migration, 6-month deprecation
- **Success rate**: 70% reduction achieved

---

## 🎯 NEXT ACTIONS (Days 2-5)

### Day 2 (Today) - Remaining Hours

1. **Complete this design document** ✅
2. **Create migration script template**
3. **Begin Priority 1 migrations** (nestgate-network core files)
4. **Run initial tests**

### Day 3

5. **Complete Priority 1** (core network crates)
6. **Begin Priority 2** (core infrastructure)
7. **Update progress tracker**

### Day 4

8. **Complete Priority 2**
9. **Begin Priority 3** (API & client code)
10. **Mid-week status check**

### Day 5

11. **Complete Priority 3**
12. **Begin Priority 4** (zero-cost & advanced)
13. **Week 1 summary report**

---

**Design Document Status**: ✅ **COMPLETE**  
**Next**: Create migration scripts and begin Priority 1 migrations  
**Timeline**: Days 2-5 (14-17 hours remaining)

---

*"Excellent canonical structure exists - migration is the path forward!"*

