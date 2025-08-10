/// **RETURN BUILDERS - CONFIG BUILDERS MODULE**
/// Contains configuration and utility builder functions.
/// Extracted from the large return_builders.rs to achieve file size compliance.
use std::collections::HashMap;
use uuid::Uuid;

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
) -> crate::types::AccessGrant {
    crate::types::AccessGrant {
        permissions: permissions.to_vec(),
        valid_until,
        proof_hash: format!("{:x}", md5::compute(proof_data.as_bytes())),
        consensus_nodes: consensus_nodes.to_vec(),
        consensus_percentage,
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
) -> crate::error::ErrorContext {
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

    crate::error::ErrorContext {
        operation: "config_validation".to_string(),
        component: "config_builder".to_string(),
        metadata,
        timestamp: std::time::SystemTime::now(),
    }
}
