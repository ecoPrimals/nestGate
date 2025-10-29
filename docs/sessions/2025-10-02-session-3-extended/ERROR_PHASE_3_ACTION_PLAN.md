# 🎯 **ERROR PHASE 3 - CORE PRODUCTION MIGRATION**

**Date Started**: October 2, 2025  
**Status**: 📋 **READY TO BEGIN**  
**Goal**: 52% → 85% error consolidation  
**Estimated Time**: 4-6 hours over 2-3 sessions

---

## 📊 **CURRENT SITUATION**

### **Deprecated Error Enums**
Located in: `code/crates/nestgate-core/src/error/idiomatic/domain_errors.rs`

| Error Enum | Status | Usage Count | Priority |
|------------|--------|-------------|----------|
| `ValidationError` | ⚠️ Deprecated | ~15 files | HIGH |
| `NetworkError` | ⚠️ Deprecated | ~10 files | HIGH |
| `StorageError` | ⚠️ Deprecated | ~8 files | HIGH |
| `SecurityError` | ⚠️ Deprecated | ~7 files | HIGH |
| `ApiError` | ⚠️ Deprecated | ~5 files | MEDIUM |
| `McpError` | ⚠️ Deprecated | ~5 files | MEDIUM |
| `HandlerError` | ⚠️ Deprecated | ~3 files | LOW |
| `ZfsError` | ✅ Keep | Domain-specific | KEEP |
| `TestingError` | ✅ Keep | Test infrastructure | KEEP |
| `PerformanceError` | ✅ Keep | Benchmarks | KEEP |

**Total Production Files**: ~20 files using deprecated errors

---

## 🗂️ **FILES TO MIGRATE**

### **Priority 1: High-Impact Security** (Session 1)
1. `security/input_validation.rs` - Validation errors
2. `security/universal_auth_adapter.rs` - Security errors
3. `security/auth.rs` - Authentication errors
4. `security/permissions.rs` - Authorization errors
5. `security/rate_limiter.rs` - Rate limiting errors

**Estimated Time**: 1.5-2 hours

---

### **Priority 2: Core Network** (Session 2)
6. `network/native_async/service.rs` - Network errors
7. `config/validation.rs` - Validation errors
8. `config_root/providers.rs` - Configuration errors

**Estimated Time**: 1-1.5 hours

---

### **Priority 3: Data Sources & Storage** (Session 2-3)
9. `data_sources/legacy/huggingface.rs` - Network/API errors
10. `data_sources/legacy/ncbi.rs` - Network/API errors
11. `zero_cost/providers.rs` - Various errors
12. `zero_cost/types.rs` - Error type definitions

**Estimated Time**: 1.5-2 hours

---

### **Priority 4: Supporting Files** (Session 3)
13. `lib.rs` - Re-exports (easy)
14. `crypto_locks.rs` - Security errors
15. `universal_security_client/client.rs` - Security errors
16. `canonical/dynamic_config/validators.rs` - Validation errors
17. `zero_cost/mod.rs` - Error exports

**Estimated Time**: 1 hour

---

## 🔧 **MIGRATION PATTERNS**

### **Pattern 1: ValidationError → NestGateUnifiedError**

**BEFORE**:
```rust
use crate::error::idiomatic::ValidationError;

fn validate_input(value: &str) -> Result<(), ValidationError> {
    if value.is_empty() {
        return Err(ValidationError::FieldValidation {
            field: Some("value".to_string()),
            message: "Cannot be empty".to_string(),
            constraint: Some("non-empty".to_string()),
        });
    }
    Ok(())
}
```

**AFTER**:
```rust
use crate::error::{NestGateUnifiedError, ValidationErrorDetails};

fn validate_input(value: &str) -> Result<(), NestGateUnifiedError> {
    if value.is_empty() {
        return Err(NestGateUnifiedError::Validation(Box::new(
            ValidationErrorDetails {
                field: "value".to_string(),
                message: "Cannot be empty".to_string(),
                constraint: Some("non-empty".to_string()),
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
use crate::error::idiomatic::NetworkError;

async fn fetch_data(url: &str) -> Result<String, NetworkError> {
    match reqwest::get(url).await {
        Ok(resp) => Ok(resp.text().await.unwrap()),
        Err(e) => Err(NetworkError::ConnectionFailed {
            address: url.to_string(),
            port: 443,
            error: e.to_string(),
            timeout: None,
        }),
    }
}
```

**AFTER**:
```rust
use crate::error::{NestGateUnifiedError, NetworkErrorDetails};

async fn fetch_data(url: &str) -> Result<String, NestGateUnifiedError> {
    match reqwest::get(url).await {
        Ok(resp) => Ok(resp.text().await.unwrap()),
        Err(e) => Err(NestGateUnifiedError::Network(Box::new(
            NetworkErrorDetails {
                operation: "fetch_data".to_string(),
                address: Some(url.to_string()),
                message: e.to_string(),
                retryable: true,
                timeout: None,
            }
        ))),
    }
}
```

---

### **Pattern 3: SecurityError → NestGateUnifiedError**

**BEFORE**:
```rust
use crate::error::idiomatic::SecurityError;

fn check_permission(user: &str, resource: &str) -> Result<(), SecurityError> {
    if !has_permission(user, resource) {
        return Err(SecurityError::AuthorizationDenied {
            operation: format!("access {}", resource),
            principal: Some(user.to_string()),
            required_permissions: Some(vec!["read".to_string()]),
        });
    }
    Ok(())
}
```

**AFTER**:
```rust
use crate::error::{NestGateUnifiedError, SecurityErrorDetails};

fn check_permission(user: &str, resource: &str) -> Result<(), NestGateUnifiedError> {
    if !has_permission(user, resource) {
        return Err(NestGateUnifiedError::Security(Box::new(
            SecurityErrorDetails {
                operation: Some(format!("access {}", resource)),
                principal: Some(user.to_string()),
                message: format!("Access denied to {}", resource),
                security_data: None,
                context: None,
            }
        )));
    }
    Ok(())
}
```

---

## 📋 **SESSION PLAN**

### **Session 1: Security Files** (1.5-2 hours)

**Goal**: Migrate 5 security-related files

**Steps**:
1. **Backup** (5 min)
   ```bash
   mkdir -p backups/error_phase3_security_$(date +%Y%m%d_%H%M%S)
   cp -r code/crates/nestgate-core/src/security backups/error_phase3_security_*/
   ```

2. **Migrate** `security/input_validation.rs` (20 min)
   - Update imports
   - Replace ValidationError with NestGateUnifiedError
   - Update error construction
   - Test compilation

3. **Migrate** `security/auth.rs` (20 min)
   - Replace SecurityError with NestGateUnifiedError
   - Update authentication error handling
   - Test compilation

4. **Migrate** remaining 3 files (40 min)
   - `security/universal_auth_adapter.rs`
   - `security/permissions.rs`
   - `security/rate_limiter.rs`

5. **Verify** (15 min)
   ```bash
   cargo check --package nestgate-core
   cargo test --package nestgate-core security
   ```

6. **Document** (10 min)
   - Update progress tracker
   - Note any issues encountered

**Expected Progress**: 52% → 60%

---

### **Session 2: Network & Config** (1-1.5 hours)

**Goal**: Migrate 3 network/config files

**Files**:
- `network/native_async/service.rs`
- `config/validation.rs`
- `config_root/providers.rs`

**Expected Progress**: 60% → 70%

---

### **Session 3: Data & Cleanup** (1.5-2 hours)

**Goal**: Complete migration, remove deprecated enums

**Tasks**:
1. Migrate remaining data source files (30 min)
2. Migrate zero_cost files (30 min)
3. Update lib.rs exports (10 min)
4. Remove deprecated enums from domain_errors.rs (20 min)
5. Final verification and cleanup (20 min)

**Expected Progress**: 70% → 85%

---

## ✅ **SUCCESS CRITERIA**

### **Per-File Success**:
- [ ] Compiles without errors
- [ ] All tests pass
- [ ] No new warnings introduced
- [ ] Backward compatibility maintained (where needed)
- [ ] Documentation updated

### **Phase Success**:
- [ ] All Priority 1-3 files migrated
- [ ] Deprecated error enums removed
- [ ] 85%+ error consolidation achieved
- [ ] Zero breaking changes
- [ ] Comprehensive documentation

---

## 🚨 **IMPORTANT NOTES**

### **DO**:
- ✅ Create backups before each session
- ✅ Test compilation after each file
- ✅ Maintain backward compatibility where needed
- ✅ Document any unusual patterns encountered
- ✅ Update progress tracker regularly

### **DON'T**:
- ❌ Migrate multiple files without testing
- ❌ Remove deprecated enums before migration complete
- ❌ Skip backup steps
- ❌ Introduce breaking changes without documentation
- ❌ Rush through files - quality over speed

---

## 📊 **PROGRESS TRACKING**

```
Session 1 (Security):     0/5 files  [          ] 0%
Session 2 (Network):      0/3 files  [          ] 0%
Session 3 (Data/Clean):   0/9 files  [          ] 0%

Overall Phase 3:          0/17 files [          ] 0%
Error Consolidation:      52%        [██████████          ]
```

---

## 🔍 **VERIFICATION COMMANDS**

### **After Each File**:
```bash
# Check compilation
cargo check --package nestgate-core --lib

# Check for deprecation warnings
cargo check 2>&1 | grep -i "deprecated"

# Verify no new errors
cargo check 2>&1 | grep -c "error:"
```

### **After Each Session**:
```bash
# Full build
cargo build --package nestgate-core

# Run tests
cargo test --package nestgate-core --lib

# Check clippy
cargo clippy --package nestgate-core
```

---

## 📞 **QUICK REFERENCE**

### **Canonical Error**:
```rust
use crate::error::{
    NestGateUnifiedError,
    ValidationErrorDetails,
    NetworkErrorDetails,
    StorageErrorDetails,
    SecurityErrorDetails,
};
```

### **Error Construction Helper** (if available):
```rust
// Check if these exist in error/mod.rs or variants/core_errors.rs
NestGateUnifiedError::validation_error(field, message)
NestGateUnifiedError::network_error(operation, message)
NestGateUnifiedError::security_error(operation, message)
```

---

## 🎯 **EXPECTED OUTCOME**

After Phase 3 completion:
- ✅ **85% error consolidation** achieved
- ✅ **~20 production files** migrated
- ✅ **Deprecated errors** removed from domain_errors.rs
- ✅ **Zero breaking changes** maintained
- ✅ **Clean architecture** - single error system
- ✅ **Documentation** comprehensive

**Timeline**: 4-6 hours over 2-3 sessions  
**Start Date**: October 2, 2025 (evening) or next session  
**Target Completion**: Within 1 week

---

**Status**: 📋 **READY TO BEGIN**  
**Next Action**: Session 1 - Migrate security files  
**Confidence**: ⭐⭐⭐⭐⭐ Maximum (proven approach)

---

*Systematic migration - following the successful trait consolidation pattern!* 🚀 