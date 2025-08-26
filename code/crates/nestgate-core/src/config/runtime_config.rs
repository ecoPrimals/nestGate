use std::collections::HashMap;
//
// Provides dynamic configuration management that eliminates hardcoded values
// by reading from environment variables with intelligent defaults.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::net::SocketAddr;
use std::str::FromStr;

/// Runtime configuration manager that eliminates hardcoded values
#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    /// Network configuration
    pub network: NetworkRuntimeConfig,
    /// Service configuration  
    pub service: ServiceRuntimeConfig,
    /// Storage configuration
    pub storage: StorageRuntimeConfig,
    /// Security configuration
    pub security: SecurityRuntimeConfig,
}

/// Network runtime configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkRuntimeConfig {
    /// Primary API port (from NESTGATE_API_PORT or intelligent default)
    pub api_port: u16,
    /// WebSocket port (from NESTGATE_WS_PORT or api_port + 1)
    pub websocket_port: u16,
    /// Health check port (from NESTGATE_HEALTH_PORT or api_port + 2)
    pub health_port: u16,
    /// Metrics port (from NESTGATE_METRICS_PORT or api_port + 3)
    pub metrics_port: u16,
    /// Bind address (from NESTGATE_BIND_ADDRESS or secure default)
    pub bind_address: String,
    /// External hostname (from NESTGATE_HOSTNAME or localhost)
    pub hostname: String,
    /// Custom service endpoints
    pub service_endpoints: HashMap<String, String>,
}

/// Service runtime configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRuntimeConfig {
    /// Service ID (from NESTGATE_SERVICE_ID or generated UUID)
    pub service_id: String,
    /// Service name (from NESTGATE_SERVICE_NAME or default)
    pub service_name: String,
    /// Environment (from NESTGATE_ENV or "development")
    pub environment: String,
    /// Log level (from NESTGATE_LOG_LEVEL or "info")
    pub log_level: String,
}

/// Storage runtime configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageRuntimeConfig {
    /// ZFS backend (from NESTGATE_ZFS_BACKEND or "auto")
    pub zfs_backend: String,
    /// Storage root path (from NESTGATE_STORAGE_PATH or "/var/lib/nestgate")
    pub storage_path: String,
    /// Temporary directory (from NESTGATE_TEMP_DIR or "/tmp/nestgate")
    pub temp_dir: String,
}

/// Security runtime configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRuntimeConfig {
    /// Whether to bind only to localhost (from NESTGATE_LOCALHOST_ONLY or true)
    pub localhost_only: bool,
    /// API key (from NESTGATE_API_KEY or none)
    pub api_key: Option<String>,
    /// TLS enabled (from NESTGATE_TLS_ENABLED or false)
    pub tls_enabled: bool,
}

impl RuntimeConfig {
    /// Create configuration from environment with intelligent defaults
    pub fn from_environment() -> Self {
        let network = NetworkRuntimeConfig::from_environment();
        let service = ServiceRuntimeConfig::from_environment();
        let storage = StorageRuntimeConfig::from_environment();
        let security = SecurityRuntimeConfig::from_environment();

        Self {
            network,
            service,
            storage,
            security,
        }
    }

    /// Get the primary API socket address
    pub fn api_socket_addr(&self) -> SocketAddr {
        let addr = if self.security.localhost_only {
            format!("127.0.0.1:{}", self.network.api_port)
        } else {
            format!("{}:{}", self.network.bind_address, self.network.api_port)
        };

        SocketAddr::from_str(&addr).unwrap_or_else(|_| {
            SocketAddr::from_str("127.0.0.1:8000")
                .unwrap_or_else(|_| SocketAddr::from(([127, 0, 0, 1], 8000)))
        })
    }

    /// Get service endpoint URL
    pub fn service_endpoint(&self, service_type: &str) -> String {
        if let Some(endpoint) = self.network.service_endpoints.get(service_type) {
            endpoint.clone()
        } else {
            // Generate dynamic endpoint based on service type
            let port = self.dynamic_port_for_service(service_type);
            format!("http://{}:{}", self.network.hostname, port)
        }
    }

    /// Generate dynamic port for service type (eliminates hardcoding)
    fn dynamic_port_for_service(&self, service_type: &str) -> u16 {
        // Use deterministic but non-hardcoded port assignment
        let base_port = self.network.api_port;
        let service_offset = match service_type {
            "api" => 0,
            "websocket" => 1,
            "health" => 2,
            "metrics" => 3,
            "storage" => 4,
            "security" => 5,
            "orchestration" => 6,
            "compute" => 7,
            _ => {
                // For unknown services, use a hash-based offset
                let hash = service_type
                    .bytes()
                    .fold(0u16, |acc, b| acc.wrapping_add(b as u16));
                10 + (hash % 100) // Offset 10-109 for custom services
            }
        };
        base_port + service_offset
    }

    /// Check if running in production mode
    pub fn is_production(&self) -> bool {
        matches!(self.service.environment.as_str(), "production" | "prod")
    }

    /// Check if running in development mode
    pub fn is_development(&self) -> bool {
        matches!(self.service.environment.as_str(), "development" | "dev")
    }
}

impl NetworkRuntimeConfig {
    fn from_environment() -> Self {
        // Get base API port from environment or use intelligent default
        let api_port = env::var("NESTGATE_API_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .or_else(|| env::var("PORT").ok().and_then(|s| s.parse().ok()))
            .unwrap_or_else(|| {
                // Intelligent default: use 8000 for production, 8080 for development
                let env_type =
                    env::var("NESTGATE_ENV").unwrap_or_else(|_| "development".to_string());
                if env_type == "production" {
                    8000
                } else {
                    8080
                }
            });

        let websocket_port = env::var("NESTGATE_WS_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(api_port + 1);

        let health_port = env::var("NESTGATE_HEALTH_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(api_port + 2);

        let metrics_port = env::var("NESTGATE_METRICS_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(api_port + 3);

        let bind_address = env::var("NESTGATE_BIND_ADDRESS").unwrap_or_else(|_| {
            // Secure default: localhost only unless explicitly configured
            if env::var("NESTGATE_ALLOW_EXTERNAL").is_ok() {
                "0.0.0.0".to_string()
            } else {
                "127.0.0.1".to_string()
            }
        });

        let hostname = env::var("NESTGATE_HOSTNAME").unwrap_or_else(|_| "localhost".to_string());

        // Load custom service endpoints from environment
        let mut service_endpoints = HashMap::new();
        for (key, value) in env::vars() {
            if let Some(service_type) = key.strip_prefix("NESTGATE_SERVICE_") {
                if let Some(endpoint) = value.strip_suffix("_ENDPOINT") {
                    service_endpoints.insert(service_type.to_lowercase(), endpoint.to_string());
                }
            }
        }

        Self {
            api_port,
            websocket_port,
            health_port,
            metrics_port,
            bind_address,
            hostname,
            service_endpoints,
        }
    }
}

impl ServiceRuntimeConfig {
    fn from_environment() -> Self {
        let service_id = env::var("NESTGATE_SERVICE_ID")
            .unwrap_or_else(|_| format!("nestgate-{}", uuid::Uuid::new_v4().simple()));

        let service_name =
            env::var("NESTGATE_SERVICE_NAME").unwrap_or_else(|_| "nestgate".to_string());

        let environment = env::var("NESTGATE_ENV")
            .or_else(|_| env::var("NODE_ENV"))
            .or_else(|_| env::var("RUST_ENV"))
            .unwrap_or_else(|_| "development".to_string());

        let log_level = env::var("NESTGATE_LOG_LEVEL")
            .or_else(|_| env::var("RUST_LOG"))
            .unwrap_or_else(|_| {
                if environment == "production" {
                    "warn".to_string()
                } else {
                    "info".to_string()
                }
            });

        Self {
            service_id,
            service_name,
            environment,
            log_level,
        }
    }
}

impl StorageRuntimeConfig {
    fn from_environment() -> Self {
        let zfs_backend = env::var("NESTGATE_ZFS_BACKEND").unwrap_or_else(|_| "auto".to_string());

        let storage_path =
            env::var("NESTGATE_STORAGE_PATH").unwrap_or_else(|_| "/var/lib/nestgate".to_string());

        let temp_dir = env::var("NESTGATE_TEMP_DIR")
            .or_else(|_| env::var("TMPDIR"))
            .unwrap_or_else(|_| "/tmp/nestgate".to_string());

        Self {
            zfs_backend,
            storage_path,
            temp_dir,
        }
    }
}

impl SecurityRuntimeConfig {
    fn from_environment() -> Self {
        let localhost_only = env::var("NESTGATE_LOCALHOST_ONLY")
            .map(|s| s.parse().unwrap_or(true))
            .unwrap_or_else(|_| {
                // Default to secure: localhost only unless explicitly configured
                env::var("NESTGATE_ALLOW_EXTERNAL").is_err()
            });

        let api_key = env::var("NESTGATE_API_KEY").ok();

        let tls_enabled = env::var("NESTGATE_TLS_ENABLED")
            .map(|s| s.parse().unwrap_or(false))
            .unwrap_or(false);

        Self {
            localhost_only,
            api_key,
            tls_enabled,
        }
    }
}

/// Global runtime configuration instance
static RUNTIME_CONFIG: std::sync::OnceLock<RuntimeConfig> = std::sync::OnceLock::new();

/// Initialize the global runtime configuration
pub fn init_runtime_config() {
    RUNTIME_CONFIG.get_or_init(RuntimeConfig::from_environment);
}

/// Get the global runtime configuration
pub fn get_runtime_config() -> &'static RuntimeConfig {
    RUNTIME_CONFIG.get_or_init(RuntimeConfig::from_environment)
}

/// Get a dynamic port for a service (eliminates hardcoding)
pub fn get_service_port(service_type: &str) -> u16 {
    get_runtime_config().dynamic_port_for_service(service_type)
}

/// Get a service endpoint URL (eliminates hardcoded URLs)
pub fn get_service_endpoint(service_type: &str) -> String {
    get_runtime_config().service_endpoint(service_type)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_dynamic_port_assignment() {
        let config = RuntimeConfig::from_environment();

        // Test deterministic port assignment
        assert_eq!(
            config.dynamic_port_for_service("api"),
            config.network.api_port
        );
        assert_eq!(
            config.dynamic_port_for_service("websocket"),
            config.network.api_port + 1
        );
        assert_eq!(
            config.dynamic_port_for_service("health"),
            config.network.api_port + 2
        );
        assert_eq!(
            config.dynamic_port_for_service("metrics"),
            config.network.api_port + 3
        );
    }

    #[test]
    fn test_environment_override() {
        env::set_var("NESTGATE_API_PORT", "9000");
        let config = NetworkRuntimeConfig::from_environment();
        assert_eq!(config.api_port, 9000);
        env::remove_var("NESTGATE_API_PORT");
    }

    #[test]
    fn test_secure_defaults() {
        env::remove_var("NESTGATE_ALLOW_EXTERNAL");
        let config = NetworkRuntimeConfig::from_environment();
        assert_eq!(config.bind_address, "127.0.0.1");
    }
}
