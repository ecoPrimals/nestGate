# 🎯 ERROR HANDLING EVOLUTION STRATEGY

**Date**: December 14, 2025  
**Phase**: P1 - Error Handling Evolution  
**Approach**: Deep architectural evolution to idiomatic Rust  
**Timeline**: 4-6 weeks

---

## 📊 CURRENT STATE

**Audit Results**:
- **Total expects**: 1,951 instances
- **Total unwraps**: 1,599 instances
- **Total panic sites**: 3,550

**Distribution** (needs detailed audit):
- Production code: Unknown (needs separation)
- Test code: Unknown (acceptable in tests)
- Examples/docs: Unknown (acceptable)

---

## 🎯 EVOLUTION PHILOSOPHY

### ❌ **NOT Doing** (Shallow Approach)

```rust
// ❌ BAD: Just wrapping in if-let (band-aid)
let value = if let Some(v) = some_option {
    v
} else {
    return Err("Missing value".into());  // Stringly-typed!
};
```

### ✅ **Doing** (Deep Evolution)

```rust
// ✅ GOOD: Comprehensive error taxonomy
#[derive(Debug, thiserror::Error)]
pub enum NestGateError {
    #[error("Configuration value missing: {key}")]
    ConfigMissing { key: String },
    
    #[error("Invalid capability: {capability}")]
    InvalidCapability { capability: String },
    
    #[error("Service discovery failed: {reason}")]
    DiscoveryFailed { reason: String },
    
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

// Idiomatic usage with ?
let value = some_option
    .ok_or_else(|| NestGateError::ConfigMissing { 
        key: "storage_endpoint".into() 
    })?;
```

---

## 📋 EVOLUTION PHASES

### Phase 1.1: Audit & Categorize (Week 1)

**Goals**:
1. Separate production vs test expects/unwraps
2. Categorize by type:
   - Configuration errors
   - I/O errors
   - Network errors
   - Discovery errors
   - Storage errors
   - Parsing errors
3. Identify invariants (where `expect` is correct)

**Deliverable**: `ERROR_AUDIT_DEC_2025.md` with breakdown

### Phase 1.2: Build Error Taxonomy (Week 1-2)

**Create comprehensive error types**:

```rust
// nestgate-core/src/error/mod.rs

/// Root error type for NestGate
#[derive(Debug, thiserror::Error)]
pub enum NestGateError {
    // Configuration Errors
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),
    
    // Network Errors
    #[error("Network error: {0}")]
    Network(#[from] NetworkError),
    
    // Storage Errors
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),
    
    // Discovery Errors
    #[error("Discovery error: {0}")]
    Discovery(#[from] DiscoveryError),
    
    // I/O Errors
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    
    // Parsing Errors
    #[error("Parse error: {0}")]
    Parse(#[from] ParseError),
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Missing required configuration: {key}")]
    Missing { key: String },
    
    #[error("Invalid configuration value for {key}: {value}")]
    Invalid { key: String, value: String },
    
    #[error("Configuration file not found: {path}")]
    FileNotFound { path: String },
}

#[derive(Debug, thiserror::Error)]
pub enum DiscoveryError {
    #[error("No providers found for capability: {capability}")]
    NoProviders { capability: String },
    
    #[error("Discovery timeout after {timeout_ms}ms")]
    Timeout { timeout_ms: u64 },
    
    #[error("Invalid service endpoint: {endpoint}")]
    InvalidEndpoint { endpoint: String },
}

// ... more specific error types
```

**Deliverable**: `nestgate-core/src/error/taxonomy.rs`

### Phase 1.3: Evolve Production Code (Week 2-4)

**Priority order**:

1. **Critical paths** (startup, configuration)
2. **Public APIs** (external interfaces)
3. **Internal utilities** (helpers)
4. **Legacy code** (older modules)

**Evolution pattern**:

```rust
// ❌ BEFORE
pub fn load_config(path: &str) -> Config {
    let content = std::fs::read_to_string(path)
        .expect("Failed to read config");  // PANIC!
    
    toml::from_str(&content)
        .expect("Failed to parse config")  // PANIC!
}

// ✅ AFTER
pub fn load_config(path: &str) -> Result<Config> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| NestGateError::Config(ConfigError::FileNotFound {
            path: path.to_string(),
        }))?;
    
    toml::from_str(&content)
        .map_err(|e| NestGateError::Config(ConfigError::Invalid {
            key: "config_file".into(),
            value: format!("Parse error: {}", e),
        }))
}

// Usage (idiomatic!)
match load_config("config.toml") {
    Ok(config) => { /* use config */ },
    Err(e) => {
        log::error!("Configuration failed: {}", e);
        // Graceful degradation or exit
    }
}
```

### Phase 1.4: Evolve Error Contexts (Week 4-5)

**Add context to errors**:

```rust
use anyhow::Context;  // For development
use thiserror::Error; // For library

pub fn discover_storage() -> Result<StorageEndpoint> {
    let registry = ServiceRegistry::new()
        .context("Failed to initialize service registry")?;
    
    registry
        .discover(PrimalCapability::Storage)
        .await
        .context("Failed to discover storage service")?
        .first()
        .ok_or_else(|| DiscoveryError::NoProviders {
            capability: "Storage".into(),
        })
        .context("No storage providers available")
}

// Produces rich error chains:
// Error: No storage providers available
// Caused by:
//   0: Failed to discover storage service
//   1: Network timeout
```

### Phase 1.5: Document Invariants (Week 5-6)

**For remaining `expect` calls** (valid invariants):

```rust
/// SAFETY INVARIANT: Port 8080 is always valid (non-zero, in range)
/// This is a compile-time constant, so expect is justified.
const API_PORT: Port = match Port::new(8080) {
    Ok(p) => p,
    Err(_) => panic!("INVARIANT VIOLATED: 8080 is always valid"),
};

// Or better: const fn validation
const API_PORT: Port = Port::new_const(8080); // Can't fail
```

**Keep expects in**:
- Test code (acceptable)
- Compile-time constants (invariants)
- Unreachable code paths (with documentation)

---

## 🎯 SUCCESS METRICS

| Metric | Current | Week 2 | Week 4 | Week 6 | Target |
|--------|---------|--------|--------|--------|--------|
| Prod expects | 1,951 | 1,500 | 1,000 | 500 | 0 |
| Prod unwraps | 1,599 | 1,200 | 800 | 400 | 0 |
| Error types | 1 | 10 | 20 | 30+ | Comprehensive |
| Test expects | ? | Document | Accept | Accept | Keep |

---

## 📋 IMPLEMENTATION CHECKLIST

### Week 1: Audit & Design
- [ ] Separate production vs test expects/unwraps
- [ ] Categorize error types needed
- [ ] Design error taxonomy
- [ ] Create base error types
- [ ] Write migration guide

### Week 2-3: Critical Paths
- [ ] Evolve configuration loading
- [ ] Evolve service discovery
- [ ] Evolve network initialization
- [ ] Evolve storage initialization
- [ ] Add error contexts

### Week 4-5: Public APIs
- [ ] Evolve public API functions
- [ ] Evolve trait implementations
- [ ] Add comprehensive error docs
- [ ] Update examples

### Week 6: Polish & Document
- [ ] Document remaining invariants
- [ ] Add error handling tests
- [ ] Update documentation
- [ ] Verify 0 production unwraps

---

## 🎓 MIGRATION PATTERNS

### Pattern 1: Option → Result

```rust
// ❌ BEFORE
let value = map.get(key).expect("Key must exist");

// ✅ AFTER
let value = map.get(key)
    .ok_or_else(|| NestGateError::missing_key(key))?;
```

### Pattern 2: Nested Results

```rust
// ❌ BEFORE
let inner = outer.unwrap().inner.unwrap();

// ✅ AFTER
let inner = outer?
    .inner
    .ok_or_else(|| NestGateError::missing_field("inner"))?;
```

### Pattern 3: Custom Error Messages

```rust
// ❌ BEFORE
let port: u16 = s.parse().expect("Invalid port");

// ✅ AFTER
let port: u16 = s.parse()
    .map_err(|_| NestGateError::invalid_port(s))?;
```

### Pattern 4: Error Context

```rust
// ❌ BEFORE
let config = load_config(path).expect("Config failed");

// ✅ AFTER
let config = load_config(path)
    .with_context(|| format!("Loading config from {}", path))?;
```

---

## 🚀 TOOLS & HELPERS

### Error Builder Pattern

```rust
impl NestGateError {
    pub fn missing_config(key: impl Into<String>) -> Self {
        NestGateError::Config(ConfigError::Missing {
            key: key.into(),
        })
    }
    
    pub fn invalid_port(value: impl std::fmt::Display) -> Self {
        NestGateError::Parse(ParseError::InvalidPort {
            value: value.to_string(),
        })
    }
    
    // ... more builders
}
```

### Automated Detection

```bash
# Find all production expects
rg "\.expect\(" --type rust code/crates/*/src | grep -v tests

# Find all production unwraps
rg "\.unwrap\(" --type rust code/crates/*/src | grep -v tests
```

---

## 📊 PROGRESS TRACKING

**Updated weekly** in `ERROR_EVOLUTION_PROGRESS.md`

**Format**:
```markdown
## Week N Progress

**Expects evolved**: 150 → 120 (-30)
**Unwraps evolved**: 200 → 150 (-50)
**Error types created**: 5
**Modules completed**: config, discovery
**Blockers**: None
```

---

## 🎯 FINAL STATE

**After 6 weeks**:
- ✅ **0 production expects** (except documented invariants)
- ✅ **0 production unwraps** (except documented invariants)
- ✅ **Comprehensive error taxonomy** (30+ error types)
- ✅ **Rich error contexts** (anyhow chains)
- ✅ **Full documentation** (error handling guide)
- ✅ **Test coverage** (error path tests)

**Result**: **A+ grade error handling** (idiomatic, safe, informative)

---

**Status**: 📋 **READY TO EXECUTE**  
**Next**: Begin Week 1 audit

**This is not cosmetic. This is deep, idiomatic Rust evolution.** 🚀

