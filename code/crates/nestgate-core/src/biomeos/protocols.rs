//! Management Protocol Definitions
//! Protocols functionality and utilities.
//! This module provides protocol definitions and utilities for Management integration.

use crate::error::NestGateUnifiedError as NestGateError;
use crate::Result;
use serde::{Deserialize, Serialize};

/// Storage tier enumeration for Management
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StorageTier {
    Hot,
    Warm,
    Cold,
    Archive,
}

impl StorageTier {
    /// Get storage tier from string
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn from_str(tier: &str) -> Result<Self>  {
        match tier.to_lowercase().as_str() {
            "hot" => Ok(Self::Hot),
            "warm" => Ok(Self::Warm),
            "cold" => Ok(Self::Cold),
            "archive" => Ok(Self::Archive),
            _ => Err(NestGateError::validation_error(&format!("Invalid storage tier: {tier}"))),
        }
    }
}

/// Parse size string to bytes
pub fn parse_size_to_bytes(size_str: &str) -> Result<u64> {
    if size_str.ends_with("Ti") || size_str.ends_with("ti") {
        let num_str = &size_str[..size_str.len() - 2];
        let num: f64 = num_str.parse().map_err(|_| {
            NestGateError::validation_error("Invalid argument")
        })?;
        Ok((num * 1024.0 * 1024.0 * 1024.0 * 1024.0) as u64)
    } else if size_str.ends_with("Gi") || size_str.ends_with("gi") {
        let num_str = &size_str[..size_str.len() - 2];
        let num: f64 = num_str.parse().map_err(|_| {
            NestGateError::validation_error("Invalid argument")
        })?;
        Ok((num * 1024.0 * 1024.0 * 1024.0) as u64)
    } else if size_str.ends_with("Mi") || size_str.ends_with("mi") {
        let num_str = &size_str[..size_str.len() - 2];
        let num: f64 = num_str.parse().map_err(|_| {
            NestGateError::validation_error("Invalid argument")
        })?;
        Ok((num * 1024.0 * 1024.0) as u64)
    } else {
        // Assume bytes
        size_str.parse().map_err(|_| {
            NestGateError::validation_error("Invalid argument")
        })
    }
} 