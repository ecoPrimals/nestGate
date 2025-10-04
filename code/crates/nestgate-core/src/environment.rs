use std::collections::HashMap;
/// Environment detection and configuration for NestGate
///
/// Supports two primary modes:
/// 1. **Orchestration-Enhanced**: All networking, ports, and service discovery handled by orchestration module
/// 2. **Standalone**: Self-contained operation with direct network protocols
///
/// The system automatically detects the environment and configures itself accordingly.
use std::env;
/// Operating mode for NestGate
#[derive(Debug, Clone, PartialEq)]
pub enum OperationMode {
    /// Standalone mode: Direct networking, self-contained
    Standalone,
    /// Orchestration-enhanced mode: Orchestration module handles networking
    OrchestrationEnhanced,
}
/// Environment detection and configuration
#[derive(Debug, Clone)]
pub struct Environment {
    /// Current operation mode
    pub mode: OperationMode,
    /// Service configuration
    pub service: ServiceConfig,
    /// Network configuration
    pub network: NetworkConfig,
    /// External service endpoints
    pub external_services: HashMap<String, String>,
}
/// **CANONICAL MODERNIZATION** - Use canonical service configuration
pub use crate::canonical_types::service::ServiceConfig;
/// Network configuration
/// **⚠️ DEPRECATED**: Use `CanonicalNetworkConfig` from `canonical_master::domains::network`
#[deprecated(since = "0.9.0", note = "Use canonical_master::domains::network::CanonicalNetworkConfig instead")]
#[derive(Debug, Clone)]
pub struct NetworkConfig {
    /// Bind interface (127.0.0.1 for standalone, delegated for orchestration)
    pub bind_interface: String,
    /// Port (0 for auto in orchestration, configured for standalone)
    pub port: u16,
    /// Service name for orchestration mode
    pub service_name: String,
    /// Enable service discovery
    pub discovery_enabled: bool,
}
impl Default for Environment {
    fn default() -> Self {
        Self::detect()
    }
}

impl Environment {
    /// Detect the current environment and configure accordingly
    pub fn detect() -> Self {
        let mode = Self::detect_mode();
        let service = Self::detect_service_config();
        let network = Self::detect_network_config(&mode);
        let external_services = Self::detect_external_services(&mode);

        Self {
            mode,
            service,
            network,
            external_services,
        }
    }

    /// Detect operation mode based on environment variables
    fn detect_mode() -> OperationMode {
        // Check for orchestration module URL
        if env::var("ORCHESTRATION_URL").is_ok() {
            OperationMode::OrchestrationEnhanced
        } else {
            OperationMode::Standalone
        }
    }

    /// Detect service configuration
    fn detect_service_config() -> ServiceConfig {
        ServiceConfig {
            name: env::var("SERVICE_NAME")
                .unwrap_or_else(|_| crate::canonical_modernization::canonical_constants::strings::DEFAULT_SERVICE_NAME.to_string()),
            version: env::var("SERVICE_VERSION")
                .unwrap_or_else(|_| crate::canonical_modernization::canonical_constants::strings::DEFAULT_SERVICE_VERSION.to_string()),
            description: env::var("SERVICE_DESCRIPTION").unwrap_or_else(|_| {
                crate::canonical_modernization::canonical_constants::strings::DEFAULT_SERVICE_DESCRIPTION.to_string()
            }),
        }
    }

    /// Detect network configuration based on mode
    fn detect_network_config(mode: &OperationMode) -> NetworkConfig {
        match mode {
            OperationMode::Standalone => NetworkConfig {
                bind_interface: env::var("NESTGATE_BIND_INTERFACE").unwrap_or_else(|_| {
                    crate::sovereignty_config::migration_helpers::get_bind_address().to_string()
                }), // Sovereignty-compliant default
                port: env::var("NESTGATE_PORT")
                    .unwrap_or_else(|_| {
                        crate::sovereignty_config::migration_helpers::get_api_port().to_string()
                    })
                    .parse()
                    .unwrap_or(8080),
                service_name: env::var("NESTGATE_SERVICE_NAME").unwrap_or_else(|_| {
                    crate::sovereignty_config::migration_helpers::get_service_name()
                }),
                discovery_enabled: env::var("NESTGATE_DISCOVERY_ENABLED")
                    .map(|v| v.parse().unwrap_or(false))
                    .unwrap_or(false),
            },
            OperationMode::OrchestrationEnhanced => NetworkConfig {
                bind_interface: env::var("NESTGATE_BIND_INTERFACE").unwrap_or_else(|_| {
                    crate::sovereignty_config::migration_helpers::get_bind_address().to_string()
                }), // Sovereignty-compliant default
                port: env::var("NESTGATE_PORT")
                    .unwrap_or_else(|_| {
                        crate::sovereignty_config::migration_helpers::get_api_port().to_string()
                    }) // Use proper port instead of 0
                    .parse()
                    .unwrap_or(8080),
                service_name: env::var("NESTGATE_SERVICE_NAME").unwrap_or_else(|_| {
                    crate::sovereignty_config::migration_helpers::get_service_name()
                }),
                discovery_enabled: env::var("NESTGATE_DISCOVERY_ENABLED")
                    .map(|v| v.parse().unwrap_or(true))
                    .unwrap_or(true), // Enable discovery in orchestration mode
            },
        }
    }

    /// Detect external service endpoints
    fn detect_external_services(mode: &OperationMode) -> HashMap<String, String> {
        let mut external_services = HashMap::new();

        // Only populate external services in orchestration mode
        match mode {
            OperationMode::Standalone => {
                // In standalone mode, no external services are required
            }
            OperationMode::OrchestrationEnhanced => {
                // Check for orchestration module URL
                if let Ok(orchestration_url) = env::var("ORCHESTRATION_URL") {
                    external_services.insert("orchestration".to_string(), orchestration_url);
                }

                // Check for security module URL
                if let Ok(security_url) = env::var("SECURITY_URL") {
                    external_services.insert("security".to_string(), security_url);
                }

                // Check for AI module URL
                if let Ok(ai_url) = env::var("AI_URL") {
                    external_services.insert("ai".to_string(), ai_url);
                }

                // Check for compute module URL
                if let Ok(compute_url) = env::var("COMPUTE_URL") {
                    external_services.insert("compute".to_string(), compute_url);
                }
            }
        }

        external_services
    }
}
