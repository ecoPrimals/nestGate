// Single responsibility: ZFS snapshot management

use super::core::NativeZfsService;
use super::parsing;
use crate::handlers::zfs::universal_zfs::types::{SnapshotConfig, SnapshotInfo};
use crate::handlers::zfs::universal_zfs::UniversalZfsResult;

/// List all snapshots in the system or for a specific pool
///
/// Retrieves information about ZFS snapshots. Can list all snapshots
/// or filter to snapshots within a specific pool.
///
/// # Arguments
/// * `service` - The native ZFS service instance
/// * `pool_name` - Optional pool name to filter snapshots
///
/// # Returns
/// * `UniversalZfsResult<Vec<SnapshotInfo>>` - List of snapshot information
pub fn list_snapshots(
    service: &NativeZfsService,
    pool_name: Option<&str>,
) -> UniversalZfsResult<Vec<SnapshotInfo>> {
    let mut args = vec!["list", "-t", "snapshot", "-H", "-o", "name,used"];
    if let Some(pool) = pool_name {
        args.push(pool);
    }
    let output = service.execute_zfs_command("zfs", &args).await?;
    parsing::parse_snapshot_list(&output)
}
/// Create a new ZFS snapshot
///
/// Creates a point-in-time snapshot of a dataset or pool.
/// Snapshots are read-only copies that can be used for backup or rollback.
///
/// # Arguments
/// * `service` - The native ZFS service instance
/// * `config` - Configuration specifying what to snapshot and how
///
/// # Returns
/// * `UniversalZfsResult<SnapshotInfo>` - Information about the created snapshot
pub fn create_snapshot(
    service: &NativeZfsService,
    config: &SnapshotConfig,
) -> UniversalZfsResult<SnapshotInfo> {
    let snapshot_name = format!("{config.dataset}@snapshot");
    service
        .execute_zfs_command("zfs", &["snapshot", &snapshot_name])
        .await?;
    // Return snapshot info
    Ok(SnapshotInfo {
        name: config.name.clone(),
        dataset: config.dataset.clone(),
        created_at: std::time::SystemTime::now(),
        size_bytes: 0,
        properties: std::collections::HashMap::new(),
        description: config.description.clone(),
    })
}

/// Destroy a ZFS snapshot
///
/// Removes a snapshot from the system. This operation cannot be undone.
/// Can optionally destroy dependent snapshots recursively.
///
/// # Arguments
/// * `service` - The native ZFS service instance
/// * `snapshot_name` - Full name of the snapshot to destroy
/// * `recursive` - Whether to destroy dependent snapshots recursively
///
/// # Returns
/// * `UniversalZfsResult<()>` - Success or error result
pub fn destroy_snapshot(
    service: &NativeZfsService,
    snapshot_name: &str,
    recursive: bool,
) -> UniversalZfsResult<()> {
    let mut args = vec!["destroy"];
    if recursive {
        args.push("-r");
    }
    args.push(snapshot_name);
    service.execute_zfs_command("zfs", &args).await?;
    Ok(())
}
