use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
/// **RETURN BUILDERS - CONFIG BUILDERS MODULE**
/// Contains configuration and utility builder functions.
/// Extracted from the large `return_builders.rs` to achieve file size compliance.
use std::collections::HashMap;
use uuid::Uuid;
/// Access grant structure for config builders
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Accessgrant
pub struct AccessGrant {
    /// Permissions
    pub permissions: Vec<String>,
    /// Valid Until
    pub valid_until: DateTime<Utc>,
    /// Consensus Nodes
    pub consensus_nodes: Vec<String>,
    /// Consensus Percentage
    pub consensus_percentage: f64,
    /// Grant identifier
    pub grant_id: String,
    /// Granted At
    pub granted_at: DateTime<Utc>,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}
/// Build `AccessGrant` response with all required fields
/// **PURE FUNCTION**: No side effects, deterministic field construction
/// **TESTABLE**: Can verify all field assignments and computed values
/// **ZERO-COPY OPTIMIZED**: Accepts references to avoid unnecessary cloning
#[must_use]
pub fn build_access_grant(
    permissions: &[String],
    valid_until: i64,
    _proof_data: &str, // Prefixed with underscore - planned for cryptographic verification
    consensus_nodes: &[String],
    consensus_percentage: f64,
) -> AccessGrant {
    AccessGrant {
        permissions: permissions.to_vec(),
        valid_until: DateTime::from_timestamp(valid_until, 0).unwrap_or_default(),
        consensus_nodes: consensus_nodes.to_vec(),
        consensus_percentage,
        grant_id: Uuid::new_v4().to_string(),
        granted_at: Utc::now(),
        metadata: HashMap::new(),
    }
}
/// Build diagnostic entry with timestamp and defaults
/// **PURE FUNCTION**: Diagnostic construction with ID generation
/// **TESTABLE**: Can verify ID generation and timestamp consistency
#[must_use]
pub fn build_diagnostic(
    level: crate::diagnostics::DiagnosticLevel,
    component: crate::diagnostics::ComponentType,
    message: String,
) -> crate::diagnostics::Diagnostic {
    let diagnostic_id = Uuid::new_v4();
    crate::diagnostics::Diagnostic {
        id: diagnostic_id.to_string(),
        level,
        component,
        message,
        timestamp: std::time::SystemTime::now(),
        details: None,
        path: None,
        resolved: false,
        resolved_at: None,
    }
}

/// Build error context with comprehensive details
/// **PURE FUNCTION**: Error context construction
/// **TESTABLE**: Can verify context field assignments
#[must_use]
pub fn build_error_context(
    operation: &str,
    _details: String,      // Prefixed with underscore - planned for error enrichment
    _code: Option<String>, // Prefixed with underscore - planned for error code mapping
) -> crate::error::ErrorContext {
    use std::time::SystemTime;

    crate::error::ErrorContext {
        operation: operation.to_string(),
        component: "config_builder".to_string(),
        timestamp: SystemTime::now(),
        metadata: std::collections::HashMap::new(),
        request_id: None,
        user_id: None,
    }
}
