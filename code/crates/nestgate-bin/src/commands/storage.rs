// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Storage management commands
//!
//! Provides storage backend listing, scanning, benchmarking, and configuration.

use crate::cli::StorageAction;
use anyhow::Result;
use nestgate_types::{EnvSource, ProcessEnv, env_var_or_default};

/// Execute storage management commands
pub async fn execute(action: StorageAction) -> Result<()> {
    match action {
        StorageAction::List => list_backends().await,
        StorageAction::Scan {
            path,
            cloud,
            network,
        } => scan_storage(path, cloud, network).await,
        StorageAction::Benchmark {
            backend,
            duration,
            size,
        } => benchmark_storage(&backend, duration, size).await,
        StorageAction::Configure { backend, set } => configure_storage(&backend, &set).await,
    }
}

/// List all available and detected storage backends
async fn list_backends() -> Result<()> {
    list_backends_from_env_source(&ProcessEnv).await
}

async fn list_backends_from_env_source(env: &(impl EnvSource + ?Sized)) -> Result<()> {
    println!("NestGate Storage Backends");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // Detect backend capabilities from the storage service layer
    let caps = nestgate_core::services::storage::capabilities::detect_backend();
    println!("\nActive Backend:");
    println!("  Type:       {:?}", caps.backend_type);
    println!("  Features:");
    if caps.native_snapshots {
        println!("    snapshots");
    }
    if caps.native_deduplication {
        println!("    deduplication");
    }
    if caps.native_compression {
        println!("    compression");
    }
    if caps.native_checksums {
        println!("    checksums");
    }
    if caps.native_replication {
        println!("    replication");
    }
    if caps.basic_operations {
        println!("    basic_operations");
    }

    // Storage path
    let storage_path =
        env_var_or_default(env, "NESTGATE_STORAGE_PATH", "/var/lib/nestgate/storage");
    println!("\nStorage Path: {storage_path}");

    if std::path::Path::new(&storage_path).exists() {
        // Check available space
        println!("  Status: Exists");
        if let Ok(metadata) = tokio::fs::metadata(&storage_path).await
            && metadata.is_dir()
        {
            println!("  Type:   Directory");
        }
    } else {
        println!("  Status: Not found (will be created on first use)");
    }

    // List supported backends
    println!("\nSupported Backends:");
    println!("  filesystem  - Local filesystem (ext4, xfs, btrfs, etc.)");
    println!("  zfs         - ZFS with snapshots, dedup, compression");
    println!("  tmpfs       - In-memory temporary storage");
    println!("  s3          - AWS S3 compatible (future)");
    println!("  nfs         - Network File System (future)");

    Ok(())
}

/// Scan for available storage at a path
async fn scan_storage(path: std::path::PathBuf, cloud: bool, network: bool) -> Result<()> {
    scan_storage_from_env_source(&ProcessEnv, path, cloud, network).await
}

async fn scan_storage_from_env_source(
    env: &(impl EnvSource + ?Sized),
    path: std::path::PathBuf,
    cloud: bool,
    network: bool,
) -> Result<()> {
    println!("Scanning for storage at: {}", path.display());

    if !path.exists() {
        println!("  Path does not exist");
        return Ok(());
    }

    let metadata = tokio::fs::metadata(&path).await?;
    println!(
        "  Type: {}",
        if metadata.is_dir() {
            "Directory"
        } else {
            "File"
        }
    );
    println!("  Readable: yes");

    // Check write permission by attempting to create a temp file
    let test_path = path.join(".nestgate_scan_test");
    let writable = tokio::fs::write(&test_path, b"test").await.is_ok();
    if writable {
        let _ = tokio::fs::remove_file(&test_path).await;
        println!("  Writable: yes");
    } else {
        println!("  Writable: no");
    }

    // Detect filesystem type
    let caps = nestgate_core::services::storage::capabilities::detect_backend();
    println!("  Backend:  {:?}", caps.backend_type);

    // Scan for existing NestGate data
    let nestgate_data = path.join("nestgate");
    if nestgate_data.exists() {
        println!("\n  Found existing NestGate data directory");
        if let Ok(mut entries) = tokio::fs::read_dir(&nestgate_data).await {
            let mut count = 0;
            while let Ok(Some(_)) = entries.next_entry().await {
                count += 1;
            }
            println!("  Objects: {count}");
        }
    }

    if cloud {
        println!("\n  Cloud scanning: Environment-based detection");
        if env.get("AWS_ACCESS_KEY_ID").is_some() {
            println!("    AWS credentials detected");
        }
        if env.get("AZURE_STORAGE_ACCOUNT").is_some() {
            println!("    Azure credentials detected");
        }
        if env.get("GOOGLE_APPLICATION_CREDENTIALS").is_some() {
            println!("    GCP credentials detected");
        }
    }

    if network {
        println!("\n  Network scanning: checking for NFS/SMB mounts");
        #[cfg(unix)]
        {
            // Check /proc/mounts for network filesystems
            if let Ok(mounts) = tokio::fs::read_to_string("/proc/mounts").await {
                let nfs_count = mounts.lines().filter(|l| l.contains("nfs")).count();
                let smb_count = mounts.lines().filter(|l| l.contains("cifs")).count();
                println!("    NFS mounts:  {nfs_count}");
                println!("    SMB/CIFS mounts: {smb_count}");
            }
        }
    }

    Ok(())
}

/// Run storage performance benchmark
async fn benchmark_storage(backend: &str, duration: u64, size_mb: u64) -> Result<()> {
    benchmark_storage_from_env_source(&ProcessEnv, backend, duration, size_mb).await
}

async fn benchmark_storage_from_env_source(
    env: &(impl EnvSource + ?Sized),
    backend: &str,
    duration: u64,
    size_mb: u64,
) -> Result<()> {
    println!("NestGate Storage Benchmark");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  Backend:    {backend}");
    println!("  Duration:   {duration}s");
    println!("  Test Size:  {size_mb} MB");
    println!();

    let storage_path = env_var_or_default(env, "NESTGATE_STORAGE_PATH", "/tmp/nestgate-benchmark");
    let bench_dir = std::path::PathBuf::from(&storage_path).join("benchmark");
    tokio::fs::create_dir_all(&bench_dir).await?;

    let size_bytes = size_mb * 1024 * 1024;
    let data = vec![0xABu8; usize::try_from(size_bytes).unwrap_or(usize::MAX)];

    // Write benchmark
    println!("Write benchmark...");
    let write_start = std::time::Instant::now();
    let write_path = bench_dir.join("bench_data");
    tokio::fs::write(&write_path, &data).await?;
    let write_elapsed = write_start.elapsed();
    #[expect(clippy::cast_precision_loss, reason = "display-only benchmark output")]
    let write_mbps = size_mb as f64 / write_elapsed.as_secs_f64();
    println!(
        "  Write: {:.1} MB/s ({:.2}ms for {} MB)",
        write_mbps,
        write_elapsed.as_millis(),
        size_mb
    );

    // Read benchmark
    println!("Read benchmark...");
    let read_start = std::time::Instant::now();
    let _read_data = tokio::fs::read(&write_path).await?;
    let read_elapsed = read_start.elapsed();
    #[expect(clippy::cast_precision_loss, reason = "display-only benchmark output")]
    let read_mbps = size_mb as f64 / read_elapsed.as_secs_f64();
    println!(
        "  Read:  {:.1} MB/s ({:.2}ms for {} MB)",
        read_mbps,
        read_elapsed.as_millis(),
        size_mb
    );

    // Cleanup
    let _ = tokio::fs::remove_dir_all(&bench_dir).await;

    println!("\nSummary:");
    println!("  Sequential Write: {write_mbps:.1} MB/s");
    println!("  Sequential Read:  {read_mbps:.1} MB/s");

    Ok(())
}

/// Configure a storage backend
async fn configure_storage(backend: &str, settings: &[String]) -> Result<()> {
    configure_storage_from_env_source(&ProcessEnv, backend, settings).await
}

async fn configure_storage_from_env_source(
    env: &(impl EnvSource + ?Sized),
    backend: &str,
    settings: &[String],
) -> Result<()> {
    println!("Configuring storage backend: {backend}");

    if settings.is_empty() {
        // Show current config
        let runtime_config = nestgate_core::config::runtime::get_config();
        println!("\nCurrent configuration:");
        println!("  API Port:    {}", runtime_config.network.api_port);
        println!(
            "  Backend:     {:?}",
            nestgate_core::services::storage::capabilities::detect_backend().backend_type
        );

        let storage_path =
            env_var_or_default(env, "NESTGATE_STORAGE_PATH", "/var/lib/nestgate/storage");
        println!("  Storage:     {storage_path}");

        println!("\nUse --set key=value to modify settings:");
        println!("  nestgate storage configure {backend} --set storage_path=/data/nestgate");
        return Ok(());
    }

    for setting in settings {
        if let Some((key, value)) = setting.split_once('=') {
            println!("  Setting {key}={value}");
            match key {
                "storage_path" => {
                    // Validate the path exists or can be created
                    let path = std::path::Path::new(value);
                    if path.exists() {
                        println!("    Path exists: {value}");
                    } else {
                        tokio::fs::create_dir_all(path).await?;
                        println!("    Created directory: {value}");
                    }
                    println!("    Set NESTGATE_STORAGE_PATH={value} in your environment");
                }
                _ => {
                    println!("    Unknown key: {key} (supported: storage_path)");
                }
            }
        } else {
            println!("  Invalid format: {setting} (expected key=value)");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_backends_succeeds() {
        let result = list_backends().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_scan_nonexistent_path() {
        let result = scan_storage(
            std::path::PathBuf::from("/tmp/nestgate_test_nonexistent_path_12345"),
            false,
            false,
        )
        .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_scan_existing_path() {
        let temp_dir = tempfile::tempdir().unwrap();
        let result = scan_storage(temp_dir.path().to_path_buf(), false, false).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_benchmark_storage_basic() {
        let result = benchmark_storage("filesystem", 1, 1).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_configure_storage_no_settings() {
        let result = configure_storage("filesystem", &[]).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_configure_storage_with_settings() {
        let temp_dir = tempfile::tempdir().unwrap();
        let setting = format!("storage_path={}", temp_dir.path().display());
        let result = configure_storage("filesystem", &[setting]).await;
        assert!(result.is_ok());
    }
}
