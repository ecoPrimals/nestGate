// **CANONICAL DOMAIN CONFIGURATIONS MODULE**
//! Module definitions and exports.
// This module provides access to all domain-specific canonical configurations
//! that consolidate scattered config structures across the `NestGate` ecosystem.
//! Module definitions and exports.
// **PHASE 2C ENHANCEMENT**: Added consolidated domain configurations that unify
//! all 100+ scattered Config structs across the ecosystem into a single system.

// **NEW**: Consolidated domain configurations - THE unified system
pub mod consolidated_domains;
// Automation configuration - canonical (NEW: November 7, 2025)
pub mod automation;
// Network configuration consolidation - modularized
pub mod network;
// Storage configuration consolidation - modular structure
pub mod storage_canonical;
// Security configuration consolidation - modular structure
pub mod security_canonical;
// Performance configuration consolidation - modularized
pub mod performance;
// Handler configuration consolidation - modular structure
pub mod handler_canonical;
// Test configuration consolidation - modular structure (dev-stubs only)
#[cfg(feature = "dev-stubs")]
pub mod test_canonical;
// ==================== CONSOLIDATED SYSTEM RE-EXPORTS ====================

// **THE UNIFIED DOMAIN SYSTEM** - Primary exports for Phase 2C
pub use consolidated_domains::{
    ApiDomainConfig, AutomationDomainConfig, BinaryDomainConfig, ConsolidatedDomainConfigs,
    ConsolidatedIntegrationConfigs, DomainConfigValidation, FsMonitorDomainConfig,
    InstallerDomainConfig, McpDomainConfig, NetworkServicesDomainConfig, PerformanceDomainConfig,
    ZfsDomainConfig,
};
// ==================== LEGACY SYSTEM RE-EXPORTS ====================

// **CANONICAL AUTOMATION CONFIGURATION** (NEW: November 7, 2025)
pub use automation::{
    AutomationConfig,
    AnalysisConfig,
    PredictionConfig,
    MlPredictionConfig,
    AiAutomationConfig,
    LifecycleConfig,
    OptimizationConfig,
    WorkflowsConfig,
    SchedulingConfig,
    TriggersConfig,
    ActionsConfig,
};

// **LEGACY**: Network configurations (will be migrated to consolidated system)
pub use network::{
    CanonicalNetworkConfig, NetworkApiConfig, NetworkDiscoveryConfig, NetworkEnvironmentConfig,
    NetworkMonitoringConfig, NetworkOrchestrationConfig, NetworkPerformanceConfig,
    NetworkProtocolConfig, NetworkSecurityConfig, NetworkVlanConfig,
};
pub use storage_canonical::{
    // Storage configurations
    CanonicalStorageConfig, StorageConfig,
    // ZFS configurations (extended November 7, 2025)
    ZfsStorageConfig, ZfsPoolConfig, ZfsDatasetConfig, ZfsSnapshotConfig,
    ZfsMaintenanceConfig, ZfsPerformanceConfig, ZfsSecurityConfig,
    ZfsMonitoringConfig, ZfsMigrationConfig,
    // ZFS sub-configurations
    RetentionPolicy, ArcCacheConfig, L2ArcConfig, ZilConfig, PrefetchConfig,
    AlertThresholds, ZfsPoolSettings,
    // ZFS enums
    ZfsCompression, ZfsRedundancy, UnifiedStorageConfig};

pub use security_canonical::{CanonicalSecurityConfig, SecurityConfig, UnifiedSecurityConfig};

pub use performance::{
    CanonicalPerformanceConfig, PerformanceConfig, UnifiedPerformanceConfig,
    UnifiedPerformanceTestConfig,
};

pub use handler_canonical::{
    CanonicalHandlerConfigs, HandlerConfig, HandlerConfigs, UnifiedHandlerConfig,
};

#[cfg(feature = "dev-stubs")]
pub use test_canonical::{CanonicalTestConfigs, TestConfig, TestConfigs, UnifiedTestConfig};
