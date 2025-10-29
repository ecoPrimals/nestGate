//! **CANONICAL DOMAIN CONFIGURATIONS MODULE**
//!
//! This module provides access to all domain-specific canonical configurations
//! that consolidate scattered config structures across the NestGate ecosystem.
//!
//! **PHASE 2C ENHANCEMENT**: Added consolidated domain configurations that unify
//! all 100+ scattered Config structs across the ecosystem into a single system.

/// **NEW**: Consolidated domain configurations - THE unified system
pub mod consolidated_domains;

/// Network configuration consolidation - modularized
pub mod network;

/// Storage configuration consolidation - modular structure
pub mod storage_canonical;

/// Security configuration consolidation - modular structure
pub mod security_canonical;

/// Performance configuration consolidation - modularized
pub mod performance;

/// Handler configuration consolidation - modular structure
pub mod handler_canonical;

/// Test configuration consolidation - modular structure
pub mod test_canonical;

// ==================== CONSOLIDATED SYSTEM RE-EXPORTS ====================

/// **THE UNIFIED DOMAIN SYSTEM** - Primary exports for Phase 2C
pub use consolidated_domains::{
    ConsolidatedDomainConfigs,
    ConsolidatedIntegrationConfigs,
    ZfsDomainConfig,
    ApiDomainConfig,
    McpDomainConfig,
    NetworkServicesDomainConfig,
    AutomationDomainConfig,
    FsMonitorDomainConfig,
    InstallerDomainConfig,
    PerformanceDomainConfig,
    BinaryDomainConfig,
    DomainConfigValidation,
};

// ==================== LEGACY SYSTEM RE-EXPORTS ====================

/// **LEGACY**: Network configurations (will be migrated to consolidated system)
pub use network::{
    CanonicalNetworkConfig,
    NetworkApiConfig,
    NetworkOrchestrationConfig,
    NetworkProtocolConfig,
    NetworkVlanConfig,
    NetworkDiscoveryConfig,
    NetworkPerformanceConfig,
    NetworkSecurityConfig,
    NetworkMonitoringConfig,
    NetworkEnvironmentConfig,
};

pub use storage_canonical::{
    CanonicalStorageConfig,
    StorageConfig,
    UnifiedStorageConfig,
};

pub use security_canonical::{
    CanonicalSecurityConfig,
    SecurityConfig,
    UnifiedSecurityConfig,
};

pub use performance::{
    CanonicalPerformanceConfig,
    PerformanceConfig,
    UnifiedPerformanceConfig,
    UnifiedPerformanceTestConfig,
};

pub use handler_canonical::{
    CanonicalHandlerConfigs,
    HandlerConfig,
    UnifiedHandlerConfig,
    HandlerConfigs,
};

pub use test_canonical::{
    CanonicalTestConfigs,
    TestConfig,
    UnifiedTestConfig,
    TestConfigs,
}; 