//! **MODERN ERROR SYSTEM DEMONSTRATION**
//!
//! This example demonstrates the evolution from non-idiomatic Result<T> patterns
//! to modern idiomatic Result<T, E> patterns using NestGateUnifiedError.
//!
//! **UPDATED**: Now using NestGateUnifiedError for all modern patterns

use nestgate_core::error::{
    // Ecosystem integration types
    AnyhowResult,
    BoxedResult,
    NestGateError,
    // Modern unified error system
    NestGateUnifiedError,
    NetworkErrorDetails,
    NetworkResult,
    // Legacy type for comparison in legacy_patterns module
    Result as LegacyResult,
    SecurityErrorDetails,
    SecurityResult,
    StorageErrorDetails,
    StorageResult,
    ValidationErrorDetails,
    // Result type aliases
    ValidationResult,
};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

// ==================== BEFORE: NON-IDIOMATIC PATTERNS ====================

/// **BEFORE**: Non-idiomatic Result<T> pattern
/// Problems: Only T is generic, poor ecosystem integration, complex error construction
mod legacy_patterns {
    use super::*;

    /// ❌ NON-IDIOMATIC: Only T is generic
    pub fn validate_user_input_legacy(input: &str) -> LegacyResult<ValidatedInput> {
        if input.is_empty() {
            // Complex error construction required
            return Err(nestgate_core::error::NestGateError::Validation(Box::new(
                nestgate_core::error::ValidationErrorData {
                    message: "Input cannot be empty".to_string(),
                    field: "input".to_string(),
                    value: input.to_string(),
                    validation_type: nestgate_core::error::ValidationType::Required,
                    context: std::collections::HashMap::new(),
                },
            )));
        }

        Ok(ValidatedInput {
            value: input.to_string(),
        })
    }

    /// ❌ NON-IDIOMATIC: Network operation with complex error handling
    pub fn connect_to_service_legacy(address: &str, port: u16) -> LegacyResult<Connection> {
        // Simulate connection failure
        std::thread::sleep(Duration::from_millis(10));

        // Complex error construction
        Err(nestgate_core::error::NestGateError::Network(Box::new(
            nestgate_core::error::NetworkErrorData {
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
            },
        )))
    }

    /// ❌ NON-IDIOMATIC: Storage operation with verbose error handling
    pub fn read_config_file_legacy(path: &str) -> LegacyResult<Config> {
        if !std::path::Path::new(path).exists() {
            return Err(nestgate_core::error::NestGateError::Storage(Box::new(
                nestgate_core::error::StorageErrorData {
                    message: format!("File not found: {}", path),
                    operation: "read".to_string(),
                    path: path.to_string(),
                    storage_type: "filesystem".to_string(),
                    context: std::collections::HashMap::new(),
                },
            )));
        }

        Ok(Config {
            data: "config data".to_string(),
        })
    }
}

// ==================== AFTER: MODERN UNIFIED ERROR PATTERNS ====================

/// **AFTER**: Modern idiomatic patterns using NestGateUnifiedError
/// Benefits: Unified error system, rich context, ecosystem integration, type safety
mod idiomatic_patterns {
    use super::*;

    /// ✅ MODERN: Using NestGateUnifiedError with ValidationErrorDetails
    pub fn validate_user_input(input: &str) -> ValidationResult<ValidatedInput> {
        if input.is_empty() {
            return Err(NestGateUnifiedError::Validation(Box::new(
                ValidationErrorDetails {
                    message: "Input cannot be empty".to_string(),
                    field: Some("input".to_string()),
                    code: Some("EMPTY_INPUT".to_string()),
                    context: HashMap::new(),
                },
            )));
        }

        if input.len() < 3 {
            return Err(NestGateUnifiedError::Validation(Box::new(
                ValidationErrorDetails {
                    message: "Must be at least 3 characters".to_string(),
                    field: Some("input".to_string()),
                    code: Some("TOO_SHORT".to_string()),
                    context: {
                        let mut ctx = HashMap::new();
                        ctx.insert("actual_length".to_string(), input.len().to_string());
                        ctx.insert("min_length".to_string(), "3".to_string());
                        ctx
                    },
                },
            )));
        }

        if input.contains("@") && !input.contains(".") {
            return Err(NestGateUnifiedError::Validation(Box::new(
                ValidationErrorDetails {
                    message: "Invalid email format".to_string(),
                    field: Some("input".to_string()),
                    code: Some("INVALID_EMAIL".to_string()),
                    context: {
                        let mut ctx = HashMap::new();
                        ctx.insert("input".to_string(), input.to_string());
                        ctx
                    },
                },
            )));
        }

        Ok(ValidatedInput {
            value: input.to_string(),
        })
    }

    /// ✅ MODERN: Network operation with NestGateUnifiedError
    pub fn connect_to_service(address: &str, port: u16) -> NetworkResult<Connection> {
        // Simulate different types of network failures
        match port {
            80 => Err(NestGateUnifiedError::Network(Box::new(
                NetworkErrorDetails {
                    message: "Connection refused".to_string(),
                    endpoint: Some(address.to_string()),
                    port: Some(port),
                    protocol: "HTTP".to_string(),
                    network_data: None,
                    context: None,
                },
            ))),
            443 => Err(NestGateUnifiedError::Network(Box::new(
                NetworkErrorDetails {
                    message: "SSL handshake timeout".to_string(),
                    endpoint: Some(address.to_string()),
                    port: Some(port),
                    protocol: "HTTPS".to_string(),
                    network_data: None,
                    context: Some({
                        let mut ctx = HashMap::new();
                        ctx.insert("operation".to_string(), "SSL handshake".to_string());
                        ctx.insert("duration".to_string(), "30s".to_string());
                        ctx
                    }),
                },
            ))),
            8080 => Err(NestGateUnifiedError::Network(Box::new(
                NetworkErrorDetails {
                    message: "DNS lookup failed".to_string(),
                    endpoint: Some(address.to_string()),
                    port: Some(port),
                    protocol: "HTTP".to_string(),
                    network_data: None,
                    context: Some({
                        let mut ctx = HashMap::new();
                        ctx.insert("hostname".to_string(), address.to_string());
                        ctx
                    }),
                },
            ))),
            _ => Ok(Connection {
                address: address.to_string(),
                port,
                connected_at: SystemTime::now(),
            }),
        }
    }

    /// ✅ MODERN: Storage operation with NestGateUnifiedError
    pub fn read_config_file(path: &str) -> StorageResult<Config> {
        if !std::path::Path::new(path).exists() {
            return Err(NestGateUnifiedError::Storage(Box::new(
                StorageErrorDetails {
                    message: format!("File not found: {}", path),
                    resource: Some(path.to_string()),
                    storage_data: None,
                    operation: Some("read".to_string()),
                    context: None,
                },
            )));
        }

        if path.starts_with("/root/") {
            return Err(NestGateUnifiedError::Storage(Box::new(
                StorageErrorDetails {
                    message: "Permission denied".to_string(),
                    resource: Some(path.to_string()),
                    storage_data: None,
                    operation: Some("read".to_string()),
                    context: Some({
                        let mut ctx = HashMap::new();
                        ctx.insert("reason".to_string(), "insufficient_permissions".to_string());
                        ctx
                    }),
                },
            )));
        }

        // Simulate disk full error
        if path.contains("large") {
            return Err(NestGateUnifiedError::Storage(Box::new(
                StorageErrorDetails {
                    message: "Disk full".to_string(),
                    resource: Some(path.to_string()),
                    storage_data: None,
                    operation: Some("write".to_string()),
                    context: Some({
                        let mut ctx = HashMap::new();
                        ctx.insert("required".to_string(), "104857600".to_string()); // 100MB
                        ctx.insert("available".to_string(), "10485760".to_string()); // 10MB
                        ctx
                    }),
                },
            )));
        }

        Ok(Config {
            data: format!("config from {}", path),
        })
    }

    /// ✅ MODERN: Security operation with NestGateUnifiedError
    pub fn authenticate_user(token: &str) -> SecurityResult<User> {
        if token.is_empty() {
            return Err(NestGateUnifiedError::Security(Box::new(
                SecurityErrorDetails {
                    message: "Authentication failed: No token provided".to_string(),
                    operation: Some("authenticate".to_string()),
                    principal: Some("unknown".to_string()),
                    security_data: None,
                    context: Some({
                        let mut ctx = HashMap::new();
                        ctx.insert("attempt_count".to_string(), "1".to_string());
                        ctx
                    }),
                },
            )));
        }

        if token == "expired_token" {
            return Err(NestGateUnifiedError::Security(Box::new(
                SecurityErrorDetails {
                    message: "Token expired".to_string(),
                    operation: Some("authenticate".to_string()),
                    principal: Some("test_user".to_string()),
                    security_data: None,
                    context: Some({
                        let mut ctx = HashMap::new();
                        ctx.insert("token_type".to_string(), "JWT".to_string());
                        ctx.insert(
                            "expired_at".to_string(),
                            format!("{:?}", SystemTime::now() - Duration::from_secs(3600)),
                        );
                        ctx
                    }),
                },
            )));
        }

        if token == "invalid_permissions" {
            return Err(NestGateUnifiedError::Security(Box::new(
                SecurityErrorDetails {
                    message: "Authorization denied".to_string(),
                    operation: Some("authorize".to_string()),
                    principal: Some("test_user".to_string()),
                    security_data: None,
                    context: Some({
                        let mut ctx = HashMap::new();
                        ctx.insert("required_permission".to_string(), "admin".to_string());
                        ctx.insert("user_permissions".to_string(), "user,read".to_string());
                        ctx
                    }),
                },
            )));
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

    /// ✅ ECOSYSTEM: Integration with serde_json using AnyhowResult
    pub fn parse_json_config(json_str: &str) -> AnyhowResult<Config> {
        let value: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| anyhow::anyhow!("JSON parsing failed: {}", e))?;

        let data = value
            .get("data")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'data' field"))?;

        Ok(Config {
            data: data.to_string(),
        })
    }

    /// ✅ ECOSYSTEM: Dynamic error handling with BoxedResult
    pub fn flexible_operation(operation_type: &str) -> BoxedResult<String> {
        match operation_type {
            "json" => {
                let result = serde_json::to_string(&Config {
                    data: "test".to_string(),
                })?;
                Ok(result)
            }
            "validation" => validate_user_input("test")
                .map(|v| v.value)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>),
            "network" => connect_to_service("localhost", 9999)
                .map(|c| format!("Connected to {}", c.address))
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>),
            _ => Err("Unknown operation type".into()),
        }
    }
}

// ==================== MIGRATION UTILITIES DEMONSTRATION ====================

/// **MIGRATION UTILITIES**: Helpers for transitioning from legacy patterns
mod migration_examples {
    use super::*;

    /// Demonstrate migration from legacy Result<T> to modern unified Result<T, E>
    pub fn migrate_legacy_function() -> ValidationResult<ValidatedInput> {
        // Call legacy function and convert to modern pattern
        match legacy_patterns::validate_user_input_legacy("test") {
            Ok(result) => Ok(result),
            Err(e) => Err(NestGateUnifiedError::Validation(Box::new(
                ValidationErrorDetails {
                    message: e.to_string(),
                    field: Some("migrated".to_string()),
                    code: None,
                    context: HashMap::new(),
                },
            ))),
        }
    }

    /// Demonstrate error chain preservation
    pub fn chained_operations() -> ValidationResult<ProcessedData> {
        let input = idiomatic_patterns::validate_user_input("test@example.com")?;
        let config = match legacy_patterns::read_config_file_legacy("/etc/config.toml") {
            Ok(cfg) => cfg,
            Err(e) => {
                return Err(NestGateUnifiedError::Validation(Box::new(
                    ValidationErrorDetails {
                        message: format!("Config loading failed: {}", e),
                        field: Some("config".to_string()),
                        code: None,
                        context: HashMap::new(),
                    },
                )))
            }
        };

        Ok(ProcessedData {
            input: input.value,
            config: config.data,
        })
    }
}

// ==================== TESTING UTILITIES DEMONSTRATION ====================

/// **TESTING**: Improved testing with unified error system
mod testing_examples {
    use super::*;

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_validation_errors() -> Result<(), Box<dyn std::error::Error>> {
            // Test empty input
            let result = idiomatic_patterns::validate_user_input("");
            assert!(result.is_err());
            match result {
                Err(NestGateUnifiedError::Validation(details)) => {
                    assert!(details.message.contains("empty"));
                }
                _ => panic!("Expected validation error"),
            }

            // Test short input
            let result = idiomatic_patterns::validate_user_input("ab");
            match result {
                Err(NestGateUnifiedError::Validation(details)) => {
                    assert_eq!(details.field, Some("input".to_string()));
                    assert!(details.message.contains("at least 3 characters"));
                }
                _ => panic!("Expected validation error"),
            }
            Ok(())
        }

        #[test]
        fn test_network_errors() -> Result<(), Box<dyn std::error::Error>> {
            let result = idiomatic_patterns::connect_to_service("localhost", 80);
            match result {
                Err(NestGateUnifiedError::Network(details)) => {
                    assert_eq!(details.endpoint, Some("localhost".to_string()));
                    assert_eq!(details.port, Some(80));
                }
                _ => panic!("Expected network error"),
            }
            Ok(())
        }

        #[test]
        fn test_storage_errors() -> Result<(), Box<dyn std::error::Error>> {
            let result = idiomatic_patterns::read_config_file("/nonexistent/path");
            match result {
                Err(NestGateUnifiedError::Storage(details)) => {
                    assert_eq!(details.resource, Some("/nonexistent/path".to_string()));
                    assert!(details.message.contains("not found"));
                }
                _ => panic!("Expected storage error"),
            }

            Ok(())
        }

        #[test]
        fn test_ecosystem_integration() -> Result<(), Box<dyn std::error::Error>> {
            let result = ecosystem_integration::parse_json_config("invalid json");
            assert!(result.is_err());

            let error_message = format!("{}", result.unwrap_err());
            assert!(error_message.contains("JSON parsing failed"));

            Ok(())
        }

        #[test]
        fn test_migration_utilities() -> Result<(), Box<dyn std::error::Error>> {
            let result = migration_examples::migrate_legacy_function();
            assert!(result.is_ok());

            Ok(())
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

        // Benchmark modern error construction
        let start = Instant::now();
        for _ in 0..ITERATIONS {
            let _ = idiomatic_patterns::validate_user_input("");
        }
        let modern_duration = start.elapsed();

        println!("Legacy error construction: {:?}", legacy_duration);
        println!("Modern unified error construction: {:?}", modern_duration);
        println!(
            "Performance difference: {:.2}%",
            (legacy_duration.as_nanos() as f64 - modern_duration.as_nanos() as f64)
                / legacy_duration.as_nanos() as f64
                * 100.0
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 **MODERN ERROR SYSTEM DEMONSTRATION**\n");

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
    println!("The modern unified error system provides:");
    println!("  ✅ Better Rust idioms (Result<T, E>)");
    println!("  ✅ Unified error types across codebase");
    println!("  ✅ Rich error context with metadata");
    println!("  ✅ Ecosystem integration (anyhow, Box<dyn Error>)");
    println!("  ✅ Zero-cost abstractions");
    println!("  ✅ Type-safe error handling");
    println!("  ✅ Better testing support");

    Ok(())
}
