use serde::{Deserialize, Serialize};
/// Dual-Mode Environment Configuration for NestGate
///
/// NestGate operates in two modes:
/// 1. **Songbird-Enhanced**: All networking, ports, and service discovery handled by Songbird
/// 2. **Standalone**: Simple local/LAN NAS with secure, configurable defaults
///
/// This eliminates hardcoded values while keeping the configuration simple and safe.
use std::collections::HashMap;
use std::env;

/// NestGate operation modes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OperationMode {
    /// Songbird-enhanced mode: Orchestrator handles networking
    SongbirdEnhanced,
    /// Standalone mode: Local/LAN NAS operation
    Standalone,
}

/// Security environments
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SecurityEnvironment {
    /// Development: Relaxed security for local development
    Development,
    /// Production: Maximum security for real deployments
    Production,
}

/// Complete environment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentConfig {
    /// Operation mode
    pub mode: OperationMode,
    /// Security environment
    pub security_env: SecurityEnvironment,
    /// Network configuration
    pub network: NetworkConfig,
    /// Service endpoints
    pub endpoints: EndpointConfig,
    /// Security settings
    pub security: SecurityConfig,
}

/// Network configuration for dual modes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Bind interface (127.0.0.1 for standalone, delegated for Songbird)
    pub bind_interface: String,
    /// Port (0 for auto in Songbird, configured for standalone)
    pub port: u16,
    /// Service name for Songbird mode
    pub service_name: Option<String>,
    /// TLS required
    pub tls_enabled: bool,
}

impl Default for EnvironmentConfig {
    fn default() -> Self {
        Self::from_environment()
    }
}

impl EnvironmentConfig {
    /// Create configuration from environment detection
    pub fn from_environment() -> Self {
        let mode = Self::detect_operation_mode();
        let security_env = Self::detect_security_environment();

        Self {
            network: NetworkConfig::for_mode(&mode, &security_env),
            endpoints: EndpointConfig::for_mode(&mode, &security_env),
            security: SecurityConfig::for_security_env(&security_env),
            mode,
            security_env,
        }
    }

    /// Detect operation mode from environment
    fn detect_operation_mode() -> OperationMode {
        if env::var("SONGBIRD_URL").is_ok() {
            OperationMode::SongbirdEnhanced
        } else {
            OperationMode::Standalone
        }
    }

    /// Detect security environment
    fn detect_security_environment() -> SecurityEnvironment {
        match env::var("NESTGATE_ENVIRONMENT").as_deref() {
            Ok("production") | Ok("prod") => SecurityEnvironment::Production,
            Ok("development") | Ok("dev") => SecurityEnvironment::Development,
            _ => {
                // Auto-detect: production if we see production indicators
                if env::var("KUBERNETES_SERVICE_HOST").is_ok()
                    || env::var("NODE_ENV")
                        .map(|v| v == "production")
                        .unwrap_or(false)
                {
                    SecurityEnvironment::Production
                } else {
                    SecurityEnvironment::Development
                }
            }
        }
    }

    /// Get the bind address for the main service
    pub fn get_bind_address(&self) -> String {
        match self.mode {
            OperationMode::SongbirdEnhanced => {
                format!("0.0.0.0:{}", self.network.port)
            }
            OperationMode::Standalone => {
                format!("{}:{}", self.network.bind_interface, self.network.port)
            }
        }
    }

    /// Get API endpoint URL
    pub fn get_api_url(&self) -> String {
        match self.mode {
            OperationMode::SongbirdEnhanced => {
                env::var("NESTGATE_API_URL").unwrap_or_else(|_| "http://nestgate-api".to_string())
            }
            OperationMode::Standalone => {
                let protocol = if self.network.tls_enabled {
                    "https"
                } else {
                    "http"
                };
                let host = env::var("NESTGATE_HOST")
                    .unwrap_or_else(|_| self.network.bind_interface.clone());
                format!("{}://{}:{}", protocol, host, self.network.port)
            }
        }
    }
}

impl NetworkConfig {
    fn for_mode(mode: &OperationMode, security_env: &SecurityEnvironment) -> Self {
        match mode {
            OperationMode::SongbirdEnhanced => Self {
                bind_interface: "0.0.0.0".to_string(),
                port: 0,
                service_name: Some(
                    env::var("NESTGATE_SERVICE_NAME").unwrap_or_else(|_| "nestgate".to_string()),
                ),
                tls_enabled: matches!(security_env, SecurityEnvironment::Production),
            },
            OperationMode::Standalone => {
                let bind_interface =
                    env::var("NESTGATE_BIND_INTERFACE").unwrap_or_else(|_| match security_env {
                        SecurityEnvironment::Development => "127.0.0.1".to_string(),
                        SecurityEnvironment::Production => env::var("NESTGATE_PRODUCTION_BIND")
                            .unwrap_or_else(|_| "127.0.0.1".to_string()),
                    });

                let port = env::var("NESTGATE_PORT")
                    .unwrap_or_else(|_| "8080".to_string())
                    .parse()
                    .unwrap_or(8080);

                Self {
                    bind_interface,
                    port,
                    service_name: None,
                    tls_enabled: matches!(security_env, SecurityEnvironment::Production),
                }
            }
        }
    }
}

/// Service endpoint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointConfig {
    /// External service endpoints (only in Songbird mode)
    pub external_services: HashMap<String, String>,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Authentication required
    pub auth_required: bool,
    /// Allowed IP ranges
    pub allowed_ip_ranges: Vec<String>,
    /// CORS origins for web API
    pub cors_origins: Vec<String>,
    /// Rate limiting
    pub rate_limit_per_ip: u32,
    /// Allowed networks
    pub allowed_networks: Vec<String>,
}

impl EndpointConfig {
    fn for_mode(mode: &OperationMode, _security_env: &SecurityEnvironment) -> Self {
        match mode {
            OperationMode::SongbirdEnhanced => {
                let mut external_services = HashMap::new();

                if let Ok(songbird_url) = env::var("SONGBIRD_URL") {
                    external_services.insert("songbird".to_string(), songbird_url);
                }

                if let Ok(beardog_url) = env::var("BEARDOG_URL") {
                    external_services.insert("beardog".to_string(), beardog_url);
                }

                Self { external_services }
            }
            OperationMode::Standalone => Self {
                external_services: HashMap::new(),
            },
        }
    }
}

impl SecurityConfig {
    fn for_security_env(security_env: &SecurityEnvironment) -> Self {
        match security_env {
            SecurityEnvironment::Development => Self {
                auth_required: false,
                allowed_ip_ranges: vec![
                    "127.0.0.1/32".to_string(),
                    "::1/128".to_string(),
                    std::env::var("NESTGATE_DEV_NETWORK_1")
                        .unwrap_or_else(|_| "192.168.0.0/16".to_string()),
                    std::env::var("NESTGATE_DEV_NETWORK_2")
                        .unwrap_or_else(|_| "10.0.0.0/8".to_string()),
                ],
                cors_origins: vec!["*".to_string()],
                rate_limit_per_ip: 1000,
                allowed_networks: vec![
                    std::env::var("NESTGATE_LAST_ALLOWED_NETWORK_1")
                        .unwrap_or_else(|_| "192.168.0.0/16".to_string()),
                    std::env::var("NESTGATE_LAST_ALLOWED_NETWORK_2")
                        .unwrap_or_else(|_| "10.0.0.0/8".to_string()),
                ],
            },
            SecurityEnvironment::Production => Self {
                auth_required: true,
                allowed_ip_ranges: env::var("NESTGATE_ALLOWED_IPS")
                    .map(|ips| ips.split(',').map(|s| s.trim().to_string()).collect())
                    .unwrap_or_else(|_| vec!["127.0.0.1/32".to_string()]),
                cors_origins: env::var("NESTGATE_CORS_ORIGINS")
                    .map(|origins| origins.split(',').map(|s| s.trim().to_string()).collect())
                    .unwrap_or_else(|_| vec![]),
                rate_limit_per_ip: 100,
                allowed_networks: vec![
                    std::env::var("NESTGATE_LAST_ALLOWED_NETWORK_1")
                        .unwrap_or_else(|_| "192.168.0.0/16".to_string()),
                    std::env::var("NESTGATE_LAST_ALLOWED_NETWORK_2")
                        .unwrap_or_else(|_| "10.0.0.0/8".to_string()),
                ],
            },
        }
    }
}

/// Environment variable constants
pub mod env_vars {
    pub const ENVIRONMENT: &str = "NESTGATE_ENVIRONMENT";
    pub const SERVICE_NAME: &str = "NESTGATE_SERVICE_NAME";
    pub const BIND_INTERFACE: &str = "NESTGATE_BIND_INTERFACE";
    pub const PORT: &str = "NESTGATE_PORT";
    pub const HOST: &str = "NESTGATE_HOST";
    pub const PRODUCTION_BIND: &str = "NESTGATE_PRODUCTION_BIND";
    pub const ALLOWED_IPS: &str = "NESTGATE_ALLOWED_IPS";
    pub const CORS_ORIGINS: &str = "NESTGATE_CORS_ORIGINS";
    pub const API_URL: &str = "NESTGATE_API_URL";
    pub const SONGBIRD_URL: &str = "SONGBIRD_URL";
    pub const BEARDOG_URL: &str = "BEARDOG_URL";
    pub const ALLOWED_NETWORKS: &str = "NESTGATE_ALLOWED_NETWORKS";
}

/// Configuration helpers
impl EnvironmentConfig {
    /// Create development standalone configuration
    pub fn development_standalone() -> Self {
        Self {
            mode: OperationMode::Standalone,
            security_env: SecurityEnvironment::Development,
            network: NetworkConfig {
                bind_interface: "127.0.0.1".to_string(),
                port: crate::constants::network::api_port(),
                service_name: None,
                tls_enabled: false,
            },
            endpoints: EndpointConfig {
                external_services: HashMap::new(),
            },
            security: SecurityConfig {
                auth_required: false,
                allowed_ip_ranges: vec!["127.0.0.1/32".to_string()],
                cors_origins: vec!["*".to_string()],
                rate_limit_per_ip: 1000,
                allowed_networks: vec![
                    std::env::var("NESTGATE_TESTING_ALLOWED_NETWORK_1")
                        .unwrap_or_else(|_| "192.168.0.0/16".to_string()),
                    std::env::var("NESTGATE_TESTING_ALLOWED_NETWORK_2")
                        .unwrap_or_else(|_| "10.0.0.0/8".to_string()),
                ],
            },
        }
    }

    /// Create production standalone configuration
    pub fn production_standalone(bind_interface: Option<String>) -> Self {
        Self {
            mode: OperationMode::Standalone,
            security_env: SecurityEnvironment::Production,
            network: NetworkConfig {
                bind_interface: bind_interface.unwrap_or_else(|| "127.0.0.1".to_string()),
                port: crate::constants::network::api_port(),
                service_name: None,
                tls_enabled: true,
            },
            endpoints: EndpointConfig {
                external_services: HashMap::new(),
            },
            security: SecurityConfig {
                auth_required: true,
                allowed_ip_ranges: vec!["127.0.0.1/32".to_string()],
                cors_origins: vec![],
                rate_limit_per_ip: 100,
                allowed_networks: vec![
                    std::env::var("NESTGATE_PRODUCTION_NETWORK_1")
                        .unwrap_or_else(|_| "192.168.0.0/16".to_string()),
                    std::env::var("NESTGATE_PRODUCTION_NETWORK_2")
                        .unwrap_or_else(|_| "10.0.0.0/8".to_string()),
                ],
            },
        }
    }

    /// Check if running in Songbird mode
    pub fn is_songbird_mode(&self) -> bool {
        matches!(self.mode, OperationMode::SongbirdEnhanced)
    }

    /// Check if running in standalone mode
    pub fn is_standalone_mode(&self) -> bool {
        matches!(self.mode, OperationMode::Standalone)
    }

    /// Check if in production security environment
    pub fn is_production(&self) -> bool {
        matches!(self.security_env, SecurityEnvironment::Production)
    }

    /// Get the AI model cache directory for this environment
    pub fn get_ai_model_cache_dir(&self) -> String {
        if self.is_songbird_mode() {
            "/mnt/nestgate/ai-models".to_string()
        } else {
            match self.security_env {
                SecurityEnvironment::Development => "/tmp/nestgate/ai-models".to_string(),
                SecurityEnvironment::Production => "/var/cache/nestgate/ai-models".to_string(),
            }
        }
    }

    /// Get the share root directory for this environment
    pub fn get_share_root(&self) -> String {
        if self.is_songbird_mode() {
            "/mnt/nestgate/shares".to_string()
        } else {
            match self.security_env {
                SecurityEnvironment::Development => "/tmp/nestgate/shares".to_string(),
                SecurityEnvironment::Production => "/srv/nestgate/shares".to_string(),
            }
        }
    }

    /// Get the discovery ports for this environment
    pub fn get_discovery_ports(&self) -> Vec<u16> {
        if self.is_songbird_mode() {
            // In Songbird mode, use service mesh ports
            vec![80, 443, 8080, 8443]
        } else {
            // In standalone mode, use local ports
            vec![8080, 3000, 3001, 8000, 9000]
        }
    }
}
