//! Dataset Operations Module
//! Single responsibility: ZFS dataset management

use super::core::NativeZfsService;
use super::parsing;
use crate::handlers::zfs::universal_zfs::types::{
    DatasetConfig, DatasetInfo, SnapshotInfo, UniversalZfsResult,
};
use std::collections::HashMap;

pub async fn list_datasets(service: &NativeZfsService) -> UniversalZfsResult<Vec<DatasetInfo>> {
    let output = service
        .execute_zfs_command("zfs", &["list", "-H", "-o", "name,used,avail,type"])
        .await?;
    parsing::parse_dataset_list(&output)
}

pub async fn create_dataset(
    service: &NativeZfsService,
    config: &DatasetConfig,
) -> UniversalZfsResult<DatasetInfo> {
    service
        .execute_zfs_command("zfs", &["create", &config.name])
        .await?;
    get_dataset(service, &config.name).await?.ok_or_else(|| {
        crate::handlers::zfs::universal_zfs::types::UniversalZfsError::NotFound {
            resource_type: "dataset".to_string(),
            name: config.name.clone(),
        }
        .into()
    })
}

pub async fn destroy_dataset(
    service: &NativeZfsService,
    dataset_name: &str,
    recursive: bool,
) -> UniversalZfsResult<()> {
    let mut args = vec!["destroy"];
    if recursive {
        args.push("-r");
    }
    args.push(dataset_name);
    service.execute_zfs_command("zfs", &args).await?;
    Ok(())
}

pub async fn get_dataset(
    service: &NativeZfsService,
    name: &str,
) -> UniversalZfsResult<Option<DatasetInfo>> {
    match service
        .execute_zfs_command("zfs", &["list", "-H", "-o", "name,used,avail,type", name])
        .await
    {
        Ok(output) => Ok(parsing::parse_dataset_list(&output)?.into_iter().next()),
        Err(_) => Ok(None),
    }
}

pub async fn get_dataset_properties(
    service: &NativeZfsService,
    dataset_name: &str,
) -> UniversalZfsResult<HashMap<String, String>> {
    let output = service
        .execute_zfs_command(
            "zfs",
            &["get", "-H", "-o", "property,value", "all", dataset_name],
        )
        .await?;
    let mut properties = HashMap::new();
    for line in output.lines() {
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() >= 2 {
            properties.insert(fields[0].to_string(), fields[1].to_string());
        }
    }
    Ok(properties)
}

pub async fn set_dataset_properties(
    service: &NativeZfsService,
    dataset_name: &str,
    properties: HashMap<String, String>,
) -> UniversalZfsResult<()> {
    for (key, value) in properties {
        service
            .execute_zfs_command("zfs", &["set", &format!("{}={}", key, value), dataset_name])
            .await?;
    }
    Ok(())
}

pub async fn list_dataset_snapshots(
    service: &NativeZfsService,
    dataset_name: &str,
) -> UniversalZfsResult<Vec<SnapshotInfo>> {
    let output = service
        .execute_zfs_command(
            "zfs",
            &[
                "list",
                "-t",
                "snapshot",
                "-H",
                "-o",
                "name,used",
                dataset_name,
            ],
        )
        .await?;
    parsing::parse_snapshot_list(&output)
}
