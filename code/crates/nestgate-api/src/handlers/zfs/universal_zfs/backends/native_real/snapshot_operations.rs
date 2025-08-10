//! Snapshot Operations Module
//! Single responsibility: ZFS snapshot management

use super::core::NativeZfsService;
use super::parsing;
use crate::handlers::zfs::universal_zfs::types::{
    SnapshotConfig, SnapshotInfo, UniversalZfsResult,
};

pub async fn list_snapshots(
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

pub async fn create_snapshot(
    service: &NativeZfsService,
    config: &SnapshotConfig,
) -> UniversalZfsResult<SnapshotInfo> {
    let snapshot_name = format!("{}@{}", config.dataset, config.name);
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

pub async fn destroy_snapshot(
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
