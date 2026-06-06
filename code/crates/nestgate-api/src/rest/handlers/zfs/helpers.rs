// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// Helper functions for ZFS dataset and snapshot operations.
// Shared utilities used by dataset and snapshot handlers.

//! Helpers for ZFS dataset and snapshot REST handlers.

use std::path::Path;
use std::{collections::HashMap, fs, sync::Arc};

use crate::rest::models::{
    ChecksumType, CompressionType, CreateDatasetRequest, Dataset, DatasetProperties, DatasetStats,
    DatasetStatus, DatasetType, StorageBackendType,
};
use nestgate_core::error::{NestGateError, Result};

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
        snapshot_count: u32::try_from(get_snapshot_count_from_engine_impl()).unwrap_or(u32::MAX),
        deduplication_ratio: 0.0,
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

fn parse_engine_json(engine: &str) -> Option<serde_json::Value> {
    serde_json::from_str(engine).ok()
}

fn json_bool(value: &serde_json::Value, key: &str) -> Option<bool> {
    value.get(key).and_then(serde_json::Value::as_bool)
}

fn json_u64(value: &serde_json::Value, key: &str) -> Option<u64> {
    value.get(key).and_then(serde_json::Value::as_u64)
}

#[expect(
    clippy::expect_used,
    reason = "1970-01-01T00:00:00Z is always representable as chrono::Utc DateTime"
)]
const fn unknown_timestamp() -> chrono::DateTime<chrono::Utc> {
    chrono::DateTime::from_timestamp(0, 0).expect("valid unix epoch timestamp")
}

fn json_str(value: &serde_json::Value, key: &str) -> Option<String> {
    value
        .get(key)
        .and_then(|v| v.as_str())
        .map(str::to_string)
}

fn parse_compression_type(value: &serde_json::Value) -> Option<CompressionType> {
    value.as_str().and_then(|raw| {
        match raw.to_ascii_lowercase().as_str() {
            "lz4" => Some(CompressionType::Lz4),
            "gzip" | "gz" => Some(CompressionType::Gzip),
            "zstd" | "zstandard" => Some(CompressionType::Zstd),
            "off" | "none" => Some(CompressionType::None),
            _ => None,
        }
    })
}

fn parse_checksum_type(value: &serde_json::Value) -> Option<ChecksumType> {
    value.as_str().and_then(|raw| {
        match raw.to_ascii_lowercase().as_str() {
            "fletcher2" => Some(ChecksumType::Fletcher2),
            "fletcher4" => Some(ChecksumType::Fletcher4),
            "sha256" => Some(ChecksumType::Sha256),
            "sha512" => Some(ChecksumType::Sha512),
            "skein" => Some(ChecksumType::Skein),
            "edonr" | "edon-r" => Some(ChecksumType::EdonR),
            _ => None,
        }
    })
}

fn parse_dataset_status(value: &serde_json::Value) -> Option<DatasetStatus> {
    value.as_str().and_then(|raw| {
        match raw.to_ascii_lowercase().as_str() {
            "online" => Some(DatasetStatus::Online),
            "offline" => Some(DatasetStatus::Offline),
            "degraded" => Some(DatasetStatus::Degraded),
            "maintenance" => Some(DatasetStatus::Maintenance),
            "error" => Some(DatasetStatus::Error),
            _ => None,
        }
    })
}

fn parse_dataset_type(value: &serde_json::Value) -> Option<DatasetType> {
    value.as_str().and_then(|raw| {
        match raw.to_ascii_lowercase().as_str() {
            "filesystem" => Some(DatasetType::Filesystem),
            "volume" => Some(DatasetType::Volume),
            "snapshot" => Some(DatasetType::Snapshot),
            "bookmark" => Some(DatasetType::Bookmark),
            _ => None,
        }
    })
}

fn parse_storage_backend(value: &serde_json::Value) -> Option<StorageBackendType> {
    value.as_str().and_then(|raw| {
        match raw.to_ascii_lowercase().as_str() {
            "filesystem" | "zfs" => Some(StorageBackendType::Filesystem),
            "memory" => Some(StorageBackendType::Memory),
            "local" => Some(StorageBackendType::Local),
            "remote" => Some(StorageBackendType::Remote),
            "cloud" => Some(StorageBackendType::Cloud),
            "network" => Some(StorageBackendType::Network),
            "block" => Some(StorageBackendType::Block),
            "file" => Some(StorageBackendType::File),
            _ => None,
        }
    })
}

fn parse_datetime(value: &serde_json::Value) -> Option<chrono::DateTime<chrono::Utc>> {
    if let Some(text) = value.as_str() {
        return chrono::DateTime::parse_from_rfc3339(text)
            .ok()
            .map(|dt| dt.with_timezone(&chrono::Utc));
    }
    value
        .as_i64()
        .and_then(|secs| chrono::DateTime::from_timestamp(secs, 0))
}

fn mount_path_for_engine(name: &str, engine: Option<&serde_json::Value>) -> Option<String> {
    engine
        .and_then(|v| json_str(v, "mountpoint").or_else(|| json_str(v, "path")))
        .or_else(|| {
            let default = default_mount_path_for_dataset(name);
            if default.exists() {
                Some(default.to_string_lossy().into_owned())
            } else {
                None
            }
        })
}

fn dataset_properties_from_engine(name: &str, engine: Option<&serde_json::Value>) -> DatasetProperties {
    let compression = engine.and_then(|v| json_bool(v, "compression")).unwrap_or(false);
    let compression_type = engine
        .and_then(|v| v.get("compression_type"))
        .and_then(parse_compression_type);
    let checksum = engine.and_then(|v| json_bool(v, "checksum")).unwrap_or(false);
    let checksum_type = engine
        .and_then(|v| v.get("checksum_type"))
        .and_then(parse_checksum_type);
    let deduplication = engine
        .and_then(|v| json_bool(v, "deduplication"))
        .unwrap_or(false);
    let encryption = engine.and_then(|v| json_bool(v, "encryption")).unwrap_or(false);
    let readonly = engine.and_then(|v| json_bool(v, "readonly")).unwrap_or(false);
    let quota = engine.and_then(|v| json_u64(v, "quota"));
    let reservation = engine.and_then(|v| json_u64(v, "reservation"));
    let mountpoint = mount_path_for_engine(name, engine);

    let mut custom = HashMap::new();
    match engine {
        None => {
            custom.insert(String::from("engine_metadata"), String::from("unavailable"));
        }
        Some(value) => {
            let status_present = value
                .get("status")
                .is_some_and(|status| parse_dataset_status(status).is_some());
            if !status_present {
                custom.insert(String::from("status"), String::from("unknown"));
            }
        }
    }

    DatasetProperties {
        name: name.to_string(),
        mountpoint,
        quota,
        reservation,
        compression,
        compression_type,
        checksum,
        checksum_type,
        deduplication,
        encryption,
        readonly,
        custom,
    }
}

/// Convert a registered engine entry to the API [`Dataset`] model.
///
/// Parses JSON engine metadata when present; fields absent from the engine entry
/// are left unset (`None`, `false`, or epoch timestamps) rather than fabricated.
pub fn convert_engine_entry_to_dataset(name: &str, engine: &str) -> Dataset {
    let engine_json = parse_engine_json(engine);
    let mount_str = mount_path_for_engine(name, engine_json.as_ref());
    let default_mount = default_mount_path_for_dataset(name);
    let mount_path = mount_str
        .as_deref()
        .map_or(default_mount.as_path(), Path::new);

    let (stat_size, stat_used, stat_avail) = statvfs_bytes_for_path(mount_path);
    let size_bytes = engine_json
        .as_ref()
        .and_then(|v| json_u64(v, "size_bytes"))
        .unwrap_or(stat_size);
    let used_bytes = engine_json
        .as_ref()
        .and_then(|v| json_u64(v, "used_bytes"))
        .unwrap_or(stat_used);
    let available_bytes = engine_json
        .as_ref()
        .and_then(|v| json_u64(v, "available_bytes"))
        .unwrap_or(stat_avail);

    let properties = dataset_properties_from_engine(name, engine_json.as_ref());
    let mut dataset_stats = dataset_stats_for_name(name);
    dataset_stats.size_bytes = size_bytes;
    dataset_stats.used_bytes = used_bytes;
    dataset_stats.available_bytes = available_bytes;

    let dataset_type = engine_json
        .as_ref()
        .and_then(|v| v.get("dataset_type"))
        .and_then(parse_dataset_type)
        .unwrap_or(DatasetType::Filesystem);
    let backend = engine_json
        .as_ref()
        .and_then(|v| v.get("backend"))
        .and_then(parse_storage_backend)
        .unwrap_or(StorageBackendType::Filesystem);
    let path = engine_json
        .as_ref()
        .and_then(|v| json_str(v, "path"))
        .unwrap_or_else(|| format!("/{name}"));
    let created = engine_json
        .as_ref()
        .and_then(|v| v.get("created"))
        .and_then(parse_datetime)
        .unwrap_or_else(unknown_timestamp);
    let modified = engine_json
        .as_ref()
        .and_then(|v| v.get("modified"))
        .and_then(parse_datetime)
        .unwrap_or_else(unknown_timestamp);
    let status = engine_json
        .as_ref()
        .and_then(|v| v.get("status"))
        .and_then(parse_dataset_status)
        .unwrap_or(DatasetStatus::Maintenance);

    Dataset {
        name: name.to_string(),
        path,
        mountpoint: mount_str,
        size_bytes,
        available_bytes,
        used_bytes,
        dataset_type,
        backend,
        properties,
        stats: dataset_stats,
        created,
        modified,
        status,
        snapshot_count: u32::try_from(get_snapshot_count_from_engine_impl()).unwrap_or(u32::MAX),
    }
}

/// Create storage backend from request
pub fn create_storage_backend(request: &CreateDatasetRequest) -> Result<Arc<serde_json::Value>> {
    match request.backend {
        StorageBackendType::Filesystem => {
            let default_path = format!("/mnt/{}", request.name);
            let path = request.description.as_deref().unwrap_or(&default_path);
            Ok(Arc::new(
                serde_json::json!({"backend": "filesystem", "path": path}),
            ))
        }
        _ => Err(NestGateError::api_with_status(
            format!("Storage backend not supported: {:?}", request.backend),
            501,
        )),
    }
}

/// Serialize engine metadata for a newly registered dataset entry.
pub fn engine_entry_json_for_create(request: &CreateDatasetRequest) -> String {
    let default_path = format!("/mnt/{}", request.name);
    let path = request
        .description
        .as_deref()
        .unwrap_or(&default_path)
        .to_string();
    let properties = request.properties.as_ref();

    serde_json::json!({
        "backend": request.backend,
        "path": path,
        "dataset_type": request.dataset_type,
        "compression": properties.map(|p| p.compression),
        "compression_type": properties.and_then(|p| p.compression_type.as_ref()),
        "checksum": properties.map(|p| p.checksum),
        "checksum_type": properties.and_then(|p| p.checksum_type.as_ref()),
        "deduplication": properties.map(|p| p.deduplication),
        "encryption": properties.map(|p| p.encryption),
        "readonly": properties.map(|p| p.readonly),
        "quota": request.quota.or_else(|| properties.and_then(|p| p.quota)),
        "reservation": properties.and_then(|p| p.reservation),
        "created": chrono::Utc::now(),
    })
    .to_string()
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parse_compression_known_types() {
        for (input, expected) in [
            ("lz4", CompressionType::Lz4),
            ("gzip", CompressionType::Gzip),
            ("gz", CompressionType::Gzip),
            ("zstd", CompressionType::Zstd),
            ("zstandard", CompressionType::Zstd),
            ("off", CompressionType::None),
            ("none", CompressionType::None),
        ] {
            assert_eq!(
                parse_compression_type(&json!(input)),
                Some(expected),
                "failed for {input}"
            );
        }
    }

    #[test]
    fn parse_compression_unknown_returns_none() {
        assert!(parse_compression_type(&json!("brotli")).is_none());
        assert!(parse_compression_type(&json!(42)).is_none());
    }

    #[test]
    fn parse_checksum_known_types() {
        for (input, expected) in [
            ("fletcher2", ChecksumType::Fletcher2),
            ("fletcher4", ChecksumType::Fletcher4),
            ("sha256", ChecksumType::Sha256),
            ("sha512", ChecksumType::Sha512),
            ("skein", ChecksumType::Skein),
            ("edonr", ChecksumType::EdonR),
            ("edon-r", ChecksumType::EdonR),
        ] {
            assert_eq!(
                parse_checksum_type(&json!(input)),
                Some(expected),
                "failed for {input}"
            );
        }
    }

    #[test]
    fn parse_checksum_unknown_returns_none() {
        assert!(parse_checksum_type(&json!("blake3")).is_none());
    }

    #[test]
    fn parse_dataset_status_known_types() {
        for (input, expected) in [
            ("online", DatasetStatus::Online),
            ("offline", DatasetStatus::Offline),
            ("degraded", DatasetStatus::Degraded),
            ("maintenance", DatasetStatus::Maintenance),
            ("error", DatasetStatus::Error),
        ] {
            assert_eq!(
                parse_dataset_status(&json!(input)),
                Some(expected),
                "failed for {input}"
            );
        }
    }

    #[test]
    fn parse_dataset_status_unknown_returns_none() {
        assert!(parse_dataset_status(&json!("rebuilding")).is_none());
    }

    #[test]
    fn parse_dataset_type_known_types() {
        for (input, expected) in [
            ("filesystem", DatasetType::Filesystem),
            ("volume", DatasetType::Volume),
            ("snapshot", DatasetType::Snapshot),
            ("bookmark", DatasetType::Bookmark),
        ] {
            assert_eq!(
                parse_dataset_type(&json!(input)),
                Some(expected),
                "failed for {input}"
            );
        }
    }

    #[test]
    fn parse_storage_backend_known_types() {
        for (input, expected) in [
            ("filesystem", StorageBackendType::Filesystem),
            ("zfs", StorageBackendType::Filesystem),
            ("memory", StorageBackendType::Memory),
            ("local", StorageBackendType::Local),
            ("remote", StorageBackendType::Remote),
            ("cloud", StorageBackendType::Cloud),
            ("network", StorageBackendType::Network),
            ("block", StorageBackendType::Block),
            ("file", StorageBackendType::File),
        ] {
            assert_eq!(
                parse_storage_backend(&json!(input)),
                Some(expected),
                "failed for {input}"
            );
        }
    }

    #[test]
    fn parse_datetime_rfc3339() {
        let v = json!("2026-06-06T12:00:00Z");
        let dt = parse_datetime(&v).unwrap();
        assert_eq!(dt.year(), 2026);
    }

    #[test]
    fn parse_datetime_unix_epoch() {
        let v = json!(1717660800_i64);
        let dt = parse_datetime(&v).unwrap();
        assert!(dt.timestamp() > 0);
    }

    #[test]
    fn parse_datetime_bad_string_returns_none() {
        assert!(parse_datetime(&json!("not-a-date")).is_none());
    }

    #[test]
    fn convert_engine_entry_with_rich_json() {
        let engine = json!({
            "backend": "zfs",
            "path": "/tank/data",
            "dataset_type": "filesystem",
            "compression": true,
            "compression_type": "lz4",
            "checksum": true,
            "checksum_type": "sha256",
            "deduplication": false,
            "encryption": true,
            "readonly": false,
            "status": "online",
            "size_bytes": 1000,
            "used_bytes": 500,
            "available_bytes": 500,
            "created": "2026-01-01T00:00:00Z",
            "modified": "2026-06-01T00:00:00Z"
        })
        .to_string();

        let ds = convert_engine_entry_to_dataset("tank/data", &engine);
        assert_eq!(ds.name, "tank/data");
        assert_eq!(ds.path, "/tank/data");
        assert_eq!(ds.backend, StorageBackendType::Filesystem);
        assert_eq!(ds.dataset_type, DatasetType::Filesystem);
        assert_eq!(ds.status, DatasetStatus::Online);
        assert_eq!(ds.size_bytes, 1000);
        assert!(ds.properties.compression);
        assert_eq!(ds.properties.compression_type, Some(CompressionType::Lz4));
        assert!(ds.properties.encryption);
    }

    #[test]
    fn convert_engine_entry_with_invalid_json() {
        let ds = convert_engine_entry_to_dataset("test-ds", "not-json");
        assert_eq!(ds.name, "test-ds");
        assert_eq!(ds.backend, StorageBackendType::Filesystem);
        assert_eq!(ds.status, DatasetStatus::Maintenance);
    }

    #[test]
    fn create_storage_backend_filesystem_succeeds() {
        let req = CreateDatasetRequest {
            name: String::from("testpool/fs"),
            dataset_type: DatasetType::Filesystem,
            backend: StorageBackendType::Filesystem,
            properties: None,
            quota: None,
            description: Some(String::from("/custom/path")),
        };
        let result = create_storage_backend(&req).unwrap();
        assert_eq!(result["backend"], "filesystem");
        assert_eq!(result["path"], "/custom/path");
    }

    #[test]
    fn create_storage_backend_unsupported_returns_error() {
        let req = CreateDatasetRequest {
            name: String::from("test"),
            dataset_type: DatasetType::Filesystem,
            backend: StorageBackendType::Cloud,
            properties: None,
            quota: None,
            description: None,
        };
        assert!(create_storage_backend(&req).is_err());
    }

    #[test]
    fn engine_entry_json_for_create_includes_fields() {
        let req = CreateDatasetRequest {
            name: String::from("testpool/new"),
            dataset_type: DatasetType::Filesystem,
            backend: StorageBackendType::Filesystem,
            properties: None,
            quota: Some(1024),
            description: Some(String::from("/mnt/data")),
        };
        let json_str = engine_entry_json_for_create(&req);
        let v: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        assert_eq!(v["path"], "/mnt/data");
        assert!(v.get("created").is_some());
        assert_eq!(v["quota"], 1024);
    }

    #[test]
    fn dataset_properties_from_engine_none_uses_defaults() {
        let props = dataset_properties_from_engine("test", None);
        assert!(!props.compression);
        assert!(!props.checksum);
        assert!(!props.deduplication);
        assert!(
            props
                .custom
                .contains_key("engine_metadata"),
        );
    }

    #[test]
    fn dataset_properties_from_engine_rich_json() {
        let v = json!({
            "compression": true,
            "compression_type": "zstd",
            "deduplication": true,
            "quota": 500,
        });
        let props = dataset_properties_from_engine("test", Some(&v));
        assert!(props.compression);
        assert_eq!(props.compression_type, Some(CompressionType::Zstd));
        assert!(props.deduplication);
        assert_eq!(props.quota, Some(500));
    }

    use chrono::Datelike as _;
}
