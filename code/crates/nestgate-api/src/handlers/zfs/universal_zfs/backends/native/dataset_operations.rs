//! Dataset Operations for Native ZFS Backend
//!
//! Contains all dataset-related operations for the native ZFS backend.

use std::collections::HashMap;
// Removed unused tracing import

use crate::handlers::zfs::universal_zfs::types::{
    DatasetConfig, DatasetInfo, DatasetType, UniversalZfsError, UniversalZfsResult,
};
use tracing::info;

use super::core::NativeZfsService;

/// List all ZFS datasets (zero-copy optimized)
pub async fn list_datasets(service: &NativeZfsService) -> UniversalZfsResult<Vec<DatasetInfo>> {
    info!("Listing ZFS datasets");

    // Execute `zfs list -H -o name,used,available,referenced,mountpoint,type`
    let output = service
        .execute_zfs_command(&[
            "list",
            "-H",
            "-o",
            "name,used,available,referenced,mountpoint,type",
        ])
        .await?;

    let mut datasets = Vec::new();
    for line in output.lines() {
        if let Some(dataset_info) = parse_dataset_line(line) {
            datasets.push(dataset_info);
        }
    }

    Ok(datasets)
}

/// Parse a single dataset line (zero-copy optimized)
fn parse_dataset_line(line: &str) -> Option<DatasetInfo> {
    let parts: Vec<&str> = line.split('\t').collect();
    if parts.len() >= 6 {
        Some(DatasetInfo {
            name: parts[0].into(),
            dataset_type: parse_dataset_type(parts[5]),
            used_space: parse_size(parts[1]).unwrap_or(0),
            available_space: parse_size(parts[2]).unwrap_or(0),
            mount_point: if parts[4] == "-" {
                None
            } else {
                Some(parts[4].into())
            },
            properties: HashMap::new(),
            created_at: std::time::SystemTime::now(),
            parent: None,
            children: Vec::new(),
        })
    } else {
        None
    }
}

/// Get information about a specific dataset (zero-copy optimized)
pub async fn get_dataset(
    service: &NativeZfsService,
    name: &str,
) -> UniversalZfsResult<Option<DatasetInfo>> {
    info!("Getting dataset info for: {}", name);

    // Execute `zfs list -H -o name,used,available,referenced,mountpoint,type dataset_name`
    let output = service
        .execute_zfs_command(&[
            "list",
            "-H",
            "-o",
            "name,used,available,referenced,mountpoint,type",
            name,
        ])
        .await;

    match output {
        Ok(output) => {
            let line = output.trim();
            if line.is_empty() {
                return Ok(None);
            }

            Ok(parse_dataset_line(line))
        }
        Err(_) => Ok(None), // Dataset doesn't exist
    }
}

/// Create a new ZFS dataset (zero-copy optimized)
pub async fn create_dataset(
    service: &NativeZfsService,
    config: &DatasetConfig,
) -> UniversalZfsResult<DatasetInfo> {
    info!("Creating dataset: {}", config.name);

    // Build zfs create command with properties
    let mut args = vec!["create"];

    // Add properties if specified - use owned strings only when necessary
    let property_strings: Vec<String> = config
        .properties
        .iter()
        .map(|(key, value)| format!("{key}={value}"))
        .collect();

    for (i, (_, _)) in config.properties.iter().enumerate() {
        args.push("-o");
        args.push(&property_strings[i]);
    }

    args.push(&config.name);

    // Execute the create command
    service.execute_zfs_command(&args).await?;

    // Get the created dataset info
    get_dataset(service, &config.name)
        .await?
        .ok_or_else(|| UniversalZfsError::internal("Failed to retrieve created dataset"))
}

/// Destroy a ZFS dataset
pub async fn destroy_dataset(service: &NativeZfsService, name: &str) -> UniversalZfsResult<()> {
    info!("Destroying dataset: {}", name);

    // Execute `zfs destroy dataset_name`
    service.execute_zfs_command(&["destroy", name]).await?;

    Ok(())
}

/// Get dataset properties (zero-copy optimized)
pub async fn get_dataset_properties(
    service: &NativeZfsService,
    name: &str,
) -> UniversalZfsResult<HashMap<String, String>> {
    info!("Getting properties for dataset: {}", name);

    // Execute `zfs get -H -o property,value all dataset_name`
    let output = service
        .execute_zfs_command(&["get", "-H", "-o", "property,value", "all", name])
        .await?;

    let mut properties = HashMap::new();
    for line in output.lines() {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() >= 2 {
            properties.insert(parts[0].into(), parts[1].into());
        }
    }

    Ok(properties)
}

/// Set dataset properties (zero-copy optimized)
pub async fn set_dataset_properties(
    service: &NativeZfsService,
    name: &str,
    properties: &HashMap<String, String>,
) -> UniversalZfsResult<()> {
    info!("Setting properties for dataset: {}", name);

    // Set each property individually
    for (key, value) in properties {
        let property_arg = format!("{key}={value}");
        service
            .execute_zfs_command(&["set", &property_arg, name])
            .await?;
    }

    Ok(())
}

/// Helper function to parse ZFS size strings (zero-copy optimized)
fn parse_size(size_str: &str) -> Option<u64> {
    if size_str == "-" {
        return Some(0);
    }

    let size_str = size_str.trim();
    if size_str.is_empty() {
        return None;
    }

    let (number_part, multiplier) = if let Some(last_char) = size_str.chars().last() {
        if last_char.is_alphabetic() {
            let mut chars = size_str.chars();
            chars.next_back();
            let number_part = chars.as_str();
            let multiplier = match last_char {
                'K' | 'k' => 1024,
                'M' | 'm' => 1024 * 1024,
                'G' | 'g' => 1024 * 1024 * 1024,
                'T' | 't' => 1024_u64 * 1024 * 1024 * 1024,
                'P' | 'p' => 1024_u64 * 1024 * 1024 * 1024 * 1024,
                _ => 1,
            };
            (number_part, multiplier)
        } else {
            (size_str, 1)
        }
    } else {
        return None;
    };

    let number: f64 = number_part.parse().ok()?;
    Some((number * multiplier as f64) as u64)
}

/// Helper function to parse dataset type from string (zero-copy optimized)
fn parse_dataset_type(type_str: &str) -> DatasetType {
    match type_str {
        "filesystem" => DatasetType::Filesystem,
        "volume" => DatasetType::Volume,
        "snapshot" => DatasetType::Snapshot,
        "bookmark" => DatasetType::Bookmark,
        _ => DatasetType::Filesystem, // Default to filesystem
    }
}
