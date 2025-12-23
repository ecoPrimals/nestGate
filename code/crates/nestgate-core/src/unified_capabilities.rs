//! Unified Capability System - Bridge Between Discovery and Application Capabilities
//!
//! **ARCHITECTURAL EVOLUTION**: This module unifies two capability systems:
//! 1. `capabilities::discovery::Capability` - Application-level capabilities (Storage, Compute, etc.)
//! 2. `universal_primal_discovery::PrimalCapability` - Discovery-level capabilities (Networking, Security, etc.)
//!
//! **PHILOSOPHY**:
//! - One capability type for the entire system
//! - Application capabilities map to primal capabilities
//! - Enables both fine-grained and coarse-grained discovery
//!
//! # Example
//!
//! ```rust,ignore
//! use nestgate_core::unified_capabilities::{UnifiedCapability, CapabilityMapper};
//!
//! // Application asks for "Storage"
//! let app_capability = UnifiedCapability::Storage;
//!
//! // Maps to primal capability for discovery
//! let primal_cap = CapabilityMapper::to_primal(&app_capability);
//!
//! // Discover service
//! let service = registry.find_by_capability(&primal_cap).await?;
//! ```

use crate::capabilities::discovery::Capability as AppCapability;
use crate::universal_primal_discovery::capability_based_discovery::PrimalCapability;
use std::fmt;

/// Unified capability type that bridges application and discovery layers
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnifiedCapability {
    // Storage capabilities
    /// General storage capability
    Storage,
    /// ZFS management and pool operations
    ZfsManagement,
    /// Object storage (S3-compatible)
    ObjectStorage,
    /// Block-level storage access
    BlockStorage,
    /// File-level storage access
    FileStorage,

    // Networking capabilities
    /// General networking capability
    Networking,
    /// HTTP API endpoints
    HttpApi,
    /// gRPC services
    Grpc,
    /// WebSocket connections
    Websocket,
    /// MQTT messaging
    Mqtt,

    // Compute capabilities
    /// General compute capability
    Compute,
    /// Task execution services
    TaskExecution,
    /// Service orchestration
    Orchestration,
    /// Job scheduling
    Scheduling,

    // Security capabilities
    /// General security capability
    Security,
    /// Authentication services
    Authentication,
    /// Authorization and access control
    Authorization,
    /// Encryption services
    Encryption,
    /// Secret management (e.g., vault)
    SecretManagement,

    // AI/ML capabilities
    /// Artificial intelligence services
    ArtificialIntelligence,
    /// ML model serving
    ModelServing,
    /// Model training
    Training,
    /// Model inference
    Inference,

    // Observability capabilities
    /// General monitoring capability
    Monitoring,
    /// Metrics collection
    Metrics,
    /// Distributed tracing
    Tracing,
    /// Logging services
    Logging,
    /// Alerting and notifications
    Alerting,

    // Ecosystem capabilities
    /// Service discovery mechanisms
    ServiceDiscovery,
    /// Health check endpoints
    HealthCheck,
    /// Configuration management
    Configuration,
    /// State management services
    StateManagement,

    // Custom capability with string identifier
    /// Custom capability defined by user
    Custom(String),
}

impl fmt::Display for UnifiedCapability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Storage => write!(f, "storage"),
            Self::ZfsManagement => write!(f, "zfs-management"),
            Self::ObjectStorage => write!(f, "object-storage"),
            Self::BlockStorage => write!(f, "block-storage"),
            Self::FileStorage => write!(f, "file-storage"),
            Self::Networking => write!(f, "networking"),
            Self::HttpApi => write!(f, "http-api"),
            Self::Grpc => write!(f, "grpc"),
            Self::Websocket => write!(f, "websocket"),
            Self::Mqtt => write!(f, "mqtt"),
            Self::Compute => write!(f, "compute"),
            Self::TaskExecution => write!(f, "task-execution"),
            Self::Orchestration => write!(f, "orchestration"),
            Self::Scheduling => write!(f, "scheduling"),
            Self::Security => write!(f, "security"),
            Self::Authentication => write!(f, "authentication"),
            Self::Authorization => write!(f, "authorization"),
            Self::Encryption => write!(f, "encryption"),
            Self::SecretManagement => write!(f, "secret-management"),
            Self::ArtificialIntelligence => write!(f, "ai"),
            Self::ModelServing => write!(f, "model-serving"),
            Self::Training => write!(f, "training"),
            Self::Inference => write!(f, "inference"),
            Self::Monitoring => write!(f, "monitoring"),
            Self::Metrics => write!(f, "metrics"),
            Self::Tracing => write!(f, "tracing"),
            Self::Logging => write!(f, "logging"),
            Self::Alerting => write!(f, "alerting"),
            Self::ServiceDiscovery => write!(f, "service-discovery"),
            Self::HealthCheck => write!(f, "health-check"),
            Self::Configuration => write!(f, "configuration"),
            Self::StateManagement => write!(f, "state-management"),
            Self::Custom(name) => write!(f, "custom:{}", name),
        }
    }
}

/// Capability mapper - converts between capability types
pub struct CapabilityMapper;

impl CapabilityMapper {
    /// Convert unified capability to primal capability for discovery
    pub fn to_primal(unified: &UnifiedCapability) -> PrimalCapability {
        match unified {
            // Storage mappings
            UnifiedCapability::Storage
            | UnifiedCapability::ZfsManagement
            | UnifiedCapability::ObjectStorage
            | UnifiedCapability::BlockStorage
            | UnifiedCapability::FileStorage => PrimalCapability::ZfsStorage,

            // Networking/API mappings
            UnifiedCapability::Networking
            | UnifiedCapability::HttpApi
            | UnifiedCapability::Grpc
            | UnifiedCapability::Websocket
            | UnifiedCapability::Mqtt => PrimalCapability::ApiGateway,

            // Compute/Orchestration mappings (using ServiceDiscovery as closest match)
            UnifiedCapability::Compute
            | UnifiedCapability::TaskExecution
            | UnifiedCapability::Orchestration
            | UnifiedCapability::Scheduling => PrimalCapability::ServiceDiscovery,

            // Security mappings
            UnifiedCapability::Security
            | UnifiedCapability::Authentication
            | UnifiedCapability::Authorization
            | UnifiedCapability::Encryption
            | UnifiedCapability::SecretManagement => PrimalCapability::Authentication,

            // AI mappings (using Custom for now)
            UnifiedCapability::ArtificialIntelligence
            | UnifiedCapability::ModelServing
            | UnifiedCapability::Training
            | UnifiedCapability::Inference => PrimalCapability::Custom("ai".to_string()),

            // Observability mappings
            UnifiedCapability::Monitoring
            | UnifiedCapability::Metrics
            | UnifiedCapability::Tracing
            | UnifiedCapability::Logging
            | UnifiedCapability::Alerting => PrimalCapability::Observability,

            // Ecosystem mappings
            UnifiedCapability::ServiceDiscovery
            | UnifiedCapability::HealthCheck
            | UnifiedCapability::Configuration
            | UnifiedCapability::StateManagement => PrimalCapability::ServiceDiscovery,

            // Custom capabilities
            UnifiedCapability::Custom(name) => PrimalCapability::Custom(name.clone()),
        }
    }

    /// Convert application capability to unified capability
    pub fn from_app(app: &AppCapability) -> UnifiedCapability {
        match app {
            AppCapability::Storage(_) => UnifiedCapability::Storage,
            AppCapability::Orchestration(_) => UnifiedCapability::Orchestration,
            AppCapability::Networking(_) => UnifiedCapability::Networking,
            AppCapability::Security(_) => UnifiedCapability::Security,
            AppCapability::AI(_) => UnifiedCapability::ArtificialIntelligence,
            AppCapability::Custom(name) => UnifiedCapability::Custom(name.clone()),
        }
    }

    /// Get environment variable name for capability
    ///
    /// This enables environment-driven discovery: `NESTGATE_CAPABILITY_{NAME}_ENDPOINT`
    pub fn env_var_name(unified: &UnifiedCapability) -> String {
        format!(
            "NESTGATE_CAPABILITY_{}_ENDPOINT",
            unified.to_string().to_uppercase().replace('-', "_")
        )
    }
}

/// Modern capability-based port resolver
///
/// **PHILOSOPHY**: No hardcoded ports. Discovery order:
/// 1. Environment variable (explicit configuration)
/// 2. Runtime discovery via CapabilityResolver
/// 3. Error (no fallback - fail fast if service not configured)
pub struct CapabilityPortResolver;

impl CapabilityPortResolver {
    /// Resolve port for a capability-based service
    ///
    /// # Arguments
    /// * `capability` - The capability to discover
    /// * `resolver` - CapabilityResolver for runtime discovery
    ///
    /// # Returns
    /// Port number if discovered, or error if service not available
    ///
    /// # Example
    /// ```rust,ignore
    /// use nestgate_core::unified_capabilities::{CapabilityPortResolver, UnifiedCapability};
    /// use nestgate_core::capability_resolver::EnvironmentResolver;
    ///
    /// let resolver = EnvironmentResolver::new();
    /// let port = CapabilityPortResolver::resolve_port(
    ///     &UnifiedCapability::HttpApi,
    ///     &resolver
    /// ).await?;
    /// ```
    pub async fn resolve_port<R: crate::capability_resolver::CapabilityResolver>(
        capability: &UnifiedCapability,
        resolver: &R,
    ) -> Result<u16, CapabilityResolutionError> {
        match resolver.resolve_capability(capability).await {
            Ok(service) => Ok(service.port),
            Err(e) => Err(CapabilityResolutionError::ServiceNotDiscovered {
                capability: capability.to_string(),
                env_var: CapabilityMapper::env_var_name(capability),
                hint: format!(
                    "Set environment variable or ensure service is registered. Error: {}",
                    e
                ),
            }),
        }
    }

    /// Resolve full endpoint (protocol + host + port) for a capability
    pub async fn resolve_endpoint<R: crate::capability_resolver::CapabilityResolver>(
        capability: &UnifiedCapability,
        resolver: &R,
    ) -> Result<String, CapabilityResolutionError> {
        match resolver.resolve_capability(capability).await {
            Ok(service) => Ok(service.url()),
            Err(e) => Err(CapabilityResolutionError::ServiceNotDiscovered {
                capability: capability.to_string(),
                env_var: CapabilityMapper::env_var_name(capability),
                hint: format!(
                    "Set environment variable or ensure service is registered. Error: {}",
                    e
                ),
            }),
        }
    }
}

/// Error type for capability resolution failures
#[derive(Debug)]
pub enum CapabilityResolutionError {
    /// Service with requested capability was not discovered
    ServiceNotDiscovered {
        /// The capability that was requested
        capability: String,
        /// The environment variable that should have been set
        env_var: String,
        /// A hint for how to resolve this error
        hint: String,
    },
    /// Invalid configuration (e.g., malformed URL, invalid port)
    InvalidConfiguration {
        /// The environment variable that had invalid configuration
        env_var: String,
        /// The value that was provided
        value: String,
        /// The reason the configuration is invalid
        reason: String,
    },
}

impl fmt::Display for CapabilityResolutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ServiceNotDiscovered {
                capability,
                env_var,
                hint,
            } => {
                write!(
                    f,
                    "Service with capability '{}' not discovered.\n\
                     Environment variable: {}\n\
                     Hint: {}",
                    capability, env_var, hint
                )
            }
            Self::InvalidConfiguration {
                env_var,
                value,
                reason,
            } => {
                write!(
                    f,
                    "Invalid configuration in {}='{}': {}",
                    env_var, value, reason
                )
            }
        }
    }
}

impl std::error::Error for CapabilityResolutionError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_to_string() {
        assert_eq!(UnifiedCapability::Storage.to_string(), "storage");
        assert_eq!(UnifiedCapability::HttpApi.to_string(), "http-api");
        assert_eq!(
            UnifiedCapability::Custom("test".to_string()).to_string(),
            "custom:test"
        );
    }

    #[test]
    fn test_env_var_name_generation() {
        let cap = UnifiedCapability::HttpApi;
        assert_eq!(
            CapabilityMapper::env_var_name(&cap),
            "NESTGATE_CAPABILITY_HTTP_API_ENDPOINT"
        );

        let cap = UnifiedCapability::ZfsManagement;
        assert_eq!(
            CapabilityMapper::env_var_name(&cap),
            "NESTGATE_CAPABILITY_ZFS_MANAGEMENT_ENDPOINT"
        );
    }

    #[test]
    fn test_storage_capabilities_map_correctly() {
        assert_eq!(
            CapabilityMapper::to_primal(&UnifiedCapability::Storage),
            PrimalCapability::ZfsStorage
        );
        assert_eq!(
            CapabilityMapper::to_primal(&UnifiedCapability::ZfsManagement),
            PrimalCapability::ZfsStorage
        );
    }

    #[test]
    fn test_networking_capabilities_map_correctly() {
        assert_eq!(
            CapabilityMapper::to_primal(&UnifiedCapability::Networking),
            PrimalCapability::ApiGateway
        );
        assert_eq!(
            CapabilityMapper::to_primal(&UnifiedCapability::HttpApi),
            PrimalCapability::ApiGateway
        );
    }

    #[tokio::test]
    async fn test_port_resolution_from_env() {
        use crate::capability_resolver::EnvironmentResolver;

        std::env::set_var(
            "NESTGATE_CAPABILITY_HTTP_API_ENDPOINT",
            "http://localhost:8888",
        );
        let resolver = EnvironmentResolver::new();
        let result =
            CapabilityPortResolver::resolve_port(&UnifiedCapability::HttpApi, &resolver).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 8888);
        std::env::remove_var("NESTGATE_CAPABILITY_HTTP_API_ENDPOINT");
    }

    #[tokio::test]
    async fn test_no_fallback_on_missing_service() {
        use crate::capability_resolver::EnvironmentResolver;

        std::env::remove_var("NESTGATE_CAPABILITY_STORAGE_ENDPOINT");
        let resolver = EnvironmentResolver::new();
        let result =
            CapabilityPortResolver::resolve_port(&UnifiedCapability::Storage, &resolver).await;
        assert!(result.is_err()); // No fallback - fail fast
    }
}
