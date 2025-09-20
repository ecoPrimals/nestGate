//! NestGate-specific migration patterns
//!
//! This module contains migration patterns specifically designed for
//! the `NestGate` codebase structure and error handling patterns.

#![allow(clippy::disallowed_types)] // Allow HashMap in utility crate

use crate::systematic_migrator::{ErrorCategory, MigrationPattern};
use std::collections::HashMap;

/// Get NestGate-specific migration patterns
#[must_use]
pub fn get_nestgate_patterns() -> HashMap<String, MigrationPattern> {
    let mut patterns = HashMap::new();

    // ===============================================================
    // NESTGATE ZERO-COST SERVICE PATTERNS
    // ===============================================================

    patterns.insert(
        "service_start_unwrap".to_string(),
        MigrationPattern {
            pattern: r"service\.start\([^)]+\)\.await\.unwrap\(\)".to_string(),
            replacement: r#"service.start(config).await.map_err(|e| {
    tracing::error!("Failed to start service: {:?}", e);
    NestGateError::Service {
        operation: "start".to_string(),
        message: format!("Service startup failed: {:?}", e),
        service_id: None,
        context: std::collections::HashMap::new(),
    }
})?"#
                .to_string(),
            error_category: ErrorCategory::Resource,
            context: "Zero-cost service startup".to_string(),
        },
    );

    patterns.insert(
        "service_stop_unwrap".to_string(),
        MigrationPattern {
            pattern: r"service\.stop\(\)\.await\.unwrap\(\)".to_string(),
            replacement: r#"service.stop().await.map_err(|e| {
    tracing::error!("Failed to stop service: {:?}", e);
    NestGateError::Service {
        operation: "stop".to_string(),
        message: format!("Service shutdown failed: {:?}", e),
        service_id: None,
        context: std::collections::HashMap::new(),
    }
})?"#
                .to_string(),
            error_category: ErrorCategory::Resource,
            context: "Zero-cost service shutdown".to_string(),
        },
    );

    patterns.insert(
        "health_check_unwrap".to_string(),
        MigrationPattern {
            pattern: r"\.health_check\(\)\.await\.unwrap\(\)".to_string(),
            replacement: r#".health_check().await.map_err(|e| {
    tracing::error!("Health check failed: {:?}", e);
    NestGateError::Health {
        check_type: "service_health".to_string(),
        message: format!("Health check failed: {:?}", e),
        component: None,
        details: std::collections::HashMap::new(),
    }
})?"#
                .to_string(),
            error_category: ErrorCategory::Resource,
            context: "Service health checks".to_string(),
        },
    );

    // ===============================================================
    // STORAGE & ZFS PATTERNS
    // ===============================================================

    patterns.insert(
        "storage_write_unwrap".to_string(),
        MigrationPattern {
            pattern: r"\.write\([^)]+\)\.await\.unwrap\(\)".to_string(),
            replacement: r#".write(path, data).await.map_err(|e| {
    tracing::error!("Storage write failed: {:?}", e);
    NestGateError::Storage {
        operation: "write".to_string(),
        path: path.to_string(),
        message: format!("Write operation failed: {:?}", e),
        storage_type: None,
        details: std::collections::HashMap::new(),
    }
})?"#
                .to_string(),
            error_category: ErrorCategory::IO,
            context: "Storage write operations".to_string(),
        },
    );

    patterns.insert(
        "storage_read_unwrap".to_string(),
        MigrationPattern {
            pattern: r"\.read\([^)]+\)\.await\.unwrap\(\)".to_string(),
            replacement: r#".read(path).await.map_err(|e| {
    tracing::error!("Storage read failed: {:?}", e);
    NestGateError::Storage {
        operation: "read".to_string(),
        path: path.to_string(),
        message: format!("Read operation failed: {:?}", e),
        storage_type: None,
        details: std::collections::HashMap::new(),
    }
})?"#
                .to_string(),
            error_category: ErrorCategory::IO,
            context: "Storage read operations".to_string(),
        },
    );

    // ===============================================================
    // COMMON UNWRAP PATTERNS FOUND IN AUDIT
    // ===============================================================

    patterns.insert(
        "simple_unwrap".to_string(),
        MigrationPattern {
            pattern: r"\.unwrap\(\)".to_string(),
            replacement: r#".map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    std::io::Error::new(std::io::ErrorKind::Other, format!("Operation failed: {:?}", e))
})?"#
                .to_string(),
            error_category: ErrorCategory::Resource,
            context: "Generic unwrap calls".to_string(),
        },
    );

    patterns.insert(
        "expect_with_message".to_string(),
        MigrationPattern {
            pattern: r#"\.expect\("([^"]+)"\)"#.to_string(),
            replacement: r#".map_err(|e| {
    tracing::error!("Expected operation failed: {} - Error: {:?}", "$1", e);
    std::io::Error::new(std::io::ErrorKind::Other, format!("{} - Error: {:?}", "$1", e))
})?"#
                .to_string(),
            error_category: ErrorCategory::Validation,
            context: "Expect calls with custom messages".to_string(),
        },
    );

    patterns.insert(
        "unwrap_or_else".to_string(),
        MigrationPattern {
            pattern: r#"\.unwrap_or_else\(\|e\|\s*panic!\("([^"]+)"\)\)"#.to_string(),
            replacement: r#".map_err(|e| {
    tracing::error!("Critical operation failed: {} - Error: {:?}", "$1", e);
    std::io::Error::new(std::io::ErrorKind::Other, format!("{} - Error: {:?}", "$1", e))
})?"#
                .to_string(),
            error_category: ErrorCategory::Resource,
            context: "Unwrap or else panic patterns".to_string(),
        },
    );

    // ===============================================================
    // RWLOCK AND MUTEX PATTERNS
    // ===============================================================

    patterns.insert(
        "rwlock_read_unwrap".to_string(),
        MigrationPattern {
            pattern: r"\.read\(\)\.unwrap\(\)".to_string(),
            replacement: r#".read().map_err(|e| {
    tracing::error!("RwLock read failed (poisoned): {:?}", e);
    std::io::Error::new(std::io::ErrorKind::Other, format!("Lock poisoned: {:?}", e))
})?"#
                .to_string(),
            error_category: ErrorCategory::Lock,
            context: "RwLock read operations".to_string(),
        },
    );

    patterns.insert(
        "rwlock_write_unwrap".to_string(),
        MigrationPattern {
            pattern: r"\.write\(\)\.unwrap\(\)".to_string(),
            replacement: r#".write().map_err(|e| {
    tracing::error!("RwLock write failed (poisoned): {:?}", e);
    std::io::Error::new(std::io::ErrorKind::Other, format!("Lock poisoned: {:?}", e))
})?"#
                .to_string(),
            error_category: ErrorCategory::Lock,
            context: "RwLock write operations".to_string(),
        },
    );

    patterns.insert(
        "mutex_lock_unwrap".to_string(),
        MigrationPattern {
            pattern: r"\.lock\(\)\.unwrap\(\)".to_string(),
            replacement: r#".lock().map_err(|e| {
    tracing::error!("Mutex lock failed (poisoned): {:?}", e);
    std::io::Error::new(std::io::ErrorKind::Other, format!("Lock poisoned: {:?}", e))
})?"#
                .to_string(),
            error_category: ErrorCategory::Lock,
            context: "Mutex lock operations".to_string(),
        },
    );

    // ===============================================================
    // JSON AND SERIALIZATION PATTERNS
    // ===============================================================

    patterns.insert(
        "json_parse_unwrap".to_string(),
        MigrationPattern {
            pattern: r"serde_json::from_str\([^)]+\)\.unwrap\(\)".to_string(),
            replacement: r#"serde_json::from_str(data).map_err(|e| {
    tracing::error!("JSON parsing failed: {:?}", e);
    std::io::Error::new(std::io::ErrorKind::InvalidData, format!("JSON parsing failed: {:?}", e))
})?"#
                .to_string(),
            error_category: ErrorCategory::Validation,
            context: "JSON deserialization".to_string(),
        },
    );

    patterns.insert(
        "json_serialize_unwrap".to_string(),
        MigrationPattern {
            pattern: r"serde_json::to_string\([^)]+\)\.unwrap\(\)".to_string(),
            replacement: r#"serde_json::to_string(data).map_err(|e| {
    tracing::error!("JSON serialization failed: {:?}", e);
    std::io::Error::new(std::io::ErrorKind::InvalidData, format!("JSON serialization failed: {:?}", e))
})?"#.to_string(),
            error_category: ErrorCategory::Validation,
            context: "JSON serialization".to_string(),
        }
    );

    // ===============================================================
    // ASYNC RUNTIME PATTERNS
    // ===============================================================

    patterns.insert(
        "tokio_spawn_unwrap".to_string(),
        MigrationPattern {
            pattern: r"tokio::spawn\([^)]+\)\.await\.unwrap\(\)".to_string(),
            replacement: r#"tokio::spawn(task).await.map_err(|e| {
    tracing::error!("Async task failed: {:?}", e);
    std::io::Error::new(std::io::ErrorKind::Other, format!("Async task failed: {:?}", e))
})?"#
                .to_string(),
            error_category: ErrorCategory::Resource,
            context: "Async task spawning".to_string(),
        },
    );

    patterns.insert(
        "runtime_block_on_unwrap".to_string(),
        MigrationPattern {
            pattern: r"\.block_on\([^)]+\)\.unwrap\(\)".to_string(),
            replacement: r#".block_on(future).map_err(|e| {
    tracing::error!("Runtime block_on failed: {:?}", e);
    std::io::Error::new(std::io::ErrorKind::Other, format!("Runtime execution failed: {:?}", e))
})?"#
                .to_string(),
            error_category: ErrorCategory::Resource,
            context: "Runtime block_on operations".to_string(),
        },
    );

    patterns
}

/// Get patterns specific to test code that should use different error handling
#[must_use]
pub fn get_nestgate_test_patterns() -> HashMap<String, MigrationPattern> {
    let mut patterns = HashMap::new();

    // Test-specific patterns can be more aggressive with panics
    patterns.insert(
        "test_assert_unwrap".to_string(),
        MigrationPattern {
            pattern: r"assert_eq!\(([^,]+)\.unwrap\(\), ([^)]+)\)".to_string(),
            replacement: r#"assert_eq!($1.expect("Test assertion failed"), $2)"#.to_string(),
            error_category: ErrorCategory::Validation,
            context: "Test assertions".to_string(),
        },
    );

    patterns
}
