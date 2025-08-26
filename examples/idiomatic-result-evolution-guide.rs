//! # Idiomatic Result<T, E> Evolution Guide
//!
//! This example demonstrates how to evolve from our current Result<T> patterns
//! to more idiomatic Result<T, E> patterns, following Songbird's insights.

use nestgate_core::error::{IdioResult, NestGateError, NetworkError, StorageError};
use nestgate_core::Result;
type AnalysisResult<T> = Result<T>;
type FlexibleResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;
use std::error::Error;

// ==================== DEBT PATTERN EXAMPLES ====================

/// **DEBT PATTERN**: Current non-idiomatic Result<T> usage
/// This forces NestGateError everywhere, limiting flexibility
mod current_debt_patterns {
    use super::*;

    // ❌ DEBT: Fixed error type, poor ecosystem integration
    pub async fn analyze_file_old(path: &str) -> Result<String> {
        // Forced to use NestGateError even for simple operations
        std::fs::read_to_string(path)
            .map_err(|e| NestGateError::simple(format!("Read failed: {e}")))
    }

    // ❌ DEBT: Doesn't work well with external libraries
    pub async fn external_integration_old() -> Result<serde_json::Value> {
        // Type mismatch issues with external crates
        let data = reqwest::get("https://api.example.com/data")
            .await
            .map_err(|e| NestGateError::network("http_get", e.to_string()))?;

        data.json()
            .await
            .map_err(|e| NestGateError::simple(e.to_string()))
    }
}

// ==================== IDIOMATIC EVOLUTION PATTERNS ====================

/// **EVOLUTION**: Idiomatic Result<T, E> patterns with flexible error types
mod idiomatic_patterns {
    use super::*;

    // ✅ IDIOMATIC: Domain-specific error type for better semantics
    pub async fn analyze_file_idiomatic(path: &str) -> AnalysisResult<String> {
        std::fs::read_to_string(path).map_err(|_| AnalysisError::FileNotFound {
            path: path.to_string(),
        })
    }

    // ✅ IDIOMATIC: Flexible error handling for ecosystem integration
    pub async fn external_integration_idiomatic() -> FlexibleResult<serde_json::Value> {
        let data = reqwest::get("https://api.example.com/data").await?;
        let json = data.json().await?;
        Ok(json)
    }

    // ✅ IDIOMATIC: Generic error handling for library functions
    pub async fn generic_operation<T, E>() -> IdioResult<T, E>
    where
        E: From<std::io::Error>,
        T: Default,
    {
        // This works naturally with any error type
        Ok(T::default())
    }
}

// ==================== MIGRATION STRATEGIES ====================

/// **STRATEGY 1**: Gradual migration with compatibility layer
mod gradual_migration {
    use super::*;

    pub struct FileProcessor;

    impl FileProcessor {
        // LEGACY: Keep existing unified API for backward compatibility
        pub async fn process_legacy(&self, path: &str) -> Result<String> {
            self.process_idiomatic(path)
                .await
                .map_err(NestGateError::from_domain_error)
        }

        // NEW: Idiomatic API for new code
        pub async fn process_idiomatic(&self, path: &str) -> AnalysisResult<String> {
            if !std::path::Path::new(path).exists() {
                return Err(AnalysisError::FileNotFound {
                    path: path.to_string(),
                });
            }

            std::fs::read_to_string(path).map_err(|_| AnalysisError::PermissionDenied {
                operation: "read".to_string(),
            })
        }

        // HYBRID: Flexible for ecosystem integration
        pub async fn process_flexible(&self, path: &str) -> FlexibleResult<String> {
            std::fs::read_to_string(path).map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        }
    }
}

/// **STRATEGY 2**: Domain-specific error evolution
mod domain_specific_evolution {
    use super::*;

    // Network operations with network-specific errors
    pub async fn connect_to_service(endpoint: &str) -> IdioResult<String, NetworkError> {
        if endpoint.is_empty() {
            return Err(NetworkError::ConnectionFailed {
                reason: "Connection failed".to_string(),
                endpoint: "empty".to_string(),
            });
        }

        // Simulate connection
        Ok("connected".to_string())
    }

    // Storage operations with storage-specific errors
    pub async fn create_dataset(name: &str) -> IdioResult<String, StorageError> {
        if name.len() > 255 {
            return Err(StorageError::OperationFailed {
                operation: "name too long".to_string(),
            });
        }

        Ok(format!("dataset_{name}"))
    }
}

// ==================== ECOSYSTEM INTEGRATION EXAMPLES ====================

mod ecosystem_integration {
    use super::*;

    // ✅ WORKS NATURALLY: External crate integration
    pub async fn http_client_example() -> IdioResult<String, reqwest::Error> {
        let response = reqwest::get("https://httpbin.org/json").await?;
        let text = response.text().await?;
        Ok(text)
    }

    // ✅ WORKS NATURALLY: Serde integration
    pub async fn json_parsing_example() -> IdioResult<serde_json::Value, serde_json::Error> {
        let json_str = r#"{"name": "NestGate", "version": "2.0"}"#;
        let value: serde_json::Value = serde_json::from_str(json_str)?;
        Ok(value)
    }

    // ✅ WORKS NATURALLY: Tokio integration
    pub async fn async_io_example() -> IdioResult<String, tokio::io::Error> {
        let content = tokio::fs::read_to_string("config.toml").await?;
        Ok(content)
    }
}

// ==================== BEST PRACTICES ====================

/// **BEST PRACTICE**: Choose the right Result type for your use case
mod best_practices {
    use super::*;

    // 🎯 USE UNIFIED: For cross-domain operations needing rich context
    pub async fn complex_system_operation() -> Result<String> {
        // Rich error context, recovery strategies, operational intelligence
        Ok("system_result".to_string())
    }

    // 🎯 USE DOMAIN-SPECIFIC: For focused domain operations
    pub async fn focused_analysis() -> AnalysisResult<String> {
        // Clear, specific error semantics
        Ok("analysis_result".to_string())
    }

    // 🎯 USE FLEXIBLE: For ecosystem integration
    pub async fn library_integration() -> FlexibleResult<String> {
        // Works with any external error type
        Ok("integration_result".to_string())
    }

    // 🎯 USE GENERIC: For reusable library functions
    pub async fn generic_utility<E>() -> IdioResult<String, E>
    where
        E: From<std::io::Error>,
    {
        // Maximum flexibility and reusability
        Ok("utility_result".to_string())
    }
}

// ==================== MIGRATION CHECKLIST ====================

/*
## 📋 MIGRATION CHECKLIST

### ✅ **IMMEDIATE WINS** (Phase 1)
- [ ] Use `AnalysisResult<T>` for file analysis operations
- [ ] Use `NetworkResult<T>` for network operations
- [ ] Use `StorageResult<T>` for storage operations
- [ ] Use `FlexibleResult<T>` for external integrations

### 🔄 **GRADUAL EVOLUTION** (Phase 2)
- [ ] Add `_idiomatic()` variants to existing methods
- [ ] Migrate tests to use domain-specific errors
- [ ] Update documentation with idiomatic examples
- [ ] Add error conversion utilities

### 🎯 **FULL EVOLUTION** (Phase 3)
- [ ] Make idiomatic patterns the default
- [ ] Keep unified patterns for complex operations
- [ ] Achieve 100% ecosystem compatibility
- [ ] Maintain zero breaking changes

### 🏆 **SUCCESS METRICS**
- [ ] Better ecosystem integration (external crates work naturally)
- [ ] Improved error semantics (domain-specific context)
- [ ] Enhanced developer experience (conventional patterns)
- [ ] Preserved unified benefits (rich context, recovery strategies)
*/

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🦀 Idiomatic Result<T, E> Evolution Examples");

    // Example: Gradual migration
    let processor = gradual_migration::FileProcessor;

    // Old API (still works)
    match processor.process_legacy("test.txt").await {
        Ok(content) => println!("✅ Legacy: {}", content),
        Err(e) => println!("❌ Legacy error: {}", e),
    }

    // New idiomatic API
    match processor.process_idiomatic("test.txt").await {
        Ok(content) => println!("✅ Idiomatic: {}", content),
        Err(AnalysisError::FileNotFound { path }) => {
            println!("📁 File not found: {}", path);
        }
        Err(e) => println!("❌ Analysis error: {}", e),
    }

    // Flexible API for ecosystem integration
    match ecosystem_integration::json_parsing_example().await {
        Ok(value) => println!("✅ JSON: {}", value),
        Err(e) => println!("❌ JSON error: {}", e),
    }

    println!("🎉 Evolution complete - idiomatic patterns working!");
    Ok(())
}
