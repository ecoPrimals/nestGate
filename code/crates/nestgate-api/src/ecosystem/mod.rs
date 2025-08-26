//
// **CANONICAL MODERNIZATION**: Universal ecosystem integration that works with
// any management system (BiomeOS, Kubernetes, Docker, etc.) without hardcoded dependencies.
//
// **ELIMINATES**: Hardcoded biomeOS integration and endpoint dependencies
// **PROVIDES**: Capability-based ecosystem discovery and integration

// **UNIVERSAL ECOSYSTEM INTEGRATION** - Replaces hardcoded BiomeOS integration
pub mod universal_ecosystem_integration;

// Re-export main types
pub use universal_ecosystem_integration::{
    UniversalEcosystemIntegration, EcosystemInfo, EcosystemType, 
    EcosystemHealthStatus, ServiceRegistrationInfo, ServiceEndpoint,
    RegistrationResult, RegistrationStatus, UniversalEcosystemEvent,
    EventResponse, CompatibilityStatus, CapabilityId
};
