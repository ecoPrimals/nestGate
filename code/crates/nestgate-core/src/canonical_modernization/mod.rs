// **CANONICAL MODERNIZATION SYSTEM**
//! Module definitions and exports.
// This module provides the complete canonical modernization framework for NestGate,
//! implementing unified systems that replace fragmented legacy patterns.

// Removed unused serde imports

// ==================== SECTION ====================

/// Canonical constants system providing unified constant definitions across the system
pub mod canonical_constants;
// Constants consolidation system - disabled for compilation compatibility
// pub mod constants_consolidation;
/// Builder patterns for canonical configurations
pub mod builders;
/// Idiomatic evolution patterns and utilities for code modernization
pub mod idiomatic_evolution;
/// Unified enumerations providing standardized enum types
pub mod unified_enums;
/// ⚠️ REMOVED: unified_types was migrated to config::canonical_primary (November 2025)
// pub mod unified_types; // REMOVED - use config::canonical_primary
/// Zero-cost trait implementations providing high-performance abstractions
pub mod zero_cost_traits;
// ==================== SECTION ====================

use crate::config::canonical_primary::NestGateCanonicalConfig;

/// **BACKWARD COMPATIBILITY**: Legacy configuration type alias for migration compatibility
pub type CanonicalModernizedConfig = NestGateCanonicalConfig;
/// **BACKWARD COMPATIBILITY**: Service metadata types and definitions
pub mod service_metadata {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use std::time::SystemTime;

    // CANONICAL MODERNIZATION: Migrated from deprecated ServiceRegistration
    pub use crate::service_discovery::types::UniversalServiceRegistration as ServiceRegistration;
    pub use crate::service_discovery::types::{ServiceCapability, ServiceEndpoint};
    pub use crate::universal_providers_zero_cost::ServiceStatus;

    /// Service dependency definition
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ServiceDependency {
        /// Name of the service this dependency references
        pub service_name: String,
        /// Version requirement specification (semver format)
        pub version_requirement: String,
        /// Whether this dependency is optional
        pub optional: bool,
        /// Additional metadata for the dependency
        pub metadata: HashMap<String, String>,
    }

    impl Default for ServiceDependency {
        fn default() -> Self {
            Self {
                service_name: "unknown".to_string(),
                version_requirement: "*".to_string(),
                optional: false,
                metadata: HashMap::new(),
            }
        }
    }

    /// Universal service metadata
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UniversalServiceMetadata {
        /// Unique identifier for the service
        pub service_id: String,
        /// Human-readable name of the service
        pub service_name: String,
        /// Version string for the service (semver format)
        pub service_version: String,
        /// Human-readable description of the service
        pub description: String,
        /// List of capabilities provided by the service
        pub capabilities: Vec<String>,
        /// Network endpoints where the service is available
        pub endpoints: Vec<ServiceEndpoint>,
        /// Services this service depends on
        pub dependencies: Vec<ServiceDependency>,
        /// Additional service metadata
        pub metadata: HashMap<String, String>,
        /// Timestamp when the service was created
        pub created_at: SystemTime,
        /// Timestamp when the service was last updated
        pub updated_at: SystemTime,
        /// Runtime configuration key-value pairs
        pub configuration: HashMap<String, String>,
        /// Classification tags for the service
        pub tags: Vec<String>,
        /// Current operational status of the service
        pub status: ServiceStatus,
    }
    impl Default for UniversalServiceMetadata {
        fn default() -> Self {
            let now = SystemTime::now();
            Self {
                service_id: String::new(),
                service_name: String::new(),
                service_version: "1.0.0".to_string(),
                description: String::new(),
                capabilities: Vec::new(),
                endpoints: Vec::new(),
                dependencies: Vec::new(),
                metadata: HashMap::new(),
                created_at: now,
                updated_at: now,
                configuration: HashMap::new(),
                tags: Vec::new(),
                status: ServiceStatus::default(),
            }
        }
    }
}

// ==================== SECTION ====================

pub use canonical_constants::*;

// Re-export from canonical_primary (unified_types deprecated)
pub use crate::config::canonical_primary::service::ServiceConfig as UnifiedServiceConfig;
// Re-export from unified_enums - only enums that actually exist
pub use crate::unified_enums::service_types::{UnifiedServiceState, UnifiedServiceType};
// pub use zero_cost_traits::*; // Unused import
pub use service_metadata::*;
