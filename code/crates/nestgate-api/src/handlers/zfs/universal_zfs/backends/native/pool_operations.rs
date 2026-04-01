// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]
//
// Contains all pool-related operations for the native ZFS backend.

//! Pool Operations module

use std::collections::HashMap;
use std::process::Command;
// Removed unused tracing import

use crate::handlers::zfs::universal_zfs_types::{
    PoolCapacity, PoolConfig, PoolHealth, PoolInfo, PoolState, ScrubStatus, UniversalZfsError,
    UniversalZfsResult,
};
use tracing::info;

use super::core::NativeZfsService;

/// Get pool information by querying pool status
async fn get_pool_status_info(
    _service: &NativeZfsService,
    pool_name: &str,
) -> UniversalZfsResult<PoolInfo> {
    use tokio::process::Command;
    // Execute zpool status command
    let output = Command::new("zpool")
        .arg("status")
        .arg(pool_name)
        .output()
        .await
        .map_err(|e| UniversalZfsError::internal(format!("Failed to execute zpool status: {e}")))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(UniversalZfsError::internal(format!(
            "zpool status failed: {stderr}"
        )));
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    parse_pool_status(&output_str)
}

/// Parse pool status from zpool status output
pub fn parse_pool_status(output: &str) -> UniversalZfsResult<PoolInfo> {
    let lines: Vec<&str> = output.lines().collect();
    if lines.is_empty() {
        return Err(UniversalZfsError::internal("Empty zpool status output"));
    }
    let mut pool_name = String::new();
    let mut state = PoolState::Unknown;
    let mut health = PoolHealth::Unknown;

    // Parse pool name and state from first line
    if let Some(first_line) = lines.first()
        && let Some(name) = first_line.strip_prefix("  pool: ")
    {
        pool_name = name.to_string();
    }

    // Parse state and health
    for line in &lines {
        if let Some(state_str) = line.strip_prefix(" state: ") {
            state = match state_str.trim() {
                "ONLINE" | "DEGRADED" | "FAULTED" => PoolState::Active,
                "OFFLINE" => PoolState::Suspended,
                "EXPORTED" => PoolState::Exported,
                _ => PoolState::Unknown,
            };

            health = match state_str.trim() {
                "ONLINE" => PoolHealth::Online,
                "DEGRADED" => PoolHealth::Degraded,
                "FAULTED" => PoolHealth::Faulted,
                "OFFLINE" => PoolHealth::Offline,
                _ => PoolHealth::Unknown,
            };
        }
    }

    Ok(PoolInfo {
        name: pool_name,
        health,
        state,
        capacity: PoolCapacity {
            total: 0, // Would need to parse from zpool list
            used: 0,
            available: 0,
        },
        scrub: Some(ScrubStatus::None),
        properties: HashMap::new(),
    })
}

/// Parse pool list from zpool list output
fn parse_pool_list(output: &str) -> UniversalZfsResult<Vec<PoolInfo>> {
    let mut pools = Vec::new();
    let lines: Vec<&str> = output.lines().collect();
    // Skip header line
    for line in lines.iter().skip(1) {
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 6 {
            continue;
        }

        let name = parts[0].to_string();
        let size_str = parts[1];
        let alloc_str = parts[2];
        let free_str = parts[3];
        let health_str = parts[4];

        // Parse sizes (basic implementation - would need better parsing)
        let total = NativeZfsService::parse_size_string(size_str).unwrap_or(0);
        let used = NativeZfsService::parse_size_string(alloc_str).unwrap_or(0);
        let available = NativeZfsService::parse_size_string(free_str).unwrap_or(0);

        let health = match health_str {
            "ONLINE" => PoolHealth::Online,
            "DEGRADED" => PoolHealth::Degraded,
            "FAULTED" => PoolHealth::Faulted,
            "OFFLINE" => PoolHealth::Offline,
            _ => PoolHealth::Unknown,
        };

        pools.push(PoolInfo {
            name: name.clone(),
            health,
            state: PoolState::Active,
            capacity: PoolCapacity {
                total,
                used,
                available,
            },
            scrub: Some(ScrubStatus::None),
            properties: HashMap::new(),
        });
    }

    Ok(pools)
}

/// List all ZFS pools
pub async fn list_pools(service: &NativeZfsService) -> UniversalZfsResult<Vec<PoolInfo>> {
    info!("Listing ZFS pools");
    let output: String = service.execute_zpool_command(&["list", "-H"]).await?;
    parse_pool_list(&output)
}
/// Get information about a specific pool
pub async fn get_pool(
    service: &NativeZfsService,
    name: &str,
) -> UniversalZfsResult<Option<PoolInfo>> {
    info!("Getting pool info for: {}", name);
    // First check if pool exists
    let pools = list_pools(service).await?;
    if !pools.iter().any(|p| p.name == name) {
        return Ok(None);
    }

    // Get detailed status
    let output: String = service.execute_zpool_command(&["status", name]).await?;
    let pool_info = parse_pool_status(&output)?;
    Ok(Some(pool_info))
}

/// Create a new ZFS pool
pub async fn create_pool(
    service: &NativeZfsService,
    config: &PoolConfig,
) -> UniversalZfsResult<PoolInfo> {
    let pool_name = &config.name;

    // Build zpool create command
    let mut cmd = Command::new("zpool");
    cmd.arg("create");

    // Add pool-specific options
    for key in config.properties.keys() {
        cmd.arg("-o").arg(format!("{key}=self.base_url"));
    }

    cmd.arg(pool_name);

    // Add _devices
    for device in &config.devices {
        cmd.arg(device);
    }

    // Execute the command
    let output = cmd
        .output()
        .map_err(|e| UniversalZfsError::internal(format!("Failed to execute zpool create: {e}")))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(UniversalZfsError::internal(format!(
            "zpool create failed: {stderr}"
        )));
    }

    // Return the created pool info by querying the pool status
    get_pool_status_info(service, pool_name).await
}

/// Destroy a ZFS pool
pub fn destroy_pool(_service: &NativeZfsService, name: &str) -> UniversalZfsResult<()> {
    // Build zpool destroy command
    let mut cmd = Command::new("zpool");
    cmd.arg("destroy");
    cmd.arg("-f"); // Force destruction
    cmd.arg(name);

    // Execute the command
    let output = cmd
        .output()
        .map_err(|_e| UniversalZfsError::internal("Failed to execute zpool command".to_string()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(UniversalZfsError::internal(format!(
            "zpool destroy failed: {stderr}"
        )));
    }

    Ok(())
}

/// Start a pool scrub
pub async fn scrub_pool(service: &NativeZfsService, name: &str) -> UniversalZfsResult<()> {
    info!("Starting scrub for pool: {}", name);
    service.execute_zpool_command(&["scrub", name]).await?;
    Ok(())
}
/// Get pool status
pub async fn get_pool_status(service: &NativeZfsService, name: &str) -> UniversalZfsResult<String> {
    info!("Getting pool status for: {}", name);
    let output = service.execute_zpool_command(&["status", name]).await?;
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_pool_status_reads_pool_and_state() {
        let out = r"  pool: tank
 state: ONLINE
";
        let p = parse_pool_status(out).expect("pool");
        assert_eq!(p.name, "tank");
        assert_eq!(p.health, PoolHealth::Online);
    }

    #[test]
    fn parse_pool_status_empty_errors() {
        assert!(parse_pool_status("").is_err());
    }
}
