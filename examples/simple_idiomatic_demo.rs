use crate::constants::magic_numbers_replacement;
//! **SIMPLE IDIOMATIC ERROR DEMONSTRATION**
//!
//! A minimal working example showing the evolution from non-idiomatic Result<T>
//! to idiomatic Result<T, E> patterns with our sophisticated unified error system.

use nestgate_core::error::{
    IdioResult,
    // New idiomatic types
    NestGateError,
    // Legacy type for comparison
    Result as LegacyResult,
    SecurityError,
    SecurityResult,
    StorageError,
    StorageResult,
    // Rich domain-specific errors
    ValidationError,
    ValidationResult,
};

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

// ==================== SUPPORTING TYPES ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatedInput {
    pub value: String,
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

// ==================== BEFORE: NON-IDIOMATIC PATTERNS ====================

/// **BEFORE**: Non-idiomatic Result<T> pattern
/// Problems: Only T is generic, complex error construction
fn validate_user_input_legacy(input: &str) -> LegacyResult<ValidatedInput> {
    if input.is_empty() {
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

// ==================== AFTER: IDIOMATIC PATTERNS ====================

/// **AFTER**: Idiomatic Result<T, E> pattern
/// Benefits: Both T and E generic, rich context, ergonomic construction
fn validate_user_input(input: &str) -> ValidationResult<ValidatedInput> {
    if input.is_empty() {
        return Err(ValidationError::FieldValidation {
            field: "input".to_string(),
            message: "Cannot be empty".to_string(),
            value: Some(input.to_string()),
        });
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

    Ok(ValidatedInput {
        value: input.to_string(),
    })
}

/// **IDIOMATIC**: Storage operation with rich error context
fn read_config_file(path: &str) -> StorageResult<Config> {
    if !std::path::Path::new(path).exists() {
        return Err(StorageError::FileNotFound {
            path: path.to_string(),
        });
    }

    if path.starts_with("/root/") {
        return Err(StorageError::PermissionDenied {
            path: path.to_string(),
            operation: "read".to_string(),
        });
    }

    // Simulate disk full error
    if path.contains("large") {
        return Err(StorageError::DiskFull {
            path: path.to_string(),
            required: 1024 * 1024 * 100, // 100MB
            available: 1024 * 1024 * 10, // 10MB
        });
    }

    Ok(Config {
        data: format!("config from {}", path),
    })
}

/// **IDIOMATIC**: Security operation with authentication context
fn authenticate_user(token: &str) -> SecurityResult<User> {
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
            expired_at: SystemTime::now() - std::time::Duration::from_secs(3600),
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

// ==================== ECOSYSTEM INTEGRATION ====================

/// **ECOSYSTEM**: Integration with external libraries using IdioResult
fn parse_json_config(json_str: &str) -> IdioResult<Config, serde_json::Error> {
    let value: serde_json::Value = serde_json::from_str(json_str)?;

    let data = value
        .get("data")
        .and_then(|v| v.as_str())
        .ok_or_else(|| serde_json::Error::custom("Missing 'data' field"))?;

    Ok(Config {
        data: data.to_string(),
    })
}

// ==================== MAIN DEMONSTRATION ====================

fn main() -> IdioResult<()> {
    println!("🔄 **SIMPLE IDIOMATIC ERROR DEMONSTRATION**\n");

    // Demonstrate validation errors
    println!("📋 **VALIDATION EXAMPLES**:");

    match validate_user_input("") {
        Ok(_) => println!("✅ Validation passed"),
        Err(e) => println!("❌ Validation failed: {}", e),
    }

    match validate_user_input("ab") {
        Ok(_) => println!("✅ Validation passed"),
        Err(e) => println!("❌ Validation failed: {}", e),
    }

    match validate_user_input("test@example.com") {
        Ok(input) => println!("✅ Validation passed: {}", input.value),
        Err(e) => println!("❌ Validation failed: {}", e),
    }

    // Demonstrate storage errors
    println!("\n💾 **STORAGE EXAMPLES**:");

    match read_config_file("/nonexistent/path") {
        Ok(_) => println!("✅ File read successful"),
        Err(e) => println!("❌ File read failed: {}", e),
    }

    match read_config_file("/root/secret.conf") {
        Ok(_) => println!("✅ File read successful"),
        Err(e) => println!("❌ File read failed: {}", e),
    }

    // Demonstrate security errors
    println!("\n🔐 **SECURITY EXAMPLES**:");

    match authenticate_user("") {
        Ok(_) => println!("✅ Authentication successful"),
        Err(e) => println!("❌ Authentication failed: {}", e),
    }

    match authenticate_user("expired_token") {
        Ok(_) => println!("✅ Authentication successful"),
        Err(e) => println!("❌ Authentication failed: {}", e),
    }

    match authenticate_user("valid_token") {
        Ok(user) => println!("✅ Authentication successful: {}", user.name),
        Err(e) => println!("❌ Authentication failed: {}", e),
    }

    // Demonstrate ecosystem integration
    println!("\n🔗 **ECOSYSTEM INTEGRATION**:");

    match parse_json_config(r#"{"data": "test"}"#) {
        Ok(config) => println!("✅ JSON parsed: {}", config.data),
        Err(e) => println!("❌ JSON parsing failed: {}", e),
    }

    match parse_json_config("invalid json") {
        Ok(_) => println!("✅ JSON parsed"),
        Err(e) => println!("❌ JSON parsing failed: {}", e),
    }

    println!("\n🎉 **DEMONSTRATION COMPLETE**");
    println!("The idiomatic error system provides:");
    println!("  ✅ Better Rust idioms (Result<T, E>)");
    println!("  ✅ Rich error context");
    println!("  ✅ Ecosystem integration");
    println!("  ✅ Zero-cost abstractions");
    println!("  ✅ Ergonomic error construction");

    Ok(())
}
