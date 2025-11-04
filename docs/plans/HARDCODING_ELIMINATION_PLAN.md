# Hardcoding Elimination Plan

**Created:** November 1, 2025  
**Status:** 🔄 **IN PROGRESS**  
**Priority:** **HIGH**

---

## 📊 **Current State (Verified)**

### Hardcoded Values Found
| Type | Count | Files | Priority |
|------|-------|-------|----------|
| **localhost/127.0.0.1** | 356 | 118 files | HIGH |
| **0.0.0.0 (bind all)** | 64 | 36 files | MEDIUM |
| **Port numbers** | 221+ | 70 files | HIGH |
| **TOTAL** | **641+** | **150+ files** | HIGH |

---

## 🎯 **Objectives**

1. ✅ **Identify all hardcoded values** (COMPLETE)
2. 🔄 **Create constants module** (IN PROGRESS)
3. 🔄 **Extract to configuration**
4. 🔄 **Replace hardcoded values**
5. 🔄 **Add environment variable support**
6. ✅ **Maintain backward compatibility**

---

## 📋 **Strategy**

### Phase 1: Foundation (Week 1)
1. Create centralized constants module
2. Add environment variable support
3. Update configuration system
4. Document new patterns

### Phase 2: High-Impact Replacements (Week 2)
1. Replace hardcoded IPs in network configs
2. Replace hardcoded ports in API servers
3. Update test fixtures to use constants
4. Verify all tests still pass

### Phase 3: Comprehensive Cleanup (Week 3)
1. Replace remaining hardcoded values
2. Add configuration validation
3. Update documentation
4. Final verification

---

## 📁 **File Categories**

### Critical (Immediate Action)
- `nestgate-api/src/bin/nestgate-api-server.rs` - Server bindings
- `nestgate-core/src/config/network_defaults.rs` - Network config (30 instances)
- `nestgate-core/src/config/runtime_config.rs` - Runtime config (13 instances)
- `nestgate-core/src/defaults.rs` - Default values (10 instances)

### High Priority
- Network configuration files (50+ instances)
- API handlers (20+ instances)
- Service discovery (25+ instances)

### Medium Priority
- Test files (200+ instances) - Can use test constants
- Documentation examples
- Disabled/archived code

---

## 🔧 **Implementation Plan**

### Step 1: Create Constants Module ✅

```rust
// code/crates/nestgate-core/src/constants/network_hardcoded.rs

/// Default network addresses - extracted from hardcoded values
pub mod addresses {
    pub const LOCALHOST_IPV4: &str = "127.0.0.1";
    pub const LOCALHOST_NAME: &str = "localhost";
    pub const BIND_ALL_IPV4: &str = "0.0.0.0";
    pub const BIND_ALL_IPV6: &str = "::";
}

/// Default network ports - extracted from hardcoded values
pub mod ports {
    pub const HTTP_DEFAULT: u16 = 8080;
    pub const HTTPS_DEFAULT: u16 = 8443;
    pub const API_DEFAULT: u16 = 3000;
    pub const METRICS_DEFAULT: u16 = 9090;
    pub const HEALTH_CHECK_DEFAULT: u16 = 8081;
}
```

### Step 2: Environment Variable Support

```rust
use std::env;

pub fn get_api_bind_address() -> String {
    env::var("NESTGATE_BIND_ADDRESS")
        .unwrap_or_else(|_| addresses::BIND_ALL_IPV4.to_string())
}

pub fn get_api_port() -> u16 {
    env::var("NESTGATE_API_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(ports::API_DEFAULT)
}
```

### Step 3: Configuration Integration

```toml
# config/production.toml
[network]
bind_address = "0.0.0.0"  # Override with NESTGATE_BIND_ADDRESS
api_port = 8080            # Override with NESTGATE_API_PORT
```

---

## 🎯 **High-Impact Targets**

### Top 10 Files to Fix First

1. **`nestgate-core/src/config/network_defaults.rs`** (30 instances)
   - Impact: Affects all network configuration
   - Strategy: Extract to constants, add env var support

2. **`nestgate-core/src/config/runtime_config.rs`** (13 instances)
   - Impact: Runtime configuration defaults
   - Strategy: Use new constants module

3. **`nestgate-core/src/defaults.rs`** (10 instances)
   - Impact: System-wide defaults
   - Strategy: Centralize in constants

4. **`nestgate-api/src/bin/nestgate-api-server.rs`** (8 instances)
   - Impact: API server startup
   - Strategy: Use configuration + env vars

5. **`nestgate-bin/tests/integration_tests.rs`** (8 instances)
   - Impact: Integration tests
   - Strategy: Use test constants

6. **`nestgate-core/src/universal_adapter/discovery.rs`** (23 instances)
   - Impact: Service discovery
   - Strategy: Configuration-based discovery

7. **`nestgate-core/src/service_discovery/dynamic_endpoints.rs`** (8 instances)
   - Impact: Dynamic endpoint resolution
   - Strategy: Use discovery configuration

8. **`nestgate-core/src/capabilities/taxonomy/capability.rs`** (27 instances)
   - Impact: Capability scanning
   - Strategy: Configurable scan ranges

9. **`nestgate-network/tests/*.rs`** (20+ instances)
   - Impact: Network tests
   - Strategy: Test constants module

10. **`nestgate-api/src/handlers/`** (15+ instances)
    - Impact: API handlers
    - Strategy: Use service configuration

---

## ⚠️ **Special Considerations**

### Tests
- **Strategy:** Create `tests/constants.rs` with test-specific values
- **Keep:** Test hardcoding is acceptable for clarity
- **Priority:** LOWER (tests should be readable)

### Localhost vs 0.0.0.0
- **localhost/127.0.0.1:** Client connections, local-only
- **0.0.0.0:** Server bindings, accept all interfaces
- **Strategy:** Different constants for each use case

### Port Ranges
- Common patterns found:
  - 3000-3999: API services
  - 8000-8999: Web servers
  - 9000-9999: Metrics/monitoring
  - 5000-5999: Custom services

---

## 📝 **Migration Checklist**

### Constants Module
- [ ] Create `constants/network_hardcoded.rs`
- [ ] Define address constants
- [ ] Define port constants
- [ ] Export from `constants/mod.rs`
- [ ] Add documentation

### Environment Variables
- [ ] Define standard env var names
- [ ] Create helper functions
- [ ] Document in deployment guide
- [ ] Add validation

### Configuration Files
- [ ] Update `config/production.toml`
- [ ] Update `config/*.toml` templates
- [ ] Add configuration examples
- [ ] Document override mechanism

### Code Updates
- [ ] Update network_defaults.rs
- [ ] Update runtime_config.rs
- [ ] Update defaults.rs
- [ ] Update API server bindings
- [ ] Update service discovery
- [ ] Update handlers

### Testing
- [ ] Run full test suite
- [ ] Verify configuration loading
- [ ] Test environment variable overrides
- [ ] Integration test verification

### Documentation
- [ ] Update deployment guide
- [ ] Update configuration guide
- [ ] Add environment variable reference
- [ ] Update examples

---

## 🚀 **Quick Wins**

### Immediate Actions (1-2 hours)
1. Create constants module
2. Update `network_defaults.rs` (30 instances)
3. Update `runtime_config.rs` (13 instances)
4. Update `defaults.rs` (10 instances)

**Impact:** ~50 instances replaced (8% of total)

### Week 1 Target
- Replace all production code hardcoding
- Leave test hardcoding for now
- ~400 instances replaced (62% of total)

---

## 📊 **Progress Tracking**

### Current Status
- [x] Identified hardcoded values (641+ instances)
- [x] Analyzed file distribution
- [x] Created elimination plan
- [ ] Created constants module
- [ ] Replaced high-impact instances
- [ ] Added environment variable support
- [ ] Updated configuration system
- [ ] Verified tests pass

### Metrics
- **Target:** 641+ instances
- **Replaced:** 0
- **Remaining:** 641+
- **Progress:** 0%

---

## 🎯 **Success Criteria**

1. ✅ **No hardcoded IPs in production code**
2. ✅ **No hardcoded ports in production code**
3. ✅ **All values configurable via environment variables**
4. ✅ **All tests passing**
5. ✅ **Documentation updated**
6. ✅ **Backward compatibility maintained**

---

## 💡 **Benefits**

### Security
- No hardcoded addresses in binary
- Environment-specific configuration
- Easier secret management

### Flexibility
- Deploy to any environment
- Dynamic port assignment
- Container-friendly

### Maintainability
- Single source of truth
- Easy to update
- Clear configuration

---

## 📅 **Timeline**

| Phase | Duration | Completion |
|-------|----------|------------|
| Phase 1: Foundation | 2-3 days | Week 1 |
| Phase 2: High-Impact | 3-4 days | Week 2 |
| Phase 3: Comprehensive | 3-4 days | Week 3 |
| **TOTAL** | **2-3 weeks** | **Week 3** |

---

## ⚠️ **Risks & Mitigation**

### Risk 1: Breaking Changes
- **Mitigation:** Extensive testing, backward compatibility
- **Impact:** Medium
- **Probability:** Low

### Risk 2: Test Failures
- **Mitigation:** Update test constants, careful review
- **Impact:** Medium
- **Probability:** Medium

### Risk 3: Configuration Complexity
- **Mitigation:** Clear documentation, sensible defaults
- **Impact:** Low
- **Probability:** Low

---

## 📚 **Resources**

- **Configuration Guide:** `docs/development/configuration.md`
- **Environment Variables:** `docs/deployment/environment-variables.md`
- **Constants Module:** `code/crates/nestgate-core/src/constants/`

---

**Status:** 🔄 IN PROGRESS  
**Priority:** HIGH  
**Next Action:** Create constants module and start high-impact replacements

---

*Created: November 1, 2025*  
*NestGate Hardcoding Elimination Initiative*

