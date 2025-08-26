use std::collections::HashMap;
///
/// This module provides configuration for security testing including authentication,
/// authorization, penetration testing, and vulnerability scanning.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// ==================== SECURITY TESTING CONFIGURATION ====================

/// **Unified security testing configuration**
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestSecurityConfig {
    /// Authentication testing
    pub auth: AuthTestConfig,
    /// Authorization testing
    pub authz: AuthzTestConfig,
    /// Penetration testing
    pub penetration: PenetrationTestConfig,
    /// Security scanning
    pub scanning: SecurityScanConfig,
    /// Vulnerability testing
    pub vulnerability: VulnerabilityTestConfig,
}

/// **Authentication test configuration**
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthTestConfig {
    /// Enable authentication tests
    pub enabled: bool,
    /// Test timeout
    pub timeout: Duration,
    /// Test credentials
    pub test_credentials: HashMap<String, String>,
}

/// **Authorization test configuration**
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthzTestConfig {
    /// Enable authorization tests
    pub enabled: bool,
    /// Test roles
    pub test_roles: Vec<String>,
    /// Permission matrix
    pub permissions: HashMap<String, Vec<String>>,
}

/// **Penetration test configuration**
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PenetrationTestConfig {
    /// Enable penetration tests
    pub enabled: bool,
    /// Target endpoints
    pub target_endpoints: Vec<String>,
    /// Attack scenarios
    pub attack_scenarios: Vec<String>,
}

/// **Security scan configuration**
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityScanConfig {
    /// Enable security scans
    pub enabled: bool,
    /// Scan types
    pub scan_types: Vec<String>,
    /// Scan depth
    pub scan_depth: String,
}

/// **Vulnerability test configuration**
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VulnerabilityTestConfig {
    /// Enable vulnerability tests
    pub enabled: bool,
    /// CVE database path
    pub cve_database: Option<String>,
    /// Severity threshold
    pub severity_threshold: String,
}
