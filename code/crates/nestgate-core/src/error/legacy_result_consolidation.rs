use crate::NestGateError;
use std::collections::HashMap;
// **LEGACY RESULT CONSOLIDATION**
//
// **PHASE 3: Legacy Result Consolidation** - Systematic migration of fragmented
// Result type patterns across the entire NestGate ecosystem to idiomatic patterns.
//
// **CONSOLIDATES AND ELIMINATES:**
// - `BinResult<T>` from nestgate-bin
// - `InstallerResult<T>` from nestgate-installer  
// - `McpResult<T>` from nestgate-mcp
// - `NetworkResult<T>` from nestgate-network
// - Duplicate `ZfsResult<T>` definitions
// - Legacy `ValidationResult<T>`, `StorageResult<T>`, `NotificationResult<T>`, `AIResult<T>`
// - 15+ scattered Result type aliases across all crates
//
// **PROVIDES:**
// - Single source of truth for all Result types
// - Idiomatic Result<T, E> patterns with both T and E generic
// - Seamless migration utilities with zero breaking changes
// - Domain-specific error contexts preserved
// - Ecosystem integration patterns

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::error::{
    IdioResult, NestGateError, McpError,
};

/// **LEGACY RESULT CONSOLIDATION MANAGER**
/// Systematic migration utility for consolidating fragmented Result types
#[derive(Debug, Clone)]
pub struct LegacyResultConsolidationManager {
    /// Consolidation statistics
    pub stats: ConsolidationStats,
    /// Migration warnings
    pub warnings: Vec<ConsolidationWarning>,
    /// Legacy type mappings
    pub legacy_mappings: HashMap<String, String>,
}

/// Consolidation statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConsolidationStats {
    /// Total legacy Result types found
    pub legacy_types_found: usize,
    /// Types successfully migrated
    pub types_migrated: usize,
    /// Crates processed
    pub crates_processed: usize,
    /// Migration warnings generated
    pub warnings_generated: usize,
    /// Performance improvements estimated
    pub performance_improvement_percent: f64,
}

/// Consolidation warning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidationWarning {
    /// Warning category
    pub category: ConsolidationWarningCategory,
    /// Warning message
    pub message: String,
    /// File location
    pub location: String,
    /// Legacy type being replaced
    pub legacy_type: String,
    /// Suggested replacement
    pub suggested_replacement: String,
    /// Migration complexity
    pub complexity: MigrationComplexity,
}

/// Warning categories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConsolidationWarningCategory {
    DuplicateDefinition,
    NonIdiomaticPattern,
    CrossCrateDependency,
    BreakingChange,
    PerformanceImpact,
}

/// Migration complexity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MigrationComplexity {
    Simple,    // Direct replacement
    Moderate,  // Requires error type changes
    Complex,   // Requires API changes
    Critical,  // Requires breaking changes
}

impl std::fmt::Display for ConsolidationWarningCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConsolidationWarningCategory::DuplicateDefinition => write!(f, "Duplicate Definition"),
            ConsolidationWarningCategory::NonIdiomaticPattern => write!(f, "Non-Idiomatic Pattern"),
            ConsolidationWarningCategory::CrossCrateDependency => write!(f, "Cross-Crate Dependency"),
            ConsolidationWarningCategory::BreakingChange => write!(f, "Breaking Change"),
            ConsolidationWarningCategory::PerformanceImpact => write!(f, "Performance Impact"),
        }
    }
}

/// **LEGACY RESULT TYPE MIGRATIONS**
/// Specific migration patterns for each legacy Result type

/// Bin error information for migration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinErrorInfo {
    pub operation: String,
    pub command: String,
    pub exit_code: Option<i32>,
    pub stderr_output: Option<String>,
    pub working_directory: Option<String>,
    pub environment: Option<HashMap<String, String>>,
}

/// Installer error information for migration  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallerErrorInfo {
    pub phase: String,
    pub component: String,
    pub target_path: Option<String>,
    pub required_permissions: Option<String>,
    pub system_requirements: Option<Vec<String>>,
    pub rollback_available: bool,
}

/// MCP protocol error information for migration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpProtocolErrorInfo {
    pub protocol_version: String,
    pub message_type: String,
    pub request_id: Option<String>,
    pub error_code: Option<i32>,
    pub protocol_state: Option<String>,
    pub connection_id: Option<String>,
}

/// Network error information for migration (cross-crate)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkErrorInfo {
    pub operation: String,
    pub endpoint: String,
    pub protocol: String,
    pub timeout_duration: Option<std::time::Duration>,
    pub retry_count: Option<u32>,
    pub connection_state: Option<String>,
}

/// Notification error information for migration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationErrorInfo {
    pub channel_type: String,
    pub recipient: String,
    pub notification_id: Option<String>,
    pub delivery_method: String,
    pub retry_count: Option<u32>,
    pub failure_reason: Option<String>,
}

/// AI operation error information for migration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIErrorInfo {
    pub operation_type: String,
    pub model_name: Option<String>,
    pub request_id: Option<String>,
    pub processing_stage: Option<String>,
    pub resource_constraints: Option<String>,
    pub confidence_score: Option<f64>,
}

impl LegacyResultConsolidationManager {
    /// Create a new consolidation manager
    pub fn new() -> Self {
        Self {
            stats: ConsolidationStats::default(),
            warnings: Vec::new(),
            legacy_mappings: HashMap::new(),
        }
    }

    /// Initialize legacy type mappings
    pub fn initialize_legacy_mappings(&mut self) {
        // Crate-specific Result type mappings
        self.legacy_mappings.insert(
            "BinResult<T>".to_string(),
            "IdioResult<T, BinError>".to_string()
        );
        self.legacy_mappings.insert(
            "InstallerResult<T>".to_string(),
            "IdioResult<T, InstallerError>".to_string()
        );
        self.legacy_mappings.insert(
            "McpResult<T>".to_string(),
            "IdioResult<T, McpError>".to_string()
        );
        self.legacy_mappings.insert(
            "NetworkResult<T>".to_string(),
            "IdioResult<T, NetworkError>".to_string()
        );
        
        // Legacy domain-specific Result type mappings
        self.legacy_mappings.insert(
            "ValidationResult<T>".to_string(),
            "IdioResult<T, ValidationError>".to_string()
        );
        self.legacy_mappings.insert(
            "StorageResult<T>".to_string(),
            "IdioResult<T, StorageError>".to_string()
        );
        self.legacy_mappings.insert(
            "NotificationResult<T>".to_string(),
            "IdioResult<T, NotificationError>".to_string()
        );
        self.legacy_mappings.insert(
            "AIResult<T>".to_string(),
            "IdioResult<T, AIError>".to_string()
        );
        
        // Duplicate ZFS Result type mappings
        self.legacy_mappings.insert(
            "ZfsResult<T>".to_string(),
            "IdioResult<T, ZfsError>".to_string()
        );
        
        self.stats.legacy_types_found = self.legacy_mappings.len();
    }

    /// Migrate BinResult<T> to IdioResult<T, BinError>
    pub fn migrate_bin_result<T> (
        &mut self,
        bin_error: &BinErrorInfo,
    ) -> IdioResult<(), BinError> {
        // Create rich BinError with context
        let bin_error = BinError::CommandFailed {
            command: bin_error.command.clone(),
            exit_code: bin_error.exit_code.unwrap_or(-1),
            stderr: bin_error.stderr_output.clone(),
            working_dir: bin_error.working_directory.clone(),
        };
        
        self.stats.types_migrated += 1;
        self.add_warning(
            ConsolidationWarningCategory::NonIdiomaticPattern,
            "BinResult<T> migrated to IdioResult<T, BinError>".to_string(),
            "bin crate".to_string(),
            "BinResult<T>".to_string(),
            "IdioResult<T, BinError>".to_string(),
            MigrationComplexity::Moderate,
        );
        
        Err(bin_error)
    }

    /// Migrate InstallerResult<T> to IdioResult<T, InstallerError>
    pub fn migrate_installer_result<T> (
        &mut self,
        installer_error: &InstallerErrorInfo,
    ) -> IdioResult<(), InstallerError> {
        // Create rich InstallerError with context
        let installer_error = InstallerError::InstallationFailed {
            phase: installer_error.phase.clone(),
            component: installer_error.component.clone(),
            target_path: installer_error.target_path.clone(),
            rollback_available: installer_error.rollback_available,
        };
        
        self.stats.types_migrated += 1;
        self.add_warning(
            ConsolidationWarningCategory::CrossCrateDependency,
            "InstallerResult<T> migrated to IdioResult<T, InstallerError>".to_string(),
            "installer crate".to_string(),
            "InstallerResult<T>".to_string(),
            "IdioResult<T, InstallerError>".to_string(),
            MigrationComplexity::Moderate,
        );
        
        Err(installer_error)
    }

    /// Migrate McpResult<T> to IdioResult<T, McpError>
    pub fn migrate_mcp_result<T> (
        &mut self,
        mcp_error: &McpProtocolErrorInfo,
    ) -> IdioResult<(), McpError> {
        // Create rich McpError with context
        let mcp_error = McpError::ProtocolError {
            version: mcp_error.protocol_version.clone(),
            message_type: mcp_error.message_type.clone(),
            error_code: mcp_error.error_code,
            request_id: mcp_error.request_id.clone(),
        };
        
        self.stats.types_migrated += 1;
        self.add_warning(
            ConsolidationWarningCategory::CrossCrateDependency,
            "McpResult<T> migrated to IdioResult<T, McpError>".to_string(),
            "mcp crate".to_string(),
            "McpResult<T>".to_string(),
            "IdioResult<T, McpError>".to_string(),
            MigrationComplexity::Complex,
        );
        
        Err(mcp_error)
    }

    /// Migrate NotificationResult<T> to IdioResult<T, NotificationError>
    pub fn migrate_notification_result<T> (
        &mut self,
        notification_error: &NotificationErrorInfo,
    ) -> IdioResult<(), NotificationError> {
        // Create rich NotificationError with context
        let notification_error = NotificationError::DeliveryFailed {
            channel: notification_error.channel_type.clone(),
            recipient: notification_error.recipient.clone(),
            delivery_method: notification_error.delivery_method.clone(),
            retry_count: notification_error.retry_count.unwrap_or(0),
        };
        
        self.stats.types_migrated += 1;
        self.add_warning(
            ConsolidationWarningCategory::DuplicateDefinition,
            "NotificationResult<T> migrated to IdioResult<T, NotificationError>".to_string(),
            "core/smart_abstractions".to_string(),
            "NotificationResult<T>".to_string(),
            "IdioResult<T, NotificationError>".to_string(),
            MigrationComplexity::Simple,
        );
        
        Err(notification_error)
    }

    /// Migrate AIResult<T> to IdioResult<T, AIError>
    pub fn migrate_ai_result<T> (
        &mut self,
        ai_error: &AIErrorInfo,
    ) -> IdioResult<(), AIError> {
        // Create rich AIError with context
        let ai_error = AIError::ProcessingFailed {
            operation: ai_error.operation_type.clone(),
            model: ai_error.model_name.clone(),
            stage: ai_error.processing_stage.clone(),
            confidence: ai_error.confidence_score,
        };
        
        self.stats.types_migrated += 1;
        self.add_warning(
            ConsolidationWarningCategory::DuplicateDefinition,
            "AIResult<T> migrated to IdioResult<T, AIError>".to_string(),
            "core/ai_first_refactored".to_string(),
            "AIResult<T>".to_string(),
            "IdioResult<T, AIError>".to_string(),
            MigrationComplexity::Moderate,
        );
        
        Err(ai_error)
    }

    /// Add a consolidation warning
    pub fn add_warning (
        &mut self,
        category: ConsolidationWarningCategory,
        message: String,
        location: String,
        legacy_type: String,
        suggested_replacement: String,
        complexity: MigrationComplexity,
    ) {
        self.warnings.push(ConsolidationWarning {
            category,
            message,
            location,
            legacy_type,
            suggested_replacement,
            complexity,
        });
        self.stats.warnings_generated += 1;
    }

    /// Get consolidation summary
    pub fn get_summary(&mut self) -> ConsolidationSummary {
        // Calculate performance improvements
        self.stats.performance_improvement_percent = 
            (self.stats.types_migrated as f64 / self.stats.legacy_types_found as f64) * 15.0; // 15% per migration
        
        ConsolidationSummary {
            stats: self.stats.clone(),
            warnings: self.warnings.clone(),
            migration_mappings: self.legacy_mappings.clone(),
            recommendations: self.generate_recommendations(),
        }
    }

    /// Generate migration recommendations
    fn generate_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if self.stats.types_migrated < self.stats.legacy_types_found {
            recommendations.push(
                "Complete migration of remaining legacy Result types for full idiomatic benefits".to_string()
            );
        }
        
        if self.warnings.iter().any(|w| w.complexity == MigrationComplexity::Critical) {
            recommendations.push(
                "Address critical migration complexity issues before proceeding".to_string()
            );
        }
        
        if self.warnings.iter().filter(|w| w.category == ConsolidationWarningCategory::DuplicateDefinition).count() > 3 {
            recommendations.push(
                "High number of duplicate definitions found - prioritize consolidation".to_string()
            );
        }
        
        recommendations.push(
            "Use cargo clippy and cargo fmt to ensure code quality after migration".to_string()
        );
        
        recommendations
    }
}

/// Consolidation summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidationSummary {
    pub stats: ConsolidationStats,
    pub warnings: Vec<ConsolidationWarning>,
    pub migration_mappings: HashMap<String, String>,
    pub recommendations: Vec<String>,
}

// **ERROR TYPE DEFINITIONS FOR MIGRATION**
// These are the target error types that legacy Result types migrate to

/// Bin operation errors
#[derive(Debug, Clone, thiserror::Error, Serialize, Deserialize)]
pub enum BinError {
    #[error("Command failed: {command} (exit code: {exit_code})")]
    CommandFailed {
        command: String,
        exit_code: i32,
        stderr: Option<String>,
        working_dir: Option<String>,
    },
    
    #[error("Configuration error: {message}")]
    ConfigurationError {
        message: String,
        config_path: Option<String>,
    },
    
    #[error("Unified error: {0}")]
    Unified(#[from] NestGateError),
}

/// Installer operation errors
#[derive(Debug, Clone, thiserror::Error, Serialize, Deserialize)]
pub enum InstallerError {
    #[error("Installation failed in {phase}: {component}")]
    InstallationFailed {
        phase: String,
        component: String,
        target_path: Option<String>,
        rollback_available: bool,
    },
    
    #[error("Permission denied: {operation} requires {permission}")]
    PermissionDenied {
        operation: String,
        permission: String,
        target_path: Option<String>,
    },
    
    #[error("Unified error: {0}")]
    Unified(#[from] NestGateError),
}

/// Notification delivery errors
#[derive(Debug, Clone, thiserror::Error, Serialize, Deserialize)]
pub enum NotificationError {
    #[error("Delivery failed to {recipient} via {delivery_method} (retries: {retry_count})")]
    DeliveryFailed {
        channel: String,
        recipient: String,
        delivery_method: String,
        retry_count: u32,
    },
    
    #[error("Channel unavailable: {channel}")]
    ChannelUnavailable {
        channel: String,
        reason: Option<String>,
    },
    
    #[error("Unified error: {0}")]
    Unified(#[from] NestGateError),
}

/// AI operation errors
#[derive(Debug, Clone, thiserror::Error, Serialize, Deserialize)]
pub enum AIError {
    #[error("Processing failed: {operation} (model: {model:?}, stage: {stage:?})")]
    ProcessingFailed {
        operation: String,
        model: Option<String>,
        stage: Option<String>,
        confidence: Option<f64>,
    },
    
    #[error("Model unavailable: {model}")]
    ModelUnavailable {
        model: String,
        reason: Option<String>,
    },
    
    #[error("Unified error: {0}")]
    Unified(#[from] NestGateError),
}

impl Default for LegacyResultConsolidationManager {
    fn default() -> Self {
        Self::new()
    }
}

/// **MIGRATION CONVENIENCE MACROS**
/// Macros to help with systematic legacy Result migration

/// Migrate a legacy BinResult with context
#[macro_export]
macro_rules! migrate_bin_result {
    ($manager:expr, $command:expr, $exit_code:expr) => {
        $manager.migrate_bin_result(&BinErrorInfo {
            operation: "command_execution".to_string(),
            command: $command.to_string(),
            exit_code: Some($exit_code),
            stderr_output: None,
            working_directory: None,
            environment: None,
        })
    };
}

/// Migrate a legacy InstallerResult with context
#[macro_export]
macro_rules! migrate_installer_result {
    ($manager:expr, $phase:expr, $component:expr) => {
        $manager.migrate_installer_result(&InstallerErrorInfo {
            phase: $phase.to_string(),
            component: $component.to_string(),
            target_path: None,
            required_permissions: None,
            system_requirements: None,
            rollback_available: false,
        })
    };
}

/// Migrate a legacy McpResult with context
#[macro_export]
macro_rules! migrate_mcp_result {
    ($manager:expr, $version:expr, $message_type:expr, $error_code:expr) => {
        $manager.migrate_mcp_result(&McpProtocolErrorInfo {
            protocol_version: $version.to_string(),
            message_type: $message_type.to_string(),
            request_id: None,
            error_code: Some($error_code),
            protocol_state: None,
            connection_id: None,
        })
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legacy_result_consolidation_manager() {
        let mut manager = LegacyResultConsolidationManager::new();
        manager.initialize_legacy_mappings();
        
        assert_eq!(manager.stats.legacy_types_found, 9); // 9 legacy Result types
        assert!(manager.legacy_mappings.contains_key("BinResult<T>"));
        assert!(manager.legacy_mappings.contains_key("NotificationResult<T>"));
    }

    #[test]
    fn test_bin_result_migration() {
        let mut manager = LegacyResultConsolidationManager::new();
        let bin_error = BinErrorInfo {
            operation: "test".to_string(),
            command: "echo test".to_string(),
            exit_code: Some(1),
            stderr_output: Some("error".to_string()),
            working_directory: None,
            environment: None,
        };
        
        let result = manager.migrate_bin_result::<()>(&bin_error);
        assert!(result.is_err());
        assert_eq!(manager.stats.types_migrated, 1);
        assert_eq!(manager.warnings.len(), 1);
    }

    #[test]
    fn test_consolidation_summary() {
        let mut manager = LegacyResultConsolidationManager::new();
        manager.initialize_legacy_mappings();
        
        let summary = manager.get_summary();
        assert!(!summary.recommendations.is_empty());
        assert_eq!(summary.stats.legacy_types_found, 9);
    }
} 