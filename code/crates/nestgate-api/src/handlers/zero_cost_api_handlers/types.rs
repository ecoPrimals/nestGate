// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Core request/response types, API status, dataset config types, and handler traits.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

/// **ZERO-COST API HANDLER TRAIT**
///
/// High-performance API handler trait with zero-cost abstractions.
#[expect(
    async_fn_in_trait,
    reason = "Internal API trait; async methods are intentional"
)]
pub trait ZeroCostApiHandler<T> {
    /// Error type for handler failures
    type Error: std::error::Error + Send + Sync + 'static;

    /// Handle API request with zero-cost processing
    async fn handle_request(
        &self,
        request: ZeroCostApiRequest<serde_json::Value>,
    ) -> Result<ZeroCostApiResponse<serde_json::Value>, Self::Error>;
}

/// **ZERO-COST REQUEST/RESPONSE TYPES**
/// High-performance data structures for API operations
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for `ZeroCostApi` operation
pub struct ZeroCostApiRequest<T> {
    /// Request payload data
    pub data: T,
    /// Unique request identifier for tracing (Arc for zero-copy sharing)
    #[serde(with = "crate::handlers::zero_cost_api_handlers::serde_helpers::arc_string_serde")]
    /// Request identifier
    pub request_id: Arc<String>,
    /// Request timestamp
    pub timestamp: std::time::SystemTime,
    /// Request metadata for extensibility (Arc for zero-copy sharing)
    #[serde(
        with = "crate::handlers::zero_cost_api_handlers::serde_helpers::arc_hashmap_serde",
        rename = "_metadata",
        alias = "metadata"
    )]
    /// Metadata
    pub metadata: Arc<HashMap<String, String>>,
}

/// **ZERO-COST API RESPONSE**
///
/// Response structure for zero-cost API operations with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for `ZeroCostApi` operation
pub struct ZeroCostApiResponse<T> {
    /// Response payload data
    pub data: T,
    /// Request identifier for correlation (Arc for zero-copy sharing)
    #[serde(with = "crate::handlers::zero_cost_api_handlers::serde_helpers::arc_string_serde")]
    /// Request identifier
    pub request_id: Arc<String>,
    /// Response status information
    pub status: ApiStatus,
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
    /// Response metadata for extensibility
    #[serde(rename = "_metadata", alias = "metadata")]
    pub metadata: HashMap<String, String>,
}

/// **API STATUS**
///
/// Status enumeration for API response outcomes.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Status values for Api
pub enum ApiStatus {
    /// Request processed successfully
    Success,
    /// Request processed with warnings
    Warning {
        /// Warning message
        message: String,
    },
    /// Request failed with error
    Error {
        /// Error code
        code: String,
        /// Error message
        message: String,
    },
}

/// **ZERO-COST DATASET MANAGER TRAIT**
///
/// High-performance dataset management trait.
#[expect(
    async_fn_in_trait,
    reason = "Internal dataset trait; async methods are intentional"
)]
pub trait ZeroCostDatasetManager {
    /// Dataset type managed by this implementation
    type Dataset: Send + Sync + Clone;
    /// Error type for dataset operations
    type Error: std::error::Error + Send + Sync + 'static;

    /// List all datasets, optionally filtered by pool
    async fn list_datasets(
        &self,
        pool_name: Option<&str>,
    ) -> Result<Vec<Self::Dataset>, Self::Error>;

    /// Get a specific dataset by name
    async fn get_dataset(&self, name: &str) -> Result<Option<Self::Dataset>, Self::Error>;

    /// Create a new dataset with the given configuration
    async fn create_dataset(&self, config: &DatasetConfig) -> Result<Self::Dataset, Self::Error>;

    /// Delete a dataset by name
    async fn delete_dataset(&self, name: &str) -> Result<(), Self::Error>;
}
/// **DATASET CONFIGURATION**
///
/// Configuration structure for creating and managing ZFS datasets.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Dataset
pub struct DatasetConfig {
    /// Dataset name identifier
    pub name: String,
    /// Parent pool name
    pub pool: String,
    /// Type of dataset (filesystem or volume)
    pub dataset_type: DatasetType,
    /// Custom properties for the dataset
    pub properties: HashMap<String, String>,
}
/// **DATASET TYPE**
///
/// Enumeration of supported ZFS dataset types.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of Dataset
pub enum DatasetType {
    /// Standard filesystem dataset for file storage
    Filesystem,
    /// Volume dataset for block storage with specified size
    Volume {
        /// Volume size in bytes
        size: u64,
    },
}

/// **DATASET INFORMATION**
///
/// Comprehensive information about a ZFS dataset.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Datasetinfo
pub struct DatasetInfo {
    /// Dataset name
    pub name: String,
    /// Parent pool name
    pub pool: String,
    /// Type of dataset
    pub dataset_type: DatasetType,
    /// Total dataset size in bytes
    pub size: u64,
    /// Used space in bytes
    pub used: u64,
    /// Available space in bytes
    pub available: u64,
    /// Mount point path (if mounted)
    pub mount_point: Option<String>,
    /// Dataset creation timestamp
    pub created_at: std::time::SystemTime,
}

/// **API ERROR TYPES**
#[derive(Debug, thiserror::Error)]
/// Errors that can occur during Api operations
pub enum ApiError {
    /// Request processing failed due to internal error
    #[error("Request processing failed")]
    /// Processingfailed
    ProcessingFailed,
    /// Request exceeded maximum processing time
    #[error("Request timeout")]
    /// Timeout
    Timeout,
    /// Request validation failed with details
    #[error("Validation error: {0}")]
    Validation(String),
    /// Internal system error occurred
    #[error("Internal error: {0}")]
    Internal(String),
}
/// **ZERO COST API ERROR**
///
/// Comprehensive error types for zero-cost API operations.
#[derive(Debug, thiserror::Error)]
/// Errors that can occur during `ZeroCostApi` operations
pub enum ZeroCostApiError {
    /// Processing operation failed due to internal error
    #[error("Processing operation failed")]
    /// Processingfailed
    ProcessingFailed,
    /// Operation exceeded the allowed timeout duration
    #[error("Timeout occurred")]
    /// Timeout
    Timeout,
    /// Input validation failed with detailed message
    #[error("Validation error: {0}")]
    Validation(String),
    /// Internal system error occurred
    #[error("Internal error: {0}")]
    Internal(String),
}
