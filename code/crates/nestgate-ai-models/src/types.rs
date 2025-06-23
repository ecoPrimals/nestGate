//! Type definitions for AI models
//!
//! Common types used across the AI models system

use serde::{Serialize, Deserialize};

/// AI model capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCapabilities {
    /// Maximum input size
    pub max_input_size: usize,
    /// Maximum output size
    pub max_output_size: usize,
    /// Supported input formats
    pub input_formats: Vec<String>,
    /// Supported output formats
    pub output_formats: Vec<String>,
    /// Performance metrics
    pub performance: ModelPerformance,
}

/// Model performance characteristics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPerformance {
    /// Average inference time in milliseconds
    pub avg_inference_time_ms: f64,
    /// Peak memory usage in bytes
    pub peak_memory_usage: usize,
    /// Throughput in inferences per second
    pub throughput_ips: f64,
}

/// Model metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetadata {
    /// Model name
    pub name: String,
    /// Model version
    pub version: String,
    /// Model description
    pub description: String,
    /// Model author
    pub author: String,
    /// Model license
    pub license: String,
    /// Tags for categorization
    pub tags: Vec<String>,
}

impl Default for ModelCapabilities {
    fn default() -> Self {
        Self {
            max_input_size: 1024,
            max_output_size: 1024,
            input_formats: vec!["float32".to_string()],
            output_formats: vec!["float32".to_string()],
            performance: ModelPerformance::default(),
        }
    }
}

impl Default for ModelPerformance {
    fn default() -> Self {
        Self {
            avg_inference_time_ms: 10.0,
            peak_memory_usage: 1024 * 1024, // 1MB
            throughput_ips: 100.0,
        }
    }
}

impl Default for ModelMetadata {
    fn default() -> Self {
        Self {
            name: "Unknown".to_string(),
            version: "1.0.0".to_string(),
            description: "No description available".to_string(),
            author: "Unknown".to_string(),
            license: "Unknown".to_string(),
            tags: vec![],
        }
    }
} 