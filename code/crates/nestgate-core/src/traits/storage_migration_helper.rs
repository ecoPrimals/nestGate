//! **STORAGE TRAIT MIGRATION HELPER**
//!
//! This module provides utilities to help migrate from fragmented storage trait
//! implementations to the unified CanonicalStorage trait.

use crate::error::{Result, StorageResult, StorageError};
use crate::traits::CanonicalStorage;
use std::future::Future;

// ==================== SECTION ====================

/// **STORAGE MIGRATION ADAPTER**
/// 
/// Helps legacy storage implementations migrate to the canonical storage trait
/// by providing adapter patterns and migration utilities.
pub struct StorageMigrationAdapter;

impl StorageMigrationAdapter {
    /// Create storage error using rich domain-specific error type
    pub fn storage_error(message: impl Into<String>, operation: impl Into<String>, path: impl Into<String>) -> StorageError {
        StorageError::FileReadError {
            path: path.into(),
            operation: operation.into(),
            error: message.into(),
            permissions: None,
        }
    }

    /// Create file not found error
    pub fn file_not_found_error(path: impl Into<String>, operation: impl Into<String>) -> StorageError {
        StorageError::FileNotFound {
            path: path.into(),
            operation: operation.into(),
            permissions: None,
        }
    }

    /// Create permission denied error
    pub fn permission_denied_error(
        path: impl Into<String>, 
        operation: impl Into<String>,
        required_permission: impl Into<String>,
        current_user: impl Into<String>
    ) -> StorageError {
        StorageError::PermissionDenied {
            path: path.into(),
            operation: operation.into(),
            required_permission: required_permission.into(),
            current_user: current_user.into(),
        }
    }

    /// Create disk full error
    pub fn disk_full_error(path: impl Into<String>, available_space: u64, required_space: u64) -> StorageError {
        StorageError::DiskFull {
            path: path.into(),
            available_space,
            required_space,
        }
    }
}

// ==================== SECTION ====================

/// **LEGACY STORAGE ADAPTER**
/// 
/// Example adapter for migrating legacy storage implementations to CanonicalStorage
/// This shows the pattern for wrapping existing storage backends.
pub struct LegacyStorageAdapter<T> {
    inner: T,
    backend_name: String,
}

impl<T> LegacyStorageAdapter<T> {
    /// Create new adapter wrapping a legacy storage backend
    pub fn new(inner: T, backend_name: impl Into<String>) -> Self {
        Self {
            inner,
            backend_name: backend_name.into(),
        }
    }

    /// Get the backend name for debugging
    pub fn backend_name(&self) -> &str {
        &self.backend_name
    }

    /// Get reference to inner implementation
    pub fn inner(&self) -> &T {
        &self.inner
    }
}

// ==================== SECTION ====================

/// Convert legacy Result types to StorageResult
pub fn convert_to_storage_result<T>(
    result: std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>,
    operation: &str,
    path: &str
) -> StorageResult<T> {
    result.map_err(|e| {
        StorageError::FileReadError {
            path: path.to_string(),
            operation: operation.to_string(),
            error: e.to_string(),
            permissions: None,
        }
    })
}

/// Convert std::io::Error to StorageError
pub fn io_error_to_storage_error(error: std::io::Error, operation: &str, path: &str) -> StorageError {
    use std::io::ErrorKind;
    
    match error.kind() {
        ErrorKind::NotFound => StorageError::FileNotFound {
            path: path.to_string(),
            operation: operation.to_string(),
            permissions: None,
        },
        ErrorKind::PermissionDenied => StorageError::PermissionDenied {
            path: path.to_string(),
            operation: operation.to_string(),
            required_permission: "read/write access".to_string(),
            current_user: "current_user".to_string(),
        },
        _ => StorageError::FileReadError {
            path: path.to_string(),
            operation: operation.to_string(),
            error: error.to_string(),
            permissions: None,
        },
    }
}

// ==================== SECTION ====================

/// **EXAMPLE**: How to migrate a legacy storage implementation
/// 
/// This shows the pattern for migrating from old trait definitions to CanonicalStorage
#[cfg(test)] // PEDANTIC: Fixed cfg condition from migration_examples
pub mod migration_example {
    use super::*;
    use crate::traits::{CanonicalService, CanonicalStorage};
    use crate::error::NestGateError;
    
    /// Example legacy storage backend
    pub struct ExampleLegacyStorage {
        base_path: String,
    }
    
    impl ExampleLegacyStorage {
        pub fn new(base_path: String) -> Self {
            Self { base_path }
        }
    }
    
    /// Migration: Implement CanonicalService for the legacy backend
    impl CanonicalService for ExampleLegacyStorage {
        type Config = String; // Simple config for example
        type Health = bool;   // Simple health for example
        type Metrics = u64;   // Simple metrics for example
        type Error = NestGateError;
        
        async fn start(&self) -> Result<(), Self::Error> {
            Ok(()) // Example implementation
        }
        
        async fn stop(&self) -> Result<(), Self::Error> {
            Ok(()) // Example implementation
        }
        
        async fn is_healthy(&self) -> Result<Self::Health, Self::Error> {
            Ok(true) // Example implementation
        }
        
        async fn get_metrics(&self) -> Result<Self::Metrics, Self::Error> {
            Ok(0) // Example implementation
        }
    }
    
    /// Migration: Implement CanonicalStorage for the legacy backend
    impl CanonicalStorage for ExampleLegacyStorage {
        type Item = Vec<u8>;
        type Key = String;
        
        async fn read(&self, key: Self::Key) -> Result<Option<Self::Item>, Self::Error> {
            // Example: Convert legacy read operation to canonical interface
            let path = format!("{}/{}", self.base_path, key);
            match std::fs::read(&path) {
                Ok(data) => Ok(Some(data)),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
                Err(e) => Err(NestGateError::Io {
                    message: e.to_string(),
                    operation: "read".to_string(),
                    path: Some(path),
                    retryable: false,
                    context: None,
                }),
            }
        }
        
        async fn write(&self, key: Self::Key, item: Self::Item) -> Result<(), Self::Error> {
            // Example: Convert legacy write operation to canonical interface
            let path = format!("{}/{}", self.base_path, key);
            std::fs::write(&path, item).map_err(|e| {
                NestGateError::Io {
                    message: e.to_string(),
                    operation: "write".to_string(),
                    path: Some(path),
                    retryable: false,
                    context: None,
                }
            })
        }
        
        async fn delete(&self, key: Self::Key) -> Result<bool, Self::Error> {
            // Example: Convert legacy delete operation to canonical interface
            let path = format!("{}/{}", self.base_path, key);
            match std::fs::remove_file(&path) {
                Ok(()) => Ok(true),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
                Err(e) => Err(NestGateError::Io {
                    message: e.to_string(),
                    operation: "delete".to_string(),
                    path: Some(path),
                    retryable: false,
                    context: None,
                }),
            }
        }
        
        async fn list(&self, _prefix: Option<Self::Key>) -> Result<Vec<Self::Key>, Self::Error> {
            // Example implementation - list files in directory
            let entries = std::fs::read_dir(&self.base_path).map_err(|e| {
                NestGateError::Io {
                    message: e.to_string(),
                    operation: "list".to_string(),
                    path: Some(self.base_path.clone()),
                    retryable: false,
                    context: None,
                }
            })?;
            
            let mut keys = Vec::new();
            for entry in entries {
                let entry = entry.map_err(|e| {
                    NestGateError::Io {
                        message: e.to_string(),
                        operation: "list_entry".to_string(),
                        path: Some(self.base_path.clone()),
                        retryable: false,
                        context: None,
                    }
                })?;
                
                if let Some(name) = entry.file_name().to_str() {
                    keys.push(name.to_string());
                }
            }
            
            Ok(keys)
        }
        
        async fn exists(&self, key: Self::Key) -> Result<bool, Self::Error> {
            let path = format!("{}/{}", self.base_path, key);
            Ok(std::path::Path::new(&path).exists())
        }
        
        async fn usage_stats(&self) -> Result<crate::traits::StorageUsageStats, Self::Error> {
            // Example implementation
            Ok(crate::traits::StorageUsageStats {
                total_items: 0,
                total_size_bytes: 0,
                available_space_bytes: Some(1024 * 1024 * 1024), // 1GB example
            })
        }
    }
} 