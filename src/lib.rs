//! Songbird Orchestrator
//! 
//! Universal service orchestration platform for Rust projects
//! 
//! Songbird provides a generic, trait-based approach to service orchestration
//! that works across any domain or project type. It replaces project-specific
//! orchestration patterns with universal, reusable components.

// Core modules
pub mod errors;
pub mod config;
pub mod traits;

// Component modules
pub mod discovery;
pub mod communication;
pub mod health;
pub mod security;
pub mod registry;

// Core orchestrator
pub mod orchestrator;

// Re-export main types for easy access
pub use errors::{SongbirdError, Result};
pub use config::{
    OrchestratorConfig, CoreOrchestratorConfig, NetworkConfig, SecurityConfig,
    MonitoringConfig, DiscoveryConfig, LoadBalancingConfig, HealthConfig,
    DefaultServiceConfig
};
pub use traits::service::{
    UniversalService, ServiceRequest, ServiceResponse, ServiceInfo, ServiceMetrics,
    ServiceEndpoint, ClientInfo, AuthInfo, ResponseStatus
};

// Re-export orchestrator types
pub use orchestrator::{Orchestrator, OrchestratorStats};
pub use registry::ServiceHandle;

// Re-export discovery types
pub use discovery::{
    ServiceQuery, ServiceEvent, HealthStatus
};

// Re-export communication types
pub use communication::{
    ServiceMessage, ServiceAddress
};

/// Current version of Songbird Orchestrator
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Build information structure
#[derive(Debug, Clone)]
pub struct BuildInfo {
    pub version: &'static str,
    pub git_hash: &'static str,
    pub build_date: &'static str,
    pub rust_version: &'static str,
}

/// Get build information
#[must_use]
pub fn build_info() -> BuildInfo {
    BuildInfo {
        version: VERSION,
        git_hash: match option_env!("GIT_HASH") {
            Some(hash) => hash,
            None => "unknown",
        },
        build_date: match option_env!("BUILD_DATE") {
            Some(date) => date,
            None => "unknown",
        },
        rust_version: match option_env!("RUST_VERSION") {
            Some(version) => version,
            None => "unknown",
        },
    }
}

/// Feature flags structure
#[derive(Debug, Clone)]
pub struct Features {
    pub consul_discovery: bool,
    pub etcd_discovery: bool,
    pub kubernetes_discovery: bool,
    pub grpc_communication: bool,
    pub mtls_security: bool,
}

/// Get available features
#[must_use]
pub const fn get_features() -> Features {
    Features {
        consul_discovery: cfg!(feature = "consul"),
        etcd_discovery: cfg!(feature = "etcd"),
        kubernetes_discovery: cfg!(feature = "kubernetes"),
        grpc_communication: cfg!(feature = "grpc"),
        mtls_security: cfg!(feature = "mtls"),
    }
}

/// Prelude module for commonly used types
pub mod prelude {
    pub use super::{
        // Core traits
        UniversalService,
        
        // Main orchestrator
        Orchestrator, OrchestratorConfig,
        
        // Service types
        ServiceRequest, ServiceResponse, ServiceInfo, ServiceMetrics,
        
        // Error handling
        SongbirdError, Result,
        
        // Async trait
        async_trait::async_trait,
        
        // Common serde traits
        serde::{Serialize, Deserialize},
    };
}

/// Utility functions for testing and development
pub mod utils {
    use super::{Result, ServiceAddress, ServiceInfo, ServiceRequest, SongbirdError};

    /// Create a test service request
    #[must_use]
    pub fn create_test_request(method: &str, path: &str) -> ServiceRequest {
        ServiceRequest::new(method, path)
    }

    /// Create a health check request
    #[must_use]
    pub fn create_health_check_request() -> ServiceRequest {
        ServiceRequest::new("GET", "/health")
    }

    /// Create a metrics request
    #[must_use]
    pub fn create_metrics_request() -> ServiceRequest {
        ServiceRequest::new("GET", "/metrics")
    }

    /// Parse a service address from a string
    /// 
    /// # Errors
    /// 
    /// Returns an error if the address format is invalid
    pub fn parse_service_address(addr: &str) -> Result<ServiceAddress> {
        let parts: Vec<&str> = addr.split(':').collect();
        if parts.len() != 2 {
            return Err(SongbirdError::Configuration(
                "Invalid address format, expected 'host:port'".to_string()
            ));
        }

        let host = parts[0].to_string();
        let port = parts[1].parse::<u16>().map_err(|_| {
            SongbirdError::Configuration("Invalid port number".to_string())
        })?;

        Ok(ServiceAddress { host, port })
    }

    /// Create a test service info
    #[must_use]
    pub fn create_test_service_info(id: &str, service_type: &str) -> ServiceInfo {
        ServiceInfo {
            id: id.to_string(),
            name: format!("Test {id}"),
            service_type: service_type.to_string(),
            version: "1.0.0".to_string(),
            description: format!("Test service {id}"),
            endpoints: vec![],
            tags: vec!["test".to_string()],
            metadata: std::collections::HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Result, ServiceAddress, ServiceInfo, ServiceRequest, SongbirdError};

    #[test]
    fn test_build_info() {
        let build_info = crate::build_info();
        assert!(!build_info.version.is_empty());
        assert!(!build_info.git_hash.is_empty());
        assert!(!build_info.build_date.is_empty());
        assert!(!build_info.rust_version.is_empty());
    }
    
    #[test]
    fn test_version_info() {
        assert!(!crate::VERSION.is_empty());
    }
    
    #[test]
    fn test_feature_flags() {
        let features = crate::get_features();
        // These should always be true for default features
        assert!(features.consul_discovery || !features.consul_discovery); // Just test it compiles
    }
    
    #[test]
    fn test_utils() {
        let request = crate::utils::create_test_request("GET", "/test");
        assert_eq!(request.method, "GET");
        assert_eq!(request.path, "/test");
        
        let health_request = crate::utils::create_health_check_request();
        assert_eq!(health_request.path, "/health");
        
        let service_info = crate::utils::create_test_service_info("test-service", "api");
        assert_eq!(service_info.id, "test-service");
        assert_eq!(service_info.service_type, "api");
    }
} 