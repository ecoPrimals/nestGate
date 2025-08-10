/// ZFS Configuration Migration to Unified Pattern
/// This demonstrates migrating from fragmented ZFS config structs to the
/// standardized unified pattern, eliminating duplication and ensuring consistency.

use std::collections::HashMap;
use serde_json;

// Import the unified config consolidation patterns
use nestgate_core::unified_config_consolidation::{
    StandardDomainConfig, ZfsExtensions, migration::migrate_zfs_config
};

// **REMOVED**: Deprecated LegacyZfsConfig struct eliminated
// Use StandardDomainConfig<ZfsExtensions> directly instead

// **REMOVED**: Legacy migration function eliminated
// Use StandardDomainConfig<ZfsExtensions>::default() and configure directly
    // **REMOVED**: Legacy migration logic eliminated
    }

/// Example showing consolidation benefits
pub fn demonstration() {
    println!("🔧 ZFS Configuration Consolidation Demonstration");
    
    // Before: Multiple fragmented config structs
    let legacy_config = LegacyZfsConfig {
        pool_name: "production-pool".to_string(),
        compression: true,
        deduplication: false,
        encryption: true,
        auto_create: false,
        host: "10.0.0.1".to_string(),
        port: 8080,
        auth_token: "zfs-secret-token".to_string(),
        enable_metrics: true,
        timeout_ms: 30000,
    };
    
    // After: Unified standardized config
    let unified_config = migrate_legacy_zfs_config(legacy_config);
    
    println!("✅ Migration successful!");
    println!("   Service: {}", unified_config.service.name);
    println!("   Pool: {}", unified_config.extensions.pool_settings.default_pool_name);
    println!("   Compression: {}", unified_config.extensions.pool_settings.enable_compression);
    println!("   Network: {}:{}", unified_config.network.bind_address, unified_config.network.port);
    
    // Validate the configuration
    match nestgate_core::unified_config_consolidation::validation::validate_domain_config(&unified_config) {
        Ok(()) => println!("✅ Configuration validation passed"),
        Err(e) => println!("❌ Configuration validation failed: {}", e),
    }
    
    // Check for common issues
    let warnings = nestgate_core::unified_config_consolidation::validation::lint_domain_config(&unified_config);
    if !warnings.is_empty() {
        println!("⚠️  Configuration warnings:");
        for warning in warnings {
            println!("   - {}", warning);
    }
    }
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_zfs_config_creation() {
        // **UPDATED**: Test direct unified config creation instead of legacy migration
        let mut config = StandardDomainConfig::with_service(
            ZfsExtensions::default(),
            "test-zfs",
            "1.0.0"
        );
        
        // Configure directly using unified types
        config.extensions.pool_settings.default_pool_name = "test-pool".to_string();
        config.extensions.pool_settings.enable_compression = true;
        config.extensions.pool_settings.enable_deduplication = false;
        config.extensions.pool_settings.enable_encryption = true;
        config.extensions.pool_settings.auto_pool_creation = true;
        config.network.port = 9090;
        config.monitoring.enable_metrics = false;
        
        // Verify direct configuration
        assert_eq!(config.service.name, "test-zfs");
        assert_eq!(config.extensions.pool_settings.default_pool_name, "test-pool");
        assert_eq!(config.extensions.pool_settings.enable_compression, true);
        assert_eq!(config.extensions.pool_settings.enable_deduplication, false);
        assert_eq!(config.extensions.pool_settings.enable_encryption, true);
        assert_eq!(config.extensions.pool_settings.auto_pool_creation, true);
        assert_eq!(config.network.port, 9090);
        assert_eq!(config.monitoring.enable_metrics, false);
    }
    
    #[test] 
    fn test_config_validation() {
        let mut config = StandardDomainConfig::with_service(
            ZfsExtensions::default(),
            "test-zfs",
            "1.0.0"
        );
        config.network.port = 8080;
        
        let result = nestgate_core::unified_config_consolidation::validation::validate_domain_config(&config);
        assert!(result.is_ok());
    }
} 