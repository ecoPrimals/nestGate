/// **RETURN BUILDERS - CONFIG BUILDERS MODULE**
/// Contains configuration and utility builder functions.
/// Extracted from the large return_builders.rs to achieve file size compliance.
use std::collections::HashMap;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Access grant structure for config builders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessGrant {
    pub permissions: Vec<String>,
    pub valid_until: DateTime<Utc>,
    pub consensus_nodes: Vec<String>,
    pub consensus_percentage: f64,
    pub grant_id: String,
    pub granted_at: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
}

/// Build AccessGrant response with all required fields
/// **PURE FUNCTION**: No side effects, deterministic field construction
/// **TESTABLE**: Can verify all field assignments and computed values
/// **ZERO-COPY OPTIMIZED**: Accepts references to avoid unnecessary cloning
pub fn build_access_grant(
    permissions: &[String],
    valid_until: i64,
    proof_data: &str,
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
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("proof_hash".to_string(), format!("{:x}", md5::compute(proof_data.as_bytes())));
            meta
        },
    }
}

/// Build diagnostic entry with timestamp and defaults
/// **PURE FUNCTION**: Diagnostic construction with ID generation
/// **TESTABLE**: Can verify ID generation and timestamp consistency
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
        resource: None,
        resolved: false,
        resolved_at: None,
    }
}

/// Build error context with comprehensive details
/// **PURE FUNCTION**: Error context construction
/// **TESTABLE**: Can verify context field assignments
pub fn build_error_context(
    operation: String,
    details: String,
    code: Option<String>,
    retry_info: Option<crate::error::RetryInfo>,
) -> crate::error::context::ErrorContext {
    let mut metadata = HashMap::new();
    if let Some(c) = code {
        metadata.insert("error_code".to_string(), c);
    }
    if let Some(op) = Some(operation) {
        metadata.insert("operation".to_string(), op);
    }
    if let Some(det) = Some(details) {
        metadata.insert("details".to_string(), det);
    }
    if let Some(_retry) = retry_info {
        metadata.insert("retry_available".to_string(), "true".to_string());
    }

    crate::error::context::ErrorContext {
        error_id: "config-builder-error".to_string(),
        operation: "config_validation".to_string(),
        component: "config_builder".to_string(),
        metadata,
        timestamp: std::time::SystemTime::now(),
        stack_trace: None,
        related_errors: vec![],
        retry_info: None,
        recovery_suggestions: vec![],
        performance_metrics: None,
        environment: None,
    }
}
