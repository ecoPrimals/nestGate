// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//
// Single responsibility: ZFS pool management operations

//! Pool Operations module

use super::core::NativeZfsService;
use super::parsing;
use crate::handlers::zfs::universal_zfs_types::{
    PoolConfig, PoolInfo, UniversalZfsError, UniversalZfsResult,
};
use tracing::info;

/// Lists all ZFS pools on the system
///
/// Returns a vector of `PoolInfo` structures containing details about each pool
/// including name, size, usage, and health status.
pub async fn list_pools(service: &NativeZfsService) -> UniversalZfsResult<Vec<PoolInfo>> {
    info!("📊 Listing all ZFS pools");
    let output = service
        .execute_zfs_command(
            "zpool",
            &["list", "-H", "-o", "name,size,used,avail,health"],
        )
        .await?;

    let pools = parsing::parse_zpool_list(&output)?;
    info!(
        "📊 Found {},
    ZFS pools",
        pools.len()
    );

    Ok(pools)
}

/// Gets detailed information about a specific ZFS pool
///
/// # Arguments
/// * `service` - The native ZFS service instance
/// * `pool_name` - Name of the pool to query
///
/// # Returns
/// Detailed `PoolInfo` including properties, capacity, and health status
pub async fn get_pool_info(
    service: &NativeZfsService,
    pool_name: &str,
) -> UniversalZfsResult<PoolInfo> {
    info!("📊 Getting detailed info for pool: {}", pool_name);
    // Get basic pool information
    let pools = list_pools(service).await?;
    let mut pool_info = pools
        .into_iter()
        .find(|p| p.name == pool_name)
        .ok_or_else(|| UniversalZfsError::NotFound {
            path: format!("pool:{pool_name}"),
        })?;

    // Get detailed device information from zpool status
    let status_output = service
        .execute_zfs_command("zpool", &["status", pool_name])
        .await?;

    // Parse device information from zpool status output
    let devices = parsing::parse_zpool_status(&status_output)?;
    info!("📊 Found {} devices in pool {}", devices.len(), pool_name);

    // Log device information for monitoring
    for device in &devices {
        if device.read_errors > 0 || device.write_errors > 0 || device.checksum_errors > 0 {
            tracing::warn!(
                "⚠️ Device {} has errors: read={}, write={}, checksum={}",
                device.name,
                device.read_errors,
                device.write_errors,
                device.checksum_errors
            );
        }
    }

    // Get pool properties
    let props_output = service
        .execute_zfs_command(
            "zpool",
            &["get", "-H", "-o", "property,value", "all", pool_name],
        )
        .await?;

    for line in props_output.lines() {
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() >= 2 {
            pool_info
                .properties
                .insert(fields[0].to_string(), fields[1].to_string());
        }
    }

    Ok(pool_info)
}

/// Creates a new ZFS pool with the specified configuration
///
/// # Arguments
/// * `service` - The native ZFS service instance
/// * `config` - Pool configuration including name and _devices
///
/// # Returns
/// Information about the newly created pool
///
/// # Errors
/// Returns error if pool creation fails or _devices are invalid
pub async fn create_pool(
    service: &NativeZfsService,
    config: &PoolConfig,
) -> UniversalZfsResult<PoolInfo> {
    info!("🔧 Creating ZFS pool: {}", config.name);
    if config._devices.is_empty() {
        return Err(UniversalZfsError::InvalidInput {
            message: "Pool creation requires at least one device".to_string(),
        });
    }

    let mut args = vec!["create", &config.name];
    args.extend(config._devices.iter().map(std::string::String::as_str));

    service.execute_zfs_command("zpool", &args).await?;

    // Return the newly created pool info
    get_pool_info(service, &config.name).await
}

/// Destroy a ZFS storage pool
///
/// Removes a ZFS pool and all its contained datasets from the system.
/// This is a destructive operation that cannot be undone.
///
/// # Arguments
/// * `service` - The native ZFS service instance
/// * `pool_name` - Name of the pool to destroy
/// * `force` - Whether to force destruction even if pool is in use
///
/// # Returns
/// * `UniversalZfsResult<()>` - Success or error result
pub async fn destroy_pool(
    service: &NativeZfsService,
    pool_name: &str,
    force: bool,
) -> UniversalZfsResult<()> {
    info!("🗑️ Destroying ZFS pool: {} (force: {})", pool_name, force);
    let mut args = vec!["destroy"];
    if force {
        args.push("-f");
    }
    args.push(pool_name);

    service.execute_zfs_command("zpool", &args).await?;
    Ok(())
}

/// Get information about a specific ZFS pool
///
/// Retrieves detailed information about a ZFS pool including
/// its capacity, health status, and configuration.
///
/// # Arguments
/// * `service` - The native ZFS service instance
/// * `name` - Name of the pool to query
///
/// # Returns
/// * `UniversalZfsResult<Option<PoolInfo>>` - Pool info or None if not found
pub async fn get_pool(
    service: &NativeZfsService,
    name: &str,
) -> UniversalZfsResult<Option<PoolInfo>> {
    match get_pool_info(service, name).await {
        Ok(pool) => Ok(Some(pool)),
        Err(UniversalZfsError::NotFound { .. }) => Ok(None),
        Err(e) => Err(e),
    }
}
/// Start a scrub operation on a ZFS pool
///
/// Initiates a data integrity check (scrub) on the specified pool.
/// This verifies all data and _metadata for consistency and repairs any errors found.
///
/// # Arguments
/// * `service` - The native ZFS service instance
/// * `name` - Name of the pool to scrub
///
/// # Returns
/// * `UniversalZfsResult<()>` - Success or error result
pub async fn scrub_pool(service: &NativeZfsService, name: &str) -> UniversalZfsResult<()> {
    info!("🧹 Starting scrub for pool: {}", name);
    service
        .execute_zfs_command("zpool", &["scrub", name])
        .await?;
    Ok(())
}
/// Get the current status of a ZFS pool
///
/// Retrieves detailed status information about a pool including
/// device health, error counts, and ongoing operations.
///
/// # Arguments
/// * `service` - The native ZFS service instance
/// * `name` - Name of the pool to get status for
///
/// # Returns
/// * `UniversalZfsResult<String>` - Pool status information
pub async fn get_pool_status(service: &NativeZfsService, name: &str) -> UniversalZfsResult<String> {
    service
        .execute_zfs_command("zpool", &["status", name])
        .await
}
