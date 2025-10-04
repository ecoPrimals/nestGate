# ⚠️ **ERROR CONSOLIDATION ACTION PLAN**

**Date**: October 2, 2025  
**Status**: In Progress (50% → Target: 85%)  
**Priority**: 🔴 HIGH - Next major milestone after trait unification

---

## 📊 **CURRENT STATE**

### **Canonical Error System** ✅
**Location**: `code/crates/nestgate-core/src/error/variants/core_errors.rs`

```rust
pub enum NestGateUnifiedError {
    Configuration(Box<ConfigurationErrorDetails>),
    Api(Box<ApiErrorDetails>),
    Storage(Box<StorageErrorDetails>),
    Network(Box<NetworkErrorDetails>),
    Security(Box<SecurityErrorDetails>),
    Automation(Box<AutomationErrorDetails>),
    System(Box<SystemErrorDetails>),
    Internal(Box<InternalErrorDetails>),
    External(Box<ExternalErrorDetails>),
    Validation(Box<ValidationErrorDetails>),
    Timeout(Box<TimeoutErrorDetails>),
    Io(Box<IoErrorDetails>),
    ResourceExhausted(Box<ResourceExhaustedErrorDetails>),
    Testing(Box<TestingErrorDetails>),
    Performance(Box<PerformanceErrorDetails>),
    Handler(Box<HandlerErrorDetails>),
}
```

**Status**: ✅ **COMPLETE** - Comprehensive error system established

---

## 🔴 **DEPRECATED ERRORS TO MIGRATE**

### **Domain Errors** (domain_errors.rs)
**Status**: ❌ **DEPRECATED** but still in use

| Error Enum | Variants | Usage | Priority | Status |
|------------|----------|-------|----------|--------|
| `ValidationError` | 15 | Tests, examples | HIGH | ⚠️ Deprecated |
| `NetworkError` | 18 | Tests, tools | HIGH | ⚠️ Deprecated |
| `StorageError` | 20 | Tests, tools | HIGH | ⚠️ Deprecated |
| `SecurityError` | 21 | Tests, examples | HIGH | ⚠️ Deprecated |
| `ZfsError` | 20 | Tests | MEDIUM | ⚠️ Keep (domain-specific) |
| `ApiError` | 24 | Tests | HIGH | ⚠️ Deprecated |
| `McpError` | 29 | Tests | HIGH | ⚠️ Deprecated |
| `TestingError` | 16 | Tests | LOW | ⚠️ Keep (test-specific) |
| `PerformanceError` | 18 | Benches | LOW | ⚠️ Keep (bench-specific) |
| `HandlerError` | 15 | Tests | MEDIUM | ⚠️ Deprecated |

**Total**: 15 error enums with 206+ variants

---

## 🎯 **MIGRATION STRATEGY**

### **Phase 1: Low-Hanging Fruit** ⏭️ (This Session)
**Goal**: Clean up obvious deprecated usage in non-critical code

1. **Update Test Files** (30 min)
   - Replace deprecated errors in integration tests
   - Update error assertions to use NestGateUnifiedError
   - Files: `tests/unit/core_error_system_tests.rs`, etc.

2. **Update Examples** (20 min)
   - Replace deprecated errors in example code
   - Show proper NestGateUnifiedError usage
   - Files: `examples/error_consolidation_demo.rs`, etc.

3. **Update Templates** (15 min)
   - Remove deprecated error references
   - Update error templates to use unified system
   - Files: `ecosystem-expansion/templates/error-template.rs`

**Expected Progress**: 50% → 55%

---

### **Phase 2: Tool Migration** (Next Session)
**Goal**: Update development tools to use unified errors

1. **Unwrap Migrator Tool** (45 min)
   - Update error type fixer to use NestGateUnifiedError
   - Remove deprecated error patterns
   - File: `tools/unwrap-migrator/src/error_type_fixer.rs`

2. **Clone Optimizer Tool** (30 min)
   - Update error handling
   - Use unified error system

**Expected Progress**: 55% → 60%

---

### **Phase 3: Core Migration** (Future Sessions)
**Goal**: Migrate production code error handling

1. **Domain Errors Removal** (2 hours)
   - Remove deprecated enums from domain_errors.rs
   - Update all imports
   - Ensure all usages migrated

2. **Scattered Errors** (2 hours)
   - Consolidate 40+ scattered error enums
   - Migrate to appropriate NestGateUnifiedError variants

**Expected Progress**: 60% → 85%

---

## 📋 **DETAILED MIGRATION PATTERNS**

### **Pattern 1: ValidationError → NestGateUnifiedError**

**BEFORE**:
```rust
use crate::error::ValidationError;

fn validate_config(config: &Config) -> Result<(), ValidationError> {
    if config.name.is_empty() {
        return Err(ValidationError::FieldValidation {
            field: Some("name".to_string()),
            message: "Name cannot be empty".to_string(),
            constraint: None,
        });
    }
    Ok(())
}
```

**AFTER**:
```rust
use crate::error::{NestGateUnifiedError, ValidationErrorDetails};

fn validate_config(config: &Config) -> Result<(), NestGateUnifiedError> {
    if config.name.is_empty() {
        return Err(NestGateUnifiedError::Validation(Box::new(
            ValidationErrorDetails {
                field: "name".to_string(),
                message: "Name cannot be empty".to_string(),
                constraint: None,
                value: None,
                code: None,
            }
        )));
    }
    Ok(())
}
```

---

### **Pattern 2: NetworkError → NestGateUnifiedError**

**BEFORE**:
```rust
use crate::error::NetworkError;

async fn connect(addr: &str, port: u16) -> Result<Connection, NetworkError> {
    match TcpStream::connect((addr, port)).await {
        Ok(stream) => Ok(Connection::new(stream)),
        Err(e) => Err(NetworkError::ConnectionFailed {
            address: addr.to_string(),
            port,
            error: e.to_string(),
            timeout: None,
        }),
    }
}
```

**AFTER**:
```rust
use crate::error::{NestGateUnifiedError, NetworkErrorDetails};

async fn connect(addr: &str, port: u16) -> Result<Connection, NestGateUnifiedError> {
    match TcpStream::connect((addr, port)).await {
        Ok(stream) => Ok(Connection::new(stream)),
        Err(e) => Err(NestGateUnifiedError::Network(Box::new(
            NetworkErrorDetails {
                operation: "connect".to_string(),
                address: Some(format!("{}:{}", addr, port)),
                message: e.to_string(),
                retryable: true,
                timeout: None,
            }
        ))),
    }
}
```

---

### **Pattern 3: StorageError → NestGateUnifiedError**

**BEFORE**:
```rust
use crate::error::StorageError;

async fn read_file(path: &str) -> Result<Vec<u8>, StorageError> {
    match std::fs::read(path) {
        Ok(data) => Ok(data),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            Err(StorageError::FileNotFound {
                path: path.to_string(),
                operation: Some("read".to_string()),
            })
        }
        Err(e) => Err(StorageError::Unified(NestGateError::from(e))),
    }
}
```

**AFTER**:
```rust
use crate::error::{NestGateUnifiedError, StorageErrorDetails};

async fn read_file(path: &str) -> Result<Vec<u8>, NestGateUnifiedError> {
    match std::fs::read(path) {
        Ok(data) => Ok(data),
        Err(e) => Err(NestGateUnifiedError::Storage(Box::new(
            StorageErrorDetails {
                operation: "read".to_string(),
                path: Some(path.to_string()),
                message: e.to_string(),
                kind: if e.kind() == std::io::ErrorKind::NotFound {
                    "NotFound"
                } else {
                    "IOError"
                }.to_string(),
                retryable: false,
            }
        ))),
    }
}
```

---

## 🧹 **CLEANUP TARGETS**

### **Files to Update** (Phase 1):
```
High Priority (Tests & Examples):
✓ tests/unit/core_error_system_tests.rs
✓ tests/idiomatic_error_evolution_demo.rs  
✓ examples/error_consolidation_demo.rs
✓ ecosystem-expansion/templates/error-template.rs

Medium Priority (Tools):
○ tools/unwrap-migrator/src/error_type_fixer.rs
○ tools/unwrap-migrator/src/compilation_fixer.rs

Low Priority (Keep for now):
- tests/common/test_doubles/storage_test_doubles.rs (test helpers)
- tests/e2e/workflows/* (end-to-end test infrastructure)
```

### **Deprecated Markers to Remove** (After migration):
```
- 15 error enum deprecation markers in domain_errors.rs
- Error-related migration helper files
- Temporary type aliases
```

---

## 📈 **PROGRESS TRACKING**

| Phase | Tasks | Est. Time | Progress | Status |
|-------|-------|-----------|----------|--------|
| **Phase 1** | Tests & Examples | 1-2 hours | 0/3 | ⏭️ Starting |
| **Phase 2** | Tools | 1-2 hours | 0/2 | ⏸️ Pending |
| **Phase 3** | Core Migration | 4-6 hours | 0/2 | ⏸️ Pending |
| **Cleanup** | Remove deprecated | 1 hour | 0/1 | ⏸️ Pending |

**Current**: 50%  
**Target**: 85%  
**Total Effort**: 7-11 hours over 2-3 sessions

---

## ✅ **SUCCESS CRITERIA**

1. **All test files** use NestGateUnifiedError ✅
2. **All example files** use NestGateUnifiedError ✅
3. **All production code** uses NestGateUnifiedError
4. **All deprecated enums** removed from domain_errors.rs
5. **All scattered errors** consolidated
6. **Zero deprecation warnings** related to errors
7. **Build passes** with no error-related issues

---

## 🎯 **IMMEDIATE NEXT STEPS**

### **This Session** (60-90 min):
1. ✅ Create this action plan document
2. ⏭️ Update core_error_system_tests.rs
3. ⏭️ Update error_consolidation_demo.rs
4. ⏭️ Update error-template.rs
5. ⏭️ Document progress

### **Next Session**:
1. Update unwrap-migrator tool
2. Begin core migration
3. Remove deprecated enums

---

## 📞 **QUICK REFERENCE**

**Canonical Error**: `nestgate_core::error::NestGateUnifiedError`  
**Migration Guide**: This document  
**Error Details**: `code/crates/nestgate-core/src/error/variants/core_errors.rs`

**Helper Constructors** (if available):
```rust
NestGateUnifiedError::validation(field, message)
NestGateUnifiedError::network(operation, address, message)
NestGateUnifiedError::storage(operation, path, message)
NestGateUnifiedError::security(operation, principal, message)
```

---

**Status**: 📋 **PLAN COMPLETE**  
**Ready to Execute**: ✅ YES  
**Next Action**: Begin Phase 1 - Update test files

---

*Systematic error consolidation - following the proven trait unification approach!* 🚀 