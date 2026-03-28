// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Helper functions for ZFS pool operations
//!
//! This module contains utility functions for parsing sizes, getting properties,
//! and other helper operations for ZFS pool management.

use std::collections::HashMap;
use tokio::process::Command as TokioCommand;
use tracing::info;

use crate::error::{create_zfs_error, Result, ZfsOperation};

/// Parse size string with units (simplified implementation)
///
/// Converts size strings like "10G", "500M", "1T" to bytes.
///
/// # Arguments
///
/// * `size_str` - Size string with optional unit suffix (B, K, M, G, T, P)
///
/// # Returns
///
/// * `Some(u64)` - Size in bytes if parsing succeeds
/// * `None` - If the string cannot be parsed
pub fn parse_size_with_units(size_str: &str) -> Option<u64> {
    if size_str == "-" {
        return Some(0);
    }

    let size_str = size_str.trim();
    let (number_part, unit) = if let Some(last_char) = size_str.chars().last() {
        if last_char.is_alphabetic() {
            let unit_start = size_str.len() - 1;
            (&size_str[..unit_start], &size_str[unit_start..])
        } else {
            (size_str, "")
        }
    } else {
        (size_str, "")
    };

    let number: f64 = number_part.parse().ok()?;

    use crate::constants::{BYTES_PER_GB, BYTES_PER_KB, BYTES_PER_MB, BYTES_PER_PB, BYTES_PER_TB};

    let multiplier = match unit.to_uppercase().as_str() {
        "" | "B" => 1,
        "K" => BYTES_PER_KB,
        "M" => BYTES_PER_MB,
        "G" => BYTES_PER_GB,
        "T" => BYTES_PER_TB,
        "P" => BYTES_PER_PB,
        _ => return None,
    };

    Some((number * multiplier as f64) as u64)
}

/// Get pool properties using zpool command
///
/// # Arguments
///
/// * `pool_name` - Name of the pool to query
///
/// # Returns
///
/// * `Ok(HashMap)` - Map of property names to values
/// * `Err` - If the command fails to execute
#[allow(dead_code)]
pub async fn get_pool_properties(pool_name: &str) -> Result<HashMap<String, String>> {
    let output = TokioCommand::new("zpool")
        .args(["get", "all", "-H", "-p", pool_name])
        .output()
        .await
        .map_err(|_e| {
            create_zfs_error(
                "Failed to get pool properties: error details".to_string(),
                ZfsOperation::SystemCheck,
            )
        })?;

    if !output.status.success() {
        return Ok(HashMap::new());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut properties = HashMap::new();

    for line in stdout.lines() {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() >= 3 {
            properties.insert(parts[1].to_string(), parts[2].to_string());
        }
    }

    Ok(properties)
}

/// Ensure default pool exists for testing/development
///
/// This is a development helper that ensures at least one pool exists.
/// Not intended for production use.
#[allow(dead_code)]
pub async fn ensure_default_pool() -> Result<()> {
    info!("Checking for default pool (development mode)");
    // This is a stub for development - in production, pools are managed externally
    Ok(())
}
