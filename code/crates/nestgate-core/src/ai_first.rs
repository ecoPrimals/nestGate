//! AI-First Citizen API compliance types
//! Implements the ecoPrimals AI-First Citizen API Standard
//!
//! This module provides the universal AI-first response format that enables
//! seamless integration with AI agents while maintaining human compatibility.
//!
//! ## Design Principles
//! - **AI agents are primary consumers**: Machine-readable structure first
//! - **Human compatibility**: Rich context for human understanding  
//! - **Confidence scoring**: Enable AI decision-making with uncertainty
//! - **Suggested actions**: Guide AI automation workflows
//! - **Error categorization**: Support automated error recovery

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Universal AI-first response format - ALL ENDPOINTS MUST USE THIS
///
/// This response format ensures compatibility with AI agents across the
/// ecoPrimals ecosystem while providing rich context for human operators.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
///
/// Provides machine-readable error codes, categorization for AI classification,
/// and actionable hints for automated recovery workflows.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIFirstError {
    /// Machine-readable error code (UPPER_SNAKE_CASE)
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

/// AI error categorization for machine learning
///
/// Enables AI agents to classify and respond appropriately to different
/// types of errors using consistent categorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AIErrorCategory {
    /// Transient errors that may resolve on retry
    Transient,
    /// Configuration errors requiring setup changes
    Configuration,
    /// Permission/authorization errors
    Authorization,
    /// Resource exhaustion errors
    ResourceExhaustion,
    /// Data validation errors
    Validation,
    /// External service errors
    ExternalService,
    /// Internal system errors
    Internal,
}

/// Automated retry strategy for AI agents
///
/// Provides structured guidance for AI agents on whether and how
/// to retry failed operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryStrategy {
    pub should_retry: bool,
    pub max_attempts: u32,
    pub backoff_seconds: Vec<u64>,
    pub retry_conditions: Vec<String>,
}

/// Suggested actions for AI automation
///
/// Provides concrete, actionable suggestions that AI agents can execute
/// to continue workflows or resolve issues.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestedAction {
    pub action_type: String,
    pub description: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub confidence: f64,
    pub estimated_duration_ms: u64,
}

/// AI-specific response metadata
///
/// Provides performance and optimization information that AI agents
/// can use for resource planning and workflow optimization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIResponseMetadata {
    pub operation_type: String,
    pub complexity_score: f64,
    pub resource_usage: ResourceUsage,
    pub performance_hints: Vec<String>,
    pub optimization_opportunities: Vec<String>,
}

/// Resource usage information for AI optimization
///
/// Enables AI agents to make informed decisions about resource allocation
/// and workflow scheduling based on actual resource consumption.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub cpu_time_ms: u64,
    pub memory_bytes: u64,
    pub disk_io_bytes: u64,
    pub network_io_bytes: u64,
}

/// Human interaction context when humans are involved
///
/// Provides context for operations that may require human approval
/// or have accessibility considerations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanInteractionContext {
    pub requires_human_approval: bool,
    pub user_preference_hints: Vec<String>,
    pub accessibility_requirements: Vec<String>,
}

/// Error severity for AI prioritization
///
/// Enables AI agents to prioritize error handling and escalation
/// based on business impact.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorSeverity {
    Critical, // Service impacting
    High,     // Feature impacting
    Medium,   // Performance impacting
    Low,      // Cosmetic or minor
}

impl<T> AIFirstResponse<T> {
    /// Create a successful AI-First response
    pub fn success(
        data: T,
        request_id: Uuid,
        processing_time_ms: u64,
        confidence_score: f64,
    ) -> Self {
        Self {
            success: true,
            data,
            error: None,
            request_id,
            processing_time_ms,
            ai_metadata: AIResponseMetadata::default(),
            human_context: None,
            confidence_score,
            suggested_actions: vec![],
        }
    }

    /// Create a failed AI-First response
    pub fn error(
        error: AIFirstError,
        request_id: Uuid,
        processing_time_ms: u64,
        confidence_score: f64,
    ) -> Self
    where
        T: Default,
    {
        Self {
            success: false,
            data: T::default(),
            error: Some(error),
            request_id,
            processing_time_ms,
            ai_metadata: AIResponseMetadata::default(),
            human_context: None,
            confidence_score,
            suggested_actions: vec![],
        }
    }

    /// Add suggested actions to the response
    pub fn with_suggested_actions(mut self, actions: Vec<SuggestedAction>) -> Self {
        self.suggested_actions = actions;
        self
    }

    /// Add AI metadata to the response
    pub fn with_ai_metadata(mut self, metadata: AIResponseMetadata) -> Self {
        self.ai_metadata = metadata;
        self
    }

    /// Add human interaction context
    pub fn with_human_context(mut self, context: HumanInteractionContext) -> Self {
        self.human_context = Some(context);
        self
    }
}

impl AIFirstError {
    /// Create a transient error with retry strategy
    pub fn transient(code: String, message: String) -> Self {
        Self {
            code,
            message,
            category: AIErrorCategory::Transient,
            retry_strategy: RetryStrategy {
                should_retry: true,
                max_attempts: 3,
                backoff_seconds: vec![1, 2, 4],
                retry_conditions: vec![
                    "Network connectivity restored".to_string(),
                    "Resource availability increased".to_string(),
                ],
            },
            automation_hints: vec![
                "Wait for resource availability".to_string(),
                "Check network connectivity".to_string(),
            ],
            severity: ErrorSeverity::Medium,
            requires_human_intervention: false,
            context: HashMap::new(),
        }
    }

    /// Create a configuration error requiring human attention
    pub fn configuration(code: String, message: String) -> Self {
        Self {
            code,
            message,
            category: AIErrorCategory::Configuration,
            retry_strategy: RetryStrategy {
                should_retry: false,
                max_attempts: 0,
                backoff_seconds: vec![],
                retry_conditions: vec!["Configuration updated".to_string()],
            },
            automation_hints: vec![
                "Review configuration settings".to_string(),
                "Validate required parameters".to_string(),
            ],
            severity: ErrorSeverity::High,
            requires_human_intervention: true,
            context: HashMap::new(),
        }
    }

    /// Create a critical error requiring immediate attention
    pub fn critical(code: String, message: String) -> Self {
        Self {
            code,
            message,
            category: AIErrorCategory::Internal,
            retry_strategy: RetryStrategy {
                should_retry: false,
                max_attempts: 0,
                backoff_seconds: vec![],
                retry_conditions: vec!["System recovery completed".to_string()],
            },
            automation_hints: vec![
                "Escalate to operations team".to_string(),
                "Check system health".to_string(),
                "Review error logs".to_string(),
            ],
            severity: ErrorSeverity::Critical,
            requires_human_intervention: true,
            context: HashMap::new(),
        }
    }
}

impl Default for AIResponseMetadata {
    fn default() -> Self {
        Self {
            operation_type: "unknown".to_string(),
            complexity_score: 0.5,
            resource_usage: ResourceUsage::default(),
            performance_hints: vec![],
            optimization_opportunities: vec![],
        }
    }
}

impl Default for ResourceUsage {
    fn default() -> Self {
        Self {
            cpu_time_ms: 0,
            memory_bytes: 0,
            disk_io_bytes: 0,
            network_io_bytes: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_or_create_uuid;
    use std::collections::HashMap;

    #[test]
    fn test_ai_first_response_creation() {
        let request_id = *get_or_create_uuid("ai_inference_request");
        let response = AIFirstResponse::success("test_data".to_string(), request_id, 100, 0.95);

        assert_eq!(response.success, true);
        assert_eq!(response.data, "test_data");
        assert_eq!(response.request_id, request_id);
        assert_eq!(response.processing_time_ms, 100);
        assert_eq!(response.confidence_score, 0.95);
        assert!(response.error.is_none());
    }

    #[test]
    fn test_ai_first_error_creation() {
        let error = AIFirstError::transient(
            "NETWORK_TIMEOUT".to_string(),
            "Network operation timed out".to_string(),
        );

        assert_eq!(error.code, "NETWORK_TIMEOUT");
        assert!(matches!(error.category, AIErrorCategory::Transient));
        assert!(error.retry_strategy.should_retry);
        assert_eq!(error.retry_strategy.max_attempts, 3);
        assert!(!error.requires_human_intervention);
    }

    #[test]
    fn test_critical_error_requires_human_intervention() {
        let error = AIFirstError::critical(
            "SYSTEM_FAILURE".to_string(),
            "Critical system component failed".to_string(),
        );

        assert!(error.requires_human_intervention);
        assert!(matches!(error.severity, ErrorSeverity::Critical));
        assert!(!error.retry_strategy.should_retry);
    }

    #[test]
    fn test_response_builder_pattern() {
        let request_id = *get_or_create_uuid("ai_training_request");
        let actions = vec![SuggestedAction {
            action_type: "retry".to_string(),
            description: "Retry the operation".to_string(),
            parameters: HashMap::new(),
            confidence: 0.8,
            estimated_duration_ms: 1000,
        }];

        let response = AIFirstResponse::success(42, request_id, 50, 0.9)
            .with_suggested_actions(actions.clone());

        assert_eq!(response.suggested_actions.len(), 1);
        assert_eq!(response.suggested_actions[0].action_type, "retry");
    }
}
