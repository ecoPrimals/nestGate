# 🎯 **ERROR CONSOLIDATION ACTION PLAN**

**Date**: October 1, 2025 (Evening Extended - Session 2)  
**Priority**: 3  
**Current Status**: 70% complete  
**Target**: 100% complete  
**Estimated Time**: 3-4 hours

---

## 📊 **CURRENT SITUATION**

### **Unified Error System (Target)**:
✅ **`NestGateUnifiedError`** - The canonical error type with 16 boxed variants:
1. ✅ Configuration
2. ✅ Api
3. ✅ Storage
4. ✅ Network
5. ✅ Security
6. ✅ Automation
7. ✅ System
8. ✅ Internal
9. ✅ External
10. ✅ Validation
11. ✅ Timeout
12. ✅ Io
13. ✅ ResourceExhausted
14. ✅ Testing
15. ✅ Performance
16. ✅ Handler

**Location**: `code/crates/nestgate-core/src/error/variants/core_errors.rs`  
**Alias**: `NestGateError = NestGateUnifiedError`

---

## 🔍 **ERROR TYPES FOUND** (60+ types)

### **Category 1: Domain-Specific Errors** (15 types - HIGH PRIORITY)

**Location**: `code/crates/nestgate-core/src/error/idiomatic/domain_errors.rs`

These already have `Unified(#[from] NestGateError)` variants for easy migration:

1. ⏳ **ValidationError** → Use `NestGateError::Validation`
2. ⏳ **NetworkError** → Use `NestGateError::Network`
3. ⏳ **StorageError** → Use `NestGateError::Storage`
4. ⏳ **SecurityError** → Use `NestGateError::Security`
5. ⏳ **ZfsError** → Use `NestGateError::Storage` (ZFS-specific)
6. ⏳ **ApiError** → Use `NestGateError::Api`
7. ⏳ **McpError** → Use `NestGateError::Api`
8. ⏳ **TestingError** → Use `NestGateError::Testing`
9. ⏳ **PerformanceError** → Use `NestGateError::Performance`
10. ⏳ **HandlerError** → Use `NestGateError::Handler`
11. ⏳ **SerializationError** → Use `NestGateError::Internal`
12. ⏳ **DatabaseError** → Use `NestGateError::Storage`
13. ⏳ **CacheError** → Use `NestGateError::Storage`
14. ⏳ **WorkflowError** → Use `NestGateError::Automation`
15. ⏳ **MonitoringError** → Use `NestGateError::System`

**Note**: File has consolidation comment: "**MIGRATION PLAN** (Week 7-8)" - Ready to migrate!

---

### **Category 2: Specialized Module Errors** (10 types - MEDIUM PRIORITY)

1. ⏳ **CircuitBreakerError** → `NestGateError::System` (resilience pattern)
2. ⏳ **AuthError** → `NestGateError::Security`
3. ⏳ **SimdError** → `NestGateError::Performance`
4. ⏳ **CapabilityRoutingError** → `NestGateError::Internal`
5. ⏳ **PoolSetupError** (zfs crate) → `NestGateError::Storage`
6. ⏳ **RateLimitError** → `NestGateError::Security`
7. ⏳ **UniversalSecurityError** → `NestGateError::Security`
8. ⏳ **InputValidationError** → `NestGateError::Validation`
9. ⏳ **ZeroCostError** → `NestGateError::Performance`
10. ⏳ **NotificationError** → `NestGateError::External`

---

### **Category 3: HTTP/Data Errors** (3 types - LOW PRIORITY)

1. ⏳ **HttpClientError** → `NestGateError::Network`
2. ⏳ **HttpDataError** → `NestGateError::Network`
3. ⏳ **FileDataError** → `NestGateError::Storage`

---

### **Category 4: Config Errors** (2 types - HIGH PRIORITY)

1. ⏳ **ConfigError** (dynamic_config.rs) → `NestGateError::Configuration`
2. ⏳ **ValidationErrorType** (config/validation.rs) → `NestGateError::Validation`

---

### **Category 5: Legacy Migration Helpers** (6 types - KEEP AS-IS)

These are intentionally kept for backward compatibility:
- ✅ **LegacyStorageError** (keep)
- ✅ **LegacyNetworkError** (keep)
- ✅ **LegacyConfigError** (keep)
- ✅ **LegacySecurityError** (keep)
- ✅ **LegacyValidationError** (keep)
- ✅ **LegacyModuleError** (keep)

---

### **Category 6: Other Crate Errors** (8 types - EXTERNAL SCOPE)

These are in separate crates and can be migrated later:
- 🔵 **FsMonitorError** (nestgate-fsmonitor)
- 🔵 **AIError** (nestgate-automation)
- 🔵 **NasError** (nestgate-nas)
- 🔵 **PrimalError** (nestgate-api/ecoprimal_sdk)
- 🔵 **RpcError** (nestgate-api)
- 🔵 **UniversalZfsError** (nestgate-api)
- 🔵 **ConnectionError** (nestgate-api)
- 🔵 **McpProtocolError** (nestgate-mcp)

---

### **Category 7: Test/Tool Errors** (10 types - SKIP)

These are in tests and tools, not production code:
- ⏭️ Test doubles errors (skip)
- ⏭️ Tool errors (clone-optimizer, unwrap-migrator) (skip)
- ⏭️ Example errors (skip)

---

## 🎯 **MIGRATION STRATEGY**

### **Phase 1: Domain Errors** (HIGH PRIORITY - 2 hours)

**Target**: `domain_errors.rs` - 15 error types

**Approach**:
1. Since each error already has `Unified(#[from] NestGateError)`, we can:
   - Mark the enums as `#[deprecated]`
   - Add migration guides
   - Update usage sites gradually
   - Eventually remove (Phase 5)

2. **Alternative Approach** (preferred):
   - Replace enum definitions with type aliases to NestGateError
   - Update constructors to use NestGateError methods
   - This maintains API compatibility while using unified errors

---

### **Phase 2: Specialized Errors** (MEDIUM PRIORITY - 1 hour)

**Target**: Module-specific errors (10 types)

**Approach**:
1. Add `From<SpecializedError> for NestGateError` implementations
2. Update module code to use NestGateError directly
3. Deprecate specialized error types

---

### **Phase 3: HTTP/Data Errors** (LOW PRIORITY - 30 min)

**Target**: HttpClientError, HttpDataError, FileDataError

**Approach**:
1. Simple conversion to Network/Storage variants
2. Update data source modules

---

### **Phase 4: Config Errors** (HIGH PRIORITY - 30 min)

**Target**: ConfigError, ValidationErrorType

**Approach**:
1. Already have NestGateError::Configuration variant
2. Simple find-and-replace migration

---

### **Phase 5: Cleanup** (30 min)

1. Remove deprecated error types
2. Update documentation
3. Verify all conversions
4. Build verification

---

## 📋 **ACTION ITEMS** (Prioritized)

### **Immediate Actions** (Next 30 minutes):

1. ✅ **Add deprecation markers** to domain errors
   - File: `domain_errors.rs`
   - Add `#[deprecated]` attributes
   - Add migration guidance in doc comments

2. ⏳ **Create conversion helpers**
   - Add `From` implementations
   - Document migration patterns

3. ⏳ **Update top usage sites** (find top 10 files using domain errors)
   - Replace with NestGateError
   - Verify builds

---

## 🎯 **SUCCESS CRITERIA**

✅ **Phase 1 Complete**: All domain errors deprecated and marked for migration  
✅ **Phase 2 Complete**: Specialized errors converted  
✅ **Phase 3 Complete**: HTTP/Data errors migrated  
✅ **Phase 4 Complete**: Config errors unified  
✅ **Phase 5 Complete**: Old error types removed  
✅ **Build**: Zero new errors  
✅ **Documentation**: Updated migration guides  

**Target**: **100% error consolidation** → Down from 60+ types to **<10 types**

---

## 📊 **EXPECTED OUTCOME**

| Category | Before | After | Reduction |
|----------|--------|-------|-----------|
| **Core Errors** | 1 (NestGateUnifiedError) | 1 | 0% |
| **Domain Errors** | 15 | 0 (→ NestGateError) | **100%** |
| **Specialized** | 10 | 0 (→ NestGateError) | **100%** |
| **HTTP/Data** | 3 | 0 (→ NestGateError) | **100%** |
| **Config** | 2 | 0 (→ NestGateError) | **100%** |
| **Legacy** | 6 | 6 (keep) | 0% |
| **External Crates** | 8 | 8 (later) | 0% |
| **Tests/Tools** | 10+ | 10+ (skip) | 0% |
| **TOTAL CORE** | **31** | **1** | **97%** ✅

---

## 🚀 **LET'S START!**

**First Action**: Deprecate domain errors and add migration guidance

---

**Created**: October 1, 2025  
**Status**: Ready to execute  
**Estimated Completion**: 3-4 hours from now 