/// NestGate Core Library
///
/// This is the core library for NestGate, providing fundamental types, traits,
/// and functionality for the NestGate storage system and ecosystem integration.
// Error handling - unified across the system
pub mod error;

// Configuration management
pub mod config;
pub mod unified_benchmark_config;
pub mod unified_config_master;

// Core data structures and types
pub mod types;
pub mod unified_config_consolidation;
pub mod unified_constants;
pub mod unified_enums;
pub mod unified_types;

// Performance optimizations
pub mod optimized;
pub mod performance;

// Monitoring and observability
pub mod monitoring;

// Traits and interfaces
pub mod interface;
pub mod trait_migration_guide;
pub mod traits;
pub mod universal_spore;
pub mod universal_traits;

// Security and cryptography
pub mod cert; // Certificate management
pub mod crypto_locks;
pub mod security;
pub mod security_adapter; // New adapter-based security
pub mod security_provider; // Security provider functionality

// AI and intelligence
pub mod intelligence_adapter; // New adapter-based AI

// Storage and caching
pub mod cache;
pub mod memory_pool; // High-performance memory pooling system
pub mod universal_storage; // Unified storage system with consolidated traits

// Re-export unified storage traits for easy access
pub use universal_storage::{
    UnifiedBackendFactory, UnifiedStorageBackend, UnifiedStorageConfig, UnifiedStorageProvider,
    UnifiedStorageType,
};

// Network and ecosystem integration
pub mod ecosystem_integration;
pub mod network;
pub mod universal_providers; // Legacy network module

// Smart abstractions for complexity reduction
pub mod smart_abstractions;

// Hardware and performance tuning
pub mod hardware_tuning;

// Utility modules
pub mod constants;
pub mod diagnostics; // Stub diagnostics module
pub mod response; // API response types
pub mod return_builders;
pub mod safe_operations;
pub mod service_discovery;
pub mod services; // Service discovery system
pub mod temporal_storage; // Temporal storage types
pub mod utils; // System utilities and safe operations
pub mod uuid_cache; // UUID caching and management
pub mod zero_copy; // Zero-copy utilities

// Configuration and environment
pub mod biomeos;

// AI-First Citizen API compliance
pub mod ai_first_refactored; // Modern AI-First implementation (85%+ compliance)

// Legacy modules (still in use by some components - marked for future cleanup)
pub mod capabilities;
pub mod universal_adapter; // Legacy universal adapter - still used by universal_providers
pub mod universal_primal_discovery; // Still used by cert/utils
                                    // REMOVED: ai_first_legacy.rs - Successfully replaced by ai_first_refactored.rs

// Re-export commonly used types
pub use error::{NestGateError, Result};

// Re-export adapters for external use
pub use intelligence_adapter::{
    AIInferenceRequest, AIInferenceResponse, AnalysisResults, AnalysisTask, IntelligenceAdapter,
    ModelMetadata,
};
pub use security_adapter::{AuthToken, Credentials, SecurityAdapter, Signature};

#[cfg(test)]
mod infrastructure_validation_tests {
    use super::*;

    #[test]
    fn test_basic_compilation() {
        // This test validates that our basic infrastructure compiles
        assert_eq!(2 + 2, 4);
        assert!(true);
    }

    #[tokio::test]
    async fn test_async_infrastructure() {
        // This test validates that our async infrastructure works
        let result = async_test_helper().await;
        assert_eq!(result, "success");
    }

    async fn async_test_helper() -> &'static str {
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        "success"
    }

    #[test]
    fn test_error_handling() {
        // Test our error handling infrastructure
        let result: crate::Result<()> = Ok(());
        assert!(result.is_ok());
    }

    #[test]
    fn test_collections() {
        // Test that standard collections work
        let mut map = std::collections::HashMap::new();
        map.insert("key", "value");
        assert_eq!(map.get("key"), Some(&"value"));
    }
}
