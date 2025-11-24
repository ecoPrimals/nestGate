// Removed unused import for pedantic perfection
// Commented out until available: CapabilityCategory, CapabilityRequest

/// AI capability discovery for ecosystem services
pub mod ai;
/// Orchestration capability discovery for coordination services
pub mod orchestration;
/// Security capability discovery for authentication and authorization
pub mod security;
// **CAPABILITIES DISCOVERY MODULE**
// Dynamic service discovery and configuration management
//
// **UNIFIED ARCHITECTURE**: All discovery types now consolidated into
// `UnifiedDynamicDiscoveryManager` for consistent patterns and reduced duplication.
// **CORE CAPABILITY DISCOVERY MODULES**

/// Storage capability discovery for data persistence services
pub mod storage;
// **UNIFIED DYNAMIC DISCOVERY SYSTEM** - PRIMARY ARCHITECTURE
// **MODULARIZED CONFIGURATION** - Split from 909-line monolithic file

/// Discovery configuration types and builders
pub mod config;
// NOTE: unified_dynamic_config has been consolidated into the canonical configuration system

// **SUPPORTING MODULES**
// Temporarily disabled example code while updating to new API
// pub mod unified_config_example;

// Re-export capability discovery managers
pub use crate::capabilities::discovery::ai::AiCapabilityDiscovery;
pub use orchestration::OrchestrationCapabilityDiscovery;
pub use security::SecurityCapabilityDiscovery;
pub use storage::StorageCapabilityDiscovery;

// **PRIMARY UNIFIED DISCOVERY EXPORTS**
pub use config::*;
// pub use unified_dynamic_config::{UnifiedDynamicDiscoveryManager}; // Module not found - disabled

// **UNIFIED DISCOVERY CONFIGURATION** - THE CANONICAL TYPE
// Single entry point for all discovery configuration patterns

/// Unified discovery configuration type alias for consistent API
pub type DiscoveryConfig = UnifiedDynamicDiscoveryExtensions;

// **UNIFIED DISCOVERY MANAGER** - THE CANONICAL TYPE
// Single entry point for all discovery management operations

/// Unified discovery manager type alias for discovery operations
pub type DiscoveryManager = UnifiedDynamicDiscoveryExtensions;

// ==================== SECTION ====================

/// Discovers storage capabilities from available primals in the ecosystem
pub async fn discover_storage_capabilities() -> crate::Result<Vec<storage::StorageCapabilityInfo>> {
    let discovery = storage::StorageCapabilityDiscovery::new();
    discovery.discover_capabilities().await
}

/// Discovers orchestration capabilities from available primals
pub async fn discover_orchestration_capabilities(
) -> crate::Result<Vec<orchestration::OrchestrationCapabilityInfo>> {
    let discovery = orchestration::OrchestrationCapabilityDiscovery::new();
    discovery.discover_capabilities().await
}

/// Discovers security capabilities from available primals
pub async fn discover_security_capabilities() -> crate::Result<Vec<security::SecurityCapabilityInfo>>
{
    let discovery = security::SecurityCapabilityDiscovery::new();
    discovery.discover_capabilities().await
}
/// Discovers AI capabilities from available primals
pub async fn discover_ai_capabilities() -> crate::Result<Vec<ai::AiCapabilityInfo>> {
    let discovery = ai::AiCapabilityDiscovery::new();
    discovery.discover_capabilities().await
}
