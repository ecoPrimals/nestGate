// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

/// 🔍 **VALIDATION PREDICATES MODULE** 🔍
/// Pure boolean predicate functions for validation logic.
/// Extracted from complex validation chains to enable precise testing
/// and catch boolean logic mutations (|| vs &&, ! vs identity).
/// **MUTATION TESTING TARGET**: This module specifically addresses:
/// - `|| with &&` mutations in multi-condition validation
/// - `&& with ||` mutations in security checks
/// - `! with identity` mutations in negation logic
/// - Complex conditional expressions in access control
///
/// Check if an environment name represents production
///
/// **PURE FUNCTION**: No side effects, deterministic output
///
/// **TESTABLE**: Can verify exact string matching with case variations
#[must_use]
pub fn is_production_environment(environment: &str) -> bool {
    environment.to_lowercase() == "production"
}

/// Check if an environment name represents development
///
/// **PURE FUNCTION**: Simple string comparison
///
/// **TESTABLE**: Can verify case insensitive matching
#[must_use]
pub fn is_development_environment(environment: &str) -> bool {
    environment.to_lowercase() == "development"
}

/// Check if an environment name represents test environment
///
/// **PURE FUNCTION**: Simple string comparison with normalization
///
/// **TESTABLE**: Can verify exact matching behavior
#[must_use]
pub fn is_test_environment(environment: &str) -> bool {
    environment.to_lowercase() == "test"
}

/// Validate that a threshold value is within valid percentage range
///
/// **PURE FUNCTION**: Range validation with boundary conditions
///
/// **TESTABLE**: Can verify boundary conditions (0.0, 100.0) precisely
#[must_use]
pub fn is_valid_percentage_threshold(threshold: f64) -> bool {
    (0.0..=100.0).contains(&threshold)
}

/// Check if a consensus threshold is valid for production use
///
/// **PURE FUNCTION**: Range validation for consensus systems
///
/// **TESTABLE**: Can verify security-critical boundary conditions
#[must_use]
pub fn is_valid_consensus_threshold(threshold: f64) -> bool {
    (0.5..=1.0).contains(&threshold)
}

/// Validate that a port number is in the valid range
///
/// **PURE FUNCTION**: Port range validation
///
/// **TESTABLE**: Can verify port boundary conditions
#[must_use]
pub const fn is_valid_port_number(port: u16) -> bool {
    port > 0 // u16 max is 65535, so no need to check upper bound
}

/// Check if a string field is non-empty (common validation pattern)
/// **PURE FUNCTION**: Simple string validation
/// **TESTABLE**: Can verify empty string handling
#[must_use]
pub const fn is_non_empty_string(value: &str) -> bool {
    !value.is_empty()
}

/// Check if a file path is valid and safe
/// **PURE FUNCTION**: Path validation with security checking
/// **TESTABLE**: Can verify path traversal prevention
#[must_use]
pub fn is_valid_file_path(path: &str) -> bool {
    if path.is_empty() {
        return false;
    }
    // Check for path traversal attempts
    if path.contains("..") {
        return false;
    }

    // Check for null bytes
    if path.contains('\0') {
        return false;
    }

    // Check for excessive length
    if path.len() > 4096 {
        return false;
    }

    true
}

/// Validate that a numeric value is positive (common validation pattern)
/// **PURE FUNCTION**: Positivity check with zero exclusion
/// **TESTABLE**: Can verify boundary condition at zero
#[must_use]
pub const fn is_positive_number(value: u64) -> bool {
    value > 0
}

/// Check if TLS configuration has required certificate files
/// **PURE FUNCTION**: File path validation predicate
/// **TESTABLE**: Can verify both conditions must be true (&&)
#[must_use]
pub const fn has_required_tls_files(cert_file: &str, key_file: &str) -> bool {
    is_non_empty_string(cert_file) && is_non_empty_string(key_file)
}

/// Check if monitoring configuration has any notification methods
/// **PURE FUNCTION**: Optional field checking with OR logic
/// **TESTABLE**: Can verify || mutation detection (any method present)
#[must_use]
pub const fn has_notification_methods(has_email: bool, has_slack: bool, has_webhook: bool) -> bool {
    has_email || has_slack || has_webhook
}

/// Validate Prometheus configuration completeness
/// **PURE FUNCTION**: Multi-condition validation with AND logic
/// **TESTABLE**: Can verify all conditions must be true
#[must_use]
pub const fn is_prometheus_config_valid(enabled: bool, _port: u16, path: &str) -> bool {
    if !enabled {
        return true; // Disabled config is always valid
    }
    // Port 0 is valid for Prometheus (OS-assigned port)
    is_non_empty_string(path)
}

/// Check if certificate signature has valid format
/// **PURE FUNCTION**: Signature format validation with multiple prefixes
/// **TESTABLE**: Can verify OR logic for multiple valid prefixes
#[must_use]
pub fn has_valid_signature_format(signature: &str) -> bool {
    if signature.len() < 10 {
        return false;
    }
    signature.starts_with("sec_provider_sig_")
        || signature.starts_with("security_sig_")
        || signature.starts_with("vault_sig_")
        || signature.starts_with("standalone_sig_")
        || signature.starts_with("self_signed_")
}

/// Check if communication is internal based on prefix matching
/// **PURE FUNCTION**: Prefix validation with AND logic
/// **TESTABLE**: Can verify both source and destination must match
#[must_use]
pub fn is_internal_communication(source: &str, destination: &str) -> bool {
    let internal_prefixes = ["nestgate", "primal", "internal"];
    let source_internal = internal_prefixes
        .iter()
        .any(|prefix| source.starts_with(prefix));
    let dest_internal = internal_prefixes
        .iter()
        .any(|prefix| destination.starts_with(prefix));

    source_internal && dest_internal
}

/// Validate alert thresholds are all within valid ranges
/// **PURE FUNCTION**: Multiple threshold validation with AND logic
/// **TESTABLE**: Can verify all thresholds must be valid
#[must_use]
pub fn are_alert_thresholds_valid(
    cpu_threshold: f64,
    memory_threshold: f64,
    disk_threshold: f64,
    error_rate_threshold: f64,
    latency_threshold: f64,
) -> bool {
    is_valid_percentage_threshold(cpu_threshold)
        && is_valid_percentage_threshold(memory_threshold)
        && is_valid_percentage_threshold(disk_threshold)
        && is_valid_percentage_threshold(error_rate_threshold)
        // ✅ MODERN: Use epsilon for non-negative check
        && latency_threshold >= -1e-9
}

/// Check if user has required role for operation
/// **PURE FUNCTION**: Role-based access control predicate
/// **TESTABLE**: Can verify role hierarchy logic
#[must_use]
pub fn has_required_role(user_role: &str, required_role: &str) -> bool {
    // Admin role can access everything
    if user_role == "admin" {
        return true;
    }
    // Otherwise, exact role match required
    user_role == required_role
}

/// Check if user has any of the required permissions
/// **PURE FUNCTION**: Permission checking with OR logic
/// **TESTABLE**: Can verify any permission grants access
#[must_use]
pub fn has_any_required_permission(
    user_permissions: &[String],
    required_permissions: &[String],
) -> bool {
    if required_permissions.is_empty() {
        return true; // No permissions required
    }
    required_permissions.iter().any(|required| {
        user_permissions
            .iter()
            .any(|user_perm| user_perm == required)
    })
}

/// Validate system resource configuration
/// **PURE FUNCTION**: Resource limit validation with multiple conditions
/// **TESTABLE**: Can verify all resource limits are valid
#[must_use]
pub const fn are_system_resources_valid(
    worker_threads: usize,
    request_timeout: u64,
    connection_timeout: u64,
    max_connections: usize,
    db_pool_size: u32,
) -> bool {
    worker_threads > 0
        && request_timeout > 0
        && connection_timeout > 0
        && max_connections > 0
        && db_pool_size > 0
}

/// Check if security configuration requires capabilities in production
/// **PURE FUNCTION**: Environment-based security validation
/// **TESTABLE**: Can verify production security requirements
#[must_use]
pub const fn requires_security_capabilities_in_production(
    is_production: bool,
    capabilities_count: usize,
) -> bool {
    if !is_production {
        return true; // Non-production can have empty capabilities
    }
    capabilities_count > 0
}

/// Validate that monitoring is properly configured if enabled
/// **PURE FUNCTION**: Conditional validation based on enabled state
/// **TESTABLE**: Can verify enabled implies valid configuration
#[must_use]
#[allow(clippy::fn_params_excessive_bools)] // Predicate mirrors external config flags 1:1
pub const fn is_monitoring_config_complete(
    alerts_enabled: bool,
    has_notifications: bool,
    prometheus_enabled: bool,
    prometheus_valid: bool,
) -> bool {
    // If alerts are enabled, must have notifications
    let alerts_ok = !alerts_enabled || has_notifications;
    // If Prometheus is enabled, must be valid
    let prometheus_ok = !prometheus_enabled || prometheus_valid;

    alerts_ok && prometheus_ok
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 🎯 **BOOLEAN LOGIC MUTATION DETECTION TESTS**
    /// These tests are specifically designed to catch mutations in boolean operations

    #[test]
    fn test_environment_detection_mutations() {
        // ✅ CATCHES STRING COMPARISON MUTATIONS (== vs !=)
        assert!(is_production_environment("production"));
        assert!(is_production_environment("PRODUCTION"));
        assert!(is_production_environment("Production"));
        assert!(!is_production_environment("development"));
        assert!(!is_production_environment("test"));
        assert!(!is_production_environment(""));

        // ✅ CATCHES CASE SENSITIVITY MUTATIONS
        assert!(is_development_environment("development"));
        assert!(is_development_environment("DEVELOPMENT"));
        assert!(!is_development_environment("production"));

        assert!(is_test_environment("test"));
        assert!(is_test_environment("TEST"));
        assert!(!is_test_environment("production"));
    }

    #[test]
    fn test_range_validation_boundary_conditions() {
        // ✅ CATCHES COMPARISON MUTATIONS (>= vs >, <= vs <)
        assert!(is_valid_percentage_threshold(0.0)); // Boundary: exactly 0
        assert!(is_valid_percentage_threshold(100.0)); // Boundary: exactly 100
        assert!(is_valid_percentage_threshold(50.0)); // Middle value
        assert!(!is_valid_percentage_threshold(-0.1)); // Just below minimum
        assert!(!is_valid_percentage_threshold(100.1)); // Just above maximum

        // ✅ CATCHES CONSENSUS THRESHOLD LOGIC (>= vs >)
        assert!(!is_valid_consensus_threshold(0.4)); // Below minimum
        assert!(is_valid_consensus_threshold(0.5)); // Exact minimum
        assert!(is_valid_consensus_threshold(0.7)); // Valid middle
        assert!(is_valid_consensus_threshold(1.0)); // Exact maximum
        assert!(!is_valid_consensus_threshold(1.1)); // Above maximum
    }

    #[test]
    fn test_port_validation_mutations() {
        // ✅ CATCHES PORT RANGE MUTATIONS (> vs >=)
        assert!(!is_valid_port_number(0)); // Invalid: zero
        assert!(is_valid_port_number(1)); // Valid: minimum
        assert!(is_valid_port_number(8080)); // Valid: common port
        assert!(is_valid_port_number(65535)); // Valid: maximum (u16 max)
        // Note: u16 cannot exceed 65535, so upper bound check not needed
    }

    #[test]
    fn test_string_validation_mutations() {
        // ✅ CATCHES NEGATION MUTATIONS (! vs identity)
        assert!(!is_non_empty_string("")); // Empty string
        assert!(is_non_empty_string(" ")); // Whitespace
        assert!(is_non_empty_string("test")); // Normal string
        assert!(is_non_empty_string("a")); // Single character

        // ✅ CATCHES POSITIVE NUMBER MUTATIONS (> vs >=)
        assert!(!is_positive_number(0)); // Zero is not positive
        assert!(is_positive_number(1)); // One is positive
        assert!(is_positive_number(u64::MAX)); // Maximum value
    }

    #[test]
    fn test_tls_configuration_and_logic() {
        // ✅ CATCHES AND LOGIC MUTATIONS (&& vs ||)
        assert!(!has_required_tls_files("", "")); // Both empty
        assert!(!has_required_tls_files("cert.pem", "")); // Only cert
        assert!(!has_required_tls_files("", "key.pem")); // Only key
        assert!(has_required_tls_files("cert.pem", "key.pem")); // Both present

        // ✅ CATCHES SPECIFIC CASE: Both must be non-empty (AND logic)
        assert!(!has_required_tls_files("cert", "")); // AND requires both
        assert!(!has_required_tls_files("", "key")); // AND requires both
    }

    #[test]
    fn test_notification_methods_or_logic() {
        // ✅ CATCHES OR LOGIC MUTATIONS (|| vs &&)
        assert!(!has_notification_methods(false, false, false)); // None
        assert!(has_notification_methods(true, false, false)); // Email only
        assert!(has_notification_methods(false, true, false)); // Slack only
        assert!(has_notification_methods(false, false, true)); // Webhook only
        assert!(has_notification_methods(true, true, false)); // Email + Slack
        assert!(has_notification_methods(true, true, true)); // All methods

        // ✅ CATCHES SPECIFIC CASE: Any method enables notifications (OR logic)
        assert!(has_notification_methods(true, false, false)); // OR allows any
        assert!(has_notification_methods(false, true, false)); // OR allows any
    }

    #[test]
    fn test_prometheus_validation_conditional_logic() {
        // ✅ CATCHES CONDITIONAL MUTATIONS (if enabled -> must be valid)
        assert!(is_prometheus_config_valid(false, 0, "")); // Disabled is always valid
        assert!(is_prometheus_config_valid(false, 80, "/metrics")); // Disabled ignores params
        assert!(!is_prometheus_config_valid(true, 0, "")); // Enabled but empty path
        assert!(!is_prometheus_config_valid(true, 80, "")); // Enabled, valid port, empty path
        assert!(is_prometheus_config_valid(true, 0, "/metrics")); // Enabled, port 0 (OS-assigned), valid path
        assert!(is_prometheus_config_valid(true, 80, "/metrics")); // Enabled and valid

        // ✅ CATCHES IMPLICATION LOGIC: enabled -> non_empty_path (port 0 is OK for OS assignment)
        assert!(is_prometheus_config_valid(true, 9090, "/metrics")); // Full validation when enabled
    }

    #[test]
    fn test_signature_format_multiple_or_conditions() {
        // ✅ CATCHES OR CHAIN MUTATIONS (|| vs &&)
        assert!(!has_valid_signature_format("")); // Too short
        assert!(!has_valid_signature_format("short")); // Too short
        assert!(!has_valid_signature_format("invalid_prefix_12345")); // Wrong prefix

        // ✅ CATCHES EACH OR CONDITION
        assert!(has_valid_signature_format("sec_provider_sig_12345")); // First OR condition
        assert!(has_valid_signature_format("security_sig_67890")); // Second OR condition
        assert!(has_valid_signature_format("vault_sig_abcdef")); // Third OR condition
        assert!(has_valid_signature_format("standalone_sig_xyz")); // Fourth OR condition
        assert!(has_valid_signature_format("self_signed_123")); // Fifth OR condition

        // ✅ CATCHES LENGTH CHECK AND PREFIX CHECK (both required)
        assert!(!has_valid_signature_format("sec_pro")); // Valid prefix but too short
    }

    #[test]
    fn test_internal_communication_complex_and_logic() {
        // ✅ CATCHES COMPLEX AND LOGIC (both source AND destination must be internal)
        assert!(!is_internal_communication("external", "external")); // Both external
        assert!(!is_internal_communication("nestgate-api", "external")); // Mixed: internal -> external
        assert!(!is_internal_communication("external", "nestgate-core")); // Mixed: external -> internal
        assert!(is_internal_communication("nestgate-api", "nestgate-core")); // Both nestgate
        assert!(is_internal_communication("primal-auth", "primal-storage")); // Both primal
        assert!(is_internal_communication("internal-service", "internal-db")); // Both internal

        // ✅ CATCHES PREFIX MATCHING LOGIC
        assert!(is_internal_communication("nestgate-foo", "primal-bar")); // Different internal prefixes
        assert!(!is_internal_communication("nestgate-foo", "external-bar")); // One external
    }

    #[test]
    fn test_alert_thresholds_all_valid_and_logic() {
        // ✅ CATCHES AND CHAIN MUTATIONS (all must be valid)
        assert!(are_alert_thresholds_valid(50.0, 60.0, 70.0, 80.0, 100.0)); // All valid
        assert!(!are_alert_thresholds_valid(-1.0, 60.0, 70.0, 80.0, 100.0)); // CPU invalid
        assert!(!are_alert_thresholds_valid(50.0, 150.0, 70.0, 80.0, 100.0)); // Memory invalid
        assert!(!are_alert_thresholds_valid(50.0, 60.0, -10.0, 80.0, 100.0)); // Disk invalid
        assert!(!are_alert_thresholds_valid(50.0, 60.0, 70.0, 200.0, 100.0)); // Error rate invalid
        assert!(!are_alert_thresholds_valid(50.0, 60.0, 70.0, 80.0, -50.0)); // Latency invalid

        // ✅ CATCHES SPECIAL CASE: Latency threshold (>= 0, not percentage)
        assert!(are_alert_thresholds_valid(50.0, 60.0, 70.0, 80.0, 0.0)); // Latency can be 0
        assert!(are_alert_thresholds_valid(50.0, 60.0, 70.0, 80.0, 1000.0)); // Latency > 100 is ok
    }

    #[test]
    fn test_role_based_access_hierarchy() {
        // ✅ CATCHES ADMIN PRIVILEGE MUTATIONS (admin should access everything)
        assert!(has_required_role("admin", "admin")); // Admin accessing admin
        assert!(has_required_role("admin", "operator")); // Admin accessing operator
        assert!(has_required_role("admin", "readonly")); // Admin accessing readonly
        assert!(has_required_role("admin", "custom")); // Admin accessing custom

        // ✅ CATCHES NON-ADMIN ACCESS MUTATIONS (exact match only)
        assert!(has_required_role("operator", "operator")); // Exact match
        assert!(!has_required_role("operator", "admin")); // Cannot elevate
        assert!(!has_required_role("readonly", "operator")); // Cannot elevate
        assert!(!has_required_role("custom", "operator")); // Different roles
        assert!(has_required_role("custom", "custom")); // Exact match
    }

    #[test]
    fn test_permissions_any_of_or_logic() {
        let user_perms = vec!["read".to_string(), "write".to_string()];
        let required_read = vec!["read".to_string()];
        let required_admin = vec!["admin".to_string()];
        let required_multiple = vec!["read".to_string(), "admin".to_string()];
        let required_none: Vec<String> = vec![];

        // ✅ CATCHES OR LOGIC IN PERMISSION CHECKING (any required permission)
        assert!(has_any_required_permission(&user_perms, &required_read)); // Has read
        assert!(!has_any_required_permission(&user_perms, &required_admin)); // Missing admin
        assert!(has_any_required_permission(&user_perms, &required_multiple)); // Has read (first)
        assert!(has_any_required_permission(&user_perms, &required_none)); // No requirements

        // ✅ CATCHES EMPTY REQUIREMENTS CASE
        assert!(has_any_required_permission(&[], &required_none)); // No user perms, no requirements
        assert!(!has_any_required_permission(&[], &required_read)); // No user perms, has requirements
    }

    #[test]
    fn test_system_resources_all_positive_and_logic() {
        // ✅ CATCHES AND LOGIC MUTATIONS (all resources must be positive)
        assert!(are_system_resources_valid(4, 30, 10, 100, 10)); // All positive
        assert!(!are_system_resources_valid(0, 30, 10, 100, 10)); // Worker threads zero
        assert!(!are_system_resources_valid(4, 0, 10, 100, 10)); // Request timeout zero
        assert!(!are_system_resources_valid(4, 30, 0, 100, 10)); // Connection timeout zero
        assert!(!are_system_resources_valid(4, 30, 10, 0, 10)); // Max connections zero
        assert!(!are_system_resources_valid(4, 30, 10, 100, 0)); // DB pool size zero

        // ✅ CATCHES BOUNDARY CONDITIONS (> 0 vs >= 0)
        assert!(are_system_resources_valid(1, 1, 1, 1, 1)); // All minimum positive
        assert!(!are_system_resources_valid(4, 30, 10, 100, 0)); // Last parameter zero fails all
    }

    #[test]
    fn test_production_security_requirements() {
        // ✅ CATCHES CONDITIONAL LOGIC (production requires capabilities)
        assert!(requires_security_capabilities_in_production(false, 0)); // Non-prod, no capabilities
        assert!(requires_security_capabilities_in_production(false, 5)); // Non-prod, has capabilities
        assert!(!requires_security_capabilities_in_production(true, 0)); // Prod, no capabilities - INVALID
        assert!(requires_security_capabilities_in_production(true, 1)); // Prod, has capabilities
        assert!(requires_security_capabilities_in_production(true, 10)); // Prod, many capabilities

        // ✅ CATCHES IMPLICATION LOGIC: production -> (capabilities > 0)
        assert!(!requires_security_capabilities_in_production(true, 0)); // Production requires capabilities
    }

    #[test]
    fn test_monitoring_completeness_complex_and_logic() {
        // ✅ CATCHES COMPLEX CONDITIONAL AND LOGIC
        // Case 1: Nothing enabled - should be valid
        assert!(is_monitoring_config_complete(false, false, false, false));
        assert!(is_monitoring_config_complete(false, true, false, false)); // Extra notifications ok

        // Case 2: Alerts enabled - must have notifications
        assert!(!is_monitoring_config_complete(true, false, false, false)); // Alerts enabled, no notifications
        assert!(is_monitoring_config_complete(true, true, false, false)); // Alerts enabled, has notifications

        // Case 3: Prometheus enabled - must be valid
        assert!(!is_monitoring_config_complete(false, false, true, false)); // Prometheus enabled, invalid
        assert!(is_monitoring_config_complete(false, false, true, true)); // Prometheus enabled, valid

        // Case 4: Both enabled - both must be properly configured
        assert!(!is_monitoring_config_complete(true, false, true, true)); // Alerts enabled, no notifications
        assert!(!is_monitoring_config_complete(true, true, true, false)); // Prometheus enabled, invalid
        assert!(is_monitoring_config_complete(true, true, true, true)); // Both properly configured

        // ✅ CATCHES IMPLICATION CHAINS:
        // enabled_alerts -> has_notifications AND enabled_prometheus -> prometheus_valid
        assert!(is_monitoring_config_complete(false, false, false, true)); // Disabled alerts, valid prometheus
        assert!(!is_monitoring_config_complete(true, false, false, true)); // Enabled alerts need notifications
    }

    #[test]
    fn test_integration_workflow_predicates() {
        // ✅ COMPREHENSIVE INTEGRATION TEST - catches mutations in predicate chains

        // Scenario 1: Development environment setup
        let env = "development";
        assert!(is_development_environment(env));
        assert!(requires_security_capabilities_in_production(
            is_production_environment(env),
            0
        )); // Non-prod allows empty capabilities

        // Scenario 2: Production environment requirements
        let prod_env = "production";
        assert!(is_production_environment(prod_env));
        assert!(!requires_security_capabilities_in_production(
            is_production_environment(prod_env),
            0
        ));
        assert!(requires_security_capabilities_in_production(
            is_production_environment(prod_env),
            3
        ));

        // Scenario 3: System configuration validation chain
        let valid_resources = are_system_resources_valid(8, 30, 10, 1000, 20);
        let valid_thresholds = are_alert_thresholds_valid(75.0, 80.0, 85.0, 5.0, 500.0);
        let system_valid = valid_resources && valid_thresholds;
        assert!(system_valid);

        // Scenario 4: Security configuration chain
        let cert_valid = has_valid_signature_format("sec_provider_sig_abc123");
        let tls_valid = has_required_tls_files("server.crt", "server.key");
        let security_valid = cert_valid && tls_valid;
        assert!(security_valid);
    }
}
