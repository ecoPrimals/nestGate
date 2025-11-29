/// Universal Adapter Core Types
/// Core types, enums, and data structures for the NestGate Universal Adapter.
use crate::error::NestGateError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
/// Capability-specific error type
pub type CapabilityError = NestGateError;
/// Adapter-specific error type
pub type AdapterError = NestGateError;
/// Generic capability alias for backward compatibility
pub type Capability = ServiceCapability;
/// Service capability description
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Servicecapability
pub struct ServiceCapability {
    /// Capability identifier
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Detailed description
    pub description: String,
    /// Capability category
    pub category: CapabilityCategory,
    /// Version of the capability
    pub version: String,
    /// Provider name
    pub provider: String,
    /// Supported data types
    pub supported_data_types: Vec<DataType>,
    /// Performance characteristics
    pub performance_metrics: PerformanceMetrics,
    /// Resource requirements
    pub resource_requirements: ResourceRequirements,
    /// Scalability rating
    pub scalability: ScalabilityRating,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}
/// Categories of capabilities available through the universal adapter
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Capabilitycategory
pub enum CapabilityCategory {
    /// AI and machine learning capabilities
    ArtificialIntelligence,
    /// Security and cryptography capabilities
    Security,
    /// Data storage and management capabilities
    Storage,
    /// Network and communication capabilities
    Network,
    /// Compute and processing capabilities
    Compute,
    /// Orchestration and workflow capabilities
    Orchestration,
    /// Monitoring and observability capabilities
    Monitoring,
    /// Integration and connectivity capabilities
    Integration,
    /// Analytics and business intelligence capabilities
    Analytics,
    /// User interface and experience capabilities
    UserInterface,
    /// Development and deployment capabilities
    Development,
    /// Custom capability category
    Custom(String),
}
// CONSOLIDATED: Use UnifiedContentType as it better matches the granular data type needs
// of the universal adapter system. This eliminates duplicate DataType definitions.
pub use crate::unified_enums::UnifiedContentType as DataType;

/// Performance metrics for capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performancemetrics
pub struct PerformanceMetrics {
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// Throughput (operations per second)
    pub throughput_ops_per_sec: f64,
    /// Success rate percentage (0.0 to 100.0)
    pub success_rate_percent: f64,
    /// Error rate percentage (0.0 to 100.0)
    pub error_rate_percent: f64,
    /// Availability percentage (0.0 to 100.0)
    pub availability_percent: f64,
}
/// Scalability rating for capabilities
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Scalabilityrating
pub enum ScalabilityRating {
    /// Limited scalability
    Limited,
    /// Moderate scalability
    Moderate,
    /// High scalability
    High,
    /// Virtually unlimited scalability
    Unlimited,
}
/// Resource requirements for capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Resourcerequirements
pub struct ResourceRequirements {
    /// Minimum CPU cores required
    pub min_cpu_cores: u32,
    /// Minimum memory in GB
    pub min_memory_gb: f64,
    /// Minimum disk space in GB
    pub min_disk_gb: f64,
    /// Network bandwidth requirements in Mbps
    pub min_bandwidth_mbps: f64,
    /// GPU requirements (optional)
    pub gpu_requirements: Option<GpuRequirements>,
    /// Special hardware requirements
    pub special_hardware: Vec<String>,
}
/// GPU requirements specification
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Gpurequirements
pub struct GpuRequirements {
    /// Minimum GPU memory in GB
    pub min_gpu_memory_gb: f64,
    /// Required GPU compute capability
    pub min_compute_capability: String,
    /// Number of GPUs required
    pub gpu_count: u32,
    /// Specific GPU models supported
    pub supported_models: Vec<String>,
}
/// Request for capability information or execution
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for Capability operation
pub struct CapabilityRequest {
    /// Unique request identifier
    pub request_id: String,
    /// Requested capability ID
    pub capability_id: String,
    /// Request payload
    pub payload: Vec<u8>,
    /// Request metadata
    pub metadata: HashMap<String, String>,
    /// Performance requirements
    pub performance_requirements: Option<PerformanceRequirements>,
    /// Timeout for the request
    pub timeout: Option<Duration>,
    /// Priority level (0 = lowest, 10 = highest)
    pub priority: u8,
    /// Whether the request requires encryption
    pub requires_encryption: bool,
}
/// Query for discovering capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Capabilityquery
pub enum CapabilityQuery {
    /// List all available capabilities
    ListAll,
    /// Find capabilities by category
    ByCategory(CapabilityCategory),
    /// Find capabilities by data type support
    ByDataType(DataType),
    /// Find capabilities by performance criteria
    ByPerformance(PerformanceRequirements),
    /// Find capabilities by resource constraints
    ByResources(ResourceRequirements),
    /// Search capabilities by keyword
    Search(String),
    /// Get specific capability by ID
    ById(String),
}
/// Performance requirements for capability selection
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performancerequirements
pub struct PerformanceRequirements {
    /// Maximum acceptable response time in milliseconds
    pub max_response_time_ms: Option<f64>,
    /// Minimum required throughput (ops/sec)
    pub min_throughput_ops_per_sec: Option<f64>,
    /// Minimum required success rate percentage
    pub min_success_rate_percent: Option<f64>,
    /// Maximum acceptable error rate percentage
    pub max_error_rate_percent: Option<f64>,
    /// Minimum required availability percentage
    pub min_availability_percent: Option<f64>,
}
/// Response from capability request
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for Capability operation
pub struct CapabilityResponse {
    /// Original request ID
    pub request_id: String,
    /// Response payload
    pub payload: Vec<u8>,
    /// Response metadata
    pub metadata: HashMap<String, String>,
    /// Execution metrics
    pub metrics: ExecutionMetrics,
    /// Whether the response is successful
    pub success: bool,
    /// Error information (if any)
    pub error: Option<CapabilityError>,
}
/// Execution metrics for capability responses
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Executionmetrics
pub struct ExecutionMetrics {
    /// Time taken to execute the request
    pub execution_time: Duration,
    /// Resource usage during execution
    pub resource_usage: ResourceUsage,
    /// Quality metrics (if applicable)
    pub quality_metrics: Option<QualityMetrics>,
}
/// Resource usage during capability execution
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Resourceusage
pub struct ResourceUsage {
    /// CPU usage percentage
    pub cpu_usage_percent: f64,
    /// Memory usage in GB
    pub memory_usage_gb: f64,
    /// Disk I/O in MB
    pub disk_io_mb: f64,
    /// Network I/O in MB
    pub network_io_mb: f64,
    /// GPU usage percentage (if applicable)
    pub gpu_usage_percent: Option<f64>,
}
/// Quality metrics for capability outputs
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Qualitymetrics
pub struct QualityMetrics {
    /// Accuracy score (0.0 to 1.0)
    pub accuracy: Option<f64>,
    /// Precision score (0.0 to 1.0)
    pub precision: Option<f64>,
    /// Recall score (0.0 to 1.0)
    pub recall: Option<f64>,
    /// F1 score (0.0 to 1.0)
    pub f1_score: Option<f64>,
    /// Confidence level (0.0 to 1.0)
    pub confidence: Option<f64>,
    /// Custom quality metrics
    pub custom_metrics: HashMap<String, f64>,
}
impl Default for PerformanceMetrics {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            avg_response_time_ms: 0.0,
            throughput_ops_per_sec: 0.0,
            success_rate_percent: 0.0,
            error_rate_percent: 0.0,
            availability_percent: 0.0,
        }
    }
}

impl Default for ResourceRequirements {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            min_cpu_cores: 1,
            min_memory_gb: 1.0,
            min_disk_gb: 1.0,
            min_bandwidth_mbps: 1.0,
            gpu_requirements: None,
            special_hardware: Vec::new(),
        }
    }
}

impl CapabilityCategory {
    /// Get all available capability categories
    pub fn all_categories() -> Vec<Self> {
        vec![
            Self::ArtificialIntelligence,
            Self::Security,
            Self::Storage,
            Self::Network,
            Self::Compute,
            Self::Orchestration,
            Self::Monitoring,
            Self::Integration,
            Self::Analytics,
            Self::UserInterface,
            Self::Development,
        ]
    }

    /// Get category name as string
    pub fn as_str(&self) -> &str {
        match self {
            Self::ArtificialIntelligence => "ai",
            Self::Security => "security",
            Self::Storage => "storage",
            Self::Network => "network",
            Self::Compute => "compute",
            Self::Orchestration => "orchestration",
            Self::Monitoring => "monitoring",
            Self::Integration => "integration",
            Self::Analytics => "analytics",
            Self::UserInterface => "ui",
            Self::Development => "development",
            Self::Custom(name) => name,
        }
    }
}

impl DataType {
    /// Check if this data type is compatible with another
    pub fn is_compatible_with(&self, other: &Self) -> bool {
        match (self, other) {
            // Exact matches
            (a, b) if a == b => true,
            // Text-based compatibility
            (Self::Text, Self::Json) | (Self::Json, Self::Text) => true,
            (Self::Text, Self::Xml) | (Self::Xml, Self::Text) => true,
            (Self::Text, Self::Csv) | (Self::Csv, Self::Text) => true,
            // Binary compatibility
            (Self::Binary, Self::Compressed) | (Self::Compressed, Self::Binary) => {
                true
            }
            (DataType::Binary, DataType::Encrypted) | (DataType::Encrypted, DataType::Binary) => {
                true
            }
            // Custom types are compatible with anything
            (DataType::Custom(_), _) | (_, DataType::Custom(_)) => true,
            // Default: not compatible
            _ => false,
        }
    }

    /// Get MIME type for this data type
    pub fn mime_type(&self) -> &str {
        match self {
            DataType::Text => "text/plain",
            DataType::Binary => "application/octet-stream",
            DataType::Json => "application/json",
            DataType::Xml => "application/xml",
            DataType::Csv => "text/csv",
            DataType::Image => "image/*",
            DataType::Video => "video/*",
            DataType::Audio => "audio/*",
            DataType::Encrypted => "application/octet-stream",
            DataType::Compressed => "application/zip",
            DataType::Database => "application/x-database",
            DataType::TimeSeries => "application/x-timeseries",
            DataType::Geospatial => "application/geo+json",
            DataType::Graph => "application/x-graph",
            DataType::Custom(_) => "application/octet-stream",
            DataType::Html => "text/html",
            DataType::Yaml => "application/x-yaml",
            DataType::Toml => "application/toml",
            DataType::Markdown => "text/markdown",
            DataType::Pdf => "application/pdf",
        }
    }
}

impl ScalabilityRating {
    /// Get numeric score for scalability (0-10)
    pub fn score(&self) -> u8 {
        match self {
            Self::Limited => 3,
            Self::Moderate => 6,
            Self::High => 8,
            Self::Unlimited => 10,
        }
    }
}

impl PerformanceRequirements {
    /// Check if performance metrics meet these requirements
    pub fn is_satisfied_by(&self, metrics: &PerformanceMetrics) -> bool {
        if let Some(max_response_time) = self.max_response_time_ms {
            if metrics.avg_response_time_ms > max_response_time {
                return false;
            }
        }

        if let Some(min_throughput) = self.min_throughput_ops_per_sec {
            if metrics.throughput_ops_per_sec < min_throughput {
                return false;
            }
        }

        if let Some(min_success_rate) = self.min_success_rate_percent {
            if metrics.success_rate_percent < min_success_rate {
                return false;
            }
        }

        if let Some(max_error_rate) = self.max_error_rate_percent {
            if metrics.error_rate_percent > max_error_rate {
                return false;
            }
        }

        if let Some(min_availability) = self.min_availability_percent {
            if metrics.availability_percent < min_availability {
                return false;
            }
        }

        true
    }
}
