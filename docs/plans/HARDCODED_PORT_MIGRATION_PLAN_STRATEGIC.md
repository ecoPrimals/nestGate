# 🔧 **HARDCODED PORT MIGRATION PLAN - STRATEGIC**

**Date**: October 20, 2025  
**Status**: Ready to Execute  
**Priority**: HIGH (Critical for sovereignty)  
**Timeline**: 6-8 weeks (systematic migration)

---

## 📊 **CURRENT SITUATION**

### Metrics
- **Total Hardcoded Values**: 1,112 instances
- **Hardcoded Ports**: ~768 instances (69%)
- **Production Code**: ~300-400 instances
- **Test Code**: ~368-468 instances (acceptable in tests)
- **Current Migration**: ~5% complete

### Impact
- ❌ **Sovereignty Violation**: Hardcoded ports = vendor assumptions
- ❌ **Flexibility**: Cannot adapt to different environments
- ❌ **Production Risk**: Port conflicts in deployments
- ❌ **Testing**: Hard to test different configurations

---

## 🎯 **MIGRATION STRATEGY**

### Phase 1: Foundation (Week 1) ✅
**Goal**: Establish environment-variable-driven port infrastructure

**Actions**:
1. ✅ Audit all hardcoded ports (COMPLETE)
2. ✅ Create canonical port management system (EXISTS)
3. ✅ Document migration patterns (THIS DOC)

**Files**:
- `code/crates/nestgate-core/src/config/network_defaults.rs` ✅
- `code/crates/nestgate-core/src/constants/network.rs` ✅

### Phase 2: Production Code Migration (Weeks 2-4)
**Goal**: Migrate all production hardcoded ports

**Priority Order**:

#### 🔴 **CRITICAL** (Week 2)
Files with direct user impact:
1. `code/crates/nestgate-api/src/handlers/` (50-70 instances)
2. `code/crates/nestgate-core/src/universal_adapter/discovery.rs` (20-30 instances)
3. `code/crates/nestgate-network/src/` production files (30-40 instances)
4. `code/crates/nestgate-middleware/src/config/security.rs` (10-15 instances)

#### 🟡 **HIGH** (Week 3)
Files with operational impact:
1. `code/crates/nestgate-zfs/src/` (15-25 instances)
2. `code/crates/nestgate-nas/src/` (10-20 instances)
3. `code/crates/nestgate-mcp/src/` (10-15 instances)
4. `code/crates/nestgate-automation/src/` (10-15 instances)

#### 🟢 **MEDIUM** (Week 4)
Supporting modules:
1. Configuration modules (20-30 instances)
2. Registry and discovery modules (15-25 instances)
3. Monitoring and metrics (10-15 instances)

### Phase 3: Test Code Migration (Weeks 5-6)
**Goal**: Migrate test hardcoded ports for consistency

**Note**: Test hardcoded ports are acceptable but should use environment variables for flexibility.

**Files**:
- All `tests/` directories (~368-468 instances)
- Integration tests
- E2E tests
- Example code

### Phase 4: Verification (Weeks 7-8)
**Goal**: Ensure complete migration and zero regressions

**Actions**:
1. Run full test suite
2. Verify environment variable fallbacks
3. Test in multiple environments
4. Update documentation
5. Final audit

---

## 🛠️ **MIGRATION PATTERNS**

### Pattern 1: Simple Port Replacement
**BEFORE**:
```rust
let endpoint = "http://localhost:9001";
```

**AFTER**:
```rust
use crate::config::network_defaults;

let port = std::env::var("NESTGATE_API_PORT")
    .ok()
    .and_then(|p| p.parse::<u16>().ok())
    .unwrap_or_else(network_defaults::api_port);
let endpoint = format!("http://localhost:{}", port);
```

### Pattern 2: Configuration Struct
**BEFORE**:
```rust
pub struct Config {
    pub endpoint: String, // "http://localhost:9001"
}
```

**AFTER**:
```rust
use crate::config::network_defaults;

pub struct Config {
    pub endpoint: String,
}

impl Default for Config {
    fn default() -> Self {
        let port = std::env::var("NESTGATE_API_PORT")
            .ok()
            .and_then(|p| p.parse::<u16>().ok())
            .unwrap_or_else(network_defaults::api_port);
        Self {
            endpoint: format!("http://localhost:{}", port),
        }
    }
}
```

### Pattern 3: Test Code (Acceptable but Improved)
**BEFORE**:
```rust
#[test]
fn test_connection() {
    let url = "http://localhost:9001";
    // test logic
}
```

**AFTER** (Optional Enhancement):
```rust
#[test]
fn test_connection() {
    use crate::config::network_defaults;
    
    let port = std::env::var("NESTGATE_TEST_API_PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(9001); // Test-specific default is acceptable
    let url = format!("http://localhost:{}", port);
    // test logic
}
```

### Pattern 4: Discovery Endpoints
**BEFORE**:
```rust
let endpoint = format!("http://{}:8080", host);
```

**AFTER**:
```rust
use crate::config::network_defaults;

let port = std::env::var("PRIMAL_DISCOVERY_PORT")
    .ok()
    .and_then(|p| p.parse::<u16>().ok())
    .unwrap_or_else(network_defaults::discovery_port);
let endpoint = format!("http://{}:{}", host, port);
```

---

## 📋 **ENVIRONMENT VARIABLES**

### Core Ports
```bash
# API Server
export NESTGATE_API_PORT=9001

# Health Check
export NESTGATE_HEALTH_PORT=9002

# Metrics
export NESTGATE_METRICS_PORT=9003

# Admin
export NESTGATE_ADMIN_PORT=9004

# Discovery
export NESTGATE_DISCOVERY_PORT=8080

# Primal Discovery (Dynamic per primal)
export PRIMAL_DISCOVERY_PORT=8080
export ORCHESTRATION_DISCOVERY_PORT=8080
export STORAGE_DISCOVERY_PORT=8081
export COMPUTE_DISCOVERY_PORT=8082
# ... etc

# Test Ports (Optional, for test flexibility)
export NESTGATE_TEST_API_PORT=19001
export NESTGATE_TEST_HEALTH_PORT=19002
```

### Fallback Chain
```
Environment Variable → network_defaults function → Compile-time constant
```

---

## 🎯 **SUCCESS CRITERIA**

### Metrics
- ✅ 0 hardcoded ports in production code
- ✅ <10% hardcoded ports in test code (where appropriate)
- ✅ All ports configurable via environment variables
- ✅ Full test suite passing
- ✅ Documentation updated

### Testing
- ✅ Can run tests on non-standard ports
- ✅ Can deploy to different environments without code changes
- ✅ Port conflicts automatically avoided
- ✅ Discovery works with custom ports

---

## 📊 **TRACKING**

### Week 1: Foundation ✅
- [x] Audit complete
- [x] Infrastructure in place
- [x] Migration plan documented

### Week 2: Critical Production Files
- [ ] API handlers migrated
- [ ] Universal adapter migrated
- [ ] Network core migrated
- [ ] Security config migrated

### Week 3: High Priority Files
- [ ] ZFS module migrated
- [ ] NAS module migrated
- [ ] MCP module migrated
- [ ] Automation module migrated

### Week 4: Medium Priority Files
- [ ] Configuration modules migrated
- [ ] Registry/discovery migrated
- [ ] Monitoring migrated

### Week 5-6: Test Code
- [ ] Integration tests migrated
- [ ] E2E tests migrated
- [ ] Example code migrated

### Week 7-8: Verification
- [ ] Full test suite passing
- [ ] Multi-environment testing complete
- [ ] Documentation updated
- [ ] Final audit passing

---

## 🚀 **IMMEDIATE NEXT STEPS**

1. **Now**: Start Phase 2, Week 2 - Critical Production Files
2. **Target**: `code/crates/nestgate-core/src/universal_adapter/discovery.rs`
3. **Goal**: Migrate first 20-30 hardcoded ports
4. **ETA**: 2-3 hours for first batch

---

## 📝 **NOTES**

- **Test Code**: Hardcoded ports in tests are acceptable for simplicity, but we're migrating them for consistency and flexibility.
- **Documentation**: Every migration should include inline comments explaining the environment variable usage.
- **Backward Compatibility**: All migrations maintain backward compatibility through sensible defaults.
- **Infant Discovery**: The infant discovery system already uses environment variables - we're extending this pattern everywhere.

---

**Reality > Hype. Truth > Marketing. Zero Hardcoded Assumptions.**

