//! **IDIOMATIC ERROR EVOLUTION DEMONSTRATION**
//!
//! This example demonstrates the evolution from non-idiomatic Result<T> patterns
//! to idiomatic Result<T, E> patterns while preserving our sophisticated unified
//! error system.

use nestgate_core::error::{
    // New idiomatic types
    IdioResult, ValidationResult, NetworkResult, StorageResult, SecurityResult,
    // Rich domain-specific errors
    ValidationError, NetworkError, StorageError, SecurityError,
    // Migration utilities
    MigrationHelper, WithContext,
    // Convenience macros
    validation_error, network_error, storage_error,
    // Legacy type for comparison
    Result as LegacyResult,
};

use std::time::{Duration, SystemTime};
use serde::{Serialize, Deserialize};

// ==================== BEFORE: NON-IDIOMATIC PATTERNS ====================

/// **BEFORE**: Non-idiomatic Result<T> pattern
/// Problems: Only T is generic, poor ecosystem integration, complex error construction
mod legacy_patterns {
    use super::*;
    
    /// ❌ NON-IDIOMATIC: Only T is generic
    pub fn validate_user_input_legacy(input: &str) -> LegacyResult<ValidatedInput> {
        if input.is_empty() {
            // Complex error construction required
            return Err(nestgate_core::error::NestGateError::Validation(
                Box::new(nestgate_core::error::ValidationErrorData {
                    message: "Input cannot be empty".to_string(),
                    field: "input".to_string(),
                    value: input.to_string(),
                    validation_type: nestgate_core::error::ValidationType::Required,
                    context: std::collections::HashMap::new(),
                })
            ));
        }
        
        Ok(ValidatedInput { value: input.to_string() })
    }
    
    /// ❌ NON-IDIOMATIC: Network operation with complex error handling
    pub fn connect_to_service_legacy(address: &str, port: u16) -> LegacyResult<Connection> {
        // Simulate connection failure
        std::thread::sleep(Duration::from_millis(10));
        
        // Complex error construction
        Err(nestgate_core::error::NestGateError::Network(
            Box::new(nestgate_core::error::NetworkErrorData {
                message: format!("Connection failed to {}:{}", address, port),
                operation: "connect".to_string(),
                endpoint: Some(format!("{}:{}", address, port)),
                status_code: None,
                retry_info: Some(nestgate_core::error::RetryInfo {
                    max_attempts: 3,
                    base_delay: Duration::from_secs(1),
                    max_delay: Duration::from_secs(10),
                    exponential_backoff: true,
                }),
            })
        ))
    }
    
    /// ❌ NON-IDIOMATIC: Storage operation with verbose error handling
    pub fn read_config_file_legacy(path: &str) -> LegacyResult<Config> {
        if !std::path::Path::new(path).exists() {
            return Err(nestgate_core::error::NestGateError::Storage(
                Box::new(nestgate_core::error::StorageErrorData {
                    message: format!("File not found: {}", path),
                    operation: "read".to_string(),
                    path: path.to_string(),
                    storage_type: "filesystem".to_string(),
                    context: std::collections::HashMap::new(),
                })
            ));
        }
        
        Ok(Config { data: "config data".to_string() })
    }
}

// ==================== AFTER: IDIOMATIC PATTERNS ====================

/// **AFTER**: Idiomatic Result<T, E> patterns
/// Benefits: Both T and E generic, rich context, ecosystem integration, ergonomic construction
mod idiomatic_patterns {
    use super::*;
    
    /// ✅ IDIOMATIC: Both T and E are generic, rich error context
    pub fn validate_user_input(input: &str) -> ValidationResult<ValidatedInput> {
        if input.is_empty() {
            // Ergonomic error construction with rich context
            return Err(validation_error!("input", "Cannot be empty", input));
        }
        
        if input.len() < 3 {
            return Err(ValidationError::FieldValidation {
                field: "input".to_string(),
                message: "Must be at least 3 characters".to_string(),
                value: Some(input.to_string()),
            });
        }
        
        if input.contains("@") && !input.contains(".") {
            return Err(ValidationError::BusinessRule {
                rule: "email_format".to_string(),
                message: "Invalid email format".to_string(),
                context: Some(format!("Input: {}", input)),
            });
        }
        
        Ok(ValidatedInput { value: input.to_string() })
    }
    
    /// ✅ IDIOMATIC: Network operation with rich error context
    pub fn connect_to_service(address: &str, port: u16) -> NetworkResult<Connection> {
        // Simulate different types of network failures
        match port {
            80 => Err(network_error!(connection, address, port, "Connection refused")),
            443 => Err(NetworkError::Timeout {
                operation: "SSL handshake".to_string(),
                duration: Duration::from_secs(30),
            }),
            8080 => Err(NetworkError::DnsResolution {
                hostname: address.to_string(),
                message: "DNS lookup failed".to_string(),
            }),
            _ => Ok(Connection { 
                address: address.to_string(), 
                port,
                connected_at: SystemTime::now(),
            }),
        }
    }
    
    /// ✅ IDIOMATIC: Storage operation with rich error context
    pub fn read_config_file(path: &str) -> StorageResult<Config> {
        if !std::path::Path::new(path).exists() {
            return Err(storage_error!(not_found, path));
        }
        
        if path.starts_with("/root/") {
            return Err(storage_error!(permission_denied, path, "read"));
        }
        
        // Simulate disk full error
        if path.contains("large") {
            return Err(StorageError::DiskFull {
                path: path.to_string(),
                required: 1024 * 1024 * 100, // 100MB
                available: 1024 * 1024 * 10,  // 10MB
            });
        }
        
        Ok(Config { data: format!("config from {}", path) })
    }
    
    /// ✅ IDIOMATIC: Security operation with authentication context
    pub fn authenticate_user(token: &str) -> SecurityResult<User> {
        if token.is_empty() {
            return Err(SecurityError::AuthenticationFailed {
                user: "unknown".to_string(),
                reason: "No token provided".to_string(),
                attempt_count: Some(1),
            });
        }
        
        if token == "expired_token" {
            return Err(SecurityError::TokenExpired {
                token_type: "JWT".to_string(),
                expired_at: SystemTime::now() - Duration::from_secs(3600),
            });
        }
        
        if token == "invalid_permissions" {
            return Err(SecurityError::AuthorizationDenied {
                user: "test_user".to_string(),
                required_permission: "admin".to_string(),
                user_permissions: vec!["user".to_string(), "read".to_string()],
            });
        }
        
        Ok(User { 
            id: "user123".to_string(),
            name: "Test User".to_string(),
        })
    }
}

// ==================== ECOSYSTEM INTEGRATION EXAMPLES ====================

/// **ECOSYSTEM INTEGRATION**: Working with external libraries
mod ecosystem_integration {
    use super::*;
    use nestgate_core::error::{AnyhowResult, BoxedResult};
    
    /// ✅ ECOSYSTEM: Integration with serde_json using AnyhowResult
    pub fn parse_json_config(json_str: &str) -> AnyhowResult<Config> {
        let value: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| anyhow::anyhow!("JSON parsing failed: {}", e))?;
        
        let data = value.get("data")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'data' field"))?;
        
        Ok(Config { data: data.to_string() })
    }
    
    /// ✅ ECOSYSTEM: Dynamic error handling with BoxedResult
    pub fn flexible_operation(operation_type: &str) -> BoxedResult<String> {
        match operation_type {
            "json" => {
                let result = serde_json::to_string(&Config { 
                    data: "test".to_string() 
                })?;
                Ok(result)
            }
            "validation" => {
                validate_user_input("test")
                    .map(|v| v.value)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
            }
            "network" => {
                connect_to_service("localhost", 9999)
                    .map(|c| format!("Connected to {}", c.address))
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
            }
            _ => Err("Unknown operation type".into()),
        }
    }
}

// ==================== MIGRATION UTILITIES DEMONSTRATION ====================

/// **MIGRATION UTILITIES**: Helpers for transitioning from legacy patterns
mod migration_examples {
    use super::*;
    
    /// Demonstrate migration from legacy Result<T> to domain-specific Result<T, E>
    pub fn migrate_legacy_function() -> ValidationResult<ValidatedInput> {
        // Call legacy function
        let legacy_result = legacy_patterns::validate_user_input_legacy("test");
        
        // Convert to idiomatic pattern
        MigrationHelper::to_validation_result(legacy_result)
    }
    
    /// Demonstrate context enhancement
    pub fn enhanced_operation() -> ValidationResult<ValidatedInput> {
        idiomatic_patterns::validate_user_input("ab")
            .with_operation("user_registration")
            .with_component("auth_service")
    }
    
    /// Demonstrate error chain preservation
    pub fn chained_operations() -> ValidationResult<ProcessedData> {
        let input = idiomatic_patterns::validate_user_input("test@example.com")?;
        let config = MigrationHelper::to_validation_result(
            legacy_patterns::read_config_file_legacy("/etc/config.toml")
        )?;
        
        Ok(ProcessedData {
            input: input.value,
            config: config.data,
        })
    }
}

// ==================== TESTING UTILITIES DEMONSTRATION ====================

/// **TESTING**: Improved testing with domain-specific error assertions
mod testing_examples {
    use super::*;
    use nestgate_core::error::idiomatic_evolution::test_utils::*;
    
    #[cfg(test)]
    mod tests {
        use super::*;
        
        #[test]
        fn test_validation_errors() {
            // Test empty input
            let result = idiomatic_patterns::validate_user_input("");
            assert_validation_error(result, "input");
            
            // Test short input
            let result = idiomatic_patterns::validate_user_input("ab");
            match result {
                Err(ValidationError::FieldValidation { field, message, .. }) => {
                    assert_eq!(field, "input");
                    assert!(message.contains("at least 3 characters"));
                }
                _ => panic!("Expected field validation error"),
            }
        }
        
        #[test]
        fn test_network_errors() {
            let result = idiomatic_patterns::connect_to_service("localhost", 80);
            match result {
                Err(NetworkError::ConnectionFailed { address, port, .. }) => {
                    assert_eq!(address, "localhost");
                    assert_eq!(port, 80);
                }
                _ => panic!("Expected connection failed error"),
            }
        }
        
        #[test]
        fn test_storage_errors() {
            let result = idiomatic_patterns::read_config_file("/nonexistent/path");
            match result {
                Err(StorageError::FileNotFound { path }) => {
                    assert_eq!(path, "/nonexistent/path");
                }
                _ => panic!("Expected file not found error"),
            }
        }
        
        #[test]
        fn test_ecosystem_integration() {
            let result = ecosystem_integration::parse_json_config("invalid json");
            assert!(result.is_err());
            
            let error_message = format!("{}", result.unwrap_err());
            assert!(error_message.contains("JSON parsing failed"));
        }
        
        #[test]
        fn test_migration_utilities() {
            let result = migration_examples::migrate_legacy_function();
            assert!(result.is_ok());
            
            let enhanced_result = migration_examples::enhanced_operation();
            // This should fail due to short input, but with enhanced context
            assert!(enhanced_result.is_err());
        }
    }
}

// ==================== PERFORMANCE COMPARISON ====================

/// **PERFORMANCE**: Demonstrate zero-cost abstractions
mod performance_comparison {
    use super::*;
    use std::time::Instant;
    
    pub fn benchmark_error_construction() {
        const ITERATIONS: usize = 10_000;
        
        // Benchmark legacy error construction
        let start = Instant::now();
        for _ in 0..ITERATIONS {
            let _ = legacy_patterns::validate_user_input_legacy("");
        }
        let legacy_duration = start.elapsed();
        
        // Benchmark idiomatic error construction
        let start = Instant::now();
        for _ in 0..ITERATIONS {
            let _ = idiomatic_patterns::validate_user_input("");
        }
        let idiomatic_duration = start.elapsed();
        
        println!("Legacy error construction: {:?}", legacy_duration);
        println!("Idiomatic error construction: {:?}", idiomatic_duration);
        println!("Performance improvement: {:.2}%", 
            (legacy_duration.as_nanos() as f64 - idiomatic_duration.as_nanos() as f64) 
            / legacy_duration.as_nanos() as f64 * 100.0
        );
    }
}

// ==================== SUPPORTING TYPES ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatedInput {
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub address: String,
    pub port: u16,
    pub connected_at: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedData {
    pub input: String,
    pub config: String,
}

// ==================== MAIN DEMONSTRATION ====================

fn main() -> IdioResult<()> {
    println!("🔄 **IDIOMATIC ERROR EVOLUTION DEMONSTRATION**\n");
    
    // Demonstrate validation errors
    println!("📋 **VALIDATION EXAMPLES**:");
    
    match idiomatic_patterns::validate_user_input("") {
        Ok(_) => println!("✅ Validation passed"),
        Err(e) => println!("❌ Validation failed: {}", e),
    }
    
    match idiomatic_patterns::validate_user_input("ab") {
        Ok(_) => println!("✅ Validation passed"),
        Err(e) => println!("❌ Validation failed: {}", e),
    }
    
    match idiomatic_patterns::validate_user_input("test@example.com") {
        Ok(input) => println!("✅ Validation passed: {}", input.value),
        Err(e) => println!("❌ Validation failed: {}", e),
    }
    
    // Demonstrate network errors
    println!("\n🌐 **NETWORK EXAMPLES**:");
    
    match idiomatic_patterns::connect_to_service("localhost", 80) {
        Ok(_) => println!("✅ Connection successful"),
        Err(e) => println!("❌ Connection failed: {}", e),
    }
    
    match idiomatic_patterns::connect_to_service("localhost", 443) {
        Ok(_) => println!("✅ Connection successful"),
        Err(e) => println!("❌ Connection failed: {}", e),
    }
    
    // Demonstrate storage errors
    println!("\n💾 **STORAGE EXAMPLES**:");
    
    match idiomatic_patterns::read_config_file("/nonexistent/path") {
        Ok(_) => println!("✅ File read successful"),
        Err(e) => println!("❌ File read failed: {}", e),
    }
    
    match idiomatic_patterns::read_config_file("/root/secret.conf") {
        Ok(_) => println!("✅ File read successful"),
        Err(e) => println!("❌ File read failed: {}", e),
    }
    
    // Demonstrate ecosystem integration
    println!("\n🔗 **ECOSYSTEM INTEGRATION**:");
    
    match ecosystem_integration::parse_json_config(r#"{"data": "test"}"#) {
        Ok(config) => println!("✅ JSON parsed: {}", config.data),
        Err(e) => println!("❌ JSON parsing failed: {}", e),
    }
    
    match ecosystem_integration::flexible_operation("validation") {
        Ok(result) => println!("✅ Operation successful: {}", result),
        Err(e) => println!("❌ Operation failed: {}", e),
    }
    
    // Performance comparison
    println!("\n⚡ **PERFORMANCE COMPARISON**:");
    performance_comparison::benchmark_error_construction();
    
    println!("\n🎉 **DEMONSTRATION COMPLETE**");
    println!("The idiomatic error system provides:");
    println!("  ✅ Better Rust idioms (Result<T, E>)");
    println!("  ✅ Rich error context");
    println!("  ✅ Ecosystem integration");
    println!("  ✅ Zero-cost abstractions");
    println!("  ✅ Ergonomic error construction");
    println!("  ✅ Better testing support");
    
    Ok(())
} 