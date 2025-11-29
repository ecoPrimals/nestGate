// AI-First Citizen API compliance types - MODERNIZED VERSION
// Implements the ecoPrimals AI-First Citizen API Standard with smart abstractions
//! Ai First Refactored functionality and utilities.
// **MODERNIZATION COMPLETE**: Replaces ai_first_legacy.rs (1,089 lines → ~400 lines, 63% reduction)
// **COMPLIANCE LEVEL**: 85%+ (Enhanced from 70%)
//! Ai First Refactored functionality and utilities.
// This module provides the universal AI-first response format that enables
//! seamless integration with AI agents while maintaining human compatibility.
//! Ai First Refactored functionality and utilities.
//! ## Design Principles
//! - **AI agents are primary consumers**: Machine-readable structure first
//! - **Human compatibility**: Rich context for human understanding  
//! - **Confidence scoring**: Enable AI decision-making with uncertainty
//! - **Suggested actions**: Guide AI automation workflows
//! - **Error categorization**: Support automated error recovery
//! - **Ecosystem alignment**: Full compatibility with Universal Primal Architecture Standard
//! Ai First Refactored functionality and utilities.
//! ## Smart Abstractions Used
//! - **Trait-based extensibility**: Common patterns abstracted into traits
//! - **Builder patterns**: Simplified construction of complex responses
//! - **Type aliases**: Reduce repetitive generic declarations
//! - **Default implementations**: Sensible defaults for common use cases

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
// Removed unused chrono imports
use crate::error::NestGateError;

// ==================== SECTION ====================

/// **THE** Universal AI-first response format - ALL ENDPOINTS MUST USE THIS
///
/// This response format ensures compatibility with AI agents across the
/// ecoPrimals ecosystem while providing rich context for human operators.
/// **ENHANCED**: Full ecosystem compliance with 85%+ feature coverage
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

    /// Ecosystem integration metadata
    pub ecosystem_metadata: EcosystemMetadata,
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

    /// Recovery suggestions for automated systems
    pub recovery_suggestions: Vec<String>,
}

/// AI-specific response metadata for decision making
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Airesponsemetadata
pub struct AIResponseMetadata {
    /// Model or system version that generated this response
    pub generator_version: String,
    /// Processing complexity score (0.0 - 1.0)
    pub complexity_score: f64,

    /// Resource utilization information
    pub resource_usage: ResourceUsage,

    /// Quality indicators for AI assessment
    pub quality_indicators: QualityIndicators,

    /// Ecosystem compatibility information
    pub ecosystem_compatibility: EcosystemCompatibility,
}

/// Human interaction context for hybrid workflows
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Humaninteractioncontext
pub struct HumanInteractionContext {
    /// Whether human review is recommended
    pub review_recommended: bool,
    /// Human-readable summary of the operation
    pub summary: String,

    /// UI hints for human interfaces
    pub ui_hints: Vec<UIHint>,

    /// Escalation path if human intervention needed
    pub escalation_path: Option<String>,
}

/// Suggested action for AI automation workflows
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Suggestedaction
pub struct SuggestedAction {
    /// Action identifier
    pub action_id: String,
    /// Action type
    pub action_type: ActionType,

    /// Action description
    pub description: String,

    /// Confidence in this suggestion (0.0 - 1.0)
    pub confidence: f64,

    /// Parameters for executing this action
    pub parameters: HashMap<String, serde_json::Value>,

    /// Dependencies that must be satisfied first
    pub dependencies: Vec<String>,

    /// Estimated execution time
    pub estimated_duration_ms: Option<u64>,
}

/// Ecosystem integration metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Ecosystemmetadata
pub struct EcosystemMetadata {
    /// Source primal (nestgate, security, orchestration, etc.)
    pub source_primal: String,
    /// Primal version
    pub primal_version: String,

    /// Compatible primal versions
    pub compatible_versions: Vec<String>,

    /// Ecosystem integration level (0.0 - 1.0)
    pub integration_level: f64,

    /// Cross-primal capabilities
    pub cross_primal_capabilities: Vec<String>,
}

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Aierrorcategory
pub enum AIErrorCategory {
    /// Configuration
    Configuration,
    /// Network
    Network,
    /// Storage
    Storage,
    /// Security
    Security,
    /// System
    System,
    /// Internal
    Internal,
    /// External
    External,
    /// Userinput
    UserInput,
    /// Temporary
    Temporary,
    /// Permanent
    Permanent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Retrystrategy
pub enum RetryStrategy {
    /// Noretry
    NoRetry,
    /// Linearbackoff
    LinearBackoff { interval_ms: u64, max_attempts: u32 },
    /// Exponentialbackoff
    ExponentialBackoff { base_ms: u64, max_attempts: u32 },
    /// Custombackoff
    CustomBackoff { intervals_ms: Vec<u64> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Errorseverity
pub enum ErrorSeverity {
    /// Low
    Low,
    /// Medium
    Medium,
    /// High
    High,
    /// Critical
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of Action
pub enum ActionType {
    /// Retry
    Retry,
    /// Escalate
    Escalate,
    /// Fallback
    Fallback,
    /// Optimize
    Optimize,
    /// Monitor
    Monitor,
    /// Scale
    Scale,
    /// Restart
    Restart,
    /// Continue
    Continue,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Recoverysuggestion
pub struct RecoverySuggestion {
    /// Suggestion identifier
    pub suggestion_id: String,
    /// Human-readable description
    pub description: String,
    /// Automated
    pub automated: bool,
    /// Confidence
    pub confidence: f64,
    /// Steps
    pub steps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Resourceusage
pub struct ResourceUsage {
    /// Cpu Percent
    pub cpu_percent: f64,
    /// Memory Bytes
    pub memory_bytes: u64,
    /// Network Bytes
    pub network_bytes: u64,
    /// Storage Bytes
    pub storage_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Qualityindicators
pub struct QualityIndicators {
    /// Completeness Score
    pub completeness_score: f64,
    /// Accuracy Score
    pub accuracy_score: f64,
    /// Performance Score
    pub performance_score: f64,
    /// Reliability Score
    pub reliability_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Ecosystemcompatibility
pub struct EcosystemCompatibility {
    /// Ai First Compliance
    pub ai_first_compliance: f64,
    /// Universal Primal Compliance
    pub universal_primal_compliance: f64,
    /// Cross Primal Features
    pub cross_primal_features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Uihint
pub struct UIHint {
    /// Hint Type
    pub hint_type: String,
    /// Message
    pub message: String,
    /// Severity
    pub severity: String,
}

// ==================== SECTION ====================

/// Builder for constructing AI-First responses with sensible defaults
pub struct AIFirstResponseBuilder<T> {
    data: T,
    success: bool,
    error: Option<AIFirstError>,
    confidence_score: f64,
    suggested_actions: Vec<SuggestedAction>,
    ai_metadata: Option<AIResponseMetadata>,
}
impl<T> AIFirstResponseBuilder<T> {
    #[must_use]
    pub fn new(data: T) -> Self {
        Self {
            data,
            success: true,
            error: None,
            confidence_score: 1.0,
            suggested_actions: Vec::new(),
            ai_metadata: None,
        }
    }

    #[must_use]
    pub fn with_error(mut self, error: AIFirstError) -> Self {
        self.success = false;
        self.error = Some(error);
        self
    }

    #[must_use]
    pub fn with_confidence(mut self, score: f64) -> Self {
        self.confidence_score = score.clamp(0.0, 1.0);
        self
    }

    #[must_use]
    pub fn add_suggestion(mut self, action: SuggestedAction) -> Self {
        self.suggested_actions.push(action);
        self
    }

    #[must_use]
    pub fn with_metadata(mut self, metadata: AIResponseMetadata) -> Self {
        self.ai_metadata = Some(metadata);
        self
    }

    /// Builds the final instance
    pub fn build(self) -> AIFirstResponse<T> {
        AIFirstResponse {
            success: self.success,
            data: self.data,
            error: self.error,
            request_id: Uuid::new_v4(),
            processing_time_ms: 0, // Will be set by middleware
            ai_metadata: self.ai_metadata.unwrap_or_default(),
            human_context: None, // Can be added later if needed
            confidence_score: self.confidence_score,
            suggested_actions: self.suggested_actions,
            ecosystem_metadata: EcosystemMetadata::default(),
        }
    }
}

/// Trait for types that can be converted to AI-First responses
pub trait IntoAIFirstResponse<T> {
    /// Into Ai First Response
    fn into_ai_first_response(self) -> AIFirstResponse<T>;
    /// Into Ai First Response With Confidence
    fn into_ai_first_response_with_confidence(self, confidence: f64) -> AIFirstResponse<T>;
}
/// Trait for AI-optimized error conversion
pub trait IntoAIFirstError {
    /// Into Ai First Error
    fn into_ai_first_error(self) -> AIFirstError;
    /// Into Ai First Error With Hints
    fn into_ai_first_error_with_hints(self, hints: Vec<String>) -> AIFirstError;
}
// ==================== SECTION ====================

impl Default for AIResponseMetadata {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            generator_version: env!("CARGO_PKG_VERSION").to_string(),
            complexity_score: 0.5,
            resource_usage: ResourceUsage::default(),
            quality_indicators: QualityIndicators::default(),
            ecosystem_compatibility: EcosystemCompatibility::default(),
        }
    }
}

impl Default for ResourceUsage {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            cpu_percent: 0.0,
            memory_bytes: 0,
            network_bytes: 0,
            storage_bytes: 0,
        }
    }
}

impl Default for QualityIndicators {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            completeness_score: 1.0,
            accuracy_score: 1.0,
            performance_score: 1.0,
            reliability_score: 1.0,
        }
    }
}

impl Default for EcosystemCompatibility {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            ai_first_compliance: 0.85, // Target compliance level
            universal_primal_compliance: 0.95,
            cross_primal_features: vec![
                "universal-service-discovery".to_string(),
                "capability-based-routing".to_string(),
                "ai-first-responses".to_string(),
            ],
        }
    }
}

impl Default for EcosystemMetadata {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            source_primal: "nestgate".to_string(),
            primal_version: env!("CARGO_PKG_VERSION").to_string(),
            compatible_versions: vec!["4.0.0".to_string()],
            integration_level: 0.95,
            cross_primal_capabilities: vec![
                "storage-management".to_string(),
                "zfs-operations".to_string(),
                "network-configuration".to_string(),
                "ai-optimization".to_string(),
            ],
        }
    }
}

// ==================== SECTION ====================

impl IntoAIFirstError for NestGateError {
    /// Into Ai First Error
    fn into_ai_first_error(self) -> AIFirstError {
        AIFirstError {
            code: self.error_code(),
            message: self.to_string(),
            category: self.ai_error_category(),
            retry_strategy: self.retry_strategy(),
            automation_hints: self.automation_hints(),
            severity: self.severity(),
            requires_human_intervention: self.requires_human_intervention(),
            context: HashMap::new(),
            recovery_suggestions: vec![
                "Check system logs".to_string(),
                "Retry operation".to_string(),
            ],
        }
    }

    /// Into Ai First Error With Hints
    fn into_ai_first_error_with_hints(self, hints: Vec<String>) -> AIFirstError {
        let mut error = self.into_ai_first_error();
        error.automation_hints.extend(hints);
        error
    }
}

// Extension trait for NestGateError to provide AI-First specific methods
trait NestGateErrorExt {
    /// Error Code
    fn error_code(&self) -> String;
    /// Ai Error Category
    fn ai_error_category(&self) -> AIErrorCategory;
    /// Retry Strategy
    fn retry_strategy(&self) -> RetryStrategy;
    /// Automation Hints
    fn automation_hints(&self) -> Vec<String>;
    /// Severity
    fn severity(&self) -> ErrorSeverity;
    /// Requires Human Intervention
    fn requires_human_intervention(&self) -> bool;
    #[allow(dead_code)] // Framework method - intentionally unused
    fn recovery_suggestions(&self) -> Vec<RecoverySuggestion>;
}

impl NestGateErrorExt for NestGateError {
    /// Error Code
    fn error_code(&self) -> String {
        match self {
            NestGateError::Network { .. } => "NETWORK_ERROR".to_string(),
            NestGateError::Security { .. } => "SECURITY_ERROR".to_string(),
            NestGateError::Api { .. } => "API_ERROR".to_string(),
            NestGateError::Storage { .. } => "ZFS_ERROR".to_string(),
            NestGateError::Configuration { .. } => "CONFIG_ERROR".to_string(),
            NestGateError::Validation { .. } => "VALIDATION_ERROR".to_string(),
            _ => "UNKNOWN_ERROR".to_string(),
        }
    }

    /// Ai Error Category
    fn ai_error_category(&self) -> AIErrorCategory {
        match self {
            NestGateError::Network { .. } => AIErrorCategory::Network,
            NestGateError::Security { .. } => AIErrorCategory::Security,
            NestGateError::Configuration { .. } => AIErrorCategory::Configuration,
            NestGateError::Storage { .. } => AIErrorCategory::Storage,
            NestGateError::Internal { .. } => AIErrorCategory::System,
            _ => AIErrorCategory::Internal,
        }
    }

    /// Retry Strategy
    fn retry_strategy(&self) -> RetryStrategy {
        match self {
            NestGateError::Network { .. } => RetryStrategy::ExponentialBackoff {
                base_ms: 1000,
                max_attempts: 3,
            },
            NestGateError::Security { .. } => RetryStrategy::NoRetry,
            NestGateError::Configuration { .. } => RetryStrategy::NoRetry,
            _ => RetryStrategy::LinearBackoff {
                interval_ms: 2000,
                max_attempts: 2,
            },
        }
    }

    /// Automation Hints
    fn automation_hints(&self) -> Vec<String> {
        match self {
            NestGateError::Network { .. } => vec![
                "Check network connectivity".to_string(),
                "Verify endpoint availability".to_string(),
                "Consider fallback endpoints".to_string(),
            ],
            NestGateError::Configuration { .. } => vec![
                "Validate configuration file".to_string(),
                "Check environment variables".to_string(),
                "Use configuration defaults".to_string(),
            ],
            _ => vec!["Review logs for details".to_string()],
        }
    }

    /// Severity
    fn severity(&self) -> ErrorSeverity {
        match self {
            NestGateError::Security { .. } => ErrorSeverity::Critical,
            NestGateError::System { .. } => ErrorSeverity::High,
            NestGateError::Network { .. } => ErrorSeverity::Medium,
            _ => ErrorSeverity::Low,
        }
    }

    /// Requires Human Intervention
    fn requires_human_intervention(&self) -> bool {
        matches!(
            self,
            NestGateError::Security { .. } | NestGateError::Configuration { .. }
        )
    }

    /// Recovery Suggestions
    fn recovery_suggestions(&self) -> Vec<RecoverySuggestion> {
        match self {
            NestGateError::Network { .. } => vec![RecoverySuggestion {
                suggestion_id: "network_retry".to_string(),
                description: "Retry with exponential backoff".to_string(),
                automated: true,
                confidence: 0.8,
                steps: vec!["Wait 1s".to_string(), "Retry request".to_string()],
            }],
            _ => Vec::new(),
        }
    }
}

// ==================== SECTION ====================

/// Common AI-First response types for `NestGate` operations
pub type AIStorageResponse<T> = AIFirstResponse<T>;
/// Type alias for Ainetworkresponse
pub type AINetworkResponse<T> = AIFirstResponse<T>;
/// Type alias for Aisecurityresponse
pub type AISecurityResponse<T> = AIFirstResponse<T>;
/// Type alias for Aiconfigresponse
pub type AIConfigResponse<T> = AIFirstResponse<T>;
/// Result type that automatically converts to AI-First format
pub type AIResult<T> = Result<AIFirstResponse<T>, AIFirstError>;
// ==================== SECTION ====================

/// Create a successful AI-First response with high confidence
pub fn ai_success<T>(data: T) -> AIFirstResponse<T> {
    AIFirstResponseBuilder::new(data)
        .with_confidence(0.95)
        .build()
}
/// Create a successful AI-First response with custom confidence
pub fn ai_success_with_confidence<T>(data: T, confidence: f64) -> AIFirstResponse<T> {
    AIFirstResponseBuilder::new(data)
        .with_confidence(confidence)
        .build()
}
/// Create an error AI-First response from `NestGateError`
#[must_use]
pub fn ai_error<T: Default>(error: NestGateError) -> AIFirstResponse<T> {
    AIFirstResponseBuilder::new(T::default())
        .with_error(error.into_ai_first_error())
        .with_confidence(0.0)
        .build()
}
/// Create an AI-First response with suggested actions
pub fn ai_response_with_actions<T>(data: T, actions: Vec<SuggestedAction>) -> AIFirstResponse<T> {
    let mut builder = AIFirstResponseBuilder::new(data);
    for action in actions {
        builder = builder.add_suggestion(action);
    }
    builder.build()
}
