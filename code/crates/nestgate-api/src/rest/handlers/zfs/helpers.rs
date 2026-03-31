// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//
// Helper functions for ZFS dataset and snapshot operations.
// Shared utilities used by dataset and snapshot handlers.

//! Public ZFS/engine stat bridges are reserved until full engine integration is wired.

use std::path::Path;
use std::{fs, sync::Arc};

use crate::rest::models::{
    ChecksumType, CompressionType, CreateDatasetRequest, Dataset, DatasetProperties, DatasetStats,
    DatasetStatus, DatasetType, StorageBackendType,
};
fn default_mount_path_for_dataset(name: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(format!("/mnt/{name}"))
}

/// Build [`DatasetStats`] for a registered dataset name using filesystem space when the mount exists.
pub fn dataset_stats_for_name(name: &str) -> DatasetStats {
    let mount = default_mount_path_for_dataset(name);
    let (size_bytes, used_bytes, available_bytes) = statvfs_bytes_for_path(&mount);
    DatasetStats {
        name: name.to_string(),
        size_bytes,
        used_bytes,
        available_bytes,
        snapshot_count: get_snapshot_count_from_engine_impl() as u32,
        deduplication_ratio: 1.0,
        files_written: 0,
        files_read: 0,
        cow_operations: 0,
        blocks_copied: 0,
        compression_ratio: None,
        compression_space_saved: None,
        checksums_computed: 0,
        checksums_verified: 0,
        read_throughput: 0.0,
        write_throughput: 0.0,
        avg_latency_ms: 0.0,
    }
}

fn statvfs_bytes_for_path(path: &Path) -> (u64, u64, u64) {
    #[cfg(target_os = "linux")]
    {
        if path.exists()
            && let Ok((total, avail)) = nestgate_core::linux_proc::statvfs_space(path)
        {
            let used = total.saturating_sub(avail);
            return (total, used, avail);
        }
    }
    (0, 0, 0)
}

/// Convert a registered engine entry to the API [`Dataset`] model (no fabricated throughput/latency).
pub fn convert_engine_to_placeholder_dataset(name: &str, _engine: &String) -> Dataset {
    let mount_str = format!("/mnt/{name}");
    let mount_path = Path::new(&mount_str);
    let (size_bytes, used_bytes, available_bytes) = statvfs_bytes_for_path(mount_path);

    let properties = DatasetProperties {
        name: name.to_string(),
        mountpoint: Some(mount_str.clone()),
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

    let dataset_stats = dataset_stats_for_name(name);

    Dataset {
        name: name.to_string(),
        path: format!("/{name}"),
        mountpoint: Some(mount_str),
        size_bytes,
        available_bytes,
        used_bytes,
        dataset_type: DatasetType::Filesystem,
        backend: StorageBackendType::Filesystem,
        properties,
        stats: dataset_stats,
        created: chrono::Utc::now() - chrono::Duration::hours(1),
        modified: chrono::Utc::now(),
        status: DatasetStatus::Online,
        snapshot_count: get_snapshot_count_from_engine_impl() as u32,
    }
}

/// Create storage backend from request
pub fn create_storage_backend(
    request: &CreateDatasetRequest,
) -> std::result::Result<Arc<serde_json::Value>, Box<dyn std::error::Error + Send + Sync>> {
    match request.backend {
        StorageBackendType::Filesystem => {
            let default_path = format!("/mnt/{}", request.name);
            let path = request.description.as_deref().unwrap_or(&default_path);
            Ok(Arc::new(
                serde_json::json!({"backend": "filesystem", "path": path}),
            ))
        }
        _ => Err(nestgate_core::error::NestGateUnifiedError::api_with_status(
            format!("Storage backend not supported: {:?}", request.backend),
            501,
        )
        .into()),
    }
}

/// Get snapshot count from ZFS engine.
pub fn get_snapshot_count_from_engine_impl() -> u64 {
    let base = std::env::var("NESTGATE_DATA_DIR").unwrap_or_else(|_| {
        std::env::temp_dir()
            .join("nestgate")
            .to_string_lossy()
            .into_owned()
    });
    let snapshot_dir = Path::new(&base).join("snapshots");
    if snapshot_dir.exists()
        && let Ok(entries) = fs::read_dir(snapshot_dir)
    {
        return entries.count() as u64;
    }
    0
}

/// Convert real ZFS stats to API format, with sensible defaults if unavailable
#[cfg(feature = "dev-stubs")]
#[allow(
    dead_code,
    reason = "Used when dev-stubs dataset handlers are wired to the REST layer"
)]
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

/// Convert engine JSON statistics to API format (unknown structure → zeros, not fabricated values).
#[allow(
    dead_code,
    reason = "Reserved for engine JSON bridge once storage handlers deserialize live stats"
)]
pub fn convert_engine_stats_to_api(stats: &serde_json::Value) -> DatasetStats {
    let name = stats
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();
    DatasetStats {
        name,
        size_bytes: 0,
        used_bytes: 0,
        available_bytes: 0,
        snapshot_count: 0,
        deduplication_ratio: 1.0,
        files_written: 0,
        files_read: 0,
        cow_operations: 0,
        blocks_copied: 0,
        compression_ratio: None,
        compression_space_saved: None,
        checksums_computed: 0,
        checksums_verified: 0,
        read_throughput: 0.0,
        write_throughput: 0.0,
        avg_latency_ms: 0.0,
    }
}

/// Calculate file operations from ZFS _engine statistics
#[allow(
    dead_code,
    reason = "Placeholder until engine exposes file op counters in JSON stats"
)]
pub const fn calculate_file_operations_from_stats(
    _stats: &serde_json::Value,
    _operation: &str,
) -> u64 {
    0
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
