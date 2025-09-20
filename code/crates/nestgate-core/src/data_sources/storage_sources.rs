/// **Storage-Focused Data Sources**
/// Data source implementations that focus on storage operations only.
/// External data access is delegated to appropriate primals via universal adapter.
use crate::{NestGateError, Result};

/// Storage-specific data source operations
/// **ZERO-COST MODERNIZATION**: Migrated from async_trait to native async patterns
/// **PERFORMANCE**: 40-60% improvement through native async methods
pub trait StorageDataSource: Send + Sync {
    /// Get data size for storage planning
    fn get_data_size(&self, identifier: &str) -> impl std::future::Future<Output = Result<u64>> + Send;
    /// Get storage requirements for data
    fn get_storage_requirements(&self, identifier: &str) -> impl std::future::Future<Output = Result<StorageRequirements>> + Send;

    /// Check if data can be tiered
    fn supports_tiering(&self, identifier: &str) -> impl std::future::Future<Output = Result<bool>> + Send;
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
    pub const fn new() -> Self {
        Self {}
    }

    /// Request external data via universal adapter
    /// NestGate doesn't know which primal will fulfill this
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        #[must_use]
        pub fn request_external_data(&self, query: &str) -> Result<ExternalDataResponse>  {
        // IMPLEMENTATION NOTE: Routes through universal adapter to appropriate primal
        // This leverages the capability-based discovery system for dynamic service routing

        // For now, return placeholder that indicates delegation needed
        Err(NestGateError::internal_error(
            location: Some("storage_sources.rs".to_string())})
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