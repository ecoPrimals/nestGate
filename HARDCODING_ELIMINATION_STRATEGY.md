# Hardcoding Elimination Strategy

**Date**: January 16, 2026  
**Phase**: Phase 1 - Capability-Based Evolution  
**Priority**: 🔥 CRITICAL  
**Target**: Eliminate 2,300 hardcoded instances

---

## 🎯 **Problem Statement**

**Current State**:
- **2,300 hardcoded instances** of IPs, URLs, and ports
- Violates TRUE PRIMAL self-knowledge principle
- Blocks capability-based discovery adoption
- Creates brittle inter-primal dependencies

**Patterns Found**:
- `127.0.0.1` / `localhost` / `0.0.0.0`
- Hardcoded ports (`:8080`, `:3000`, etc.)
- Primal endpoint assumptions
- Test fixture hardcoding

---

## 📊 **Hardcoding Analysis**

### **High-Impact Files** (Production Code)

| File | Instances | Type | Priority |
|------|-----------|------|----------|
| `config/network_defaults.rs` | 43 | Config defaults | 🔥 P1 |
| `constants/consolidated.rs` | 29 | Constants | 🔥 P1 |
| `utils/network.rs` | 23 | Network utils | 🔥 P1 |
| `constants/network_smart.rs` | 19 | Network constants | 🔥 P1 |
| `config/external/network.rs` | 19 | External config | 🔥 P1 |
| `capabilities/discovery/registry.rs` | 17 | Discovery | ⚠️ P2 |
| `primal_discovery/migration.rs` | 16 | Migration | ⚠️ P2 |
| `config/defaults.rs` | 15 | Config defaults | ⚠️ P2 |
| `config/discovery_config.rs` | 15 | Discovery config | ⚠️ P2 |
| `capabilities/discovery/resolver.rs` | 15 | Discovery | ⚠️ P2 |

**Total in Top 10**: ~210 instances (~9% of total)

### **Categories**

**1. Configuration Defaults** (~600 instances):
- Default endpoints
- Fallback addresses
- Port assignments

**2. Constants** (~400 instances):
- Network constants
- Service URLs
- Port numbers

**3. Test Fixtures** (~800 instances):
- Test server addresses
- Mock endpoints
- Test data

**4. Discovery/Registry** (~300 instances):
- Primal endpoint assumptions
- Discovery fallbacks
- Registry defaults

**5. Miscellaneous** (~200 instances):
- Documentation examples
- Comments
- Debug logging

---

## 🎯 **Evolution Strategy**

### **Pattern: Before → After**

#### **Configuration Defaults**

**Before** (Hardcoded):
```rust
pub const DEFAULT_API_ENDPOINT: &str = "http://localhost:8080";
pub const BEARDOG_ENDPOINT: &str = "http://localhost:8001";
```

**After** (Discovery-based):
```rust
pub fn default_api_endpoint() -> Result<String> {
    capability_discovery::discover_with_fallback(
        "api",
        "NESTGATE_API_ENDPOINT",
        "http://127.0.0.1:8080"  // Last resort default
    ).await.map(|e| e.endpoint)
}

pub async fn beardog_endpoint() -> Result<String> {
    capability_discovery::discover_service("security")
        .await
        .map(|e| e.endpoint)
}
```

#### **Constants**

**Before** (Hardcoded):
```rust
pub const ORCHESTRATOR_URL: &str = "http://localhost:9000";
```

**After** (Runtime Discovery):
```rust
// No constant! Use discovery:
let orchestrator = discovery.discover_capability("orchestration").await?;
let url = orchestrator.primary_endpoint().ok_or(...)?;
```

#### **Test Fixtures**

**Before** (Hardcoded):
```rust
#[tokio::test]
async fn test_connection() {
    let url = "http://localhost:8080";
    // ...
}
```

**After** (Dynamic):
```rust
#[tokio::test]
async fn test_connection() {
    let server = test_server().await;  // Spawns on random port
    let url = server.url();
    // ...
}
```

---

## 📋 **Execution Plan**

### **Phase 1.1: High-Priority Config Files** (2-3 hours)

**Files** (Top 5, ~150 instances):
1. `config/network_defaults.rs` (43)
2. `constants/consolidated.rs` (29)
3. `utils/network.rs` (23)
4. `constants/network_smart.rs` (19)
5. `config/external/network.rs` (19)

**Actions**:
- Replace hardcoded constants with discovery functions
- Add environment variable fallbacks
- Update callers to use `async` discovery
- Add proper error handling

**Expected**: -150 hardcoded instances

---

### **Phase 1.2: Discovery & Registry** (1-2 hours)

**Files** (~50 instances):
- `capabilities/discovery/registry.rs` (17)
- `config/discovery_config.rs` (15)
- `capabilities/discovery/resolver.rs` (15)

**Actions**:
- Integrate with discovery_mechanism.rs
- Remove primal endpoint assumptions
- Use capability queries

**Expected**: -50 hardcoded instances

---

### **Phase 1.3: Configuration Module Cleanup** (2-3 hours)

**Files** (~100 instances):
- `config/defaults.rs`
- `config/canonical_defaults.rs`
- `config/sovereignty_helpers_config.rs`
- Various config files

**Actions**:
- Convert static defaults to discovery-based
- Add capability-based configuration
- Environment variable integration

**Expected**: -100 hardcoded instances

---

### **Phase 1.4: Test Fixture Evolution** (3-4 hours)

**Target**: ~800 test-related instances

**Strategy**:
- Create test helpers for dynamic ports
- Use `portpicker` for random ports
- Environment-based test configuration

**Pattern**:
```rust
// Helper
pub fn spawn_test_server() -> TestServer {
    let port = portpicker::pick_unused_port().expect("No ports free");
    TestServer::new(port)
}
```

**Expected**: -800 test instances (acceptable in tests, but use helpers)

---

### **Phase 1.5: Final Cleanup** (1-2 hours)

**Remaining**: ~200 instances

**Actions**:
- Documentation updates
- Comment cleanup
- Edge case handling

**Expected**: Total reduction to <100 instances (only last-resort defaults)

---

## 🎯 **Success Metrics**

### **Quantitative**

**Before**:
- Hardcoded instances: 2,300
- Discovery usage: ~10%
- Environment awareness: ~20%

**Target**:
- Hardcoded instances: <100 (only last-resort defaults)
- Discovery usage: ~90%
- Environment awareness: 100%

**Reduction**: ~96% (2,300 → <100)

---

### **Qualitative**

**TRUE PRIMAL Compliance**:
- ✅ Primals have only self-knowledge
- ✅ Runtime discovery of other primals
- ✅ No compile-time dependencies
- ✅ Capability-based queries

**Benefits**:
- ✅ Flexible deployment (bare metal → cloud → k8s)
- ✅ Easy testing (no port conflicts)
- ✅ Primal independence
- ✅ Configuration simplicity

---

## 📊 **Timeline**

| Phase | Effort | Impact | Status |
|-------|--------|--------|--------|
| **1.1**: High-priority config | 2-3 hrs | -150 | 🎯 Next |
| **1.2**: Discovery/registry | 1-2 hrs | -50 | ⏳ Pending |
| **1.3**: Config cleanup | 2-3 hrs | -100 | ⏳ Pending |
| **1.4**: Test fixtures | 3-4 hrs | -800 | ⏳ Pending |
| **1.5**: Final cleanup | 1-2 hrs | -200 | ⏳ Pending |
| **TOTAL** | **9-14 hrs** | **-2,200** | **Phase 1** |

**Expected Result**: <100 hardcoded instances (96% reduction!)

---

## 🎯 **Immediate Next Steps**

### **Start with Phase 1.1** (Now)

**Target File**: `config/network_defaults.rs` (43 instances)

**Actions**:
1. Read file and analyze hardcoding patterns
2. Create discovery-based replacement functions
3. Update all callers
4. Test compilation
5. Commit progress

**Expected Time**: 30-45 minutes  
**Impact**: -43 instances (2% of total)

---

## 🏆 **Expected Grade Impact**

**Before Hardcoding Elimination**:
- Hardcoding: 50/100
- Primal Self-Knowledge: 70/100
- Overall: A (94/100)

**After Hardcoding Elimination**:
- Hardcoding: 50 → 95 (+45 points!)
- Primal Self-Knowledge: 70 → 95 (+25 points!)
- Overall: A (94) → A (96) [+2 points]

**Philosophy**: TRUE PRIMAL compliance achieved! ✅

---

**Created**: January 16, 2026  
**Purpose**: Strategic plan for eliminating 2,300 hardcoded instances  
**Effort**: 9-14 hours total  
**Target**: <100 instances (96% reduction)  
**Impact**: TRUE PRIMAL philosophy compliance

---

**"From hardcoded to discovered - sovereign primals discover each other at runtime!"** 🌱🦀✨
