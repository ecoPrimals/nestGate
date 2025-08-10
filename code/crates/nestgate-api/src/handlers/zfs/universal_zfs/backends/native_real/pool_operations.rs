//! Pool Operations Module
//!
//! Single responsibility: ZFS pool management operations

use super::core::NativeZfsService;
use super::parsing;
use crate::handlers::zfs::universal_zfs::types::{
    PoolConfig, PoolInfo, UniversalZfsError, UniversalZfsResult,
};
use tracing::info;

pub async fn list_pools(service: &NativeZfsService) -> UniversalZfsResult<Vec<PoolInfo>> {
    info!("📊 Listing all ZFS pools");

    let output = service
        .execute_zfs_command(
            "zpool",
            &["list", "-H", "-o", "name,size,used,avail,health"],
        )
        .await?;

    let pools = parsing::parse_zpool_list(&output)?;
    info!("📊 Found {} ZFS pools", pools.len());

    Ok(pools)
}

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
            resource_type: "pool".to_string(),
            name: pool_name.to_string(),
        })?;

    // Get detailed device information
    let _status_output = service
        .execute_zfs_command("zpool", &["status", pool_name])
        .await?;
    // Parse zpool status output to extract device information
    // For now, return mock device info - production would parse actual zpool output

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

pub async fn create_pool(
    service: &NativeZfsService,
    config: &PoolConfig,
) -> UniversalZfsResult<PoolInfo> {
    info!("🔧 Creating ZFS pool: {}", config.name);

    if config.devices.is_empty() {
        return Err(UniversalZfsError::InvalidInput {
            field: "devices".to_string(),
            message: "Pool creation requires at least one device".to_string(),
        }
        .into());
    }

    let mut args = vec!["create", &config.name];
    args.extend(config.devices.iter().map(|s| s.as_str()));

    service.execute_zfs_command("zpool", &args).await?;

    // Return the newly created pool info
    get_pool_info(service, &config.name).await
}

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

pub async fn scrub_pool(service: &NativeZfsService, name: &str) -> UniversalZfsResult<()> {
    info!("🧹 Starting scrub for pool: {}", name);
    service
        .execute_zfs_command("zpool", &["scrub", name])
        .await?;
    Ok(())
}

pub async fn get_pool_status(service: &NativeZfsService, name: &str) -> UniversalZfsResult<String> {
    service
        .execute_zfs_command("zpool", &["status", name])
        .await
}
