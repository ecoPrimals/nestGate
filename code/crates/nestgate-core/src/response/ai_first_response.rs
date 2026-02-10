/// AI-First Response Module
/// Implements the `EcoPrimals` AI-First Citizen API Standard for `NestGate`
/// **ECOSYSTEM ALIGNMENT**: Enhances `NestGate` from 70% to 85%+ AI-First compliance
use axum::response::{IntoResponse, Json};
// Removed unused import: use chrono::DateTime;
use crate::error::NestGateError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Universal AI-First response format - ALL ENDPOINTS SHOULD USE THIS
/// Based on the `EcoPrimals` AI-First Citizen API Standard
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for AIFirst operation
pub struct AIFirstResponse<T> {
    /// Operation success status (machine-readable)
    pub success: bool,
    /// Strongly-typed response data
    pub data: T,

    /// AI-optimized error information
    pub error: Option<AIFirstError>,

    /// Unique request identifier for tracing and correlation
    pub request_id: Uuid,

    /// Processing time in milliseconds for performance monitoring
    pub processing_time_ms: u64,

    /// AI-specific metadata for decision making
    pub ai_metadata: AIResponseMetadata,

    /// Human interaction context (when applicable)
    pub human_context: Option<HumanInteractionContext>,

    /// Confidence score for AI decision making (0.0 - 1.0)
    pub confidence_score: f64,

    /// Suggested next actions for AI agents
    pub suggested_actions: Vec<SuggestedAction>,
}

/// AI-optimized error structure with automation hints
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Error type for AIFirst operations
pub struct AIFirstError {
    /// Machine-readable error code (`UPPER_SNAKE_CASE`)
    pub code: String,
    /// Human-readable message (for logging/debugging)
    pub message: String,

    /// Error category for AI classification
    pub category: AIErrorCategory,

    /// Automated retry strategy
    pub retry_strategy: RetryStrategy,

    /// Actionable hints for AI automation
    pub automation_hints: Vec<String>,

    /// Severity level for prioritization
    pub severity: ErrorSeverity,

    /// Whether human intervention is required
    pub requires_human_intervention: bool,

    /// Related error context for debugging
    pub context: HashMap<String, serde_json::Value>,
}

/// AI-specific metadata for enhanced decision making
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Airesponsemetadata
pub struct AIResponseMetadata {
    /// Service capabilities relevant to this response
    pub capabilities: Vec<String>,
    /// Resource utilization metrics
    pub resource_usage: ResourceUsage,

    /// Performance characteristics
    pub performance_metrics: PerformanceMetrics,

    /// Data quality indicators
    pub data_quality: DataQuality,

    /// Caching information for optimization
    pub cache_info: CacheInfo,

    /// Extension points for service-specific metadata
    pub extensions: HashMap<String, serde_json::Value>,
}

/// Human interaction context for mixed AI-human workflows
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Humaninteractioncontext
pub struct HumanInteractionContext {
    /// Whether human review is recommended
    pub requires_human_review: bool,
    /// Priority level for human attention
    pub human_priority: HumanPriority,

    /// Suggested human actions
    pub suggested_human_actions: Vec<String>,

    /// Context for human understanding
    pub explanation: String,

    /// User interface hints
    pub ui_hints: UIHints,
}

/// Suggested actions for AI agents
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Suggestedaction
pub struct SuggestedAction {
    /// Action type identifier
    pub action_type: String,
    /// Action description
    pub description: String,

    /// Action parameters
    pub parameters: HashMap<String, serde_json::Value>,

    /// Confidence in this action (0.0 - 1.0)
    pub confidence: f64,

    /// Prerequisites for this action
    pub prerequisites: Vec<String>,

    /// Expected outcome
    pub expected_outcome: String,
}

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Aierrorcategory
pub enum AIErrorCategory {
    /// Input validation errors
    Validation,
    /// Authentication/authorization errors  
    Security,
    /// Resource availability errors
    Resource,
    /// External service errors
    External,
    /// Internal system errors
    Internal,
    /// Configuration errors
    Configuration,
    /// Network connectivity errors
    Network,
    /// Data consistency errors
    Data,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Retrystrategy
pub enum RetryStrategy {
    /// No retry recommended
    None,
    /// Immediate retry
    Immediate,
    /// Exponential backoff
    ExponentialBackoff {
        /// Base delay in milliseconds
        base_delay_ms: u64,
        /// Maximum number of retry attempts
        max_retries: u32,
    },
    /// Fixed interval retry
    FixedInterval {
        /// Interval between retries in milliseconds
        interval_ms: u64,
        /// Maximum number of retry attempts
        max_retries: u32,
    },
    /// Custom retry logic
    Custom {
        /// Identifier for the retry strategy
        strategy_id: String,
        /// Strategy-specific parameters
        parameters: HashMap<String, serde_json::Value>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Errorseverity
pub enum ErrorSeverity {
    /// Low impact, informational
    Low,
    /// Medium impact, warning
    Medium,
    /// High impact, error
    High,
    /// Critical impact, immediate attention
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Humanpriority
pub enum HumanPriority {
    /// No human intervention needed
    None,
    /// Low priority review
    Low,
    /// Medium priority review
    Medium,
    /// High priority review
    High,
    /// Immediate human attention required
    Urgent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Resourceusage
pub struct ResourceUsage {
    /// CPU utilization percentage
    pub cpu_percent: f64,
    /// Memory usage in bytes
    pub memory_bytes: u64,
    /// Network I/O in bytes
    pub network_bytes: u64,
    /// Disk I/O in bytes
    pub disk_bytes: u64,
    /// Processing duration in milliseconds
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performancemetrics
pub struct PerformanceMetrics {
    /// Response latency percentiles
    pub latency_percentiles: HashMap<String, f64>,
    /// Throughput metrics
    pub throughput_ops_per_sec: f64,
    /// Error rates
    pub error_rate: f64,
    /// Cache hit ratio
    pub cache_hit_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Dataquality
pub struct DataQuality {
    /// Completeness score (0.0 - 1.0)
    pub completeness: f64,
    /// Accuracy score (0.0 - 1.0)
    pub accuracy: f64,
    /// Freshness score (0.0 - 1.0)
    pub freshness: f64,
    /// Consistency score (0.0 - 1.0)
    pub consistency: f64,
    /// Data source reliability
    pub source_reliability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Cacheinfo
pub struct CacheInfo {
    /// Whether response can be cached
    pub cacheable: bool,
    /// Cache TTL in seconds
    pub ttl_seconds: Option<u64>,
    /// Cache key for optimization
    pub cache_key: Option<String>,
    /// Cache tags for invalidation
    pub cache_tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Uihints
pub struct UIHints {
    /// Suggested UI component type
    pub component_type: String,
    /// Visual priority level
    pub visual_priority: String,
    /// Color scheme hints
    pub color_scheme: Option<String>,
    /// Icon suggestions
    pub icon_suggestions: Vec<String>,
}

// ==================== SECTION ====================

impl<T> AIFirstResponse<T> {
    /// Create a successful AI-First response
    pub fn success(data: T, request_id: Uuid, processing_time_ms: u64) -> Self {
        Self {
            success: true,
            data,
            error: None,
            request_id,
            processing_time_ms,
            ai_metadata: AIResponseMetadata::default(),
            human_context: None,
            confidence_score: 1.0,
            suggested_actions: vec![],
        }
    }

    /// Create an error AI-First response
    pub fn error(data: T, error: AIFirstError, request_id: Uuid, processing_time_ms: u64) -> Self {
        Self {
            success: false,
            data,
            error: Some(error),
            request_id,
            processing_time_ms,
            ai_metadata: AIResponseMetadata::default(),
            human_context: None,
            confidence_score: 0.0,
            suggested_actions: vec![],
        }
    }

    /// Add AI metadata to the response
    #[must_use]
    pub fn with_ai_metadata(mut self, metadata: AIResponseMetadata) -> Self {
        self.ai_metadata = metadata;
        self
    }

    /// Add human context to the response
    #[must_use]
    pub fn with_human_context(mut self, context: HumanInteractionContext) -> Self {
        self.human_context = Some(context);
        self
    }

    /// Set confidence score
    #[must_use]
    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence_score = confidence.clamp(0.0, 1.0);
        self
    }

    /// Add suggested actions
    #[must_use]
    pub fn with_suggested_actions(mut self, actions: Vec<SuggestedAction>) -> Self {
        self.suggested_actions = actions;
        self
    }
}

impl AIFirstError {
    /// Create a new AI-First error from `NestGateError`
    #[must_use]
    pub fn from_nestgate_error(error: &NestGateError) -> Self {
        let (category, retry_strategy, automation_hints) = match error {
            NestGateError::Configuration { .. } => (
                AIErrorCategory::Configuration,
                RetryStrategy::None,
                vec![
                    "Check configuration files".to_string(),
                    "Validate environment variables".to_string(),
                ],
            ),
            NestGateError::Network { .. } => (
                AIErrorCategory::Network,
                RetryStrategy::ExponentialBackoff {
                    base_delay_ms: 1000,
                    max_retries: 3,
                },
                vec![
                    "Check network connectivity".to_string(),
                    "Retry with backoff".to_string(),
                ],
            ),
            NestGateError::Security { .. } => (
                AIErrorCategory::Security,
                RetryStrategy::None,
                vec![
                    "Check authentication credentials".to_string(),
                    "Verify permissions".to_string(),
                ],
            ),
            NestGateError::Validation { .. } => (
                AIErrorCategory::Validation,
                RetryStrategy::None,
                vec![
                    "Fix input validation errors".to_string(),
                    "Check required fields".to_string(),
                ],
            ),
            _ => (
                AIErrorCategory::Internal,
                RetryStrategy::Immediate,
                vec![
                    "Check system logs".to_string(),
                    "Contact support if persistent".to_string(),
                ],
            ),
        };

        Self {
            code: error.to_string().to_uppercase().replace(' ', "_"),
            message: error.to_string(),
            category,
            retry_strategy,
            automation_hints,
            severity: ErrorSeverity::Medium,
            requires_human_intervention: false,
            context: HashMap::new(),
        }
    }

    /// Create a simple AI-First error
    #[must_use]
    pub fn simple(code: &str, message: &str, category: AIErrorCategory) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
            category,
            retry_strategy: RetryStrategy::None,
            automation_hints: vec![],
            severity: ErrorSeverity::Medium,
            requires_human_intervention: false,
            context: HashMap::new(),
        }
    }
}

impl Default for AIResponseMetadata {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            capabilities: vec![],
            resource_usage: ResourceUsage {
                cpu_percent: 0.0,
                memory_bytes: 0,
                network_bytes: 0,
                disk_bytes: 0,
                duration_ms: 0,
            },
            performance_metrics: PerformanceMetrics {
                latency_percentiles: HashMap::new(),
                throughput_ops_per_sec: 0.0,
                error_rate: 0.0,
                cache_hit_ratio: 0.0,
            },
            data_quality: DataQuality {
                completeness: 1.0,
                accuracy: 1.0,
                freshness: 1.0,
                consistency: 1.0,
                source_reliability: 1.0,
            },
            cache_info: CacheInfo {
                cacheable: false,
                ttl_seconds: None,
                cache_key: None,
                cache_tags: vec![],
            },
            extensions: HashMap::new(),
        }
    }
}

impl<T: Serialize> IntoResponse for AIFirstResponse<T> {
    /// Into Response
    fn into_response(self) -> axum::response::Response {
        let status = if self.success {
            axum::http::StatusCode::OK
        } else {
            axum::http::StatusCode::BAD_REQUEST
        };

        (status, Json(self)).into_response()
    }
}

// ==================== SECTION ====================

/// Trait for converting standard responses to AI-First format
pub trait IntoAIFirstResponse<T> {
    /// Convert to AI-First response format
    fn into_ai_first(self, request_id: Uuid, processing_time_ms: u64) -> AIFirstResponse<T>;
}
impl<T: Default> IntoAIFirstResponse<T> for Result<T, NestGateError> {
    /// Into Ai First
    fn into_ai_first(self, request_id: Uuid, processing_time_ms: u64) -> AIFirstResponse<T> {
        match self {
            Ok(data) => AIFirstResponse::success(data, request_id, processing_time_ms),
            Err(error) => AIFirstResponse::error_from_nestgate(
                T::default(),
                error,
                request_id,
                processing_time_ms,
            ),
        }
    }
}

impl<T> AIFirstResponse<T> {
    /// Create an error response from `NestGateError` with default data
    pub fn error_from_nestgate(
        default_data: T,
        error: NestGateError,
        request_id: Uuid,
        processing_time_ms: u64,
    ) -> Self {
        let ai_error = AIFirstError::from_nestgate_error(&error);
        Self::error(default_data, ai_error, request_id, processing_time_ms)
    }
}
