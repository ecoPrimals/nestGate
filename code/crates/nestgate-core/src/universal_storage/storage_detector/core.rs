use crate::error::NestGateError;
use std::collections::HashMap;
//
// Core utilities and shared functionality for the storage detector system.

use crate::{Result};
use super::types::*;

/// Core utilities for storage detection
pub struct StorageDetectorUtils;

impl StorageDetectorUtils {
    /// Validate storage identifier format
    pub fn validate_storage_id(id: &str) -> Result<()> {
        if id.is_empty() {
            return Err(NestGateError::validation_error(
                "storage_id",
                "Storage identifier cannot be empty",
                Some(id.to_string())
            ));
        }
        
        if id.len() > 255 {
            return Err(NestGateError::validation_error(
                "storage_id",
                "Storage identifier too long (max 255 characters)",
                Some(id.to_string())
            ));
        }
        
        Ok(())
    }

    /// Normalize storage path for consistent identification
    pub fn normalize_path(path: &str) -> String {
        // Remove trailing slashes and normalize path separators
        let normalized = path.trim_end_matches('/').replace('\\', "/");
        if normalized.is_empty() {
            "/".to_string()
        } else {
            normalized
        }
    }

    /// Generate unique storage identifier
    pub fn generate_storage_id(storage_type: &str, path: &str) -> String {
        let normalized_path = Self::normalize_path(path);
        let hash = Self::simple_hash(&normalized_path);
        format!("{}_{:x}", storage_type.to_lowercase(), hash)
    }

    /// Simple hash function for generating IDs
    fn simple_hash(input: &str) -> u32 {
        let mut hash = 0u32;
        for byte in input.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u32);
        }
        hash
    }

    /// Merge storage metadata
    pub fn merge_metadata(
        base: &mut HashMap<String, String>,
        additional: HashMap<String, String>,
    ) {
        for (key, value) in additional {
            base.insert(key, value);
        }
    }

    /// Format storage size for display
    pub fn format_storage_size(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB"];
        let mut size = bytes as f64;
        let mut unit_index = 0;

        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }

        if unit_index == 0 {
            format!("{} {}", bytes, UNITS[unit_index])
        } else {
            format!("{:.2} {}", size, UNITS[unit_index])
        }
    }

    /// Calculate percentage with safe division
    pub fn safe_percentage(used: u64, total: u64) -> f64 {
        if total == 0 {
            0.0
        } else {
            (used as f64 / total as f64) * 100.0
        }
    }

    /// Check if storage type supports specific capability
    pub fn storage_supports_capability(
        storage_type: &str,
        capability: &str,
    ) -> bool {
        match (storage_type.to_lowercase().as_str(), capability.to_lowercase().as_str()) {
            ("zfs", "compression") => true,
            ("zfs", "deduplication") => true,
            ("zfs", "snapshots") => true,
            ("zfs", "encryption") => true,
            ("btrfs", "compression") => true,
            ("btrfs", "snapshots") => true,
            ("ext4", "journaling") => true,
            ("xfs", "journaling") => true,
            ("ntfs", "compression") => true,
            ("apfs", "snapshots") => true,
            ("apfs", "encryption") => true,
            _ => false,
        }
    }
} 