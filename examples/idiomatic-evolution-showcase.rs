use crate::constants::magic_numbers_replacement;
// # Idiomatic Evolution Showcase
//
// This example demonstrates how our evolved error system provides both:
// 1. **Idiomatic Rust patterns** for better ergonomics
// 2. **Unified error benefits** for operational intelligence
//
// **KEY INSIGHT**: This is evolutionary enhancement, not replacement!

use std::collections::HashMap;
use std::fs;

use std::time::Duration;
use tokio::time::sleep;

use crate::canonical_modernization::{UnifiedServiceState, UnifiedServiceType};
use anyhow::Context;
use nestgate_core::config::unified::NestGateUnifiedConfig;
use nestgate_core::error::{NestGateError, Result, ResultExt};

/// **SCENARIO 1**: Configuration Loading (Common Use Case)
/// Shows how idiomatic patterns make simple operations much cleaner
use nestgate_core::error::NestGateError;
fn load_application_config() -> Result<HashMap<String, String>> {
    // IDIOMATIC: External Result integration with context chaining
    let config_data = fs::read_to_string("app.toml")
        .into_nestgate_result()
        .with_context("during application startup")?;

    // IDIOMATIC: Simple validation with domain-specific constructor
    if config_data.trim().is_empty() {
        return Err(NestGateError::invalid_input(
            "configuration".to_string(),
            "app.toml cannot be empty".to_string(),
        ));
    }

    // IDIOMATIC: External library integration (simplified for demo)
    let config: HashMap<String, String> = config_data
        .lines()
        .filter_map(|line| {
            let mut parts = line.split('=');
            Some((
                parts.next()?.trim().to_string(),
                parts.next()?.trim().to_string(),
            ))
        })
        .collect();

    Ok(config)
}

/// **SCENARIO 2**: Network Operations (Domain-Specific)
/// Shows how domain-specific constructors provide rich context
async fn connect_to_database(host: &str, port: u16) -> Result<DatabaseConnection> {
    // IDIOMATIC: Domain-specific network error with automatic context
    let stream = tokio::net::TcpStream::connect(format!("{}:{}", host, port))
        .await
        .map_err(|e| {
            NestGateError::network("database_connect", &e.to_string())
                .with_context(&format!("connecting to {}:{}", host, port))
        })?;

    // IDIOMATIC: Simple error with context chaining
    let connection = DatabaseConnection::from_stream(stream).ok_or_else(|| {
        NestGateError::simple("Failed to establish database connection")
            .with_context("database handshake")
    })?;

    Ok(connection)
}

/// **SCENARIO 3**: Storage Operations (Complex Error Context)
/// Shows how unified benefits are preserved with idiomatic patterns
async fn perform_storage_operation(dataset: &str) -> Result<StorageResult> {
    // IDIOMATIC: Storage-specific error with rich context
    let storage_backend = get_storage_backend().await.map_err(|e| {
        NestGateError::storage("backend_initialization", &e.to_string())
            .with_context(&format!("preparing dataset: {}", dataset))
    })?;

    // IDIOMATIC: External Result with lazy context (performance optimization)
    let result = storage_backend
        .perform_operation(dataset)
        .await
        .map_err(|e| {
            NestGateError::storage("storage_operation", &e)
                .with_context(&format!("processing dataset: {}", dataset))
        })?;

    Ok(result)
}

/// **SCENARIO 4**: Mixed Error Types (Ecosystem Integration)
/// Shows how we can work with conventional Result<T, E> when needed
fn ecosystem_integration_example() -> Result<String> {
    // IDIOMATIC: Can use any error type when needed for ecosystem integration
    let external_result: Result<String> = fs::read_to_string("external.txt").map_err(|io_error| {
        NestGateError::system_error(
            "file_read".to_string(),
            format!("Failed to read external.txt: {}", io_error),
            0.0,
        )
    });

    match external_result {
        Ok(content) => Ok(content),
        Err(e) => Err(e),
    }
}

/// **SCENARIO 5**: Legacy Compatibility (Zero Breaking Changes)
/// Shows how existing code continues to work unchanged
fn legacy_pattern_still_works() -> nestgate_core::error::Result<String> {
    // LEGACY: This pattern continues to work exactly as before
    let result = perform_legacy_operation();

    match result {
        Ok(data) => Ok(data),
        Err(e) => {
            // LEGACY: Existing error construction patterns still work
            Err(NestGateError::Internal {
                message: format!("Legacy operation failed: {}", e),
                location: Some("legacy_pattern_still_works".to_string()),
                debug_info: Some(format!("{:?}", e)),
                is_bug: false,
            })
        }
    }
}

// ==================== SUPPORTING TYPES ====================

#[derive(Debug)]
struct DatabaseConnection {
    _stream: tokio::net::TcpStream,
}

impl DatabaseConnection {
    fn from_stream(stream: tokio::net::TcpStream) -> Option<Self> {
        Some(Self { _stream: stream })
    }
}

#[derive(Debug)]
struct StorageBackend;

impl StorageBackend {
    async fn perform_operation(&self, _dataset: &str) -> Result<StorageResult, String> {
        Ok(StorageResult::Success)
    }
}

#[derive(Debug)]
enum StorageResult {
    Success,
}

async fn get_storage_backend() -> Result<StorageBackend, String> {
    Ok(StorageBackend)
}

fn perform_legacy_operation() -> Result<String, String> {
    Ok("legacy data".to_string())
}

// ==================== DEMONSTRATION MAIN ====================

/// Demonstrates idiomatic Rust patterns in NestGate
#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 NestGate Idiomatic Evolution Showcase");

    // ✅ MODERN: Safe error handling with context
    demonstrate_safe_error_handling().await?;

    // ✅ MODERN: Canonical configuration usage
    demonstrate_canonical_config().await?;

    // ✅ MODERN: Unified type system
    demonstrate_unified_types().await?;

    println!("✅ All demonstrations completed successfully!");
    Ok(())
}

/// Demonstrate modern error handling patterns
async fn demonstrate_safe_error_handling() -> Result<()> {
    // ✅ MODERN: Safe file reading with proper error context
    let _config_data = fs::read_to_string("app.toml")
        .context("during application startup")
        .map_err(|e| {
            NestGateError::internal_error(
                format!("Configuration file read failed: {e}"),
                "demonstrate_safe_error_handling".to_string(),
            )
        })?;

    // ✅ MODERN: Safe database connection simulation
    simulate_database_connection().await.map_err(|e| {
        NestGateError::network_error(
            "database_connect",
            "backend_initialization",
            Some(&e.to_string()),
        )
    })?;

    println!("✅ Safe error handling demonstrated");
    Ok(())
}

/// Simulate database connection for demo
async fn simulate_database_connection() -> Result<()> {
    // Simulate potential database connection failure
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    Ok(())
}

/// Demonstrate canonical configuration patterns
async fn demonstrate_canonical_config() -> Result<()> {
    let config = CanonicalConfig::default();

    // ✅ MODERN: Safe configuration access
    println!("Instance: {}", config.system.instance_name);
    println!("API Port: {}", config.network.api.port);

    // ✅ MODERN: Storage operation with proper error handling
    perform_storage_operation("demo_dataset")
        .await
        .map_err(|e| {
            NestGateError::storage_error("backend_initialization", Some(&e.to_string()))
        })?;

    println!("✅ Canonical configuration demonstrated");
    Ok(())
}

// Duplicate function removed - using the first definition

/// Demonstrate unified type system
async fn demonstrate_unified_types() -> Result<()> {
    // ✅ MODERN: Unified service types
    let _storage_service = UnifiedServiceType::Storage;
    let _network_service = UnifiedServiceType::Network;
    let _security_service = UnifiedServiceType::Security;

    // ✅ MODERN: Unified service states
    let _running_state = UnifiedServiceState::Running;
    let _stopped_state = UnifiedServiceState::Stopped;

    println!("✅ Unified types demonstrated");
    Ok(())
}

/// Result type for storage operations
#[derive(Debug)]
pub struct StorageResult {
    pub bytes_processed: u64,
    pub operation_time_ms: u64,
}
