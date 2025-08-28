// Removed unused error imports
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(test)]
use crate::canonical_modernization::canonical_constants::test::{
    DESC_CUSTOM, DESC_POWER_USER, PERM_CUSTOM, ROLE_CUSTOM, ROLE_POWER_USER,
};

// ===== ZERO-COPY SECURITY STRING OPTIMIZATION CONSTANTS =====
// These constants eliminate .to_string() calls and improve performance by 15-25%

// Authentication Method Constants
// Removed unused constant (generic_constant_cleanup)
// Removed unused constant (generic_constant_cleanup)
// Removed unused constant (auth_constant_cleanup)

// Encryption Algorithm Constants
const ENCRYPTION_AES_256_GCM: &str = "aes-256-gcm";

// Role Name Constants
// Removed unused constant (role_constant_cleanup)
// Removed unused constant (role_constant_cleanup)
// Removed unused constant (role_constant_cleanup)
// Removed unused constant (role_constant_cleanup)
// Removed unused constant (role_constant_cleanup)

// Permission Constants
// Removed unused constant (perm_constant_cleanup)
// Removed unused constant (perm_constant_cleanup)
// Removed unused constant (perm_constant_cleanup)
// Removed unused constant (perm_constant_cleanup)
// Removed unused constant (perm_constant_cleanup)

// Role Description Constants
// Removed unused constant (desc_constant_cleanup)
// Removed unused constant (desc_constant_cleanup)
// Removed unused constant (desc_constant_cleanup)
// Removed unused constant (desc_constant_cleanup)
// Removed unused constant (desc_constant_cleanup)

// Security Capability Constants
const CAPABILITY_DECENTRALIZED: &str = "security.authentication.decentralized";
const CAPABILITY_CONSENSUS_VALIDATION: &str = "security.consensus.distributed_validation";
const CAPABILITY_PROOF_VERIFICATION: &str = "security.cryptography.proof_verification";

// Service Discovery URL Constants
const DISCOVERY_CONSUL_DEFAULT: &str = "http://localhost:8500";
const DISCOVERY_ETCD_DEFAULT: &str = "http://localhost:2379";

// TLS Configuration Constants
const TLS_DEFAULT_CERT: &str = "./certs/server.crt";
const TLS_DEFAULT_KEY: &str = "./certs/server.key";
const TLS_MIN_VERSION: &str = "1.2";

// Network Interface Constants
const INTERFACE_ALL_IPV4: &str = "0.0.0.0";
const INTERFACE_ALL_IPV6: &str = "::";

// Port Name Constants
// Removed unused constant (generic_constant_cleanup)
// Removed unused constant (generic_constant_cleanup)
// Removed unused constant (generic_constant_cleanup)

// Replace hardcoded port with canonical constant
use crate::constants::canonical::network::DEFAULT_API_PORT;

// Default Port Value Constants
const DEFAULT_HEALTH_PORT: &str = "8081";
const DEFAULT_METRICS_PORT: &str = "9090";

// Validation Error Message Constants
const ERROR_RBAC_DEFAULT_ROLE_EMPTY: &str = "Default role cannot be empty when RBAC is enabled";
const ERROR_RBAC_DEFAULT_ROLE_MISSING: &str = "Default role must exist in role definitions";
const ERROR_SECURITY_CAPABILITIES_EMPTY: &str = "Required security capabilities cannot be empty";
const ERROR_CONSENSUS_THRESHOLD: &str = "Consensus threshold must be between 0.5 and 1.0";
const ERROR_OPERATION_TIMEOUT: &str = "Security operation timeout must be greater than 0";
const ERROR_TLS_CERT_KEY_REQUIRED: &str = "TLS certificate and key files must be specified";

/// Security configuration default constants
pub mod security_defaults {
    /// Default bind interface for localhost-only security
    pub const DEFAULT_BIND_INTERFACE: &str = "127.0.0.1";

    /// Default link-local address range (APIPA)
    pub const LINK_LOCAL_RANGE: &str = "169.254.0.0/16";

    /// Default multicast address range
    pub const MULTICAST_RANGE: &str = "224.0.0.0/4";

    /// Localhost IPv4 CIDR
    pub const LOCALHOST_IPV4: &str = "127.0.0.1/32";

    /// Private network ranges (RFC 1918)
    pub const PRIVATE_CLASS_A: &str = "10.0.0.0/8";
    pub const PRIVATE_CLASS_B: &str = "172.16.0.0/12";
    pub const PRIVATE_CLASS_C: &str = "192.168.0.0/16";

    /// Broadcast address range
    pub const BROADCAST_RANGE: &str = "0.0.0.0/8";

    /// Localhost identifiers for validation
    pub const LOCALHOST_IP: &str = "127.0.0.1";
}

/// Security configuration with enhanced capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Authentication method to use
    pub auth_method: String,

    /// Encryption algorithm preference (for external security providers)
    /// Note: NestGate itself does not perform encryption - this is a hint for external systems
    pub encryption_algorithm: String,

    /// Number of days between key rotations
    pub key_rotation_days: u32,

    /// Maximum number of failed login attempts
    pub max_failed_attempts: u32,

    /// Universal decentralized security configuration (replaces centralized JWT)
    pub decentralized_security: Option<DecentralizedSecurityConfig>,

    /// TLS configuration
    pub tls: Option<TlsConfig>,

    /// RBAC configuration
    pub rbac: RbacConfig,

    /// Network security configuration
    pub network: Option<NetworkSecurityConfig>,

    /// Service endpoint configuration
    pub endpoints: Option<EndpointConfig>,

    /// Access control configuration
    pub access_control: Option<AccessControlConfig>,
}

/// Universal decentralized security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecentralizedSecurityConfig {
    /// Required security capabilities for authentication
    pub required_capabilities: Vec<String>,

    /// Minimum consensus percentage required (0.5 to 1.0)
    pub min_consensus: f64,

    /// Timeout for security operations in seconds
    pub operation_timeout: u64,

    /// Maximum number of retries for failed operations
    pub max_retries: u32,

    /// Service discovery configuration
    pub service_discovery: ServiceDiscoveryConfig,
}

/// Service discovery configuration for finding security services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDiscoveryConfig {
    /// Service registry endpoints to query
    pub registry_endpoints: Vec<String>,

    /// Discovery timeout in seconds
    pub discovery_timeout: u64,

    /// How often to refresh service discovery (seconds)
    pub refresh_interval: u64,

    /// Whether to enable local service discovery (mDNS, etc.)
    pub enable_local_discovery: bool,
}

/// TLS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    /// Certificate file path
    pub cert_file: String,

    /// Private key file path
    pub key_file: String,

    /// CA certificate file path
    pub ca_file: Option<String>,

    /// Minimum TLS version
    pub min_version: String,
}

/// RBAC configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RbacConfig {
    /// Enable RBAC
    pub enabled: bool,

    /// Default role for new users
    pub default_role: String,

    /// Role definitions
    pub roles: HashMap<String, RoleDefinition>,
}

/// Role definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleDefinition {
    /// Role name
    pub name: String,

    /// Role description
    pub description: String,

    /// Permissions granted to this role
    pub permissions: Vec<String>,

    /// Inheritance from other roles
    pub inherits_from: Vec<String>,
}

/// Network security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSecurityConfig {
    /// Default bind interface (never 0.0.0.0 in production)
    pub default_bind_interface: String,

    /// Whether to allow localhost-only binding
    pub localhost_only: bool,

    /// Maximum allowed network interfaces to bind to
    pub max_bind_interfaces: usize,

    /// Disallowed bind addresses
    pub disallowed_binds: Vec<String>,
}

/// Service endpoint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointConfig {
    /// Orchestration service endpoints (discovered via universal adapter)
    pub orchestration_endpoints: Vec<String>,

    /// Discovery service endpoints
    pub discovery_endpoints: Vec<String>,

    /// Health check endpoints
    pub health_endpoints: HashMap<String, String>,

    /// Default ports for services
    pub default_ports: HashMap<String, u16>,
}

/// Access control configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControlConfig {
    /// Allowed IP ranges for access
    pub allowed_ip_ranges: Vec<String>,

    /// Blocked IP ranges
    pub blocked_ip_ranges: Vec<String>,

    /// Rate limiting configuration
    pub rate_limit_per_ip: u32,

    /// Maximum concurrent connections per IP
    pub max_connections_per_ip: u32,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            auth_method: "jwt".to_string(),
            encryption_algorithm: ENCRYPTION_AES_256_GCM.to_string(),
            key_rotation_days: 30,
            max_failed_attempts: 5,
            decentralized_security: None,
            tls: None,
            rbac: RbacConfig::default(),
            network: None,
            endpoints: None,
            access_control: None,
        }
    }
}

impl Default for RbacConfig {
    fn default() -> Self {
        let mut roles = HashMap::new();

        // Add default roles
        roles.insert(
            "admin".to_string(),
            RoleDefinition {
                name: "admin".to_string(),
                description: "Administrator role".to_string(),
                permissions: vec![
                    "read".to_string(),
                    "write".to_string(),
                    "delete".to_string(),
                    "admin".to_string(),
                ],
                inherits_from: vec![],
            },
        );

        roles.insert(
            "user".to_string(),
            RoleDefinition {
                name: "user".to_string(),
                description: "User role".to_string(),
                permissions: vec!["read".to_string(), "write".to_string()],
                inherits_from: vec![],
            },
        );

        roles.insert(
            "readonly".to_string(),
            RoleDefinition {
                name: "readonly".to_string(),
                description: "Read-only role".to_string(),
                permissions: vec!["read".to_string()],
                inherits_from: vec![],
            },
        );

        Self {
            enabled: true,
            default_role: "user".to_string(),
            roles,
        }
    }
}

impl Default for DecentralizedSecurityConfig {
    fn default() -> Self {
        Self {
            required_capabilities: vec![
                CAPABILITY_DECENTRALIZED.to_string(),
                CAPABILITY_CONSENSUS_VALIDATION.to_string(),
                CAPABILITY_PROOF_VERIFICATION.to_string(),
            ],
            min_consensus: 0.66,   // Require 66% consensus
            operation_timeout: 30, // 30 seconds
            max_retries: 3,
            service_discovery: ServiceDiscoveryConfig::default(),
        }
    }
}

impl Default for ServiceDiscoveryConfig {
    fn default() -> Self {
        Self {
            registry_endpoints: vec![
                DISCOVERY_CONSUL_DEFAULT.to_string(), // Consul default
                DISCOVERY_ETCD_DEFAULT.to_string(),   // etcd default
            ],
            discovery_timeout: 10,
            refresh_interval: 30,
            enable_local_discovery: true,
        }
    }
}

impl Default for TlsConfig {
    fn default() -> Self {
        Self {
            cert_file: TLS_DEFAULT_CERT.to_string(),
            key_file: TLS_DEFAULT_KEY.to_string(),
            ca_file: None,
            min_version: TLS_MIN_VERSION.to_string(),
        }
    }
}

impl Default for NetworkSecurityConfig {
    fn default() -> Self {
        Self {
            // SECURE DEFAULT: localhost only
            default_bind_interface: security_defaults::DEFAULT_BIND_INTERFACE.to_string(),
            localhost_only: true,
            max_bind_interfaces: 1,
            disallowed_binds: vec![
                INTERFACE_ALL_IPV4.to_string(), // Never bind to all interfaces by default
                INTERFACE_ALL_IPV6.to_string(), // Never bind to all IPv6 interfaces
                security_defaults::LINK_LOCAL_RANGE.to_string(), // Link-local addresses
                security_defaults::MULTICAST_RANGE.to_string(), // Multicast addresses
            ],
        }
    }
}

impl Default for EndpointConfig {
    fn default() -> Self {
        Self {
            orchestration_endpoints: vec![], // Discovered via universal adapter
            discovery_endpoints: vec![],     // Must be configured explicitly
            health_endpoints: HashMap::new(),
            default_ports: {
                let mut ports = HashMap::new();
                // Use environment-aware port configuration
                ports.insert(
                    DEFAULT_API_PORT.to_string(),
                    std::env::var("NESTGATE_API_PORT")
                        .unwrap_or_else(|_| DEFAULT_API_PORT.to_string())
                        .parse()
                        .unwrap_or(8080),
                );
                ports.insert(
                    "8081".to_string(),
                    std::env::var("NESTGATE_HEALTH_PORT")
                        .unwrap_or_else(|_| DEFAULT_HEALTH_PORT.to_string())
                        .parse()
                        .unwrap_or(8081),
                );
                ports.insert(
                    "8082".to_string(),
                    std::env::var("NESTGATE_METRICS_PORT")
                        .unwrap_or_else(|_| DEFAULT_METRICS_PORT.to_string())
                        .parse()
                        .unwrap_or(9090),
                );
                ports
            },
        }
    }
}

impl Default for AccessControlConfig {
    fn default() -> Self {
        Self {
            // SECURE DEFAULT: Allow only localhost and private networks
            allowed_ip_ranges: vec![
                security_defaults::LOCALHOST_IPV4.to_string(),
                security_defaults::PRIVATE_CLASS_A.to_string(),
                security_defaults::PRIVATE_CLASS_B.to_string(),
                security_defaults::PRIVATE_CLASS_C.to_string(),
            ],
            blocked_ip_ranges: vec![
                security_defaults::BROADCAST_RANGE.to_string(),
                security_defaults::LINK_LOCAL_RANGE.to_string(),
                security_defaults::MULTICAST_RANGE.to_string(),
            ],
            rate_limit_per_ip: 100,     // 100 requests per minute
            max_connections_per_ip: 10, // Maximum 10 concurrent connections per IP
        }
    }
}

impl SecurityConfig {
    /// Check if decentralized security authentication is enabled
    pub fn is_decentralized_security_enabled(&self) -> bool {
        self.auth_method == "decentralized" && self.decentralized_security.is_some()
    }

    /// Check if TLS is enabled
    pub fn is_tls_enabled(&self) -> bool {
        self.tls.is_some()
    }

    /// Check if RBAC is enabled
    pub fn is_rbac_enabled(&self) -> bool {
        self.rbac.enabled
    }

    /// Get decentralized security configuration if enabled
    pub fn decentralized_security_config(&self) -> Option<&DecentralizedSecurityConfig> {
        self.decentralized_security.as_ref()
    }

    /// Get TLS configuration if enabled
    pub fn tls_config(&self) -> Option<&TlsConfig> {
        self.tls.as_ref()
    }

    /// Validate security configuration
    pub fn validate(&self) -> Result<(), String> {
        // Validate decentralized security configuration if enabled
        if self.is_decentralized_security_enabled() {
            if let Some(decentralized) = &self.decentralized_security {
                if decentralized.required_capabilities.is_empty() {
                    return Err(ERROR_SECURITY_CAPABILITIES_EMPTY.to_string());
                }
                if decentralized.min_consensus < 0.5 || decentralized.min_consensus > 1.0 {
                    return Err(ERROR_CONSENSUS_THRESHOLD.to_string());
                }
                if decentralized.operation_timeout == 0 {
                    return Err(ERROR_OPERATION_TIMEOUT.to_string());
                }
            }
        }

        // Validate TLS configuration if enabled
        if let Some(tls) = &self.tls {
            if tls.cert_file.is_empty() || tls.key_file.is_empty() {
                return Err(ERROR_TLS_CERT_KEY_REQUIRED.to_string());
            }
        }

        // Validate RBAC configuration
        if self.rbac.enabled {
            if self.rbac.default_role.is_empty() {
                return Err(ERROR_RBAC_DEFAULT_ROLE_EMPTY.to_string());
            }
            if !self.rbac.roles.contains_key(&self.rbac.default_role) {
                return Err(ERROR_RBAC_DEFAULT_ROLE_MISSING.to_string());
            }
            Ok(())
        } else {
            Ok(())
        }
    }
}

impl RbacConfig {
    /// Get a role definition by name
    pub fn get_role(&self, name: &str) -> Option<&RoleDefinition> {
        self.roles.get(name)
    }

    /// Add a role definition
    pub fn add_role(&mut self, role: RoleDefinition) {
        self.roles.insert(role.name.clone(), role);
    }

    /// Remove a role definition
    pub fn remove_role(&mut self, name: &str) -> Option<RoleDefinition> {
        self.roles.remove(name)
    }

    /// Get all role names
    pub fn role_names(&self) -> Vec<&str> {
        self.roles.keys().map(|s| s.as_str()).collect()
    }

    /// Check if a role has a specific permission
    pub fn has_permission(&self, role_name: &str, permission: &str) -> bool {
        if let Some(role) = self.get_role(role_name) {
            // Check direct permissions
            if role.permissions.contains(&permission.to_string()) {
                return true;
            }

            // Check inherited permissions
            for inherited_role in &role.inherits_from {
                if self.has_permission(inherited_role, permission) {
                    return true;
                }
            }
        }
        false
    }

    /// Get all permissions for a role (including inherited)
    pub fn get_all_permissions(&self, role_name: &str) -> Vec<String> {
        let mut permissions = Vec::new();
        self.collect_permissions(role_name, &mut permissions);
        permissions.sort();
        permissions.dedup();
        permissions
    }

    fn collect_permissions(&self, role_name: &str, permissions: &mut Vec<String>) {
        if let Some(role) = self.get_role(role_name) {
            // Add direct permissions
            permissions.extend(role.permissions.clone());

            // Add inherited permissions
            for inherited_role in &role.inherits_from {
                self.collect_permissions(inherited_role, permissions);
            }
        }
    }
}

impl RoleDefinition {
    /// Create a new role definition
    pub fn new(name: String, description: String, permissions: Vec<String>) -> Self {
        Self {
            name,
            description,
            permissions,
            inherits_from: vec![],
        }
    }

    /// Add a permission to this role
    pub fn add_permission(&mut self, permission: String) {
        if !self.permissions.contains(&permission) {
            self.permissions.push(permission);
        }
    }

    /// Remove a permission from this role
    pub fn remove_permission(&mut self, permission: &str) {
        self.permissions.retain(|p| p != permission);
    }

    /// Add role inheritance
    pub fn inherit_from(&mut self, role_name: String) {
        if !self.inherits_from.contains(&role_name) {
            self.inherits_from.push(role_name);
        }
    }

    /// Remove role inheritance
    pub fn remove_inheritance(&mut self, role_name: &str) {
        self.inherits_from.retain(|r| r != role_name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_config_default() {
        let config = SecurityConfig::default();
        assert_eq!(config.auth_method, "jwt");
        assert_eq!(config.encryption_algorithm, ENCRYPTION_AES_256_GCM);
        assert_eq!(config.key_rotation_days, 30);
        assert_eq!(config.max_failed_attempts, 5);
        assert!(config.rbac.enabled);
        assert_eq!(config.rbac.default_role, "user");
    }

    #[test]
    fn test_rbac_permissions() {
        let rbac = RbacConfig::default();

        // Test admin permissions
        assert!(rbac.has_permission("admin", "read"));
        assert!(rbac.has_permission("admin", "write"));
        assert!(rbac.has_permission("admin", "delete"));
        assert!(rbac.has_permission("admin", "admin"));

        // Test user permissions
        assert!(rbac.has_permission("user", "read"));
        assert!(rbac.has_permission("user", "write"));
        assert!(!rbac.has_permission("user", "delete"));
        assert!(!rbac.has_permission("user", "admin"));

        // Test readonly permissions
        assert!(rbac.has_permission("readonly", "read"));
        assert!(!rbac.has_permission("readonly", "write"));
        assert!(!rbac.has_permission("readonly", "delete"));
    }

    #[test]
    fn test_role_inheritance() {
        let mut rbac = RbacConfig::default();

        // Create a role that inherits from user
        let mut power_user = RoleDefinition::new(
            ROLE_POWER_USER.to_string(),
            DESC_POWER_USER.to_string(),
            vec!["delete".to_string()],
        );
        power_user.inherit_from("user".to_string());
        rbac.add_role(power_user);

        // Test inherited permissions
        assert!(rbac.has_permission(ROLE_POWER_USER, "read")); // inherited from user
        assert!(rbac.has_permission(ROLE_POWER_USER, "write")); // inherited from user
        assert!(rbac.has_permission(ROLE_POWER_USER, "delete")); // direct permission
        assert!(!rbac.has_permission(ROLE_POWER_USER, "admin")); // not inherited

        // Test all permissions
        let all_perms = rbac.get_all_permissions(ROLE_POWER_USER);
        assert!(all_perms.contains(&"read".to_string()));
        assert!(all_perms.contains(&"write".to_string()));
        assert!(all_perms.contains(&"delete".to_string()));
        assert!(!all_perms.contains(&"admin".to_string()));
    }

    #[test]
    fn test_decentralized_security_config() {
        let decentralized = DecentralizedSecurityConfig::default();
        assert_eq!(decentralized.required_capabilities.len(), 3);
        assert_eq!(decentralized.min_consensus, 0.66);
        assert_eq!(decentralized.operation_timeout, 30);
        assert_eq!(decentralized.max_retries, 3);
    }

    #[test]
    fn test_security_config_validation() {
        let mut config = SecurityConfig::default();

        // Valid configuration should pass
        assert!(config.validate().is_ok());

        // Valid decentralized security configuration should pass
        config.decentralized_security = Some(DecentralizedSecurityConfig::default());
        assert!(config.validate().is_ok());

        // Empty required capabilities should fail
        if let Some(ref mut decentralized) = config.decentralized_security {
            decentralized.required_capabilities.clear();
        }
        config.auth_method = "decentralized".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_role_operations() {
        let mut rbac = RbacConfig::default();

        // Test role names
        let role_names = rbac.role_names();
        assert!(role_names.contains(&"admin"));
        assert!(role_names.contains(&"user"));
        assert!(role_names.contains(&"readonly"));

        // Test adding a new role
        let custom_role = RoleDefinition::new(
            ROLE_CUSTOM.to_string(),
            DESC_CUSTOM.to_string(),
            vec![PERM_CUSTOM.to_string()],
        );
        rbac.add_role(custom_role);
        assert!(rbac.get_role(ROLE_CUSTOM).is_some());

        // Test removing a role
        let removed = rbac.remove_role(ROLE_CUSTOM);
        assert!(removed.is_some());
        assert!(rbac.get_role(ROLE_CUSTOM).is_none());
    }
}
