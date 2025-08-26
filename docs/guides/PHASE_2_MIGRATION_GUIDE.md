# Phase 2 Migration Guide: Domain-Specific Result Types

## 🎯 **Overview**

This guide covers the systematic migration of NestGate modules from unified `Result<T>` to domain-specific `Result<T, E>` types, building on the successful Phase 1 foundation.

---

## 📋 **Migration Strategy**

### **High-Impact Migration Targets**

#### **1. Security Module (`src/security/`)**
- **Current**: Uses local `ValidationError` + unified `Result<T>`
- **Target**: Migrate to `ValidationResult<T>` and `SecurityResult<T>`
- **Impact**: 🔥 **HIGH** - Core security validation functions
- **Files**: 
  - `input_validation.rs` (validation functions)
  - `universal_auth_adapter.rs` (authentication)
  - `hardening.rs` (security hardening)

#### **2. Network Module (`src/network/`)**
- **Current**: Uses unified `Result<T>` for all network operations
- **Target**: Migrate to `NetworkResult<T>`
- **Impact**: 🔥 **HIGH** - All network communication
- **Files**:
  - `native_async/production.rs` (connection handling)
  - `native_async/traits.rs` (network traits)
  - `native_async/service.rs` (network services)

#### **3. Storage Module (`src/universal_storage/`)**
- **Current**: Uses unified `Result<T>` for all storage operations
- **Target**: Migrate to `StorageResult<T>`
- **Impact**: 🔥 **HIGH** - All data persistence
- **Files**:
  - `unified_storage_traits.rs` (storage interfaces)
  - `enterprise/backend/storage_ops.rs` (storage operations)
  - `coordinator.rs` (storage coordination)

---

## 🚀 **Migration Process**

### **Step 1: Import the New Types**

```rust
// Add to existing imports
use crate::error::{
    ValidationResult, SecurityResult, NetworkResult, StorageResult,
    ValidationError, SecurityError, NetworkError, StorageError
};
```

### **Step 2: Update Function Signatures**

#### **Before (Unified)**
```rust
pub fn validate_email(&self, field: &str, email: &str) -> Result<String> {
    // validation logic
}

pub async fn connect(&self, config: &Config) -> Result<Connection> {
    // network logic  
}

pub async fn read(&self, path: &str) -> Result<Vec<u8>> {
    // storage logic
}
```

#### **After (Domain-Specific)**
```rust
pub fn validate_email(&self, field: &str, email: &str) -> ValidationResult<String> {
    // validation logic - same implementation
}

pub async fn connect(&self, config: &Config) -> NetworkResult<Connection> {
    // network logic - same implementation
}

pub async fn read(&self, path: &str) -> StorageResult<Vec<u8>> {
    // storage logic - same implementation
}
```

### **Step 3: Update Error Creation**

#### **Before (Using .into())**
```rust
return Err(ValidationError::InvalidFormat {
    field: field.to_string(),
}.into());
```

#### **After (Direct Domain Error)**
```rust
return Err(ValidationError::InvalidFormat {
    field: field.to_string(),
});
```

### **Step 4: Update Error Handling**

#### **Pattern Matching (Enhanced)**
```rust
match validate_email("email", "invalid") {
    Ok(email) => println!("Valid: {}", email),
    Err(ValidationError::InvalidFormat { field }) => {
        println!("Invalid format in field: {}", field);
    }
    Err(ValidationError::TooShort { field, min_length }) => {
        println!("Field '{}' too short, min: {}", field, min_length);
    }
    // More specific error handling
}
```

#### **Conversion to Unified (When Needed)**
```rust
fn mixed_operation() -> Result<Data> {
    let validated = validate_email("email", email)
        .map_err(|e| e.into())?; // Auto-converts to NestGateError
    
    let network_data = fetch_data(url)
        .map_err(|e| e.into())?; // Auto-converts to NestGateError
        
    Ok(process(validated, network_data))
}
```

---

## 📁 **Module-Specific Migration Plans**

### **Security Module Migration**

#### **Phase 2A: Input Validation (`input_validation.rs`)**

```rust
// Current local ValidationError conflicts with our domain error
// Strategy: Rename local error and migrate functions

// 1. Rename local ValidationError to InputValidationError
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum InputValidationError {
    #[error("Field '{field}' is too short (minimum: {min_length})")]
    TooShort { field: String, min_length: usize },
    // ... other variants
}

// 2. Update function signatures
impl InputValidator {
    pub fn validate_email(&self, field: &str, email: &str) -> ValidationResult<String> {
        if !self.patterns.email.is_match(email) {
            return Err(ValidationError::InvalidField {
                field: field.to_string(),
                reason: "invalid email format".to_string(),
            });
        }
        // ... rest of logic
    }
    
    pub fn validate_password(&self, field: &str, password: &str) -> SecurityResult<String> {
        if password.len() < self.config.min_password_length {
            return Err(SecurityError::WeakCredentials {
                requirement: "minimum length".to_string(),
                provided: "too short".to_string(),
            });
        }
        // ... rest of logic
    }
}
```

#### **Phase 2B: Authentication (`universal_auth_adapter.rs`)**

```rust
impl AuthenticationAdapter {
    pub async fn authenticate(&self, credentials: &Credentials) -> SecurityResult<AuthToken> {
        // Rate limiting check
        if !self.check_rate_limit(credentials.username()).await? {
            return Err(SecurityError::RateLimited {
                resource: "authentication".to_string(),
                retry_after: Duration::from_secs(60),
            });
        }
        
        // Authentication logic...
        Ok(token)
    }
}
```

### **Network Module Migration**

#### **Phase 2C: Connection Handling (`native_async/production.rs`)**

```rust
impl NativeAsyncProtocolHandler for ProductionProtocolHandler {
    async fn connect(&self, config: &Self::Config) -> NetworkResult<Self::Connection> {
        let connection = NetworkConnection {
            connection_id: uuid::Uuid::new_v4().to_string(),
            protocol: "http".to_string(),
            local_address: config.bind_address.to_string(),
            remote_address: config.bind_address.to_string(),
            established_at: chrono::Utc::now(),
            status: ConnectionStatus::Connecting,
            metadata: std::collections::HashMap::new(),
        };

        // Store connection with error handling
        let mut connections = self.connections.write().await;
        let connection_id = connection.connection_id.clone();
        connections.insert(connection_id, connection.clone());

        Ok(connection)
    }

    async fn send_request(
        &self,
        _connection: &Self::Connection,
        request: Self::Request,
    ) -> NetworkResult<Self::Response> {
        // Enhanced error handling with domain-specific errors
        let response = NetworkResponse {
            request_id: request.request_id,
            status_code: 200,
            headers: HashMap::new(),
            body: b"Success".to_vec(),
            processing_time: Duration::from_millis(10),
        };

        Ok(response)
    }
}
```

### **Storage Module Migration**

#### **Phase 2D: Storage Operations (`enterprise/backend/storage_ops.rs`)**

```rust
#[async_trait]
impl CanonicalStorageBackend for EnterpriseFilesystemBackend {
    async fn read(&self, path: &str) -> StorageResult<Vec<u8>> {
        let start = SystemTime::now();
        let full_path = self.full_path(path);

        let result = tokio::fs::read(&full_path).await.map_err(|e| {
            StorageError::ReadFailed {
                path: path.to_string(),
                reason: e.to_string(),
            }
        });

        let duration = start.elapsed().unwrap_or_default();
        self.update_metrics("read", duration, result.is_ok()).await;

        result
    }

    async fn write(&self, path: &str, data: &[u8]) -> StorageResult<()> {
        let start = SystemTime::now();
        let full_path = self.full_path(path);

        // Ensure parent directory exists
        if let Some(parent) = full_path.parent() {
            tokio::fs::create_dir_all(parent).await.map_err(|e| {
                StorageError::WriteFailed {
                    path: path.to_string(),
                    reason: format!("Failed to create directory: {e}"),
                }
            })?;
        }

        let result = tokio::fs::write(&full_path, data).await.map_err(|e| {
            StorageError::WriteFailed {
                path: path.to_string(),
                reason: e.to_string(),
            }
        });

        let duration = start.elapsed().unwrap_or_default();
        self.update_metrics("write", duration, result.is_ok()).await;

        result
    }
}
```

---

## ✅ **Migration Checklist**

### **Per Module**
- [ ] **Import Analysis**: Identify all `Result<T>` usage
- [ ] **Function Inventory**: List all functions returning `Result<T>`
- [ ] **Error Analysis**: Categorize errors by domain
- [ ] **Signature Updates**: Update function signatures to domain-specific types
- [ ] **Error Creation**: Update error creation to use domain errors
- [ ] **Test Updates**: Update tests to handle domain-specific errors
- [ ] **Documentation**: Update function documentation

### **Integration Points**
- [ ] **Cross-Module Calls**: Handle functions that call across domains
- [ ] **API Boundaries**: Ensure REST handlers work with new types
- [ ] **Error Conversion**: Verify automatic conversion to unified errors
- [ ] **Backward Compatibility**: Ensure no breaking changes

---

## 🧪 **Testing Strategy**

### **Unit Tests**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::{ValidationError, ValidationResult};

    #[test]
    fn test_email_validation_domain_specific() {
        let validator = InputValidator::new().unwrap();
        
        // Test successful validation
        let result: ValidationResult<String> = validator.validate_email("email", "user@example.com");
        assert!(result.is_ok());
        
        // Test domain-specific error
        let result: ValidationResult<String> = validator.validate_email("email", "invalid");
        match result {
            Err(ValidationError::InvalidField { field, reason }) => {
                assert_eq!(field, "email");
                assert!(reason.contains("format"));
            }
            _ => panic!("Expected ValidationError::InvalidField"),
        }
    }
}
```

### **Integration Tests**
```rust
#[tokio::test]
async fn test_cross_domain_integration() {
    // Test that domain errors convert properly in mixed scenarios
    let result = mixed_validation_and_network_operation().await;
    
    match result {
        Ok(data) => assert!(!data.is_empty()),
        Err(e) => {
            // Should be unified NestGateError after conversion
            assert!(matches!(e, NestGateError::Validation(_) | NestGateError::Network(_)));
        }
    }
}
```

---

## 📈 **Success Metrics**

### **Technical Metrics**
- **Domain Clarity**: 80% of errors use domain-specific types
- **Type Safety**: 100% compile-time error type checking
- **Error Specificity**: 50% reduction in generic error messages
- **Test Coverage**: Maintain 90%+ coverage with domain-specific tests

### **Developer Experience**
- **Error Debugging**: Faster error diagnosis with domain context
- **Code Readability**: Clearer error handling patterns
- **IDE Support**: Better autocomplete and error suggestions
- **Documentation**: Self-documenting error types

---

## 🚀 **Rollout Plan**

### **Week 1: Security Module**
- Day 1-2: Input validation migration
- Day 3-4: Authentication migration  
- Day 5: Testing and validation

### **Week 2: Network Module**
- Day 1-2: Connection handling migration
- Day 3-4: Protocol handler migration
- Day 5: Testing and validation

### **Week 3: Storage Module**
- Day 1-2: Core storage operations migration
- Day 3-4: Enterprise backend migration
- Day 5: Testing and validation

### **Week 4: Integration & Polish**
- Day 1-2: Cross-module integration testing
- Day 3-4: Performance validation
- Day 5: Documentation and examples

---

## 🎯 **Expected Outcomes**

### **Immediate Benefits**
- ✅ **Better Error Context**: Domain-specific error information
- ✅ **Improved Debugging**: Clearer error sources and types
- ✅ **Enhanced Testing**: More specific test assertions
- ✅ **Type Safety**: Compile-time error type verification

### **Long-term Benefits**
- ✅ **Ecosystem Integration**: Better compatibility with Rust error libraries
- ✅ **Maintainability**: Clearer error handling patterns
- ✅ **Developer Onboarding**: Self-documenting error types
- ✅ **Future-Proofing**: Aligned with Rust ecosystem evolution

---

**🚀 Phase 2 will transform NestGate from having idiomatic error types to having idiomatic error *usage* throughout the codebase, maximizing the benefits of our Phase 1 foundation.** 