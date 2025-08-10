/// **Storage-Focused Data Sources**
/// Data source implementations that focus on storage operations only.
/// External data access is delegated to appropriate primals via universal adapter.

use crate::error::{NestGateError, Result};
use async_trait::async_trait;
use std::path::PathBuf;

/// Storage-specific data source operations
#[async_trait]
pub trait StorageDataSource {
    /// Get data size for storage planning
    async fn get_data_size(&self, identifier: &str) -> Result<u64>;

    /// Get storage requirements for data
    async fn get_storage_requirements(&self, identifier: &str) -> Result<StorageRequirements>;

    /// Check if data can be tiered
    async fn supports_tiering(&self, identifier: &str) -> Result<bool>;
    }

/// Storage requirements for data
#[derive(Debug, Clone)]
pub struct StorageRequirements {
    pub min_space_bytes: u64,
    pub recommended_tier: StorageTier,
    pub access_pattern: AccessPattern,
    pub retention_policy: Option<RetentionPolicy>,
    }

/// Storage tiers for data classification
#[derive(Debug, Clone)]
pub enum StorageTier {
    Hot,    // Frequent access - NVMe/SSD
    Warm,   // Regular access - SSD
    Cold,   // Infrequent access - HDD
    Archive, // Long-term storage - Tape/Cloud
    }

/// Data access patterns for storage optimization
#[derive(Debug, Clone)]
pub enum AccessPattern {
    Sequential,
    Random,
    Write_Heavy,
    Read_Heavy,
    Append_Only,
    }

/// Data retention policies
#[derive(Debug, Clone)]
pub struct RetentionPolicy {
    pub retain_for_days: u32,
    pub auto_delete: bool,
    pub archive_before_delete: bool,
    }

/// External data adapter - delegates to universal adapter
pub struct ExternalDataAdapter {
    // NestGate doesn't know about specific external services
    // It only knows how to request data through universal adapter
    }

impl ExternalDataAdapter {
    pub fn new() -> Self {
        Self {}
    }

    /// Request external data via universal adapter
    /// NestGate doesn't know which primal will fulfill this
    pub async fn request_external_data(&self, query: &str) -> Result<ExternalDataResponse> {
        // IMPLEMENTATION NOTE: Routes through universal adapter to appropriate primal
        // This leverages the capability-based discovery system for dynamic service routing

        // For now, return placeholder that indicates delegation needed
        Err(NestGateError::Internal {
            message: format!("External data request '{}' needs universal adapter routing", query),
            location: Some(file!().to_string()),
            debug_info: Some("NestGate only knows storage - external data delegated".to_string()),
            is_bug: false,
        })
    }
    }

/// Response from external data requests
#[derive(Debug, Clone)]
pub struct ExternalDataResponse {
    pub data_identifier: String,
    pub storage_requirements: StorageRequirements,
    pub source_info: String, // Which primal fulfilled the request
    }

impl Default for ExternalDataAdapter {
    fn default() -> Self {
        Self::new()
    }
    }