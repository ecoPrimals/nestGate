/*! 
NestGate AI Models

This crate provides AI model integration for the NestGate NAS system.
*/

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

pub mod manager;
pub mod optimizer;
pub mod inference;
pub mod registry;
pub mod memory;
pub mod types;

pub use manager::ModelManager;
pub use optimizer::ModelOptimizer;
pub use inference::InferenceService;
pub use registry::ModelRegistry;
pub use memory::GPUMemoryManager;
pub use types::*;

/// Model types supported by the NestGate MCP NAS node
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModelType {
    /// Optimizes data placement across storage tiers
    StorageOptimizer,
    /// Predicts AI workload patterns
    WorkloadPredictor,
    /// Optimizes cache allocation and prefetching
    CacheOptimizer,
    /// Detects storage performance anomalies
    AnomalyDetector,
}

/// Model formats supported by the NestGate MCP NAS node
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModelFormat {
    /// ONNX model format
    ONNX,
    /// TensorRT optimized model format
    TensorRT,
    /// PyTorch model format (limited support)
    PyTorch,
}

/// Model priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Priority {
    /// Low priority, can be unloaded when memory is needed
    Low,
    /// Medium priority, should be kept in memory if possible
    Medium,
    /// High priority, should be kept in memory except in extreme cases
    High,
    /// Critical priority, must always be kept in memory
    Critical,
}

/// Optimization level for models
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OptimizationLevel {
    /// No optimization
    None,
    /// Basic optimization
    Low,
    /// Standard optimization
    Medium,
    /// High optimization
    High,
}

/// Deployment status for models
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeploymentStatus {
    /// Model is active and ready for inference
    Active,
    /// Model is loaded but not actively processing requests
    Idle,
    /// Model is being loaded
    Loading,
    /// Model is being unloaded
    Unloading,
    /// Model deployment failed
    Failed,
}

/// Configuration for model deployment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// Unique identifier for the model
    pub id: String,
    /// Type of model
    pub model_type: ModelType,
    /// Format of the model
    pub format: ModelFormat,
    /// Path to the model file
    pub path: PathBuf,
    /// Estimated size of the model in memory (bytes)
    pub size: usize,
    /// Priority of the model
    pub priority: Priority,
    /// Minimum compute capability required
    pub min_compute_capability: f32,
}

/// Information about a deployed model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelDeployment {
    /// Unique identifier for the model
    pub id: String,
    /// Type of model
    pub model_type: ModelType,
    /// When the model was deployed
    pub deployed_at: DateTime<Utc>,
    /// Current status of the model
    pub status: DeploymentStatus,
    /// Memory usage in bytes
    pub memory_usage: usize,
    /// Level of optimization applied
    pub optimization_level: OptimizationLevel,
}

/// Tier optimization results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierOptimization {
    /// When the optimization was performed
    pub optimized_at: DateTime<Utc>,
    /// Number of datasets moved to warm storage
    pub to_warm_count: usize,
    /// Number of datasets moved to cold storage
    pub to_cold_count: usize,
    /// Estimated performance improvement (percentage)
    pub estimated_performance_improvement: f64,
}

/// Result type for AI model operations
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_model_types() {
        let model_type = ModelType::StorageOptimizer;
        assert_eq!(model_type, ModelType::StorageOptimizer);
    }

    #[tokio::test]
    async fn test_model_config() {
        let config = ModelConfig {
            id: "test-model".to_string(),
            model_type: ModelType::StorageOptimizer,
            format: ModelFormat::ONNX,
            path: PathBuf::from("/tmp/test.onnx"),
            size: 1024 * 1024, // 1MB
            priority: Priority::Medium,
            min_compute_capability: 7.5,
        };
        
        assert_eq!(config.id, "test-model");
        assert_eq!(config.model_type, ModelType::StorageOptimizer);
    }
} 