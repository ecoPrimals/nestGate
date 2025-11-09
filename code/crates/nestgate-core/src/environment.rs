use std::collections::HashMap;
use std::env;

/// Environment detection and configuration for NestGate
///
/// Supports two primary modes:
/// 1. **Orchestration-Enhanced**: All networking, ports, and service discovery handled by orchestration module
/// 2. **Standalone**: Self-contained operation with direct network protocols
///
/// The system automatically detects the environment and configures itself accordingly.
/// **CANONICAL MODERNIZATION** - Use canonical service configuration
pub use crate::canonical_types::service::ServiceConfig;

/// Operating mode for NestGate
#[derive(Debug, Clone, PartialEq)]
pub enum OperationMode {
    /// Standalone mode: Direct networking, self-contained
    Standalone,
    /// Orchestration-enhanced mode: Orchestration module handles networking
    OrchestrationEnhanced,
}

/// Environment detection and configuration
///
/// **MODERNIZED**: Simplified structure - removed deprecated NetworkConfig
/// For comprehensive network configuration, use `config::canonical_primary::domains::network::CanonicalNetworkConfig`
#[derive(Debug, Clone)]
pub struct Environment {
    /// Current operation mode
    pub mode: OperationMode,
    /// Service configuration
    pub service: ServiceConfig,
    /// Network bind interface (127.0.0.1 for standalone, 0.0.0.0 for orchestration)
    pub bind_interface: String,
    /// Network port
    pub port: u16,
    /// Service name for orchestration mode
    pub service_name: String,
    /// Enable service discovery
    pub discovery_enabled: bool,
    /// External service endpoints
    pub external_services: HashMap<String, String>,
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
        let (bind_interface, port, service_name, discovery_enabled) =
            Self::detect_network_settings(&mode);
        let external_services = Self::detect_external_services(&mode);

        Self {
            mode,
            service,
            bind_interface,
            port,
            service_name,
            discovery_enabled,
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
        use crate::canonical_types::service::{ServiceId, ServiceState, ServiceType};
        let mut metadata = std::collections::HashMap::new();
        metadata.insert(
            "environment".to_string(),
            env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
        );

        ServiceConfig {
            id: ServiceId::default(),
            name: env::var("SERVICE_NAME").unwrap_or_else(|_| "nestgate".to_string()),
            service_type: ServiceType::Api,
            state: ServiceState::Running,
            port: None,
            host: None,
            metadata,
            created_at: std::time::SystemTime::now(),
            updated_at: std::time::SystemTime::now(),
        }
    }

    /// Detect network settings based on mode
    /// Returns: (bind_interface, port, service_name, discovery_enabled)
    fn detect_network_settings(mode: &OperationMode) -> (String, u16, String, bool) {
        let bind_interface = match mode {
            OperationMode::Standalone => {
                env::var("NESTGATE_BIND_INTERFACE").unwrap_or_else(|_| "127.0.0.1".to_string())
            } // Sovereignty-compliant default
            OperationMode::OrchestrationEnhanced => {
                env::var("NESTGATE_BIND_INTERFACE").unwrap_or_else(|_| "0.0.0.0".to_string())
            }
        };

        let port = env::var("NESTGATE_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .unwrap_or(8080);

        let service_name =
            env::var("NESTGATE_SERVICE_NAME").unwrap_or_else(|_| "nestgate".to_string());

        let discovery_enabled = match mode {
            OperationMode::Standalone => env::var("NESTGATE_DISCOVERY_ENABLED")
                .map(|v| v.parse().unwrap_or(false))
                .unwrap_or(false),
            OperationMode::OrchestrationEnhanced => env::var("NESTGATE_DISCOVERY_ENABLED")
                .map(|v| v.parse().unwrap_or(true))
                .unwrap_or(true), // Enable discovery in orchestration mode
        };

        (bind_interface, port, service_name, discovery_enabled)
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
