//! Core types for NestGate automation system

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;
use nestgate_core::types::StorageTier;

pub mod config;
pub mod ecosystem;
pub mod optimization;
pub mod prediction;

pub use config::*;
pub use ecosystem::*;
pub use optimization::{
    AgeThresholds, OptimizationPlan, OptimizationResult, PerformanceExpectation, PropertyChange,
    SizeThresholds, TierThresholds,
};
pub use prediction::*;

/// Main error type for automation operations
#[derive(Debug, thiserror::Error)]
pub enum AutomationError {
    /// Configuration error
    #[error("Configuration error: {0}")]
    Configuration(String),

    /// Discovery error
    #[error("Discovery error: {0}")]
    Discovery(String),

    /// Connection error
    #[error("Connection error: {0}")]
    Connection(String),

    /// Network error
    #[error("Network error: {0}")]
    NetworkError(String),

    /// File analysis error
    #[error("File analysis error: {0}")]
    FileAnalysis(String),

    /// Service error
    #[error("Service error: {0}")]
    Service(String),

    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),

    /// Analysis related errors
    #[error("Analysis error: {0}")]
    AnalysisError(String),

    /// Cache related errors
    #[error("Cache error: {0}")]
    Cache(String),
}

impl From<nestgate_core::NestGateError> for AutomationError {
    fn from(err: nestgate_core::NestGateError) -> Self {
        match err {
            nestgate_core::NestGateError::Internal(msg) => AutomationError::Internal(msg),
            nestgate_core::NestGateError::InvalidInput(msg) => AutomationError::Configuration(msg),
            nestgate_core::NestGateError::Network(msg) => AutomationError::NetworkError(msg),
            nestgate_core::NestGateError::Database(msg) => AutomationError::Internal(msg),
            nestgate_core::NestGateError::Authentication(msg) => AutomationError::Service(msg),
            nestgate_core::NestGateError::Authorization(msg) => AutomationError::Service(msg),
            // Handle remaining variants with a catch-all
            _ => AutomationError::Internal(format!("Unhandled error: {:?}", err)),
        }
    }
}

/// Access type for tracking file operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessType {
    Read,
    Write,
    Metadata,
}

/// Task priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskPriority {
    Low,
    Normal,
    High,
    Critical,
}

/// Service health status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ServiceHealth {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

/// File characteristics for analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCharacteristics {
    pub is_frequently_accessed: bool,
    pub is_sequential_access: bool,
    pub is_compressible: bool,
    pub is_dedupable: bool,
}

/// Access event for pattern tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessEvent {
    pub timestamp: SystemTime,
    pub access_type: AccessType,
    pub file_path: String,
    pub bytes_accessed: u64,
}

/// File analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileAnalysis {
    pub path: String,
    pub size: u64,
    pub file_type: FileType,
    pub modified: u64,
    pub characteristics: FileCharacteristics,
}

/// Access patterns for a file or dataset
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AccessPatterns {
    pub daily_access_count: u32,
    pub average_file_size: u64,
    pub read_write_ratio: f64,
    pub sequential_access_ratio: f64,
    pub peak_access_hours: Vec<u8>,
    pub last_access: Option<SystemTime>,
}

/// Performance statistics for tiers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TierStats {
    pub average_latency_ms: f64,
    pub throughput_mbps: f64,
    pub utilization_percent: f64,
    pub error_rate: f64,
}

/// Overall tier performance statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TierPerformanceStats {
    pub hot_tier_performance: TierStats,
    pub warm_tier_performance: TierStats,
    pub cold_tier_performance: TierStats,
    pub archive_tier_performance: TierStats,
}

/// Training example for ML models
#[derive(Debug, Clone)]
pub struct TrainingExample {
    pub file_analysis: FileAnalysis,
    pub access_patterns: AccessPatterns,
    pub actual_tier: StorageTier,
    pub performance_outcome: f64,
}

/// Context information for storage operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageContext {
    pub available_pools: Vec<String>,
    pub dataset_count: u32,
    pub total_capacity: u64,
    pub performance_stats: TierPerformanceStats,
}

/// Context information for dataset operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetContext {
    pub name: String,
    pub current_tier: StorageTier,
    pub used_space: u64,
    pub available_space: u64,
    pub compression_ratio: Option<f64>,
    pub file_count: Option<u64>,
    pub mount_point: String,
    pub properties: HashMap<String, String>,
}

pub enum FileClassification {
    Text,
    Binary,
    Compressed,
    Encrypted,
}
