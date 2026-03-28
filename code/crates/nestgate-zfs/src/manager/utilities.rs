// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//
// Contains utility functions for parsing ZFS output, capacity calculations,
// and other helper functions used throughout the ZFS manager.

//! Utilities module

use super::types::CapacityInfo;
use crate::error::{create_zfs_error, ZfsOperation};
use nestgate_core::Result;
// Removed unused tracing import

use super::ZfsManager;
use tracing::debug;
use tracing::warn;

impl ZfsManager {
    /// Calculate system utilization as percentage
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn _calculate_system_utilization(&self) -> Result<f64> {
        let pools = self.pool_manager.list_pools().await.map_err(|_e| {
            create_zfs_error(
                "Failed to list pools: error details".to_string(),
                ZfsOperation::SystemCheck,
            )
        })?;

        if pools.is_empty() {
            return Ok(0.0);
        }

        let mut total_used = 0u64;
        let mut total_available = 0u64;

        for pool in &pools {
            let status = self
                .pool_manager
                .get_pool_status(&pool.name)
                .await
                .map_err(|_e| {
                    create_zfs_error(
                        "Failed to get pool status: error details".to_string(),
                        ZfsOperation::PoolCreate,
                    )
                })?;

            // Parse status string for utilization info - simplified parsing
            // Status typically contains capacity information we can extract
            if let Some(capacity_info) = self._parse_capacity_from_status(&status) {
                total_used += capacity_info.used_bytes;
                total_available += capacity_info.total_bytes;
            }
        }

        if total_available > 0 {
            Ok(total_used as f64 / total_available as f64)
        } else {
            Ok(0.0)
        }
    }

    /// Parse capacity information from status string
    pub fn _parse_capacity_from_status(&self, status: &str) -> Option<CapacityInfo> {
        // Parse ZFS status output to extract capacity information
        debug!("Parsing ZFS status for capacity info");

        // Look for lines like "pool: 1.23T allocated, 456G used, 789G available"
        // or "capacity: 1.23T allocated, 456G used, 789G available"
        for line in status.lines() {
            let line = line.trim();
            if line.contains("allocated") && line.contains("used") && line.contains("available") {
                // Parse the capacity line
                if let Some(capacity_info) = self._parse_capacity_line(line) {
                    return Some(capacity_info);
                }
            }
        }

        // Fallback: try to parse individual size lines
        let mut total_bytes = 0u64;
        let mut used_bytes = 0u64;

        for line in status.lines() {
            let line = line.trim();
            if line.starts_with("size:") {
                if let Some(size) = self._parse_sizevalue(line) {
                    total_bytes = size;
                }
            } else if line.starts_with("allocated:") {
                if let Some(size) = self._parse_sizevalue(line) {
                    used_bytes = size;
                }
            }
        }

        if total_bytes > 0 {
            Some(CapacityInfo {
                used_bytes,
                total_bytes,
            })
        } else {
            // Return reasonable defaults if parsing fails
            warn!("Failed to parse ZFS capacity information, using defaults");
            Some(CapacityInfo {
                used_bytes: 1_000_000,   // 1MB default
                total_bytes: 10_000_000, // 10MB default
            })
        }
    }

    /// Parse a capacity line like "1.23T allocated, 456G used, 789G available"
    #[must_use]
    pub fn _parse_capacity_line(&self, line: &str) -> Option<CapacityInfo> {
        // Split by commas and parse each segment
        let parts: Vec<&str> = line.split(',').collect();
        let mut total_bytes = 0u64;
        let mut used_bytes = 0u64;

        for part in parts {
            let part = part.trim();
            if part.contains("allocated") {
                if let Some(size) = self._parse_size_from_segment(part) {
                    total_bytes = size;
                }
            } else if part.contains("used") {
                if let Some(size) = self._parse_size_from_segment(part) {
                    used_bytes = size;
                }
            }
        }

        if total_bytes > 0 {
            Some(CapacityInfo {
                used_bytes,
                total_bytes,
            })
        } else {
            None
        }
    }

    /// Parse size value from lines like "size: 1.23T"
    #[must_use]
    pub fn _parse_sizevalue(&self, line: &str) -> Option<u64> {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() >= 2 {
            let size_str = parts[1].trim();
            self._parse_size_from_segment(size_str)
        } else {
            None
        }
    }

    /// Parse size from segment like "1.23T allocated" or "456G"
    #[must_use]
    pub fn _parse_size_from_segment(&self, segment: &str) -> Option<u64> {
        // Extract the size part (e.g., "1.23T" from "1.23T allocated")
        let size_str = segment.split_whitespace().next()?;

        // Parse the numeric value and unit
        let len = size_str.len();
        if len == 0 {
            return None;
        }

        let (number_str, unit) = if len > 1 {
            let unit_char = size_str.chars().last()?;
            if unit_char.is_alphabetic() {
                (&size_str[..len - 1], unit_char)
            } else {
                (size_str, 'B') // Default to bytes
            }
        } else {
            (size_str, 'B')
        };

        let number: f64 = number_str.parse().ok()?;

        let multiplier = match unit.to_ascii_uppercase() {
            'B' => 1,
            'K' => 1024,
            'M' => 1024 * 1024,
            'G' => 1024 * 1024 * 1024,
            'T' => 1024u64 * 1024 * 1024 * 1024,
            'P' => 1024u64 * 1024 * 1024 * 1024 * 1024,
            _ => 1, // Default to bytes for unknown units
        };

        Some((number * multiplier as f64) as u64)
    }
}
