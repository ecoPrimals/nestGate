# 🎯 HARDCODING MIGRATION - PHASE 1 EXECUTION PLAN
## Week 2 Active Migration - Smart Pattern-Based Evolution

**Date**: December 14, 2025  
**Status**: In Progress  
**Target**: 10-15 Quick Wins to establish patterns

---

## 📊 CURRENT STATE ANALYSIS

### Constants Module Status: ✅ EXCELLENT FOUNDATION

**Discovery**: The constants module already has significant infrastructure in place!

**Existing Capabilities** (`code/crates/nestgate-core/src/constants/`):
1. ✅ **Port defaults centralized** (`port_defaults.rs`, `ports.rs`)
2. ✅ **Environment variable support** (helper functions with `get_*_port()`)
3. ✅ **Migration documentation** (README.md with patterns)
4. ✅ **Consolidated constants** (`consolidated.rs` - NetworkConstants struct)
5. ✅ **Smart network helpers** (`network_smart.rs`)

**Key Finding**: Infrastructure is ready - just need to **use it consistently** across codebase!

---

## 🎯 MIGRATION STRATEGY

### Phase 1: Low-Hanging Fruit (This Session)

**Target Files**: Find uses of hardcoded ports that should use constants

**Pattern**:
```rust
// ❌ OLD: Hardcoded inline
let port = 8080;
let addr = format!("127.0.0.1:{}", 8080);

// ✅ NEW: Use centralized constant
use nestgate_core::constants::port_defaults::DEFAULT_API_PORT;
let port = DEFAULT_API_PORT;

// ✅ BEST: Use environment-aware helper
use nestgate_core::constants::network_hardcoded;
let port = network_hardcoded::get_api_port(); // Checks env first
```

### Phase 2: Environment Configuration (Week 2)

**Pattern**:
```rust
// ✅ MODERN: Use EnvironmentConfig
use nestgate_core::config::environment::EnvironmentConfig;

let config = EnvironmentConfig::from_env()?;
let port = config.network.port.get();
```

### Phase 3: Capability-Based (Week 3-4)

**Pattern**:
```rust
// ✅ IDEAL: Capability-based discovery  
let registry = ServiceRegistry::new(vec![PrimalCapability::API]).await?;
let service = registry.discover(PrimalCapability::API).await?;
let endpoint = service.endpoint(); // Port discovered, not hardcoded!
```

---

## 📋 PHASE 1 TARGETS (10-15 Quick Wins)

### Category A: Test Files (Low Risk, High Value)

**Target**: Replace hardcoded ports in test setup with constants

**Files to Check**:
- `code/crates/nestgate-core/tests/*.rs`
- `code/crates/nestgate-api/tests/*.rs`
- `code/crates/nestgate-network/tests/*.rs`

**Pattern**:
```rust
// ❌ BEFORE
#[tokio::test]
async fn test_api_connection() {
    let addr = "127.0.0.1:8080".parse().unwrap();
    // ...
}

// ✅ AFTER
use nestgate_core::constants::port_defaults::DEFAULT_API_PORT;

#[tokio::test]
async fn test_api_connection() {
    let addr = format!("127.0.0.1:{}", DEFAULT_API_PORT).parse().unwrap();
    // ...
}
```

**Impact**: Low risk (tests only), establishes pattern, easy wins

### Category B: Config Defaults (Medium Risk, High Value)

**Target**: Ensure config modules use constants, not inline literals

**Files to Check**:
- `code/crates/nestgate-core/src/config/*.rs`
- `code/crates/nestgate-core/src/config/runtime/*.rs`

**Pattern**:
```rust
// ❌ BEFORE
impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            port: 8080, // Hardcoded!
        }
    }
}

// ✅ AFTER
use crate::constants::port_defaults::DEFAULT_API_PORT;

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            port: DEFAULT_API_PORT, // From constants!
        }
    }
}
```

### Category C: Example Code (Low Risk, Educational Value)

**Target**: Update examples to show best practices

**Files**:
- `examples/*.rs`
- `code/crates/nestgate-core/src/constants/migration_example.rs`

**Pattern**: Demonstrate modern approach in all examples

---

## 🚀 EXECUTION PLAN

### Step 1: Quick Audit (15 min)
```bash
# Find inline port literals in tests
rg ":\s*8080|:\s*9090|:\s*3000" code/crates/*/tests/ --type rust

# Find inline port literals in config defaults
rg "port:\s*[0-9]{4}" code/crates/nestgate-core/src/config/ --type rust
```

### Step 2: Replace 5-10 Test Instances (20 min)
- Pick easiest test files
- Replace hardcoded ports with constants
- Verify tests still pass

### Step 3: Replace 3-5 Config Defaults (15 min)
- Update Default impl blocks
- Use port_defaults constants
- Document pattern

### Step 4: Update 2-3 Examples (10 min)
- Show modern best practices
- Include comments explaining why

### Step 5: Verify & Document (10 min)
- Run tests
- Build check
- Document patterns for team

**Total Time**: ~70 minutes for 10-15 migrations

---

## 📊 SUCCESS METRICS

### Immediate (This Session)
- [ ] 10-15 hardcoded values replaced
- [ ] Pattern documented and proven
- [ ] Tests passing
- [ ] Examples updated

### Week 2-4 Progress Tracking
```
Total hardcoded values: 916
Phase 1 target:         15 (1.6%)
Week 2 target:          100 (11%)  
Week 3 target:          250 (27%)
Week 4 target:          458 (50%) ✅
```

---

## 💡 MIGRATION INSIGHTS

### What We Learned

1. **Infrastructure Already Exists** ✅
   - Constants module well-organized
   - Helper functions ready
   - Documentation in place

2. **Need Consistent Usage**
   - Some code uses constants
   - Some code still hardcodes
   - Pattern not enforced

3. **Three-Tier Approach**
   - Tier 1: Constants (immediate fix)
   - Tier 2: Environment config (Week 2)
   - Tier 3: Capability discovery (Week 3-4)

### Best Practices

**DO**:
- ✅ Use centralized constants from `port_defaults`
- ✅ Use environment-aware helpers when available
- ✅ Document why in comments
- ✅ Test after each change

**DON'T**:
- ❌ Create new magic numbers
- ❌ Duplicate constants in multiple places
- ❌ Skip environment variable support
- ❌ Break existing tests

---

## 🔍 DETAILED MIGRATION GUIDE

### For Test Files

**Pattern 1: Simple port literal**
```rust
// ❌ BEFORE
let url = "http://localhost:8080";

// ✅ AFTER
use nestgate_core::constants::port_defaults::DEFAULT_API_PORT;
let url = format!("http://localhost:{}", DEFAULT_API_PORT);
```

**Pattern 2: Socket address**
```rust
// ❌ BEFORE
let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();

// ✅ AFTER
use nestgate_core::constants::port_defaults::DEFAULT_API_PORT;
let addr: SocketAddr = format!("127.0.0.1:{}", DEFAULT_API_PORT).parse().unwrap();
```

### For Config Modules

**Pattern 1: Default impl**
```rust
// ❌ BEFORE
impl Default for MyConfig {
    fn default() -> Self {
        Self {
            api_port: 8080,
            metrics_port: 9090,
        }
    }
}

// ✅ AFTER
use crate::constants::port_defaults::{DEFAULT_API_PORT, DEFAULT_METRICS_PORT};

impl Default for MyConfig {
    fn default() -> Self {
        Self {
            api_port: DEFAULT_API_PORT,
            metrics_port: DEFAULT_METRICS_PORT,
        }
    }
}
```

**Pattern 2: Builder pattern**
```rust
// ❌ BEFORE
pub fn with_port(mut self, port: u16) -> Self {
    self.port = port;
    self
}

// DEFAULT_API_PORT
impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            port: DEFAULT_API_PORT, // Use constant as default
            // ...
        }
    }
}
```

### For Production Code

**Pattern: Use environment-aware helper**
```rust
// ✅ BEST PRACTICE
use nestgate_core::constants::network_hardcoded;

// Gets from environment, falls back to default
let api_port = network_hardcoded::get_api_port();
let metrics_port = network_hardcoded::get_metrics_port();
let health_port = network_hardcoded::get_health_port();
```

---

## 📈 PROGRESS TRACKING

### Migration Counter

```
Session Start:  916 hardcoded values
After Phase 1:  901 (-15, 1.6% complete)
Week 2 Target:  816 (-100, 11% complete)
Week 3 Target:  666 (-250, 27% complete)
Week 4 Target:  458 (-458, 50% complete) ✅
```

### Quality Metrics

**Before**:
- Hardcoded ports: ~322 instances
- Hardcoded IPs: ~594 instances
- Environment support: Partial

**After Phase 1** (Target):
- Using constants: +15 instances
- Pattern documented: ✅
- Test coverage: Maintained
- Build: Green ✅

---

## 🎯 NEXT ACTIONS

### Immediate (This Session)
1. Run grep to find easy targets
2. Replace 5-10 in tests
3. Replace 3-5 in config
4. Update 2-3 examples
5. Verify and document

### Week 2
1. Systematic test file migration
2. All config defaults using constants
3. Examples showcase best practices
4. 100 migrations complete

### Week 3-4
1. Production code uses environment config
2. API handlers capability-aware
3. Network code uses discovery
4. 458 migrations complete (50%)

---

**Status**: Ready to Execute  
**Next**: Run grep audit and identify first 15 targets  
**Timeline**: 70 minutes for Phase 1  
**Confidence**: HIGH - Infrastructure ready, pattern clear

🚀 **Let's systematically evolve from hardcoding to capability-based discovery!**

