use crate::error::NestGateError;
use std::future::Future;


use std::time::SystemTime;

use crate::{Result};
use crate::error::StorageResult; // Use canonical StorageResult

use crate::universal_storage::{
    canonical_storage::{
        CanonicalStorageBackend, CanonicalStorageHealth, CanonicalStorageMetadata,
    },
    // UnifiedStorageCapability consolidated into UnifiedServiceType
};

use super::core::EnterpriseStorageBackend;

impl CanonicalStorageBackend for EnterpriseStorageBackend {
    async fn capabilities(
        &self,
    ) -> Result<Vec<crate::unified_enums::UnifiedServiceType>> {
        // Convert ServiceCapability to UnifiedServiceType for trait compatibility
        Ok(vec![
            crate::unified_enums::UnifiedServiceType::Storage,
            crate::unified_enums::UnifiedServiceType::Network,
            crate::unified_enums::UnifiedServiceType::Security,
            crate::unified_enums::UnifiedServiceType::Monitoring,
            crate::unified_enums::UnifiedServiceType::Generic,
        ])
    }

        let path = path.to_string();
        let full_path = self.full_path(&path);
        let metrics = self.metrics.clone();

        async move {
            let start = SystemTime::now();

            let result = tokio::fs::read(&full_path)
                .await
                .map_err(|e| NestGateError::storage_error(
                    &format!("Read failed for path '{path}': {e}"),
                    "read_file",
                    Some(&path),
                ));

            let _duration = start.elapsed().unwrap_or_default();
            // Update metrics (simplified) - duration could be used for performance tracking
            if let Ok(mut metrics_guard) = metrics.try_write() {
                metrics_guard.concurrent_operations =
                    metrics_guard.concurrent_operations.saturating_add(1);
                if result.is_ok() {
                    if let Ok(data) = &result {
                        metrics_guard.throughput_mb_per_sec += ((data.len() as f64)) / 1024.0 / 1024.0;
                    }
                } else {
                    metrics_guard.error_rate += 0.01; // Increment error rate
                }
                metrics_guard.last_updated = SystemTime::now();
            }

            result
        }
    }

        let path = path.to_string();
        let full_path = self.full_path(&path);
        let data = data.to_vec();
        let metrics = self.metrics.clone();

        async move {
            let start = SystemTime::now();

            // Ensure parent directory exists
            if let Some(parent) = full_path.parent() {
                if let Err(e) = tokio::fs::create_dir_all(parent).await {
                                         return Err(NestGateError::storage_error(
                        &format!("Failed to create directory for path {path}: {e}"),
                        "write_operation",
                        Some(&path),
                    ));
                }
            }

            let result =
                tokio::fs::write(&full_path, &data)
                    .await
                    .map_err(|e| NestGateError::storage_error(
                        &format!("Failed to write to path {path}: {e}"),
                        "write_operation",
                        Some(&path),
                    ));

            let _duration = start.elapsed().unwrap_or_default();
            // Update metrics (simplified) - duration could be used for performance tracking  
            if let Ok(mut metrics_guard) = metrics.try_write() {
                metrics_guard.concurrent_operations =
                    metrics_guard.concurrent_operations.saturating_add(1);
                if result.is_ok() {
                    metrics_guard.throughput_mb_per_sec += ((data.len() as f64)) / 1024.0 / 1024.0;
                } else {
                    metrics_guard.error_rate += 0.01; // Increment error rate
                }
                metrics_guard.last_updated = SystemTime::now();
            }

            result
        }
    }

        let path = path.to_string();
        let full_path = self.full_path(&path);
        let metrics = self.metrics.clone();

        async move {
            let start = SystemTime::now();

            let result =
                tokio::fs::remove_file(&full_path)
                    .await
                    .map_err(|e| NestGateError::storage_error(
                        &format!("Failed to delete path {path}: {e}"),
                        "delete_operation",
                        Some(&path),
                    ));

            let _duration = start.elapsed().unwrap_or_default();
            // Update metrics (simplified) - duration could be used for performance tracking
            if let Ok(mut metrics_guard) = metrics.try_write() {
                metrics_guard.concurrent_operations =
                    metrics_guard.concurrent_operations.saturating_add(1);
                if result.is_err() {
                    metrics_guard.error_rate += 0.01; // Increment error rate
                }
                metrics_guard.last_updated = SystemTime::now();
            }

            result
        }
    }

        let path = path.to_string();
        let full_path = self.full_path(&path);

        async move {
            let mut entries =
                tokio::fs::read_dir(&full_path)
                    .await
                    .map_err(|e| NestGateError::storage_error(
                        &format!("Failed to list directory {path}: {e}"),
                        "list_operation",
                        Some(&path),
                    ))?;

            let mut results = Vec::new();
            while let Some(entry) =
                entries
                    .next_entry()
                    .await
                    .map_err(|e| NestGateError::storage_error(
                        &format!("Failed to read directory entry for {path}: {e}"),
                        "read_operation",
                        Some(&path),
                    ))?
            {
                if let Some(name) = entry.file_name().to_str() {
                    results.push(name.to_string());
                }
            }

            Ok(results)
        }
    }

    fn metadata(
        &self,
    ) -> impl Future<Output = StorageResult<CanonicalStorageMetadata>> + Send {
        let path = path.to_string();
        let full_path = self.full_path(&path);

        async move {
            let metadata =
                tokio::fs::metadata(&full_path)
                    .await
                    .map_err(|e| NestGateError::storage_error(
                        &format!("Failed to get metadata for {path}: {e}"),
                        "metadata_operation",
                        Some(&path),
                    ))?;

            Ok(CanonicalStorageMetadata {
                size: metadata.len(),
                created: metadata.created().unwrap_or(SystemTime::now()),
                modified: metadata.modified().unwrap_or(SystemTime::now()),
                is_directory: metadata.is_dir(),
                permissions: None,
            })
        }
    }

    fn health_check(&self) -> impl Future<Output = Result<CanonicalStorageHealth>> + Send {
        let root_path = self.root_path().to_path_buf();

        async move {
            let is_healthy = root_path.exists() && root_path.is_dir();

            // Simplified health check (disk space info would require additional implementation)
            Ok(CanonicalStorageHealth {
                is_healthy,
                backend_type: "enterprise_filesystem".to_string(),
                available_space: None, // Could implement if needed
                total_space: None,     // Could implement if needed
                last_check: SystemTime::now(),
            })
        }
    }
}

impl EnterpriseStorageBackend {
    /// Get disk space information for health checks
    async fn get_disk_space_info(&self) -> Result<(Option<u64>, Option<u64>)> {
        // Platform-specific disk space calculation would go here
        // For now, return None to indicate information unavailable
        Ok((None, None))
    }
}
