//! **BIOMEOS PROTOCOLS AND PARSING UTILITIES**
//!
//! Protocol definitions and parsing utilities for BiomeOS integration.
//! Extracted from biomeos.rs for file size compliance.

use crate::{Result, canonical_types::storage::StorageTier};
use super::types::VolumeSpec;

impl VolumeSpec {
    /// Parse size string to bytes (e.g., "100Gi" -> bytes)
    pub fn size_bytes(&self) -> Result<u64> {
        parse_size(&self.size)
    }

    /// Convert to storage tier enum
    pub fn storage_tier(&self) -> Result<StorageTier> {
        match self.tier.to_lowercase().as_str() {
            "hot" => Ok(StorageTier::Hot),
            "warm" => Ok(StorageTier::Warm),
            "cold" => Ok(StorageTier::Cold),
            "cache" => Ok(StorageTier::Cache),
            _ => Err(crate::NestGateError::Internal {
                message: format!("Unknown storage tier: {}", self.tier),
                component: "biomeos_protocols".to_string(),
                location: Some(format!("{}:{}", file!(), line!())),
                is_bug: false,
                context: Some(crate::error::context::ErrorContext {
                    error_id: "error".to_string(),
                    stack_trace: None,
                    related_errors: vec![],
                    operation: "storage_tier_conversion".to_string(),
                    component: "biomeos".to_string(),
                    metadata: {
                        let mut map = std::collections::HashMap::new();
                        map.insert("provided_tier".to_string(), self.tier.clone());
                        map.insert("available_tiers".to_string(), "hot, warm, cold, cache".to_string());
                        map
                    },
                    timestamp: std::time::SystemTime::now(),
                    retry_info: None,
                    recovery_suggestions: vec!["Use one of: hot, warm, cold, cache".to_string()],
                    performance_metrics: None,
                    environment: None,
                }),
            }),
        }
    }
}

/// Parse size string to bytes
fn parse_size(size_str: &str) -> Result<u64> {
    let size_str = size_str.trim();

    if size_str.ends_with("Gi") || size_str.ends_with("gi") {
        let num_str = &size_str[..size_str.len() - 2];
        let num: f64 = num_str
            .parse()
            .map_err(|_parse_err| crate::NestGateError::Validation {
                field: "size_format".to_string(),
                message: format!("Invalid size format: {size_str}"),
                value: Some(size_str.to_string()),
                current_value: Some(size_str.to_string()),
                expected: Some("Valid format: <number>Gi (e.g., '4.5Gi')".to_string()),
                context: None,
            })?;
        Ok((num * 1024.0 * 1024.0 * 1024.0) as u64)
    } else if size_str.ends_with("Ti") || size_str.ends_with("ti") {
        let num_str = &size_str[..size_str.len() - 2];
        let num: f64 = num_str
            .parse()
            .map_err(|_parse_err| crate::NestGateError::Validation {
                field: "size_format".to_string(),
                message: format!("Invalid size format: {size_str}"),
                value: Some(size_str.to_string()),
                current_value: Some(size_str.to_string()),
                expected: Some("Valid format: <number>Ti (e.g., '2.5Ti')".to_string()),
                context: None,
            })?;
        Ok((num * 1024.0 * 1024.0 * 1024.0 * 1024.0) as u64)
    } else if size_str.ends_with("Mi") || size_str.ends_with("mi") {
        let num_str = &size_str[..size_str.len() - 2];
        let num: f64 = num_str
            .parse()
            .map_err(|_parse_err| crate::NestGateError::Validation {
                field: "size_format".to_string(),
                message: format!("Invalid size format: {size_str}"),
                value: Some(size_str.to_string()),
                current_value: Some(size_str.to_string()),
                expected: Some("Valid format: <number>Mi (e.g., '512Mi')".to_string()),
                context: None,
            })?;
        Ok((num * 1024.0 * 1024.0) as u64)
    } else {
        // Assume bytes if no suffix
        size_str
            .parse()
            .map_err(|_parse_err| crate::NestGateError::Validation {
                field: "size_format".to_string(),
                message: format!("Invalid size format: {size_str}"),
                value: Some(size_str.to_string()),
                current_value: Some(size_str.to_string()),
                expected: Some(
                    "Valid format: number, <number>Mi, <number>Gi, or <number>Ti".to_string(),
                ),
                context: None,
            })
    }
} 