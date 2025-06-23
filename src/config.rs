//! Configuration types for the orchestrator

use serde::{Deserialize, Serialize};

// Network configuration constants
pub const DEFAULT_LOCALHOST: &str = "127.0.0.1";
pub const DEFAULT_ALL_INTERFACES: &str = "0.0.0.0";
pub const DEFAULT_IPV6_LOCALHOST: &str = "::1";
pub const DEFAULT_IPV6_ALL_INTERFACES: &str = "::";

// Default ports for different services
pub mod default_ports {
    pub const ORCHESTRATOR: u16 = 8090;
    pub const API: u16 = 8080;
    pub const MCP: u16 = 8081;
    pub const WEBSOCKET: u16 = 8082;
    pub const METRICS: u16 = 8083;
    pub const HEALTH: u16 = 8084;
    pub const ZFS_API: u16 = 8085;
    pub const NETWORK_SERVICE: u16 = 8086;
}

/// Network binding configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Interface to bind to (localhost, all interfaces, or specific IP)
    pub bind_interface: String,
    /// Port to bind to (0 for auto-assignment)
    pub port: u16,
    /// Whether to enable IPv6 support
    pub ipv6_enabled: bool,
    /// Whether to bind to localhost only (secure) or all interfaces
    pub localhost_only: bool,
    /// Custom host override
    pub custom_host: Option<String>,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            bind_interface: DEFAULT_LOCALHOST.to_string(),
            port: 0, // Auto-assign
            ipv6_enabled: false,
            localhost_only: true, // Secure by default
            custom_host: None,
        }
    }
}

impl NetworkConfig {
    /// Create a new network config for localhost binding (secure)
    pub fn localhost(port: u16) -> Self {
        Self {
            bind_interface: DEFAULT_LOCALHOST.to_string(),
            port,
            ipv6_enabled: false,
            localhost_only: true,
            custom_host: None,
        }
    }
    
    /// Create a new network config for all interfaces (less secure, for production)
    pub fn all_interfaces(port: u16) -> Self {
        Self {
            bind_interface: DEFAULT_ALL_INTERFACES.to_string(),
            port,
            ipv6_enabled: false,
            localhost_only: false,
            custom_host: None,
        }
    }
    
    /// Create a new network config with custom host
    pub fn custom_host(host: &str, port: u16) -> Self {
        Self {
            bind_interface: host.to_string(),
            port,
            ipv6_enabled: false,
            localhost_only: host == DEFAULT_LOCALHOST || host == DEFAULT_IPV6_LOCALHOST,
            custom_host: Some(host.to_string()),
        }
    }
    
    /// Get the full bind address
    pub fn bind_address(&self) -> String {
        if let Some(ref custom) = self.custom_host {
            format!("{}:{}", custom, self.port)
        } else {
            format!("{}:{}", self.bind_interface, self.port)
        }
    }
    
    /// Get the interface to bind to
    pub fn interface(&self) -> &str {
        if let Some(ref custom) = self.custom_host {
            custom
        } else {
            &self.bind_interface
        }
    }
    
    /// Check if this is a secure localhost-only binding
    pub fn is_localhost_only(&self) -> bool {
        self.localhost_only || 
        self.bind_interface == DEFAULT_LOCALHOST || 
        self.bind_interface == DEFAULT_IPV6_LOCALHOST ||
        self.custom_host.as_ref().is_some_and(|h| h == DEFAULT_LOCALHOST || h == DEFAULT_IPV6_LOCALHOST)
    }
    
    /// Check if this exposes the service to external networks
    pub fn is_externally_accessible(&self) -> bool {
        !self.is_localhost_only()
    }
}

/// Runtime environment types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuntimeEnvironment {
    Development,
    Testing,
    Staging,
    Production,
}

/// Environment-based configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentConfig {
    /// Environment type (development, testing, production)
    pub environment: RuntimeEnvironment,
    /// Whether to use secure defaults
    pub secure_defaults: bool,
    /// Whether to allow external access
    pub allow_external_access: bool,
}

impl Default for EnvironmentConfig {
    fn default() -> Self {
        Self {
            environment: RuntimeEnvironment::Development,
            secure_defaults: true,
            allow_external_access: false,
        }
    }
}

impl EnvironmentConfig {
    /// Get default network config for this environment
    pub fn default_network_config(&self, service_port: u16) -> NetworkConfig {
        match (&self.environment, self.allow_external_access) {
            (RuntimeEnvironment::Development, false) => NetworkConfig::localhost(service_port),
            (RuntimeEnvironment::Testing, _) => NetworkConfig::localhost(service_port),
            (RuntimeEnvironment::Production, true) => NetworkConfig::all_interfaces(service_port),
            (RuntimeEnvironment::Production, false) => NetworkConfig::localhost(service_port),
            (RuntimeEnvironment::Staging, true) => NetworkConfig::all_interfaces(service_port),
            (RuntimeEnvironment::Staging, false) => NetworkConfig::localhost(service_port),
            (RuntimeEnvironment::Development, true) => {
                // Allow external access in development if explicitly requested
                NetworkConfig::all_interfaces(service_port)
            }
        }
    }
} 