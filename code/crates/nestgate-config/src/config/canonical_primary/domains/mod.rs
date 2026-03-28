// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

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
    ActionsConfig, AiAutomationConfig, AnalysisConfig, AutomationConfig, LifecycleConfig,
    MlPredictionConfig, OptimizationConfig, PredictionConfig, SchedulingConfig, TriggersConfig,
    WorkflowsConfig,
};

// **LEGACY**: Network configurations (will be migrated to consolidated system)
pub use network::{
    ApiConfig, CanonicalNetworkConfig, NetworkDiscoveryConfig, NetworkEnvironmentConfig,
    NetworkMonitoringConfig, NetworkOrchestrationConfig, NetworkPerformanceConfig,
    NetworkProtocolConfig, NetworkSecurityConfig, NetworkVlanConfig,
};
// Note: NetworkApiConfig removed - ApiConfig is now the canonical type
pub use storage_canonical::{
    AlertThresholds,
    ArcCacheConfig,
    // Storage configurations
    CanonicalStorageConfig,
    L2ArcConfig,
    PrefetchConfig,
    // ZFS sub-configurations
    RetentionPolicy,
    StorageConfig,
    UnifiedStorageConfig,
    // ZFS enums
    ZfsCompression,
    ZfsDatasetConfig,
    ZfsMaintenanceConfig,
    ZfsMigrationConfig,
    ZfsMonitoringConfig,
    ZfsPerformanceConfig,
    ZfsPoolConfig,
    ZfsPoolSettings,
    ZfsRedundancy,
    ZfsSecurityConfig,
    ZfsSnapshotConfig,
    // ZFS configurations (extended November 7, 2025)
    ZfsStorageConfig,
    ZilConfig,
};

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
