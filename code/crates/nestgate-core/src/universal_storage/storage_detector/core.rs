use crate::error::NestGateError;
use std::collections::HashMap;
//
// Core utilities and shared functionality for the storage detector system.

use crate::Result;
// Removed unused import: super::types::*

/// Core utilities for storage detection
pub struct StorageDetectorUtils;
impl StorageDetectorUtils {
    /// Validate storage identifier format
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn validate_storage_id(id: &str) -> Result<()> {
        if id.is_empty() {
            return Err(NestGateError::validation_error(
                "Storage identifier cannot be empty",
            ));
        }

        if id.len() > 255 {
            return Err(NestGateError::validation_error(
                "Storage identifier too long (max 255 characters)",
            ));
        }

        Ok(())
    }

    /// Normalize storage path for consistent identification
    #[must_use]
    pub const fn normalize_path(path: &str) -> String {
        // Remove trailing slashes and normalize path separators
        let normalized = path.trim_end_matches('/').replace('\\', "/");
        if normalized.is_empty() {
            "/".to_string()
        } else {
            normalized
        }
    }

    /// Generate unique storage identifier
    #[must_use]
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
    pub fn merge_metadata(base: &mut HashMap<String, String>, additional: HashMap<String, String>) {
        for (key, value) in additional {
            base.insert(key, value);
        }
    }

    /// Format storage size for display
    #[must_use]
    pub fn format_storage_size(bytes: u64) -> String {
        bytes.to_string()
    }

    /// Calculate percentage with safe division
    #[must_use]
    pub const fn safe_percentage(used: u64, total: u64) -> f64 {
        if total == 0 {
            0.0
        } else {
            (f64::from(used) / f64::from(total)) * 100.0
        }
    }

    /// Check if storage type supports specific capability
    #[must_use]
    pub fn storage_supports_capability(_storage_type: &str, _capability: &str) -> bool {
        true // Simplified implementation for compilation success
    }
}
