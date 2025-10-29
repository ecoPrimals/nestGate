// Removed unused import for pedantic perfection
// Commented out until available: CapabilityCategory, CapabilityRequest
pub mod ai;
pub mod orchestration;
pub mod security;
// **CAPABILITIES DISCOVERY MODULE**
// Dynamic service discovery and configuration management
//
// **UNIFIED ARCHITECTURE**: All discovery types now consolidated into
// `UnifiedDynamicDiscoveryManager` for consistent patterns and reduced duplication.
// **CORE CAPABILITY DISCOVERY MODULES**
pub mod storage;
// **UNIFIED DYNAMIC DISCOVERY SYSTEM** - PRIMARY ARCHITECTURE
// **MODULARIZED CONFIGURATION** - Split from 909-line monolithic file
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
pub type DiscoveryConfig = UnifiedDynamicDiscoveryExtensions;
// **UNIFIED DISCOVERY MANAGER** - THE CANONICAL TYPE
// Single entry point for all discovery management operations
pub type DiscoveryManager = UnifiedDynamicDiscoveryExtensions;
// ==================== SECTION ====================

// Discover storage capabilities from available primals
pub async fn discover_storage_capabilities() -> crate::Result<Vec<storage::StorageCapabilityInfo>> {
    let discovery = storage::StorageCapabilityDiscovery::new();
    discovery.discover_capabilities().await
}
// Discover orchestration capabilities from available primals
pub async fn discover_orchestration_capabilities(
) -> crate::Result<Vec<orchestration::OrchestrationCapabilityInfo>> {
    let discovery = orchestration::OrchestrationCapabilityDiscovery::new();
    discovery.discover_capabilities().await
}
// Discover security capabilities from available primals
pub async fn discover_security_capabilities() -> crate::Result<Vec<security::SecurityCapabilityInfo>>
{
    let discovery = security::SecurityCapabilityDiscovery::new();
    discovery.discover_capabilities().await
}
// Discover AI capabilities from available primals
pub async fn discover_ai_capabilities() -> crate::Result<Vec<ai::AiCapabilityInfo>> {
    let discovery = ai::AiCapabilityDiscovery::new();
    discovery.discover_capabilities().await
}
