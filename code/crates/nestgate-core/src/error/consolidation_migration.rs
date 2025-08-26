use crate::NestGateError;
use std::collections::HashMap;
// **ERROR CONSOLIDATION MIGRATION UTILITY**
//
// This module provides systematic migration of fragmented error types to the unified
// NestGateError system, eliminating duplicate error handling patterns across the codebase.
//
// **CONSOLIDATES**:
// - 30+ fragmented error enums across all crates
// - Multiple ErrorSeverity and ErrorCategory duplicates
// - Scattered domain-specific error types
// - Inconsistent error handling patterns
//
// **PROVIDES**:
// - Automated migration utilities
// - Error mapping and conversion functions
// - Consolidation statistics and reporting
// - Backward compatibility during migration

use crate::error::{NestGateError, ErrorContext};
use crate::error::domain_errors::*;
use crate::error::unified_error_consolidation::{ConsolidatedOperationError, ErrorCategory};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// **ERROR CONSOLIDATION MANAGER**
/// Handles systematic migration of fragmented error types to unified NestGateError
#[derive(Debug)]
pub struct ErrorConsolidationManager {
    /// Migration statistics
    pub stats: ConsolidationStats,
    /// Migration warnings and issues
    pub warnings: Vec<ConsolidationWarning>,
    /// Error type mappings
    pub mappings: HashMap<String, ErrorMapping>,
}

/// Consolidation statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConsolidationStats {
    /// Total error types found
    pub total_error_types: u32,
    /// Error types successfully consolidated
    pub consolidated_count: u32,
    /// Error types requiring manual migration
    pub manual_migration_count: u32,
    /// Consolidation progress percentage
    pub consolidation_progress: f64,
    /// Domain-specific consolidation counts
    pub domain_counts: HashMap<String, u32>,
}

/// Consolidation warning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidationWarning {
    /// Warning category
    pub category: ConsolidationWarningCategory,
    /// Warning message
    pub message: String,
    /// Source error type
    pub source_error_type: String,
    /// Suggested action
    pub suggested_action: String,
}

/// Consolidation warning categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsolidationWarningCategory {
    /// Error type has unique fields that may be lost
    UniqueFieldsLoss,
    /// Error type requires manual mapping
    ManualMappingRequired,
    /// Error type has complex conversion logic
    ComplexConversion,
    /// Error type is used in public API
    PublicApiBreaking,
    /// Error type has custom Display implementation
    CustomDisplayLoss,
}

impl std::fmt::Display for ConsolidationWarningCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConsolidationWarningCategory::UniqueFieldsLoss => write!(f, "UniqueFieldsLoss"),
            ConsolidationWarningCategory::ManualMappingRequired => write!(f, "ManualMappingRequired"),
            ConsolidationWarningCategory::ComplexConversion => write!(f, "ComplexConversion"),
            ConsolidationWarningCategory::PublicApiBreaking => write!(f, "PublicApiBreaking"),
            ConsolidationWarningCategory::CustomDisplayLoss => write!(f, "CustomDisplayLoss"),
        }
    }
}

/// Error type mapping configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorMapping {
    /// Source error type name
    pub source_type: String,
    /// Target NestGateError variant
    pub target_variant: String,
    /// Domain for the error
    pub domain: String,
    /// Category for consolidated errors
    pub category: ErrorCategory,
    /// Whether automatic migration is possible
    pub automatic_migration: bool,
    /// Custom conversion function name (if needed)
    pub custom_converter: Option<String>,
}

impl ErrorConsolidationManager {
    /// Create new consolidation manager
    pub fn new() -> Self {
        let mut manager = Self {
            stats: ConsolidationStats::default(),
            warnings: Vec::new(),
            mappings: HashMap::new(),
        };
        
        manager.initialize_error_mappings();
        manager
    }

    /// Initialize standard error type mappings
    fn initialize_error_mappings(&mut self) {
        // ZFS-related errors
        self.add_mapping("ZfsError", "Zfs", "zfs", ErrorCategory::Storage, true, None);
        self.add_mapping("UniversalZfsError", "Zfs", "zfs", ErrorCategory::Storage, true, None);
        self.add_mapping("PoolSetupError", "Zfs", "zfs", ErrorCategory::Storage, true, Some("convert_pool_setup_error"));
        
        // Network-related errors
        self.add_mapping("NetworkError", "Network", "network", ErrorCategory::Network, true, None);
        self.add_mapping("ConnectionError", "Network", "network", ErrorCategory::Network, true, Some("convert_connection_error"));
        self.add_mapping("RpcError", "Network", "network", ErrorCategory::Network, true, Some("convert_rpc_error"));
        
        // API-related errors
        self.add_mapping("ApiError", "Api", "api", ErrorCategory::External, true, None);
        self.add_mapping("EcosystemError", "Api", "ecosystem", ErrorCategory::External, true, Some("convert_ecosystem_error"));
        self.add_mapping("PrimalError", "Api", "primal", ErrorCategory::External, true, Some("convert_primal_error"));
        
        // Security-related errors
        self.add_mapping("SecurityError", "Security", "security", ErrorCategory::Security, true, None);
        self.add_mapping("InputValidationError", "Security", "validation", ErrorCategory::Security, true, Some("convert_validation_error"));
        self.add_mapping("RateLimitError", "Security", "rate_limit", ErrorCategory::Security, true, Some("convert_rate_limit_error"));
        
        // System-related errors
        self.add_mapping("AutomationError", "Automation", "automation", ErrorCategory::System, true, None);
        self.add_mapping("InstallerError", "Internal", "installer", ErrorCategory::System, true, Some("convert_installer_error"));
        self.add_mapping("FsMonitorError", "Internal", "fsmonitor", ErrorCategory::System, true, Some("convert_fsmonitor_error"));
        
        // MCP-related errors
        self.add_mapping("McpError", "Mcp", "mcp", ErrorCategory::External, true, None);
        self.add_mapping("McpProtocolError", "Mcp", "mcp", ErrorCategory::External, true, Some("convert_mcp_protocol_error"));
        
        // Testing-related errors (keep separate for test infrastructure)
        self.add_mapping("TestStorageError", "Testing", "test_storage", ErrorCategory::Internal, false, Some("convert_test_storage_error"));
        self.add_mapping("ServiceTestError", "Testing", "test_service", ErrorCategory::Internal, false, Some("convert_service_test_error"));
        
        // Update statistics
        self.stats.total_error_types = self.mappings.len() as u32;
    }

    /// Add error mapping
    fn add_mapping(
        &mut self,
        source_type: &str,
        target_variant: &str,
        domain: &str,
        category: ErrorCategory,
        automatic: bool,
        converter: Option<&str>,
    ) {
        self.mappings.insert(
            source_type.to_string(),
            ErrorMapping {
                source_type: source_type.to_string(),
                target_variant: target_variant.to_string(),
                domain: domain.to_string(),
                category,
                automatic_migration: automatic,
                custom_converter: converter.map(|s| s.to_string()),
            },
        );
        
        // Update domain counts
        *self.stats.domain_counts.entry(domain.to_string()).or_insert(0) += 1;
    }

    /// **MIGRATE ZFS ERROR**
    /// Convert ZfsError variants to unified NestGateError::Zfs
    pub fn migrate_zfs_error(&mut self, zfs_error: &ZfsErrorInfo) -> NestGateError {
        self.stats.consolidated_count += 1;
        
        let zfs_data = ZfsErrorData {
            message: format!("ZFS operation '{}' failed", zfs_error.operation),
            pool: zfs_error.pool_name.clone(),
            dataset: zfs_error.dataset_name.clone(),
            snapshot: None,
            command: Some(zfs_error.operation.clone()),
            recovery_suggestions: zfs_error.recovery_suggestion.map(|s| vec![s]).unwrap_or_default(),
        };
        
        NestGateError::Zfs(Box::new(zfs_data))
    }

    /// **MIGRATE API ERROR**
    /// Convert ApiError variants to unified NestGateError::Api
    pub fn migrate_api_error(&mut self, api_error: &ApiErrorInfo) -> NestGateError {
        self.stats.consolidated_count += 1;
        
        let api_data = ApiErrorData {
            message: format!("{} {} failed: {}", api_error.method, api_error.endpoint, api_error.error_type),
            path: api_error.endpoint.clone(),
        };
        
        NestGateError::Api(Box::new(api_data))
    }

    /// **MIGRATE NETWORK ERROR**
    /// Convert NetworkError variants to unified NestGateError::Network
    pub fn migrate_network_error(&mut self, network_error: &NetworkErrorInfo) -> NestGateError {
        self.stats.consolidated_count += 1;
        
        let network_data = NetworkErrorData {
            message: format!("Network operation '{}' failed: {}", network_error.operation, network_error.error_type),
            operation: network_error.operation.clone(),
        };
        
        NestGateError::Network(Box::new(network_data))
    }

    /// **MIGRATE SECURITY ERROR**
    /// Convert SecurityError variants to unified NestGateError::Security
    pub fn migrate_security_error(&mut self, security_error: &SecurityErrorInfo) -> NestGateError {
        self.stats.consolidated_count += 1;
        
        let security_data = SecurityErrorData {
            message: format!("Security operation '{}' failed in domain '{}'", security_error.operation, security_error.security_domain),
            principal: security_error.user_id.clone(),
        };
        
        NestGateError::Security(Box::new(security_data))
    }

    /// **GENERIC ERROR CONSOLIDATION**
    /// Convert any error to ConsolidatedOperationError for gradual migration
    pub fn consolidate_generic_error(
        &mut self,
        error_type: &str,
        operation: &str,
        message: &str,
        domain: &str,
    ) -> NestGateError {
        self.stats.consolidated_count += 1;
        
        let mapping = self.mappings.get(error_type);
        let category = mapping.map(|m| m.category.clone()).unwrap_or(ErrorCategory::Unknown);
        
        let consolidated_error = ConsolidatedOperationError {
            domain: domain.to_string(),
            operation: operation.to_string(),
            message: message.to_string(),
            category,
            context: HashMap::new(),
            retryable: false,
            recovery_suggestions: vec![
                "Check system logs for more details".to_string(),
                "Retry the operation".to_string(),
            ],
        };
        
        // For now, map to Internal error with consolidated data
        NestGateError::Internal {
            location: format!("{}::{}", domain, operation),
            is_bug: false,
        }
    }

    /// Add consolidation warning
    pub fn add_warning(
        &mut self,
        category: ConsolidationWarningCategory,
        message: String,
        source_error_type: String,
        suggested_action: String,
    ) {
        self.warnings.push(ConsolidationWarning {
            category,
            message,
            source_error_type,
            suggested_action,
        });
    }

    /// Get consolidation summary
    pub fn get_summary(&self) -> ConsolidationSummary {
        let progress = if self.stats.total_error_types > 0 {
            (self.stats.consolidated_count as f64 / self.stats.total_error_types as f64) * 100.0
        } else {
            100.0
        };
        
        ConsolidationSummary {
            stats: ConsolidationStats {
                consolidation_progress: progress,
                ..self.stats.clone()
            },
            warnings_count: self.warnings.len(),
            automatic_migrations: self.mappings.values().filter(|m| m.automatic_migration).count(),
            manual_migrations: self.mappings.values().filter(|m| !m.automatic_migration).count(),
        }
    }
}

/// Consolidation summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidationSummary {
    /// Consolidation statistics
    pub stats: ConsolidationStats,
    /// Number of warnings generated
    pub warnings_count: usize,
    /// Number of automatic migrations available
    pub automatic_migrations: usize,
    /// Number of manual migrations required
    pub manual_migrations: usize,
}

// ==================== ERROR INFO STRUCTURES FOR MIGRATION ====================

/// ZFS error information for migration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsErrorInfo {
    pub operation: String,
    pub pool_name: Option<String>,
    pub dataset_name: Option<String>,
    pub error_code: Option<String>,
    pub system_error: Option<String>,
    pub recovery_suggestion: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
}

/// API error information for migration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorInfo {
    pub endpoint: String,
    pub method: String,
    pub status_code: u16,
    pub error_type: String,
    pub user_message: Option<String>,
    pub debug_info: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
}

/// Network error information for migration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkErrorInfo {
    pub operation: String,
    pub endpoint: String,
    pub error_type: String,
    pub timeout_duration: Option<std::time::Duration>,
    pub retry_count: Option<u32>,
    pub last_attempt: Option<SystemTime>,
    pub metadata: Option<HashMap<String, String>>,
}

/// Security error information for migration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityErrorInfo {
    pub security_domain: String,
    pub operation: String,
    pub user_id: Option<String>,
    pub resource: Option<String>,
    pub permission_required: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
}

impl Default for ErrorConsolidationManager {
    fn default() -> Self {
        Self::new()
    }
}

/// **MIGRATION CONVENIENCE MACROS**
/// Macros to help with systematic error migration

/// Migrate a ZFS error with context
#[macro_export]
macro_rules! migrate_zfs_error {
    ($manager:expr, $operation:expr, $pool:expr, $error:expr) => {
        $manager.migrate_zfs_error(&ZfsErrorInfo {
            operation: $operation.to_string(),
            pool_name: Some($pool.to_string()),
            dataset_name: None,
            error_code: Some($error.to_string()),
            system_error: None,
            recovery_suggestion: Some("Check ZFS pool status".to_string()),
            metadata: None,
        })
    };
}

/// Migrate an API error with context
#[macro_export]
macro_rules! migrate_api_error {
    ($manager:expr, $method:expr, $endpoint:expr, $status:expr, $message:expr) => {
        $manager.migrate_api_error(&ApiErrorInfo {
            endpoint: $endpoint.to_string(),
            method: $method.to_string(),
            status_code: $status,
            error_type: "request_error".to_string(),
            user_message: Some($message.to_string()),
            debug_info: None,
            metadata: None,
        })
    };
}

/// Migrate a network error with context
#[macro_export]
macro_rules! migrate_network_error {
    ($manager:expr, $operation:expr, $endpoint:expr, $error_type:expr) => {
        $manager.migrate_network_error(&NetworkErrorInfo {
            operation: $operation.to_string(),
            endpoint: $endpoint.to_string(),
            error_type: $error_type.to_string(),
            timeout_duration: None,
            retry_count: None,
            last_attempt: Some(std::time::SystemTime::now()),
            metadata: None,
        })
    };
} 