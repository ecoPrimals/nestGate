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
/// Unified types system providing consistent type definitions
pub mod unified_types;
/// Zero-cost trait implementations providing high-performance abstractions
pub mod zero_cost_traits;
// ==================== SECTION ====================

use crate::config::canonical_master::NestGateCanonicalConfig;

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
        pub service_name: String,
        pub version_requirement: String,
        pub optional: bool,
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
        pub service_id: String,
        pub service_name: String,
        pub service_version: String,
        pub description: String,
        pub capabilities: Vec<String>,
        pub endpoints: Vec<ServiceEndpoint>,
        pub dependencies: Vec<ServiceDependency>,
        pub metadata: HashMap<String, String>,
        pub created_at: SystemTime,
        pub updated_at: SystemTime,
        pub configuration: HashMap<String, String>,
        pub tags: Vec<String>,
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

// Re-export from unified_types - only types that actually exist
pub use unified_types::{UnifiedNetworkConfig, UnifiedServiceConfig};
// Re-export from unified_enums - only enums that actually exist
pub use crate::unified_enums::service_types::{UnifiedServiceState, UnifiedServiceType};
// pub use zero_cost_traits::*; // Unused import
pub use service_metadata::*;
