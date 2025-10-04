///
/// Provides security policy management for MCP protocol connections.
/// Part of the modular security architecture.
use nestgate_core::error::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
/// Security policy for MCP operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPolicy {
    /// Policy name
    pub name: String,
    /// Policy description
    pub description: String,
    /// Access control rules
    pub access_control: AccessControl,
    /// Rate limiting rules
    pub rate_limits: RateLimits,
    /// Session policies
    pub session_policy: SessionPolicy,
    /// Audit requirements
    pub audit_requirements: AuditRequirements,
}
/// Access control configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControl {
    /// Allowed IP addresses/ranges
    pub allowed_ips: Vec<String>,
    /// Blocked IP addresses/ranges
    pub blocked_ips: Vec<String>,
    /// Allowed user agents
    pub allowed_user_agents: Vec<String>,
    /// Require TLS for connections
    pub require_tls: bool,
    /// Minimum TLS version
    pub min_tls_version: String,
    /// Allowed authentication methods
    pub allowed_auth_methods: Vec<String>,
}
/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimits {
    /// Requests per minute per client
    pub requests_per_minute: u32,
    /// Maximum concurrent connections per client
    pub max_concurrent_connections: u32,
    /// Request burst size
    pub burst_size: u32,
    /// Rate limit window duration
    pub window_duration: Duration,
}
/// Session policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionPolicy {
    /// Maximum session duration
    pub max_session_duration: Duration,
    /// Session idle timeout
    pub idle_timeout: Duration,
    /// Require session renewal
    pub require_renewal: bool,
    /// Maximum sessions per user
    pub max_sessions_per_user: u32,
}
/// Audit requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditRequirements {
    /// Log all access attempts
    pub log_access_attempts: bool,
    /// Log failed authentication attempts
    pub log_failed_auth: bool,
    /// Log administrative actions
    pub log_admin_actions: bool,
    /// Retention period for audit logs
    pub log_retention_days: u32,
}
/// Policy manager for MCP security
#[derive(Debug)]
pub struct PolicyManager {
    /// Active security policies
    policies: HashMap<String, SecurityPolicy>,
    /// Default policy name
    default_policy: String,
}
impl PolicyManager {
    /// Create new policy manager with default policies
    #[must_use]
    pub fn new() -> Self {
        let mut manager = Self {
            policies: HashMap::new(),
            default_policy: "default".to_string(),
        };

        // Create default policy
        manager.create_default_policies();
        manager
    }

    /// Create default security policies
    fn create_default_policies(&mut self) {
        // Default security policy
        let default_policy = SecurityPolicy {
            name: "default".to_string(),
            description: "Default MCP security policy".to_string(),
            access_control: AccessControl {
                allowed_ips: vec!["0.0.0.0/0".to_string()], // Allow all by default
                blocked_ips: vec![],
                allowed_user_agents: vec![],
                require_tls: false, // Development-friendly default
                min_tls_version: "1.2".to_string(),
                allowed_auth_methods: vec!["token".to_string(), "certificate".to_string()],
            }
            rate_limits: RateLimits {
                requests_per_minute: 1000,
                max_concurrent_connections: 100,
                burst_size: 50,
                window_duration: Duration::from_secs(60),
            }
            session_policy: SessionPolicy {
                max_session_duration: Duration::from_secs(3600), // 1 hour
                idle_timeout: Duration::from_secs(1800),         // 30 minutes
                require_renewal: false,
                max_sessions_per_user: 10,
            }
            audit_requirements: AuditRequirements {
                log_access_attempts: true,
                log_failed_auth: true,
                log_admin_actions: true,
                log_retention_days: 30,
            }
        };

        // Strict security policy for production
        let strict_policy = SecurityPolicy {
            name: "strict".to_string(),
            description: "Strict security policy for production".to_string(),
            access_control: AccessControl {
                allowed_ips: vec!["10.0.0.0/8".to_string(), "172.16.0.0/12".to_string()],
                blocked_ips: vec![],
                allowed_user_agents: vec![],
                require_tls: true,
                min_tls_version: "1.3".to_string(),
                allowed_auth_methods: vec!["certificate".to_string()],
            }
            rate_limits: RateLimits {
                requests_per_minute: 100,
                max_concurrent_connections: 20,
                burst_size: 10,
                window_duration: Duration::from_secs(60),
            }
            session_policy: SessionPolicy {
                max_session_duration: Duration::from_secs(1800), // 30 minutes
                idle_timeout: Duration::from_secs(600),          // 10 minutes
                require_renewal: true,
                max_sessions_per_user: 3,
            }
            audit_requirements: AuditRequirements {
                log_access_attempts: true,
                log_failed_auth: true,
                log_admin_actions: true,
                log_retention_days: 90,
            }
        };

        self.policies.insert("default".to_string(), default_policy);
        self.policies.insert("strict".to_string(), strict_policy);
    }

    /// Add a new security policy
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        #[must_use]
        pub fn add_policy(&mut self, policy: SecurityPolicy) -> Result<()>  {
        let name = policy.name.clone();
        self.policies.insert(name, policy);
        Ok(())
    }

    /// Get a policy by name
    pub fn get_policy(&self, name: &str) -> Option<&SecurityPolicy> {
        self.policies.get(name)
    }

    /// Get the default policy
    pub fn get_default_policy(&self) -> &SecurityPolicy {
        self.policies.get(&self.default_policy).unwrap_or_else(|| {
            tracing::error!("Expect failed: Default policy must exist");
            panic!(
                "Default policy must exist - this indicates a logic error in policy initialization"
            );
        })
    }

    /// Set the default policy
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        #[must_use]
        pub fn set_default_policy(&mut self, name: &str) -> Result<()>  {
        if !self.policies.contains_key(name) {
            return Err(NestGateError::mcp_error(
                &format!("Policy '{"actual_error_details"}' does not exist"),
                "set_default_policy",
                None,
            ));
        }
        self.default_policy = name.to_string();
        Ok(())
    }

    /// Remove a policy
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        #[must_use]
        pub fn remove_policy(&mut self, name: &str) -> Result<()>  {
        if name == self.default_policy {
            return Err(NestGateError::mcp_error(
                "Cannot remove the default policy",
                "remove_policy",
                None,
            ));
        }
        self.policies.remove(name);
        Ok(())
    }

    /// List all policies
    pub fn list_policies(&self) -> Vec<&SecurityPolicy> {
        self.policies.values().collect()
    }

    /// Validate access against a policy
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn validate_access(
        &self,
        policy_name: &str,
        client_ip: &str,
        user_agent: &str,
    ) -> Result<bool>  {
        let policy = self.get_policy(policy_name).ok_or_else(|| {
            NestGateError::mcp_error(
                &format!("Policy '{"actual_error_details"}' not found"),
                "validate_access",
                None,
            )
        })?;

        // Check IP allowlist
        if !policy.access_control.allowed_ips.is_empty() {
            let allowed = policy
                .access_control
                .allowed_ips
                .iter()
                .any(|allowed_ip| self.ip_matches(client_ip, allowed_ip));
            if !allowed {
                return Ok(false);
            }
        }

        // Check IP blocklist
        let blocked = policy
            .access_control
            .blocked_ips
            .iter()
            .any(|blocked_ip| self.ip_matches(client_ip, blocked_ip));
        if blocked {
            return Ok(false);
        }

        // Check user agent if specified
        if !policy.access_control.allowed_user_agents.is_empty() {
            let allowed = policy
                .access_control
                .allowed_user_agents
                .iter()
                .any(|allowed_ua| user_agent.contains(allowed_ua));
            if !allowed {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Simple IP matching (simplified for demo - would use proper CIDR matching in production)
    fn ip_matches(&self, client_ip: &str, pattern: &str) -> bool {
        if pattern == "0.0.0.0/0" {
            return true; // Allow all
        }

        // Simple exact match for now
        client_ip == pattern || pattern.starts_with(&client_ip[..client_ip.rfind('.').unwrap_or(0)])
    }
}

impl Default for PolicyManager {
    fn default() -> Self {
        Self::new()
    }
}
