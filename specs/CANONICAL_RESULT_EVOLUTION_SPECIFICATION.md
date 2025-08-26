# Canonical Result<T,E> Evolution Specification

**STATUS**: 🔄 **ACTIVE EVOLUTION** - Transitioning from Result<T> to IdioResult<T,E>  
**VERSION**: 1.0 - Canonical Modernization Phase  
**SONGBIRD INSIGHT**: Applied - Both T and E should be generic for maximum idiomaticity  
**MIGRATION PRIORITY**: High - Critical for ecosystem integration

---

## 🎯 **EXECUTIVE SUMMARY**

Based on analysis of canonical Rust patterns in `../songbird` and other ecosystem projects, we are evolving our Result system from the current `Result<T>` pattern to a more idiomatic `Result<T,E>` approach while **preserving all benefits** of our unified error architecture.

## 🔍 **CURRENT STATE ANALYSIS**

### **Result Pattern Fragmentation** ❌
Our codebase currently has **7 different Result type patterns**:

```rust
// 1. Core unified pattern (most common)
pub type Result<T> = std::result::Result<T, NestGateError>;

// 2. Idiomatic evolution (implemented but underused)  
pub type IdioResult<T, E = NestGateError> = std::result::Result<T, E>;

// 3. Domain-specific patterns (good direction)
pub type ValidationResult<T> = IdioResult<T, ValidationError>;
pub type NetworkResult<T> = IdioResult<T, NetworkError>;
pub type StorageResult<T> = IdioResult<T, StorageError>;
pub type SecurityResult<T> = IdioResult<T, SecurityError>;

// 4. Crate-specific patterns (fragmentation)
pub type Result<T> = std::result::Result<T, InstallerError>;      // nestgate-installer
pub type Result<T> = std::result::Result<T, NestGateBinError>;    // nestgate-bin  
pub type Result<T> = std::result::Result<T, crate::error::Error>; // nestgate-mcp

// 5. Universal ZFS pattern (domain-specific, good)
pub type UniversalZfsResult<T> = Result<T, UniversalZfsError>;

// 6. Notification pattern (should be IdioResult)
pub type NotificationResult<T> = std::result::Result<T, NotificationError>;

// 7. AI-specific pattern (should be IdioResult)  
pub type AIResult<T> = Result<AIFirstResponse<T>, AIFirstError>;
```

### **Usage Analysis**
- **Result<T>**: ~2,100+ usages (needs evolution)
- **IdioResult<T,E>**: ~50 usages (should be primary)
- **Domain-specific**: ~200 usages (good pattern)
- **Fragmented crate patterns**: ~150 usages (needs unification)

---

## 🏗️ **CANONICAL EVOLUTION STRATEGY**

### **Design Principles**
1. **PRESERVE UNIFICATION** - Keep all unified error system benefits
2. **ENHANCE IDIOMATICITY** - Make both T and E generic by default  
3. **GRADUAL MIGRATION** - Zero breaking changes, evolutionary approach
4. **ECOSYSTEM INTEGRATION** - Better anyhow/thiserror compatibility
5. **MAINTAIN SOVEREIGNTY** - Error handling respects user autonomy

### **Target Architecture**

#### **Primary Pattern (Canonical)**
```rust
/// **CANONICAL**: Idiomatic Result with unified default
/// This is our target pattern - both T and E are generic
pub type IdioResult<T, E = NestGateError> = std::result::Result<T, E>;

/// **BACKWARD COMPATIBLE**: Existing Result type (transition phase)
pub type Result<T> = IdioResult<T>;  // Uses NestGateError by default
```

#### **Domain-Specific Patterns (Encouraged)**
```rust
/// **DOMAIN EXCELLENCE**: Specialized error types for specific domains
pub type ValidationResult<T> = IdioResult<T, ValidationError>;
pub type NetworkResult<T> = IdioResult<T, NetworkError>; 
pub type StorageResult<T> = IdioResult<T, StorageError>;
pub type SecurityResult<T> = IdioResult<T, SecurityError>;
pub type ZfsResult<T> = IdioResult<T, ZfsError>;
```

#### **Ecosystem Integration Patterns**
```rust
/// **ECOSYSTEM**: Better integration with Rust error ecosystem
pub type AnyhowResult<T> = IdioResult<T, anyhow::Error>;
pub type BoxedResult<T> = IdioResult<T, Box<dyn std::error::Error + Send + Sync>>;
```

---

## 📋 **MIGRATION PLAN**

### **Phase 1: Foundation Enhancement** ✅ **COMPLETE**
- [x] Implement `IdioResult<T, E = NestGateError>` type
- [x] Create domain-specific Result types
- [x] Add ecosystem integration patterns
- [x] Ensure backward compatibility

### **Phase 2: Gradual Adoption** 🔄 **IN PROGRESS** 
- [ ] **New Code**: Use `IdioResult<T,E>` for all new functions
- [ ] **Domain Functions**: Migrate to domain-specific Result types
- [ ] **External Integration**: Use `AnyhowResult<T>` for external library integration
- [ ] **Crate Unification**: Standardize fragmented crate-specific patterns

### **Phase 3: Legacy Migration** 📅 **PLANNED**
- [ ] **High-Impact Functions**: Convert critical `Result<T>` usages
- [ ] **Test Functions**: Enhance test ergonomics with flexible error types  
- [ ] **Documentation**: Update all examples to use idiomatic patterns
- [ ] **Performance**: Ensure zero-cost abstractions maintained

---

## 🎯 **IMPLEMENTATION GUIDELINES**

### **For New Code** ✅ **RECOMMENDED**
```rust
// ✅ CANONICAL: Use IdioResult with explicit error type
pub async fn process_data(input: &str) -> IdioResult<ProcessedData, ProcessingError> {
    // Implementation
}

// ✅ DOMAIN-SPECIFIC: Use specialized Result types
pub async fn validate_config(config: &Config) -> ValidationResult<ValidatedConfig> {
    // Implementation  
}

// ✅ ECOSYSTEM: Use AnyhowResult for external library integration
pub async fn fetch_external_data(url: &str) -> AnyhowResult<serde_json::Value> {
    let response = reqwest::get(url).await?;
    let data = response.json().await?;
    Ok(data)
}
```

### **For Existing Code** 🔄 **GRADUAL MIGRATION**
```rust
// 🔄 TRANSITION: Current pattern (maintain compatibility)
pub async fn existing_function() -> Result<Data> {
    // Keep existing implementation during transition
}

// 🎯 TARGET: Enhanced pattern (for refactoring)  
pub async fn existing_function() -> IdioResult<Data> {
    // Same implementation, more idiomatic signature
}
```

### **For Domain-Specific Code** 🎯 **HIGH PRIORITY**
```rust
// 🔄 CURRENT: Generic error type
pub async fn create_zfs_pool(name: &str) -> Result<Pool> { ... }

// ✅ TARGET: Domain-specific error type
pub async fn create_zfs_pool(name: &str) -> ZfsResult<Pool> { ... }
```

---

## 📊 **PROGRESS TRACKING**

### **Migration Metrics**
- **Total Functions**: ~2,500 functions with Result types
- **IdioResult Usage**: 50/2,500 (2%) ➜ Target: 80%
- **Domain-Specific Usage**: 200/2,500 (8%) ➜ Target: 60%
- **Legacy Result<T> Usage**: 2,100/2,500 (84%) ➜ Target: 20%
- **Fragmented Patterns**: 150/2,500 (6%) ➜ Target: 0%

### **Priority Areas**
1. **ZFS Operations** - High impact, domain-specific errors ideal
2. **Network Operations** - External integration, AnyhowResult beneficial
3. **Configuration** - Validation errors, ValidationResult perfect fit
4. **Security** - Specialized errors, SecurityResult appropriate
5. **API Endpoints** - Mixed errors, IdioResult<T,E> flexible

---

## 🔧 **TOOLING SUPPORT**

### **Automated Migration Tools**
```bash
# Search for Result<T> patterns that should be IdioResult<T,E>
grep -r "-> Result<" --include="*.rs" code/

# Find domain-specific functions that should use specialized Result types
grep -r "zfs.*-> Result<" --include="*.rs" code/  # Should be ZfsResult<T>
grep -r "network.*-> Result<" --include="*.rs" code/  # Should be NetworkResult<T>
```

### **Linting Rules** (Future)
```rust
// Custom clippy rule to encourage IdioResult usage
#[warn(nestgate::prefer_idio_result)]
pub fn new_function() -> Result<Data> { ... }  // Should suggest IdioResult<Data>
```

---

## 🎉 **BENEFITS ACHIEVED**

### **Ecosystem Integration** ✅
- Better compatibility with `anyhow`, `thiserror`, `eyre`
- Easier integration with external crates
- More conventional Rust patterns

### **Developer Experience** ✅  
- More flexible error handling
- Better test ergonomics
- Clearer domain boundaries

### **Maintained Benefits** ✅
- All unified error system benefits preserved
- Rich error context and recovery strategies maintained
- Sovereignty compliance unchanged
- Zero performance impact

---

## 📚 **REFERENCES**

- **Songbird Patterns**: `../songbird/src/error/` - Canonical Result<T,E> usage
- **Rust Error Handling Book**: https://doc.rust-lang.org/book/ch09-00-error-handling.html
- **Thiserror Documentation**: https://docs.rs/thiserror/
- **Anyhow Documentation**: https://docs.rs/anyhow/

---

**NEXT STEPS**: See `RESULT_EVOLUTION_PROGRESS.md` for current migration status and action items. 