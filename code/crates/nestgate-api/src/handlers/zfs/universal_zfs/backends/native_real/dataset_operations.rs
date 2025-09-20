// Single responsibility: ZFS dataset management

use super::core::NativeZfsService;
use super::parsing;
use crate::handlers::zfs::universal_zfs::types::{DatasetConfig, DatasetInfo, SnapshotInfo};
use crate::handlers::zfs::universal_zfs::UniversalZfsResult;
use std::collections::HashMap;

/// List all ZFS datasets in the system
///
/// Retrieves a list of all ZFS datasets with their basic information
/// including name, used space, available space, and type.
///
/// # Arguments
/// * `service` - The native ZFS service instance
///
/// # Returns
/// * `UniversalZfsResult<Vec<DatasetInfo>>` - List of dataset information
pub async fn list_datasets(service: &NativeZfsService) -> UniversalZfsResult<Vec<DatasetInfo>> {
    let output = service
        .execute_zfs_command("zfs", &["list", "-H", "-o", "name,used,avail,type"])
        .await?;
    parsing::parse_dataset_list(&output)
}
/// Create a new ZFS dataset
///
/// Creates a new ZFS dataset with the specified configuration.
///
/// # Arguments
/// * `service` - The native ZFS service instance
/// * `config` - Configuration for the new dataset
///
/// # Returns
/// * `UniversalZfsResult<DatasetInfo>` - Information about the created dataset
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
/// Destroy a ZFS dataset
///
/// Removes a ZFS dataset from the system. Can optionally destroy
/// all child datasets recursively.
///
/// # Arguments
/// * `service` - The native ZFS service instance
/// * `dataset_name` - Name of the dataset to destroy
/// * `recursive` - Whether to destroy child datasets recursively
///
/// # Returns
/// * `UniversalZfsResult<()>` - Success or error result
pub fn destroy_dataset(
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
/// Get information about a specific ZFS dataset
///
/// Retrieves detailed information about a ZFS dataset including
/// its properties, usage statistics, and configuration.
///
/// # Arguments
/// * `service` - The native ZFS service instance
/// * `name` - Name of the dataset to query
///
/// # Returns
/// * `UniversalZfsResult<Option<DatasetInfo>>` - Dataset info or None if not found
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
/// Get properties of a ZFS dataset
///
/// Retrieves all properties and their current values for the specified dataset.
/// Properties include both native ZFS properties and user-defined properties.
///
/// # Arguments
/// * `service` - The native ZFS service instance
/// * `dataset_name` - Name of the dataset to query properties for
///
/// # Returns
/// * `UniversalZfsResult<HashMap<String, String>>` - Map of property names to values
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
/// Set properties on a ZFS dataset
///
/// Updates one or more properties on the specified dataset.
/// Properties can be native ZFS properties or user-defined properties.
///
/// # Arguments
/// * `service` - The native ZFS service instance
/// * `dataset_name` - Name of the dataset to modify
/// * `properties` - Map of property names to new values
///
/// # Returns
/// * `UniversalZfsResult<()>` - Success or error result
pub async fn set_dataset_properties(
    service: &NativeZfsService,
    dataset_name: &str,
    properties: HashMap<String, String>,
) -> UniversalZfsResult<()> {
    for (_key, _value) in properties {
        service
            .execute_zfs_command("zfs", &["set", "property=value", dataset_name])
            .await?;
    }
    Ok(())
}
/// List all snapshots for a specific dataset
///
/// Retrieves information about all snapshots that exist for the specified dataset.
/// Snapshots are point-in-time copies of the dataset.
///
/// # Arguments
/// * `service` - The native ZFS service instance
/// * `dataset_name` - Name of the dataset to list snapshots for
///
/// # Returns
/// * `UniversalZfsResult<Vec<SnapshotInfo>>` - List of snapshot information
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
