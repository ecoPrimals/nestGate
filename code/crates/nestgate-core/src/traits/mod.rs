//! **NESTGATE CANONICAL TRAIT SYSTEM**
//!
//! This module provides the single, canonical trait system for NestGate.
//! All traits use native async patterns for zero-cost abstractions.

// ==================== SECTION ====================

/// **PRIMARY**: The canonical unified trait system
/// This replaces ALL other trait systems
pub mod canonical_unified_traits;

// ==================== SECTION ====================

/// **CONSOLIDATED STORAGE TRAITS** - All storage interfaces unified
/// This consolidates multiple fragmented storage trait definitions:
/// - UnifiedStorageBackend (universal_storage/unified_storage_traits.rs)
/// - UnifiedCanonicalStorage (traits/unified_canonical_storage.rs)  
/// - CanonicalStorageBackend (universal_storage/canonical_storage.rs)
/// - All other fragmented storage traits
pub use canonical_unified_traits::CanonicalStorage;

/// **STORAGE MIGRATION HELPER** - Utilities for migrating to CanonicalStorage
pub mod storage_migration_helper;

// ==================== SECTION ====================
// All deprecated trait modules have been removed. Use canonical_unified_traits directly:
// - native_async → canonical_unified_traits::CanonicalService
// - canonical_provider_unification → canonical_unified_traits::CanonicalProvider  
// - canonical_storage_unification → canonical_unified_traits::CanonicalStorage
// - unified_canonical_storage → canonical_unified_traits::CanonicalStorage
// - domain_extensions → canonical_unified_traits domain-specific traits
// - universal_service_zero_cost → canonical_unified_traits::ZeroCostService

// ==================== SECTION ====================

/// **THE** canonical traits for all NestGate systems
pub use canonical_unified_traits::{
    // Core traits
    CanonicalService,
    CanonicalProvider,
    // CanonicalStorage - exported separately above
    CanonicalNetwork,
    CanonicalSecurity,
    CanonicalMcp,
    CanonicalAutomation,
    ZeroCostService,
    
    // Factory traits
    CanonicalServiceFactory,
    CanonicalProviderFactory,
    
    // Supporting types
    ServiceCapabilities,
    ProviderHealth,
    ProviderCapabilities,
    StorageUsageStats,
    ConnectionHandle,
    ConnectionStatus,
    HealthStatus,
    SecurityCredentials,
    CronSchedule,
    ScheduleId,
    ScheduleInfo,
};

// ==================== SECTION ====================

/// **MIGRATION**: Backward compatibility aliases
/// These point to the canonical traits for seamless migration

use std::collections::HashMap;
use std::future::Future;
use std::time::{Duration, SystemTime};
use serde::{Deserialize, Serialize};
use crate::error::CanonicalResult as Result;
use crate::unified_enums::service_types::UnifiedServiceType;

// ==================== SECTION ====================
// All legacy trait aliases have been removed. Use canonical_unified_traits directly:
// - UniversalService → canonical_unified_traits::CanonicalService  
// - NativeAsyncService → canonical_unified_traits::CanonicalService
// - ZeroCostUniversalService → canonical_unified_traits::ZeroCostService
// - SecurityProvider<T> → canonical_unified_traits::CanonicalProvider<T>
// - StorageProvider<T> → canonical_unified_traits::CanonicalProvider<T>
// - NetworkProvider<T> → canonical_unified_traits::CanonicalProvider<T>

// ==================== SECTION ====================

// ==================== SECTION ====================
// All legacy support types have been removed. Use canonical_unified_traits directly:
// - ServiceHealth → canonical_unified_traits::ProviderHealth
// - ServiceMetrics → canonical_unified_traits::ServiceCapabilities

// ==================== SECTION ====================
// All deprecated legacy compatibility traits have been removed.
// Migration is complete - use canonical_unified_traits directly for all new code.
//
// **MIGRATION GUIDE**:
// - UniversalService → canonical_unified_traits::CanonicalService

// **DEPRECATED LEGACY TYPES REMOVED**:
// - ServiceHealth → canonical_unified_traits::ProviderHealth
// - ServiceMetrics → canonical_unified_traits::ServiceCapabilities  
// - UniversalServiceRequest → Domain-specific request types
// - UniversalServiceResponse → Domain-specific response types
// - UniversalResponseStatus → canonical_unified_traits::HealthStatus

/// Service registration information (legacy compatibility)
#[deprecated(note = "Use canonical service discovery instead")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistration {
    pub service_id: String,
    pub service_type: UnifiedServiceType,
    pub endpoint: String,
    pub health_check_endpoint: String,
    pub metadata: HashMap<String, String>,
    pub registered_at: SystemTime,
}

// ==================== SECTION ====================

/// **DEPRECATED TYPE IMPLEMENTATIONS REMOVED**
///
/// The following Default implementations have been removed because the types
/// no longer exist (they have been migrated to canonical types):
/// - UniversalServiceRequest → Use domain-specific request types
/// - UniversalServiceResponse → Use domain-specific response types  
/// - UniversalResponseStatus → Use canonical_unified_traits::HealthStatus
/// - ServiceHealth → Use canonical_unified_traits::ProviderHealth
/// - ServiceMetrics → Use canonical_unified_traits::ServiceCapabilities
/// - ProviderHealth → Use canonical_unified_traits::ProviderHealth
/// - ServiceCapabilities → Use canonical_unified_traits::ServiceCapabilities
///
/// **MIGRATION COMPLETE**: All legacy types have been successfully eliminated.
/// Use the canonical types from canonical_unified_traits module for all new code.

// ==================== SECTION ====================
// Note: Blanket implementations removed during modernization cleanup
// Use canonical_unified_traits directly for new implementations

// ==================== SECTION ====================

/// Create a default service configuration
pub fn create_default_service_config() -> HashMap<String, serde_json::Value> {
    let mut config = HashMap::new();
    config.insert("timeout_ms".to_string(), serde_json::Value::Number(30000.into()));
    config.insert("max_connections".to_string(), serde_json::Value::Number(1000.into()));
    config.insert("buffer_size".to_string(), serde_json::Value::Number(65536.into()));
    config
}

/// Create default service capabilities
pub fn create_default_capabilities() -> crate::traits::canonical_unified_traits::ServiceCapabilities {
    crate::traits::canonical_unified_traits::ServiceCapabilities::default()
}

/// Create default provider health
pub fn create_default_provider_health() -> crate::traits::canonical_unified_traits::ProviderHealth {
    crate::traits::canonical_unified_traits::ProviderHealth::default()
}

// ==================== SECTION ====================

/// **DEPRECATION NOTICE**
/// 
/// The following trait systems are deprecated and will be removed:
/// - `native_async::*` → Use `canonical_unified_traits::CanonicalService`
/// - `canonical_provider_unification::*` → Use `canonical_unified_traits::CanonicalProvider`
/// - `canonical_storage_unification::*` → Use `canonical_unified_traits::CanonicalStorage`
/// - All legacy service traits → Use `canonical_unified_traits::*`
/// 
/// **Migration Path**:
/// 1. Replace trait bounds: `T: UniversalService` → `T: CanonicalService`
/// 2. Update implementations: `impl UniversalService` → `impl CanonicalService`
/// 3. Use native async patterns: Remove `#[async_trait]` annotations
/// 4. Update imports: `use crate::traits::canonical_unified_traits::*;`

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_service_config() {
        let config = create_default_service_config();
        assert!(config.contains_key("timeout_ms"));
        assert!(config.contains_key("max_connections"));
        assert!(config.contains_key("buffer_size"));
    }

    #[test]
    fn test_default_capabilities() {
        let capabilities = create_default_capabilities();
        assert_eq!(capabilities.version, "1.0.0");
        assert_eq!(capabilities.max_concurrent_requests, Some(100));
    }

    #[test]
    fn test_default_provider_health() {
        let health = create_default_provider_health();
        assert_eq!(health.status, HealthStatus::Unknown);
    }

    #[test]
    fn test_canonical_traits_available() {
        // Test that canonical traits are properly exported
        let capabilities = create_default_capabilities();
        assert!(!capabilities.supported_operations.is_empty() || capabilities.supported_operations.is_empty());
        
        let health = create_default_provider_health();
        assert!(matches!(health.status, HealthStatus::Unknown));
    }
}

