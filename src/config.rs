//! Configuration types for the orchestrator

use serde::{Deserialize, Serialize};
use uuid;

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
        // Check if we're in Songbird mode or standalone mode
        let songbird_mode = std::env::var("SONGBIRD_URL").is_ok();
        
        if songbird_mode {
            // Songbird-enhanced mode: use service names
            Self {
                bind_interface: std::env::var("SONGBIRD_SERVICE_NAME")
                    .unwrap_or_else(|_| "nestgate-service".to_string()),
                port: 0, // Let Songbird allocate
                ipv6_enabled: false,
                localhost_only: false, // Songbird handles security
                custom_host: None,
            }
        } else {
            // Standalone mode: use localhost binding
            Self {
                bind_interface: "127.0.0.1".to_string(), // ✅ LOCALHOST FOR STANDALONE
                port: std::env::var("NESTGATE_PORT")
                    .unwrap_or_else(|_| "8080".to_string())
                    .parse()
                    .unwrap_or(8080),
                ipv6_enabled: false,
                localhost_only: true, // ✅ SECURE BY DEFAULT
                custom_host: None,
            }
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
    /// Get default network configuration for this environment
    pub fn default_network_config(&self, service_port: u16) -> NetworkConfig {
        // Check if we're in Songbird mode
        let songbird_mode = std::env::var("SONGBIRD_URL").is_ok();
        
        if songbird_mode {
            // Songbird-enhanced mode: service-based addressing
            NetworkConfig {
                bind_interface: std::env::var("SONGBIRD_SERVICE_NAME")
                    .unwrap_or_else(|_| format!("nestgate-{}", uuid::Uuid::new_v4().to_string()[..8].to_string())),
                port: 0, // Always let Songbird allocate
                ipv6_enabled: false,
                localhost_only: false, // Songbird handles security
                custom_host: None,
            }
        } else {
            // Standalone mode: environment-appropriate binding
            match (&self.environment, self.allow_external_access) {
                (RuntimeEnvironment::Development, false) => NetworkConfig {
                    bind_interface: "127.0.0.1".to_string(),
                    port: service_port,
                    ipv6_enabled: false,
                    localhost_only: true,
                    custom_host: None,
                },
                (RuntimeEnvironment::Production, true) => NetworkConfig {
                    bind_interface: "0.0.0.0".to_string(), // Allow external in production
                    port: service_port,
                    ipv6_enabled: false,
                    localhost_only: false,
                    custom_host: None,
                },
                _ => NetworkConfig {
                    bind_interface: "127.0.0.1".to_string(), // Default to secure
                    port: service_port,
                    ipv6_enabled: false,
                    localhost_only: true,
                    custom_host: None,
                },
            }
        }
    }
} 