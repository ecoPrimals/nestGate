//! **ERROR CONTEXT AND RECOVERY**
//!
//! This module provides error context, retry information, and recovery
//! mechanisms for the unified error system.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

// ==================== SECTION ====================

/// Rich error context with debugging and recovery information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorContext {
    /// Unique error ID for tracking
    pub error_id: String,
    /// Component or service where error occurred
    pub component: String,
    /// Operation being performed when error occurred
    pub operation: String,
    /// Timestamp when error occurred
    pub timestamp: SystemTime,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
    /// Stack trace or call chain
    pub stack_trace: Option<Vec<String>>,
    /// Related error IDs for correlation
    pub related_errors: Vec<String>,
    /// Retry information if applicable
    pub retry_info: Option<RetryInfo>,
    /// Recovery suggestions
    pub recovery_suggestions: Vec<String>,
    /// Performance metrics at time of error
    pub performance_metrics: Option<super::data::PerformanceMetrics>,
    /// Environment information
    pub environment: Option<EnvironmentInfo>,
}

/// Retry information for recoverable errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryInfo {
    /// Current retry attempt number
    pub attempt: u32,
    /// Maximum retry attempts allowed
    pub max_attempts: u32,
    /// Delay before next retry
    pub retry_delay: Duration,
    /// Base delay for retry calculations
    pub base_delay: Duration,
    /// Whether to use exponential backoff
    pub exponential_backoff: bool,
    /// Exponential backoff multiplier
    pub backoff_multiplier: f64,
    /// Maximum retry delay
    pub max_delay: Duration,
    /// Time of last retry attempt
    pub last_attempt: SystemTime,
    /// Next scheduled retry time
    pub next_retry: SystemTime,
    /// Jitter for retry timing
    pub jitter_ms: u64,
}

/// Environment information at time of error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentInfo {
    /// Hostname or instance ID
    pub hostname: String,
    /// Process ID
    pub process_id: u32,
    /// Thread ID
    pub thread_id: Option<String>,
    /// Service version
    pub version: String,
    /// Environment name (dev, staging, prod)
    pub environment: String,
    /// Additional environment variables
    pub env_vars: HashMap<String, String>,
    /// System information
    pub system_info: Option<SystemInfo>,
}

/// System information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    /// Operating system
    pub os: String,
    /// OS version
    pub os_version: String,
    /// Architecture
    pub arch: String,
    /// Available memory in bytes
    pub memory_total: u64,
    /// Used memory in bytes
    pub memory_used: u64,
    /// CPU count
    pub cpu_count: u32,
    /// Load averages
    pub load_average: Option<[f64; 3]>,
    /// Uptime in seconds
    pub uptime: u64,
}

// ==================== SECTION ====================

/// Recovery strategy for different error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecoveryStrategy {
    /// Retry the operation with backoff
    Retry {
        max_attempts: u32,
        base_delay: Duration,
        max_delay: Duration,
        backoff_multiplier: f64,
    },
    /// Fallback to alternative implementation
    Fallback {
        fallback_service: String,
        fallback_config: HashMap<String, String>,
    },
    /// Circuit breaker pattern
    CircuitBreaker {
        failure_threshold: u32,
        recovery_timeout: Duration,
        half_open_max_calls: u32,
    },
    /// Manual intervention required
    Manual {
        escalation_contact: String,
        urgency_level: UrgencyLevel,
        remediation_steps: Vec<String>,
    },
    /// Graceful degradation
    Degrade {
        degraded_functionality: Vec<String>,
        impact_description: String,
        estimated_recovery_time: Option<Duration>,
    },
}

/// Urgency level for manual intervention
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UrgencyLevel {
    Low,
    Medium,
    High,
    Critical,
    Emergency,
}

/// Recovery action result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryResult {
    /// Whether recovery was successful
    pub success: bool,
    /// Recovery strategy used
    pub strategy_used: String,
    /// Time taken for recovery
    pub recovery_time: Duration,
    /// Additional information about recovery
    pub details: String,
    /// Metrics after recovery
    pub post_recovery_metrics: Option<super::data::PerformanceMetrics>,
}

// ==================== SECTION ====================

/// Error correlation information for tracking related errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorCorrelation {
    /// Primary correlation ID
    pub correlation_id: String,
    /// Request or transaction ID
    pub request_id: Option<String>,
    /// Session ID
    pub session_id: Option<String>,
    /// User ID
    pub user_id: Option<String>,
    /// Trace ID for distributed tracing
    pub trace_id: Option<String>,
    /// Span ID for distributed tracing
    pub span_id: Option<String>,
    /// Parent error ID
    pub parent_error_id: Option<String>,
    /// Child error IDs
    pub child_error_ids: Vec<String>,
}

/// Error aggregation for pattern analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorAggregation {
    /// Error pattern signature
    pub pattern_signature: String,
    /// Number of occurrences
    pub occurrence_count: u64,
    /// First occurrence time
    pub first_occurrence: SystemTime,
    /// Last occurrence time
    pub last_occurrence: SystemTime,
    /// Affected components
    pub affected_components: Vec<String>,
    /// Common metadata patterns
    pub common_metadata: HashMap<String, String>,
    /// Trend information
    pub trend: ErrorTrend,
}

/// Error trend information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorTrend {
    /// Error rate is increasing
    Increasing { rate_per_hour: f64 },
    /// Error rate is decreasing
    Decreasing { rate_per_hour: f64 },
    /// Error rate is stable
    Stable { average_rate: f64 },
    /// Error is sporadic
    Sporadic { last_burst: SystemTime },
}

// ==================== SECTION ====================

impl ErrorContext {
    /// Create a new error context
    pub fn new(component: &str, operation: &str) -> Self {
        Self {
            error_id: uuid::Uuid::new_v4().to_string(),
            component: component.to_string(),
            operation: operation.to_string(),
            timestamp: SystemTime::now(),
            metadata: HashMap::new(),
            stack_trace: None,
            related_errors: Vec::new(),
            retry_info: None,
            recovery_suggestions: Vec::new(),
            performance_metrics: None,
            environment: None,
        }
    }

    /// Add metadata to error context
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }

    /// Add retry information
    pub fn with_retry_info(mut self, retry_info: RetryInfo) -> Self {
        self.retry_info = Some(retry_info);
        self
    }

    /// Add recovery suggestion
    pub fn with_suggestion(mut self, suggestion: &str) -> Self {
        self.recovery_suggestions.push(suggestion.to_string());
        self
    }

    /// Add stack trace
    pub fn with_stack_trace(mut self, stack_trace: Vec<String>) -> Self {
        self.stack_trace = Some(stack_trace);
        self
    }

    /// Add related error ID
    pub fn with_related_error(mut self, error_id: &str) -> Self {
        self.related_errors.push(error_id.to_string());
        self
    }

    /// Add environment information
    pub fn with_environment(mut self, environment: EnvironmentInfo) -> Self {
        self.environment = Some(environment);
        self
    }

    /// Add performance metrics
    pub fn with_performance_metrics(mut self, metrics: super::data::PerformanceMetrics) -> Self {
        self.performance_metrics = Some(metrics);
        self
    }
}

impl RetryInfo {
    /// Create new retry info with default settings
    pub fn new(max_attempts: u32) -> Self {
        let now = SystemTime::now();
        Self {
            attempt: 1,
            max_attempts: 3,
            retry_delay: Duration::from_millis(100),
            base_delay: Duration::from_millis(100), // PEDANTIC: Added missing field
            exponential_backoff: true, // PEDANTIC: Added missing field
            backoff_multiplier: 2.0,
            max_delay: Duration::from_secs(30),
            last_attempt: now,
            next_retry: now + Duration::from_millis(100),
            jitter_ms: 50,
        }
    }

    /// Check if more retries are allowed
    pub fn can_retry(&self) -> bool {
        self.attempt < self.max_attempts
    }

    /// Calculate next retry delay with exponential backoff
    pub fn next_delay(&self) -> Duration {
        let base_delay_ms = self.retry_delay.as_millis() as f64;
        let backoff_delay_ms = base_delay_ms * self.backoff_multiplier.powi(self.attempt as i32);
        let jitter = (rand::random::<f64>() * 2.0 - 1.0) * self.jitter_ms as f64;
        let total_delay_ms = (backoff_delay_ms + jitter).max(0.0);
        
        let delay = Duration::from_millis(total_delay_ms as u64);
        if delay > self.max_delay {
            self.max_delay
        } else {
            delay
        }
    }

    /// Increment retry attempt and update timing
    pub fn increment_attempt(&mut self) {
        self.attempt += 1;
        self.last_attempt = SystemTime::now();
        self.retry_delay = self.next_delay();
        self.next_retry = self.last_attempt + self.retry_delay;
    }
}

impl Default for ErrorContext {
    fn default() -> Self {
        Self::new("unknown", "unknown")
    }
}

impl Default for RetryInfo {
    fn default() -> Self {
        Self::new(3)
    }
}
