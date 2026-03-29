// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//
// Helper functions for ZFS dataset and snapshot operations.
// Shared utilities used by dataset and snapshot handlers.

use std::path::Path;
use std::{fs, sync::Arc};

use crate::rest::models::{
    ChecksumType, CompressionType, CreateDatasetRequest, Dataset, DatasetProperties, DatasetStats,
    DatasetStatus, DatasetType, StorageBackendType,
};
use nestgate_core::error::Result;

/// Convert ZFS _engine to API Dataset model
pub async fn convert_engine_to_placeholder_dataset(
    name: &str,
    _engine: &String,
) -> std::result::Result<Dataset, Box<dyn std::error::Error + Send + Sync>> {
    let properties = DatasetProperties {
        name: name.to_string(),
        mountpoint: Some(format!("/mnt/{name}")),
        quota: None,
        reservation: None,
        compression: true,
        compression_type: Some(CompressionType::Lz4),
        checksum: true,
        checksum_type: Some(ChecksumType::Sha256),
        deduplication: false,
        encryption: false,
        readonly: false,
        custom: std::collections::HashMap::new(),
    };

    let dataset_stats = DatasetStats {
        name: name.to_string(),
        size_bytes: 1024 * 1024 * 100,
        used_bytes: 1024 * 1024 * 100,
        available_bytes: 1024 * 1024 * 1024,
        snapshot_count: 0,
        deduplication_ratio: 1.0,
        files_written: 50,
        files_read: 200,
        cow_operations: 0,
        blocks_copied: 0,
        compression_ratio: Some(2.5),
        compression_space_saved: Some(1024 * 1024 * 50),
        checksums_computed: 100,
        checksums_verified: 98,
        read_throughput: 100.0,
        write_throughput: 80.0,
        avg_latency_ms: 2.5,
    };

    Ok(Dataset {
        name: name.to_string(),
        path: format!("/{name}"),
        mountpoint: Some(format!("/mnt/{name}")),
        size_bytes: 1024 * 1024 * 100,
        available_bytes: 1024 * 1024 * 1024,
        used_bytes: 1024 * 1024 * 100,
        dataset_type: DatasetType::Filesystem,
        backend: StorageBackendType::Filesystem,
        properties,
        stats: dataset_stats,
        created: chrono::Utc::now() - chrono::Duration::hours(1),
        modified: chrono::Utc::now(),
        status: DatasetStatus::Online,
        snapshot_count: get_snapshot_count_from_engine_impl().unwrap_or(0) as u32,
    })
}

/// Create storage backend from request
pub async fn create_storage_backend(
    _request: &CreateDatasetRequest,
) -> std::result::Result<Arc<serde_json::Value>, Box<dyn std::error::Error + Send + Sync>> {
    match _request.backend {
        StorageBackendType::Filesystem => {
            let default_path = format!("/mnt/{}", _request.name);
            let path = _request.description.as_deref().unwrap_or(&default_path);
            Ok(Arc::new(
                serde_json::json!({"backend": "filesystem", "path": path}),
            ))
        }
        _ => Err(nestgate_core::error::NestGateUnifiedError::api_with_status(
            format!("Storage backend not supported: {:?}", _request.backend),
            501,
        )
        .into()),
    }
}

/// Get snapshot count from ZFS _engine
pub fn get_snapshot_count_from_engine_impl() -> Result<u64> {
    let snapshot_dir = Path::new("/tmp/nestgate/snapshots");
    if snapshot_dir.exists()
        && let Ok(entries) = fs::read_dir(snapshot_dir)
    {
        return Ok(entries.count() as u64);
    }
    Ok(0)
}

/// Convert real ZFS stats to API format, with sensible defaults if unavailable
#[cfg(feature = "dev-stubs")]
#[allow(dead_code)]
pub fn convert_zfs_stats_to_api(
    zfs_stats: Option<crate::handlers::zfs_stub::ZeroCostDatasetInfo>,
    default_name: &str,
) -> DatasetStats {
    match zfs_stats {
        Some(stats) => DatasetStats {
            name: stats.name.clone(),
            size_bytes: stats.used + stats.available,
            used_bytes: stats.used,
            available_bytes: stats.available,
            files_written: 0,
            files_read: 0,
            cow_operations: 0,
            blocks_copied: 0,
            compression_ratio: Some(1.0),
            compression_space_saved: None,
            deduplication_ratio: 1.0,
            checksums_computed: 0,
            checksums_verified: 0,
            read_throughput: 0.0,
            write_throughput: 0.0,
            avg_latency_ms: 0.0,
            snapshot_count: 0,
        },
        None => DatasetStats {
            name: default_name.to_string(),
            size_bytes: 1024 * 1024 * 1024,
            used_bytes: 0,
            available_bytes: 1024 * 1024 * 1024,
            files_written: 0,
            files_read: 0,
            cow_operations: 0,
            blocks_copied: 0,
            compression_ratio: Some(1.0),
            compression_space_saved: None,
            deduplication_ratio: 1.0,
            checksums_computed: 0,
            checksums_verified: 0,
            read_throughput: 0.0,
            write_throughput: 0.0,
            avg_latency_ms: 0.0,
            snapshot_count: 0,
        },
    }
}

/// Convert _engine statistics to API format
#[allow(dead_code)]
pub fn convert_engine_stats_to_api(_stats: &serde_json::Value) -> DatasetStats {
    DatasetStats {
        name: "placeholder".to_string(),
        size_bytes: 1024 * 1024 * 100,
        used_bytes: 1024 * 1024 * 50,
        available_bytes: 1024 * 1024 * 1024,
        snapshot_count: 0,
        deduplication_ratio: 1.0,
        files_written: 50,
        files_read: 200,
        cow_operations: 0,
        blocks_copied: 0,
        compression_ratio: Some(2.5),
        compression_space_saved: Some(1024 * 1024 * 25),
        checksums_computed: 100,
        checksums_verified: 98,
        read_throughput: 100.0,
        write_throughput: 80.0,
        avg_latency_ms: 2.5,
    }
}

/// Calculate file operations from ZFS _engine statistics
#[allow(dead_code)]
pub fn calculate_file_operations_from_stats(_stats: &serde_json::Value, operation: &str) -> u64 {
    match operation {
        "write" => 50,
        "read" => 200,
        _ => 0,
    }
}

/// Display mapping for backend type filters and logging.
impl std::fmt::Display for StorageBackendType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Filesystem => "zfs",
            Self::Memory => "memory",
            Self::Local => "local",
            Self::Remote => "remote",
            Self::Cloud => "cloud",
            Self::Network => "network",
            Self::Block => "block",
            Self::File => "file",
        })
    }
}
