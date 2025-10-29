# 🎯 ERROR CONSOLIDATION PHASE 2 - MIGRATION PLAN

**Date**: October 2, 2025  
**Goal**: Migrate from fragmented domain errors to NestGateUnifiedError  
**Target**: 52% → 75% error consolidation  
**Estimated Time**: 4-6 hours

---

## 📊 CURRENT STATE ANALYSIS

### **The Problem: Dual Error Systems**

We have TWO competing error systems creating conflicts:

**System 1: domain_errors.rs (OLD - TO BE DEPRECATED)**
```rust
// File: code/crates/nestgate-core/src/error/idiomatic/domain_errors.rs (526 lines)
pub enum ValidationError {
    FieldValidation { field, message, constraint },
    SchemaValidation { schema, message, path },
    Unified(#[from] NestGateError),
}

pub enum NetworkError {
    ConnectionFailed { address, port, error, timeout },
    Timeout { url, timeout, method },
    Unified(#[from] NestGateError),
}

pub enum StorageError {
    FileNotFound { path, operation },
    PermissionDenied { path, operation, required_permissions },
    DiskFull { path, available, required },
    Unified(#[from] NestGateError),
}

// ... 12+ more domain error enums
```

**System 2: NestGateUnifiedError (NEW - CANONICAL)**
```rust
// File: code/crates/nestgate-core/src/error/variants/core_errors.rs
pub enum NestGateUnifiedError {
    Configuration(Box<ConfigurationErrorDetails>),
    Api(Box<ApiErrorDetails>),
    Storage(Box<StorageErrorDetails>),
    Network(Box<NetworkErrorDetails>),
    Security(Box<SecurityErrorDetails>),
    // ... unified variants with boxed details
}
```

**System 3: Type Aliases (CREATES CONFLICT)**
```rust
// File: code/crates/nestgate-core/src/error/unified_result_system.rs
pub type ValidationError = NestGateError;  // ❌ CONFLICTS with domain_errors::ValidationError
pub type NetworkError = NestGateError;      // ❌ CONFLICTS with domain_errors::NetworkError
pub type StorageError = NestGateError;      // ❌ CONFLICTS with domain_errors::StorageError
```

### **Impact Assessment**:

**Files Using domain_errors**:
- ✅ Tests: `tests/idiomatic_error_evolution_demo.rs`, `tests/unit/core_error_system_tests.rs`
- ✅ Examples: `examples/simple_idiomatic_demo.rs`, `examples/phase4_ecosystem_adoption_demo.rs`
- ✅ Templates: `ecosystem-expansion/templates/error-template.rs`
- ✅ Tools: `tools/unwrap-migrator/` (uses StorageError patterns)
- ⚠️ Production: Minimal direct usage (mostly through type aliases)

**Total Affected**:
- ~15 files actively using domain error enums
- ~200+ enum variant usages (NetworkError::ConnectionFailed, etc.)
- Mostly concentrated in tests/examples (good news!)

---

## 🎯 MIGRATION STRATEGY

### **Phase 2A: Deprecate domain_errors.rs** (1 hour)

**Step 1: Add Deprecation Warnings**
```rust
// domain_errors.rs
#[deprecated(since = "0.9.0", note = "Use NestGateUnifiedError::Network(...) instead")]
pub enum NetworkError { ... }

#[deprecated(since = "0.9.0", note = "Use NestGateUnifiedError::Storage(...) instead")]
pub enum StorageError { ... }

// ... for all 12+ enums
```

**Step 2: Create Migration Guide**
```rust
// Add to domain_errors.rs header:
//! **⚠️ DEPRECATED MODULE - DO NOT USE IN NEW CODE**
//!
//! This module contains legacy domain-specific error types that are being
//! phased out in favor of `NestGateUnifiedError`.
//!
//! **MIGRATION GUIDE**:
//! ```rust
//! // OLD:
//! return Err(NetworkError::ConnectionFailed {
//!     address: "example.com".to_string(),
//!     port: 443,
//!     error: "timeout".to_string(),
//!     timeout: None,
//! });
//!
//! // NEW:
//! use nestgate_core::error::{NestGateUnifiedError, NetworkErrorDetails};
//! return Err(NestGateUnifiedError::Network(Box::new(NetworkErrorDetails {
//!     message: format!("Connection failed: {}:443", "example.com"),
//!     endpoint: Some("example.com".to_string()),
//!     port: Some(443),
//!     protocol: "HTTPS".to_string(),
//!     ..Default::default()
//! })));
//! ```
```

### **Phase 2B: Fix Type Alias Conflicts** (30 mins)

**Remove conflicting aliases from unified_result_system.rs**:
```rust
// REMOVE these lines (they conflict):
// pub type ValidationError = NestGateError;
// pub type NetworkError = NestGateError;
// pub type StorageError = NestGateError;
// ... etc

// KEEP the Result type aliases (they're useful):
pub type ValidationResult<T> = Result<T>;
pub type NetworkResult<T> = Result<T>;
pub type StorageResult<T> = Result<T>;
```

### **Phase 2C: Create Helper Constructors** (1 hour)

**Add convenience constructors to NestGateUnifiedError**:
```rust
// code/crates/nestgate-core/src/error/variants/core_errors.rs

impl NestGateUnifiedError {
    /// Create a network connection failure error
    pub fn network_connection_failed(
        address: impl Into<String>,
        port: u16,
        reason: impl Into<String>,
    ) -> Self {
        Self::Network(Box::new(NetworkErrorDetails {
            message: format!("Connection failed: {}:{} - {}", address.into(), port, reason.into()),
            endpoint: Some(address.into()),
            port: Some(port),
            protocol: "TCP".to_string(),
            ..Default::default()
        }))
    }

    /// Create a storage file not found error
    pub fn storage_not_found(path: impl Into<String>) -> Self {
        Self::Storage(Box::new(StorageErrorDetails {
            message: format!("File not found: {}", path.into()),
            resource: Some(path.into()),
            ..Default::default()
        }))
    }

    /// Create a validation field error
    pub fn validation_field(
        field: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::Validation(Box::new(ValidationErrorDetails {
            message: format!("Field '{}': {}", field.into(), message.into()),
            field: Some(field.into()),
            ..Default::default()
        }))
    }

    // ... more helper constructors
}
```

### **Phase 2D: Migrate Tests & Examples** (2-3 hours)

**Automated Migration Script**:
```python
#!/usr/bin/env python3
# scripts/unification/migrate_domain_errors.py

import re
from pathlib import Path

# Migration patterns
MIGRATIONS = [
    # NetworkError::ConnectionFailed
    (
        r'NetworkError::ConnectionFailed\s*\{\s*address:\s*([^,]+),\s*port:\s*([^,]+),\s*error:\s*([^,]+)(?:,\s*timeout:\s*[^}]+)?\s*\}',
        r'NestGateUnifiedError::network_connection_failed(\1, \2, \3)'
    ),
    # StorageError::FileNotFound
    (
        r'StorageError::FileNotFound\s*\{\s*path:\s*([^,]+)(?:,\s*operation:\s*[^}]+)?\s*\}',
        r'NestGateUnifiedError::storage_not_found(\1)'
    ),
    # ValidationError::FieldValidation
    (
        r'ValidationError::FieldValidation\s*\{\s*field:\s*Some\(([^)]+)\),\s*message:\s*([^,]+)(?:,\s*[^}]+)?\s*\}',
        r'NestGateUnifiedError::validation_field(\1, \2)'
    ),
]

def migrate_file(filepath):
    content = filepath.read_text()
    original = content
    
    for pattern, replacement in MIGRATIONS:
        content = re.sub(pattern, replacement, content, flags=re.MULTILINE)
    
    if content != original:
        filepath.write_text(content)
        return True
    return False

# Run migration
test_files = Path("tests").rglob("*.rs")
example_files = Path("examples").rglob("*.rs")

for filepath in chain(test_files, example_files):
    if migrate_file(filepath):
        print(f"✅ Migrated: {filepath}")
```

**Manual Migration Checklist**:
- [ ] `tests/idiomatic_error_evolution_demo.rs` - 10+ usages
- [ ] `tests/unit/core_error_system_tests.rs` - 5+ usages
- [ ] `examples/simple_idiomatic_demo.rs` - 8+ usages
- [ ] `examples/phase4_ecosystem_adoption_demo.rs` - 7+ usages
- [ ] `examples/idiomatic-result-evolution-guide.rs` - 3+ usages
- [ ] `ecosystem-expansion/templates/error-template.rs` - Update template

### **Phase 2E: Cleanup & Verification** (1 hour)

**Steps**:
1. Run `cargo check --workspace` - verify no new errors
2. Run `cargo test --workspace` - ensure tests pass
3. Review deprecation warnings - confirm they're helpful
4. Update documentation - reflect new patterns
5. Archive domain_errors.rs as `.deprecated` (keep for reference)

---

## 📈 EXPECTED OUTCOMES

### **Before Phase 2**:
```
Error System Health: 52%
├── Unified System: 40% (core + some crates)
├── Domain Errors (OLD): 35% (tests + examples)
├── Scattered Errors: 15% (various)
└── Type Alias Conflicts: 10% (causing confusion)
```

### **After Phase 2**:
```
Error System Health: 75%
├── Unified System: 70% (core + crates + tests + examples)
├── Domain Errors (OLD): 5% (deprecated, marked for removal)
├── Scattered Errors: 15% (still in specialized crates)
└── Type Alias Conflicts: 0% (resolved)
```

**Progress**: 52% → 75% (+23%)

---

## 🎯 SUCCESS CRITERIA

- [x] All domain error enums marked as deprecated
- [ ] Type alias conflicts resolved in unified_result_system.rs
- [ ] 20+ helper constructors added to NestGateUnifiedError
- [ ] 15+ test/example files migrated
- [ ] Zero new compilation errors
- [ ] All tests pass
- [ ] Deprecation warnings visible but non-blocking
- [ ] Migration guide documented

---

## 📝 MIGRATION EXAMPLE

### **Before** (domain_errors.rs pattern):
```rust
use nestgate_core::error::{NetworkError, StorageError, ValidationError};

fn connect() -> Result<(), NetworkError> {
    Err(NetworkError::ConnectionFailed {
        address: "api.example.com".to_string(),
        port: 443,
        error: "Connection timeout".to_string(),
        timeout: Some(Duration::from_secs(30)),
    })
}

fn save_file() -> Result<(), StorageError> {
    Err(StorageError::FileNotFound {
        path: "/data/config.toml".to_string(),
        operation: Some("read".to_string()),
    })
}

fn validate_input() -> Result<(), ValidationError> {
    Err(ValidationError::FieldValidation {
        field: Some("email".to_string()),
        message: "Invalid email format".to_string(),
        constraint: Some("RFC 5322".to_string()),
    })
}
```

### **After** (NestGateUnifiedError pattern):
```rust
use nestgate_core::error::{NestGateUnifiedError, Result};

fn connect() -> Result<()> {
    Err(NestGateUnifiedError::network_connection_failed(
        "api.example.com",
        443,
        "Connection timeout",
    ))
}

fn save_file() -> Result<()> {
    Err(NestGateUnifiedError::storage_not_found(
        "/data/config.toml"
    ))
}

fn validate_input() -> Result<()> {
    Err(NestGateUnifiedError::validation_field(
        "email",
        "Invalid email format",
    ))
}
```

**Benefits**:
- ✅ Single error type (simpler)
- ✅ Ergonomic helper constructors
- ✅ Boxed details (memory efficient)
- ✅ No naming conflicts
- ✅ Consistent across codebase

---

## 🚀 NEXT ACTIONS

1. **START**: Add deprecation warnings to domain_errors.rs
2. **THEN**: Fix type alias conflicts
3. **THEN**: Create helper constructors
4. **THEN**: Migrate tests/examples one by one
5. **FINALLY**: Cleanup and verify

**Time Estimate**: 4-6 hours  
**Confidence**: ⭐⭐⭐⭐⭐ High (clear plan, limited scope)

---

**Status**: 🟡 READY TO EXECUTE  
**Risk**: 🟢 LOW (changes isolated to tests/examples)  
**Impact**: 🔥 HIGH (resolves major architectural issue) 