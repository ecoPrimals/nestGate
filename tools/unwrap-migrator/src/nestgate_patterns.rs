// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! NestGate-specific migration patterns
//!
//! This module contains migration patterns specifically designed for
//! the `NestGate` codebase structure and error handling patterns.

use crate::systematic_migrator::{ErrorCategory, MigrationPattern};
use std::collections::HashMap;

/// Get NestGate-specific migration patterns
#[must_use]
pub fn get_nestgate_patterns() -> HashMap<String, MigrationPattern> {
    let mut patterns = HashMap::new();
    add_nestgate_service_patterns(&mut patterns);
    add_nestgate_storage_patterns(&mut patterns);
    add_nestgate_common_patterns(&mut patterns);
    add_nestgate_lock_patterns(&mut patterns);
    add_nestgate_json_patterns(&mut patterns);
    add_nestgate_async_patterns(&mut patterns);
    patterns
}

fn add_nestgate_service_patterns(patterns: &mut HashMap<String, MigrationPattern>) {
    patterns.insert(
        "service_start_unwrap".to_string(),
        MigrationPattern {
            pattern: r"service\.start\([^)]+\)\.await\.unwrap\(\)".to_string(),
            replacement: concat!(
                r#"service.start(config).await.map_err(|e| {
    tracing::error!("Failed to start service: "#,
                "{:?}",
                r#"", e);
    NestGateError::Service {
        operation: "start".to_string(),
        message: format!("Service startup failed: "#,
                "{:?}",
                r#"", e),
        service_id: None,
        context: std::collections::HashMap::new(),
    }
})?"#
            )
            .to_string(),
            error_category: ErrorCategory::Resource,
            context: "Zero-cost service startup".to_string(),
        },
    );

    patterns.insert(
        "service_stop_unwrap".to_string(),
        MigrationPattern {
            pattern: r"service\.stop\(\)\.await\.unwrap\(\)".to_string(),
            replacement: concat!(
                r#"service.stop().await.map_err(|e| {
    tracing::error!("Failed to stop service: "#,
                "{:?}",
                r#"", e);
    NestGateError::Service {
        operation: "stop".to_string(),
        message: format!("Service shutdown failed: "#,
                "{:?}",
                r#"", e),
        service_id: None,
        context: std::collections::HashMap::new(),
    }
})?"#
            )
            .to_string(),
            error_category: ErrorCategory::Resource,
            context: "Zero-cost service shutdown".to_string(),
        },
    );

    patterns.insert(
        "health_check_unwrap".to_string(),
        MigrationPattern {
            pattern: r"\.health_check\(\)\.await\.unwrap\(\)".to_string(),
            replacement: concat!(
                r#".health_check().await.map_err(|e| {
    tracing::error!("Health check failed: "#,
                "{:?}",
                r#"", e);
    NestGateError::Health {
        check_type: "service_health".to_string(),
        message: format!("Health check failed: "#,
                "{:?}",
                r#"", e),
        component: None,
        details: std::collections::HashMap::new(),
    }
})?"#
            )
            .to_string(),
            error_category: ErrorCategory::Resource,
            context: "Service health checks".to_string(),
        },
    );
}

fn add_nestgate_storage_patterns(patterns: &mut HashMap<String, MigrationPattern>) {
    patterns.insert(
        "storage_write_unwrap".to_string(),
        MigrationPattern {
            pattern: r"\.write\([^)]+\)\.await\.unwrap\(\)".to_string(),
            replacement: concat!(
                r#".write(path, data).await.map_err(|e| {
    tracing::error!("Storage write failed: "#,
                "{:?}",
                r#"", e);
    NestGateError::Storage {
        operation: "write".to_string(),
        path: path.to_string(),
        message: format!("Write operation failed: "#,
                "{:?}",
                r#"", e),
        storage_type: None,
        details: std::collections::HashMap::new(),
    }
})?"#
            )
            .to_string(),
            error_category: ErrorCategory::IO,
            context: "Storage write operations".to_string(),
        },
    );

    patterns.insert(
        "storage_read_unwrap".to_string(),
        MigrationPattern {
            pattern: r"\.read\([^)]+\)\.await\.unwrap\(\)".to_string(),
            replacement: concat!(
                r#".read(path).await.map_err(|e| {
    tracing::error!("Storage read failed: "#,
                "{:?}",
                r#"", e);
    NestGateError::Storage {
        operation: "read".to_string(),
        path: path.to_string(),
        message: format!("Read operation failed: "#,
                "{:?}",
                r#"", e),
        storage_type: None,
        details: std::collections::HashMap::new(),
    }
})?"#
            )
            .to_string(),
            error_category: ErrorCategory::IO,
            context: "Storage read operations".to_string(),
        },
    );
}

fn add_nestgate_common_patterns(patterns: &mut HashMap<String, MigrationPattern>) {
    patterns.insert(
        "simple_unwrap".to_string(),
        MigrationPattern {
            pattern: r"\.unwrap\(\)".to_string(),
            replacement: concat!(
                r#".map_err(|e| {
    tracing::error!("Operation failed: "#,
                "{:?}",
                r#"", e);
    std::io::Error::new(std::io::ErrorKind::Other, format!("Operation failed: "#,
                "{:?}",
                r#"", e))
})?"#
            )
            .to_string(),
            error_category: ErrorCategory::Resource,
            context: "Generic unwrap calls".to_string(),
        },
    );

    patterns.insert(
        "expect_with_message".to_string(),
        MigrationPattern {
            pattern: r#"\.expect\("([^"]+)"\)"#.to_string(),
            replacement: concat!(
                r#".map_err(|e| {
    tracing::error!("Expected operation failed: {} - Error: "#,
                "{:?}",
                r#"", "$1", e);
    std::io::Error::new(std::io::ErrorKind::Other, format!("{} - Error: "#,
                "{:?}",
                r#"", "$1", e))
})?"#
            )
            .to_string(),
            error_category: ErrorCategory::Validation,
            context: "Expect calls with custom messages".to_string(),
        },
    );

    patterns.insert(
        "unwrap_or_else".to_string(),
        MigrationPattern {
            pattern: r#"\.unwrap_or_else\(\|e\|\s*panic!\("([^"]+)"\)\)"#.to_string(),
            replacement: concat!(
                r#".map_err(|e| {
    tracing::error!("Critical operation failed: {} - Error: "#,
                "{:?}",
                r#"", "$1", e);
    std::io::Error::new(std::io::ErrorKind::Other, format!("{} - Error: "#,
                "{:?}",
                r#"", "$1", e))
})?"#
            )
            .to_string(),
            error_category: ErrorCategory::Resource,
            context: "Unwrap or else panic patterns".to_string(),
        },
    );
}

fn add_nestgate_lock_patterns(patterns: &mut HashMap<String, MigrationPattern>) {
    patterns.insert(
        "rwlock_read_unwrap".to_string(),
        MigrationPattern {
            pattern: r"\.read\(\)\.unwrap\(\)".to_string(),
            replacement: concat!(
                r#".read().map_err(|e| {
    tracing::error!("RwLock read failed (poisoned): "#,
                "{:?}",
                r#"", e);
    std::io::Error::new(std::io::ErrorKind::Other, format!("Lock poisoned: "#,
                "{:?}",
                r#"", e))
})?"#
            )
            .to_string(),
            error_category: ErrorCategory::Lock,
            context: "RwLock read operations".to_string(),
        },
    );

    patterns.insert(
        "rwlock_write_unwrap".to_string(),
        MigrationPattern {
            pattern: r"\.write\(\)\.unwrap\(\)".to_string(),
            replacement: concat!(
                r#".write().map_err(|e| {
    tracing::error!("RwLock write failed (poisoned): "#,
                "{:?}",
                r#"", e);
    std::io::Error::new(std::io::ErrorKind::Other, format!("Lock poisoned: "#,
                "{:?}",
                r#"", e))
})?"#
            )
            .to_string(),
            error_category: ErrorCategory::Lock,
            context: "RwLock write operations".to_string(),
        },
    );

    patterns.insert(
        "mutex_lock_unwrap".to_string(),
        MigrationPattern {
            pattern: r"\.lock\(\)\.unwrap\(\)".to_string(),
            replacement: concat!(
                r#".lock().map_err(|e| {
    tracing::error!("Mutex lock failed (poisoned): "#,
                "{:?}",
                r#"", e);
    std::io::Error::new(std::io::ErrorKind::Other, format!("Lock poisoned: "#,
                "{:?}",
                r#"", e))
})?"#
            )
            .to_string(),
            error_category: ErrorCategory::Lock,
            context: "Mutex lock operations".to_string(),
        },
    );
}

fn add_nestgate_json_patterns(patterns: &mut HashMap<String, MigrationPattern>) {
    patterns.insert(
        "json_parse_unwrap".to_string(),
        MigrationPattern {
            pattern: r"serde_json::from_str\([^)]+\)\.unwrap\(\)".to_string(),
            replacement: concat!(
                r#"serde_json::from_str(data).map_err(|e| {
    tracing::error!("JSON parsing failed: "#,
                "{:?}",
                r#"", e);
    std::io::Error::new(std::io::ErrorKind::InvalidData, format!("JSON parsing failed: "#,
                "{:?}",
                r#"", e))
})?"#
            )
            .to_string(),
            error_category: ErrorCategory::Validation,
            context: "JSON deserialization".to_string(),
        },
    );

    patterns.insert(
        "json_serialize_unwrap".to_string(),
        MigrationPattern {
            pattern: r"serde_json::to_string\([^)]+\)\.unwrap\(\)".to_string(),
            replacement: concat!(
                r#"serde_json::to_string(data).map_err(|e| {
    tracing::error!("JSON serialization failed: "#,
                "{:?}",
                r#"", e);
    std::io::Error::new(std::io::ErrorKind::InvalidData, format!("JSON serialization failed: "#,
                "{:?}",
                r#"", e))
})?"#
            )
            .to_string(),
            error_category: ErrorCategory::Validation,
            context: "JSON serialization".to_string(),
        },
    );
}

fn add_nestgate_async_patterns(patterns: &mut HashMap<String, MigrationPattern>) {
    patterns.insert(
        "tokio_spawn_unwrap".to_string(),
        MigrationPattern {
            pattern: r"tokio::spawn\([^)]+\)\.await\.unwrap\(\)".to_string(),
            replacement: concat!(
                r#"tokio::spawn(task).await.map_err(|e| {
    tracing::error!("Async task failed: "#,
                "{:?}",
                r#"", e);
    std::io::Error::new(std::io::ErrorKind::Other, format!("Async task failed: "#,
                "{:?}",
                r#"", e))
})?"#
            )
            .to_string(),
            error_category: ErrorCategory::Resource,
            context: "Async task spawning".to_string(),
        },
    );

    patterns.insert(
        "runtime_block_on_unwrap".to_string(),
        MigrationPattern {
            pattern: r"\.block_on\([^)]+\)\.unwrap\(\)".to_string(),
            replacement: concat!(
                r#".block_on(future).map_err(|e| {
    tracing::error!("Runtime block_on failed: "#,
                "{:?}",
                r#"", e);
    std::io::Error::new(std::io::ErrorKind::Other, format!("Runtime execution failed: "#,
                "{:?}",
                r#"", e))
})?"#
            )
            .to_string(),
            error_category: ErrorCategory::Resource,
            context: "Runtime block_on operations".to_string(),
        },
    );
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
