//! **IDIOMATIC RESULT<T, E> MIGRATION DEMONSTRATION**
//!
//! This example demonstrates the evolution from non-idiomatic `Result<T>` patterns
//! to idiomatic `Result<T, E>` patterns while preserving all benefits of the 
//! sophisticated unified error system.
//!
//! **Phase 2: Gradual Adoption** - New Code Migration Examples

use nestgate_core::error::{
    // Idiomatic Result types - USE THESE FOR NEW CODE
    IdioResult, ValidationResult, NetworkResult, StorageResult, SecurityResult,
    ZfsResult, ApiResult, McpResult, AnyhowResult, BoxedResult,
    
    // Domain-specific error types
    ValidationError, NetworkError, StorageError, SecurityError, ZfsError,
    ApiError, McpError,
    
    // Migration utilities
    MigrationHelper, WithContext,
    
    // Legacy Result type (for comparison)
    Result as LegacyResult,
    NestGateError,
};

use serde_json;
use std::time::{SystemTime, Duration};

// ==================== PHASE 2.1: NEW CODE MIGRATION ====================

/// **BEFORE: Non-idiomatic pattern**
/// This function uses the old Result<T> pattern (only T is generic)
fn validate_config_old(config_data: &str) -> LegacyResult<ValidatedConfig> {
    if config_data.is_empty() {
        return Err(NestGateError::Configuration {
            message: "Configuration data cannot be empty".to_string(),
            field: Some("config_data".to_string()),
            suggested_fix: Some("Provide valid configuration data".to_string()),
        });
    }
    
    if config_data.len() > 10000 {
        return Err(NestGateError::Configuration {
            message: "Configuration data too large".to_string(),
            field: Some("config_data".to_string()),
            suggested_fix: Some("Reduce configuration size".to_string()),
        });
    }
    
    Ok(ValidatedConfig {
        data: config_data.to_string(),
        validated_at: SystemTime::now(),
    })
}

/// **AFTER: Idiomatic pattern with rich context**
/// This function uses the new ValidationResult<T> pattern with domain-specific errors
fn validate_config_new(config_data: &str) -> ValidationResult<ValidatedConfig> {
    if config_data.is_empty() {
        return Err(ValidationError::FieldValidation {
            field: "config_data".to_string(),
            message: "Configuration data cannot be empty".to_string(),
            value: Some(config_data.to_string()),
            constraint: Some("non-empty".to_string()),
        });
    }
    
    if config_data.len() > 10000 {
        return Err(ValidationError::ConstraintViolation {
            constraint: "max_length".to_string(),
            expected: "10000".to_string(),
            actual: config_data.len().to_string(),
            field: Some("config_data".to_string()),
        });
    }
    
    Ok(ValidatedConfig {
        data: config_data.to_string(),
        validated_at: SystemTime::now(),
    })
}

// ==================== PHASE 2.2: DOMAIN-SPECIFIC MIGRATION ====================

/// **NETWORK OPERATIONS** - Before and After
mod network_operations {
    use super::*;
    
    /// **BEFORE: Generic Result<T>**
    pub fn connect_to_database_old(url: &str, port: u16) -> LegacyResult<DatabaseConnection> {
        if url.is_empty() {
            return Err(NestGateError::Network {
                data: nestgate_core::error::domain_errors::NetworkErrorData {
                    message: "URL cannot be empty".to_string(),
                    operation: "connect_to_database".to_string(),
                },
                context: None,
            });
        }
        
        // Simulate connection attempt
        std::thread::sleep(Duration::from_millis(100));
        
        Err(NestGateError::Network {
            data: nestgate_core::error::domain_errors::NetworkErrorData {
                message: "Connection refused".to_string(),
                operation: "connect_to_database".to_string(),
            },
            context: None,
        })
    }
    
    /// **AFTER: Domain-specific NetworkResult<T>**
    pub fn connect_to_database_new(url: &str, port: u16) -> NetworkResult<DatabaseConnection> {
        if url.is_empty() {
            return Err(NetworkError::ConnectionFailed {
                address: url.to_string(),
                port,
                error: "URL cannot be empty".to_string(),
                timeout: Some(Duration::from_secs(30)),
                retry_count: Some(0),
            });
        }
        
        // Simulate connection attempt with timeout
        std::thread::sleep(Duration::from_millis(100));
        
        Err(NetworkError::Timeout {
            operation: "database_connection".to_string(),
            duration: Duration::from_secs(30),
            address: Some(format!("{}:{}", url, port)),
        })
    }
}

/// **STORAGE OPERATIONS** - Before and After
mod storage_operations {
    use super::*;
    
    /// **BEFORE: Generic Result<T>**
    pub fn read_config_file_old(path: &str) -> LegacyResult<String> {
        if !std::path::Path::new(path).exists() {
            return Err(NestGateError::Storage {
                data: nestgate_core::error::domain_errors::StorageErrorData {
                    message: format!("File not found: {}", path),
                    path: path.to_string(),
                },
                context: None,
            });
        }
        
        Ok("config file contents".to_string())
    }
    
    /// **AFTER: Domain-specific StorageResult<T>**
    pub fn read_config_file_new(path: &str) -> StorageResult<String> {
        if !std::path::Path::new(path).exists() {
            return Err(StorageError::FileNotFound {
                path: path.to_string(),
                operation: "read_config_file".to_string(),
                permissions: Some("read".to_string()),
            });
        }
        
        // Simulate permission check
        if path.starts_with("/root/") {
            return Err(StorageError::PermissionDenied {
                path: path.to_string(),
                operation: "read".to_string(),
                required_permission: "root".to_string(),
                current_user: "nestgate".to_string(),
            });
        }
        
        Ok("config file contents".to_string())
    }
}

/// **SECURITY OPERATIONS** - Before and After
mod security_operations {
    use super::*;
    
    /// **BEFORE: Generic Result<T>**
    pub fn authenticate_user_old(token: &str) -> LegacyResult<AuthenticatedUser> {
        if token.is_empty() {
            return Err(NestGateError::Security {
                data: nestgate_core::error::domain_errors::SecurityErrorData {
                    message: "Token cannot be empty".to_string(),
                    principal: "unknown".to_string(),
                },
                context: None,
            });
        }
        
        Ok(AuthenticatedUser {
            id: "user123".to_string(),
            username: "testuser".to_string(),
        })
    }
    
    /// **AFTER: Domain-specific SecurityResult<T>**
    pub fn authenticate_user_new(token: &str) -> SecurityResult<AuthenticatedUser> {
        if token.is_empty() {
            return Err(SecurityError::InvalidToken {
                token_type: "Bearer".to_string(),
                reason: "Token cannot be empty".to_string(),
                expired: false,
            });
        }
        
        if token == "expired_token" {
            return Err(SecurityError::TokenExpired {
                token_type: "JWT".to_string(),
                expired_at: SystemTime::now(),
            });
        }
        
        Ok(AuthenticatedUser {
            id: "user123".to_string(),
            username: "testuser".to_string(),
        })
    }
}

// ==================== PHASE 2.3: ECOSYSTEM INTEGRATION ====================

/// **ECOSYSTEM INTEGRATION EXAMPLES**
mod ecosystem_integration {
    use super::*;
    
    /// **JSON PARSING with anyhow integration**
    pub fn parse_json_config(data: &str) -> AnyhowResult<serde_json::Value> {
        // Direct integration with serde_json using anyhow
        serde_json::from_str(data).map_err(Into::into)
    }
    
    /// **BOXED ERROR for dynamic error handling**
    pub fn flexible_operation(operation_type: &str) -> BoxedResult<String> {
        match operation_type {
            "json" => {
                let result: Result<serde_json::Value, _> = serde_json::from_str("invalid json");
                result.map(|_| "success".to_string()).map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
            },
            "validation" => {
                let result = validate_config_new("");
                result.map(|_| "success".to_string()).map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
            },
            _ => Ok("unknown operation".to_string())
        }
    }
    
    /// **IDIOMATIC with external error types**
    pub fn parse_duration(input: &str) -> IdioResult<Duration, std::num::ParseIntError> {
        let seconds: u64 = input.parse()?;
        Ok(Duration::from_secs(seconds))
    }
}

// ==================== MIGRATION UTILITIES DEMONSTRATION ====================

/// **MIGRATION HELPERS in action**
mod migration_examples {
    use super::*;
    
    /// **Legacy function that returns Result<T>**
    fn legacy_validation() -> LegacyResult<String> {
        Ok("validated data".to_string())
    }
    
    /// **Migrated function using MigrationHelper**
    pub fn migrated_validation() -> ValidationResult<String> {
        MigrationHelper::to_validation_result(legacy_validation())
    }
    
    /// **Context enhancement example**
    pub fn enhanced_operation() -> ValidationResult<String> {
        validate_config_new("test config")
            .map(|config| config.data)
            .with_operation("enhanced_operation")
            .with_component("migration_demo")
    }
}

// ==================== COMPREHENSIVE DEMONSTRATION ====================

/// **MAIN DEMONSTRATION FUNCTION**
pub fn run_idiomatic_migration_demo() -> IdioResult<()> {
    println!("🔄 **IDIOMATIC RESULT<T, E> MIGRATION DEMONSTRATION**");
    println!("====================================================");
    
    // Phase 2.1: New Code Migration
    println!("\n📋 **PHASE 2.1: NEW CODE MIGRATION**");
    
    // Old pattern
    match validate_config_old("") {
        Ok(_) => println!("✅ Old validation succeeded"),
        Err(e) => println!("❌ Old validation failed: {}", e),
    }
    
    // New pattern
    match validate_config_new("") {
        Ok(_) => println!("✅ New validation succeeded"),
        Err(e) => println!("❌ New validation failed: {} (Rich context: {:?})", e, e),
    }
    
    // Phase 2.2: Domain-Specific Migration
    println!("\n🌐 **PHASE 2.2: DOMAIN-SPECIFIC MIGRATION**");
    
    // Network operations
    match network_operations::connect_to_database_new("", 5432) {
        Ok(_) => println!("✅ Database connection succeeded"),
        Err(e) => println!("❌ Database connection failed: {} (Type: NetworkError)", e),
    }
    
    // Storage operations
    match storage_operations::read_config_file_new("/nonexistent/file.conf") {
        Ok(_) => println!("✅ File read succeeded"),
        Err(e) => println!("❌ File read failed: {} (Type: StorageError)", e),
    }
    
    // Security operations
    match security_operations::authenticate_user_new("expired_token") {
        Ok(_) => println!("✅ Authentication succeeded"),
        Err(e) => println!("❌ Authentication failed: {} (Type: SecurityError)", e),
    }
    
    // Phase 2.3: Ecosystem Integration
    println!("\n🔗 **PHASE 2.3: ECOSYSTEM INTEGRATION**");
    
    // JSON parsing with anyhow
    match ecosystem_integration::parse_json_config("invalid json") {
        Ok(_) => println!("✅ JSON parsing succeeded"),
        Err(e) => println!("❌ JSON parsing failed: {} (Type: anyhow::Error)", e),
    }
    
    // Flexible operation with boxed errors
    match ecosystem_integration::flexible_operation("json") {
        Ok(_) => println!("✅ Flexible operation succeeded"),
        Err(e) => println!("❌ Flexible operation failed: {} (Type: BoxedError)", e),
    }
    
    // Migration utilities
    println!("\n🔧 **MIGRATION UTILITIES**");
    
    match migration_examples::migrated_validation() {
        Ok(data) => println!("✅ Migrated validation succeeded: {}", data),
        Err(e) => println!("❌ Migrated validation failed: {}", e),
    }
    
    match migration_examples::enhanced_operation() {
        Ok(data) => println!("✅ Enhanced operation succeeded: {}", data),
        Err(e) => println!("❌ Enhanced operation failed: {}", e),
    }
    
    println!("\n🎉 **MIGRATION DEMONSTRATION COMPLETE**");
    println!("Benefits achieved:");
    println!("  ✅ Idiomatic Rust patterns (Result<T, E>)");
    println!("  ✅ Rich error context for better debugging");
    println!("  ✅ Domain-specific error types");
    println!("  ✅ Ecosystem integration (anyhow, thiserror)");
    println!("  ✅ Zero breaking changes");
    println!("  ✅ Enhanced developer experience");
    
    Ok(())
}

// ==================== SUPPORTING TYPES ====================

#[derive(Debug, Clone)]
pub struct ValidatedConfig {
    pub data: String,
    pub validated_at: SystemTime,
}

#[derive(Debug, Clone)]
pub struct DatabaseConnection {
    pub url: String,
    pub port: u16,
    pub connected_at: SystemTime,
}

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: String,
    pub username: String,
}

// ==================== MAIN FUNCTION ====================

fn main() -> IdioResult<()> {
    run_idiomatic_migration_demo()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_idiomatic_validation() {
        // Test new idiomatic pattern
        let result = validate_config_new("valid config");
        assert!(result.is_ok());
        
        let result = validate_config_new("");
        assert!(result.is_err());
        
        if let Err(ValidationError::FieldValidation { field, .. }) = result {
            assert_eq!(field, "config_data");
        } else {
            panic!("Expected FieldValidation error");
        }
    }
    
    #[test]
    fn test_domain_specific_errors() {
        // Test network error
        let result = network_operations::connect_to_database_new("", 5432);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), NetworkError::ConnectionFailed { .. }));
        
        // Test storage error
        let result = storage_operations::read_config_file_new("/nonexistent");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), StorageError::FileNotFound { .. }));
        
        // Test security error
        let result = security_operations::authenticate_user_new("expired_token");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), SecurityError::TokenExpired { .. }));
    }
    
    #[test]
    fn test_migration_utilities() {
        let result = migration_examples::migrated_validation();
        assert!(result.is_ok());
        
        // Verify it's the correct type
        let _: ValidationResult<String> = result;
    }
    
    #[test]
    fn test_ecosystem_integration() {
        let result = ecosystem_integration::parse_json_config(r#"{"key": "value"}"#);
        assert!(result.is_ok());
        
        let result = ecosystem_integration::parse_duration("42");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Duration::from_secs(42));
    }
} 