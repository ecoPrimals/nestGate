//! **CONFIGURATION UNIFICATION DEMONSTRATION**
//!
//! This example demonstrates the migration from fragmented configuration
//! systems to the unified canonical configuration system.
//!
//! **SHOWS**:
//! - Migration from UnifiedApiHandlerConfig to canonical ApiConfig
//! - Consolidation of scattered handler configurations
//! - Type alias resolution and migration utilities
//! - Production vs development configuration presets

use nestgate_core::config::canonical_config::{
    NestGateCanonicalUnifiedConfig, ConfigMigrationManager,
};
use nestgate_core::config::canonical_config::migration::{
    FragmentedApiHandlerConfig, FragmentedZfsHandlerConfig, FragmentedPoolConfig,
};
use serde_json::json;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 **NESTGATE CONFIGURATION UNIFICATION DEMONSTRATION**\n");

    // ==================== PHASE 1: FRAGMENTED CONFIGURATION EXAMPLE ====================
    
    println!("📊 **PHASE 1: FRAGMENTED CONFIGURATION PATTERNS (BEFORE)**");
    println!("Current state: Multiple scattered config types across crates\n");

    // Simulate fragmented configuration from nestgate-api
    let fragmented_api_config = FragmentedApiHandlerConfig {
        zfs: Some(FragmentedZfsHandlerConfig {
            pool: Some(FragmentedPoolConfig {
                raid_level: Some("raidz2".to_string()),
                compression: Some("zstd".to_string()),
                dedup: Some(false),
                encryption: Some(true),
                properties: {
                    let mut props = HashMap::new();
                    props.insert("atime".to_string(), "off".to_string());
                    props.insert("recordsize".to_string(), "1M".to_string());
                    props
                },
                legacy_cache_size: Some(8 * 1024 * 1024 * 1024), // 8GB legacy setting
            }),
            dataset: None,
            snapshot: None,
            service: None,
        }),
        performance: None,
        dashboard: None,
        load_testing: None,
        custom_properties: {
            let mut custom = HashMap::new();
            custom.insert("experimental_feature_x".to_string(), json!(true));
            custom.insert("debug_pool_operations".to_string(), json!(false));
            custom
        },
    };

    println!("❌ **FRAGMENTED CONFIG EXAMPLE**:");
    println!("   - UnifiedApiHandlerConfig (type alias to StandardDomainConfig)");
    println!("   - ZfsHandlerConfig with legacy fields");
    println!("   - Custom properties scattered across handlers");
    println!("   - No standardized structure or validation\n");

    // ==================== PHASE 2: MIGRATION PROCESS ====================
    
    println!("🔄 **PHASE 2: MIGRATION TO CANONICAL CONFIGURATION**");
    
    let mut migration_manager = ConfigMigrationManager::new();
    
    // Migrate the fragmented API configuration
    println!("Migrating fragmented API handler configuration...");
    let canonical_api_config = migration_manager.migrate_api_handler_config(&fragmented_api_config)?;
    
    // Create full canonical configuration
    let mut canonical_config = NestGateCanonicalUnifiedConfig::default();
    canonical_config.api = canonical_api_config;
    
    println!("✅ **MIGRATION COMPLETED**");
    
    // Show migration statistics
    let summary = migration_manager.get_summary();
    println!("📈 **MIGRATION STATISTICS**:");
    println!("   - Configurations migrated: {}", summary.stats.configs_migrated);
    println!("   - Handler configs consolidated: {}", summary.stats.handler_configs_consolidated);
    println!("   - Success rate: {:.1}%", summary.success_rate);
    println!("   - Warnings generated: {}", summary.warnings_count);
    
    if !migration_manager.warnings.is_empty() {
        println!("\n⚠️  **MIGRATION WARNINGS**:");
        for warning in &migration_manager.warnings {
            println!("   - {}: {}", warning.category, warning.message);
            if let Some(source) = &warning.source {
                println!("     Source: {}", source);
            }
            println!("     Action: {}", warning.suggested_action);
        }
    }
    println!();

    // ==================== PHASE 3: CANONICAL CONFIGURATION BENEFITS ====================
    
    println!("🎯 **PHASE 3: CANONICAL CONFIGURATION BENEFITS**");
    
    println!("✅ **UNIFIED STRUCTURE**:");
    println!("   - Single NestGateCanonicalUnifiedConfig for entire system");
    println!("   - ZFS config: RAID-{}, compression={}, encryption={}",
        canonical_config.api.zfs_handlers.pool.raid_level.as_ref().unwrap_or(&"default".to_string()),
        canonical_config.api.zfs_handlers.pool.compression.as_ref().unwrap_or(&"default".to_string()),
        canonical_config.api.zfs_handlers.pool.encryption.unwrap_or(false)
    );
    println!("   - Legacy cache_size migrated to arc_max: {} bytes",
        canonical_config.api.zfs_handlers.pool.arc_max.unwrap_or(0)
    );
    println!("   - Custom properties preserved in handler_extensions");
    
    println!("\n✅ **TYPE SAFETY**:");
    println!("   - Compile-time validation of all configuration");
    println!("   - No runtime configuration parsing errors");
    println!("   - Automatic default value application");
    
    println!("\n✅ **ENVIRONMENT INTEGRATION**:");
    println!("   - Environment variables: {:?}", canonical_config.environment.env_vars);
    println!("   - Configuration files: {:?}", canonical_config.environment.config_files);
    
    // ==================== PHASE 4: CONFIGURATION PRESETS ====================
    
    println!("\n🚀 **PHASE 4: CONFIGURATION PRESETS**");
    
    // Production configuration
    println!("🏭 **PRODUCTION CONFIGURATION**:");
    let prod_config = NestGateCanonicalUnifiedConfig::production();
    println!("   - Security hardening: {}", prod_config.features.security_hardening);
    println!("   - Debug mode: {}", prod_config.features.debug_mode);
    println!("   - Performance monitoring: {}", prod_config.features.performance_monitoring);
    println!("   - Authentication required: {}", prod_config.api.handler_extensions.security.require_auth);
    println!("   - ZFS monitoring enabled: {}", prod_config.api.zfs_handlers.performance.monitoring_enabled);
    
    // Development configuration
    println!("\n🛠️  **DEVELOPMENT CONFIGURATION**:");
    let dev_config = NestGateCanonicalUnifiedConfig::development();
    println!("   - Experimental features: {}", dev_config.features.experimental);
    println!("   - Debug endpoints: {}", dev_config.api.handler_extensions.feature_flags.debug_endpoints);
    println!("   - Authentication required: {}", dev_config.api.handler_extensions.security.require_auth);
    println!("   - Security hardening: {}", dev_config.features.security_hardening);
    
    // ==================== PHASE 5: CONFIGURATION MERGING ====================
    
    println!("\n🔀 **PHASE 5: CONFIGURATION MERGING**");
    
    // Demonstrate configuration merging
    let merged_config = canonical_config.merge(dev_config);
    println!("✅ **MERGED CONFIGURATION**:");
    println!("   - Combines migrated config with development preset");
    println!("   - ZFS RAID level preserved: {}", 
        merged_config.api.zfs_handlers.pool.raid_level.as_ref().unwrap_or(&"default".to_string()));
    println!("   - Debug endpoints enabled: {}", 
        merged_config.api.handler_extensions.feature_flags.debug_endpoints);
    println!("   - Custom properties count: {}", 
        merged_config.api.handler_extensions.custom_handlers.len());
    
    // ==================== SUMMARY ====================
    
    println!("\n🎉 **CONFIGURATION UNIFICATION COMPLETE**");
    println!("📈 **BENEFITS ACHIEVED**:");
    println!("   ✅ Single source of truth for all configuration");
    println!("   ✅ Type-safe configuration with compile-time validation");
    println!("   ✅ Automatic migration from fragmented configs");
    println!("   ✅ Production and development presets");
    println!("   ✅ Environment variable integration");
    println!("   ✅ Configuration merging and composition");
    println!("   ✅ Comprehensive migration warnings and statistics");
    
    println!("\n🔄 **MIGRATION PATH**:");
    println!("   1. Identify fragmented configuration types");
    println!("   2. Use ConfigMigrationManager to migrate");
    println!("   3. Apply appropriate configuration preset");
    println!("   4. Merge configurations as needed");
    println!("   5. Deploy with single canonical configuration");
    
    println!("\n🎯 **NEXT STEPS**:");
    println!("   - Implement automation config migration");
    println!("   - Add error system consolidation");
    println!("   - Apply zero-cost trait unification");
    println!("   - Complete technical debt elimination");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_configuration_migration() {
        let fragmented_config = FragmentedApiHandlerConfig {
            zfs: Some(FragmentedZfsHandlerConfig {
                pool: Some(FragmentedPoolConfig {
                    raid_level: Some("mirror".to_string()),
                    compression: Some("lz4".to_string()),
                    dedup: Some(false),
                    encryption: Some(false),
                    properties: HashMap::new(),
                    legacy_cache_size: Some(1024 * 1024 * 1024), // 1GB
                }),
                dataset: None,
                snapshot: None,
                service: None,
            }),
            performance: None,
            dashboard: None,
            load_testing: None,
            custom_properties: HashMap::new(),
        };

        let mut migration_manager = ConfigMigrationManager::new();
        let result = migration_manager.migrate_api_handler_config(&fragmented_config);
        
        assert!(result.is_ok());
        let api_config = result.unwrap();
        
        // Verify migration
        assert_eq!(api_config.zfs_handlers.pool.raid_level, Some("mirror".to_string()));
        assert_eq!(api_config.zfs_handlers.pool.compression, Some("lz4".to_string()));
        assert_eq!(api_config.zfs_handlers.pool.arc_max, Some(1024 * 1024 * 1024));
        
        // Check migration statistics
        let summary = migration_manager.get_summary();
        assert_eq!(summary.stats.configs_migrated, 1);
        assert_eq!(summary.stats.handler_configs_consolidated, 1);
    }

    #[test]
    fn test_configuration_presets() {
        let prod_config = NestGateCanonicalUnifiedConfig::production();
        let dev_config = NestGateCanonicalUnifiedConfig::development();
        
        // Production should be secure
        assert!(!prod_config.features.experimental);
        assert!(!prod_config.features.debug_mode);
        assert!(prod_config.features.security_hardening);
        assert!(prod_config.api.handler_extensions.security.require_auth);
        
        // Development should be permissive
        assert!(dev_config.features.experimental);
        assert!(dev_config.features.debug_mode);
        assert!(!dev_config.features.security_hardening);
        assert!(!dev_config.api.handler_extensions.security.require_auth);
    }

    #[test]
    fn test_configuration_merging() {
        let mut config1 = NestGateCanonicalUnifiedConfig::default();
        config1.api.zfs_handlers.pool.raid_level = Some("raidz1".to_string());
        
        let mut config2 = NestGateCanonicalUnifiedConfig::default();
        config2.features.debug_mode = true;
        
        let merged = config1.merge(config2);
        
        // Should preserve both settings
        assert_eq!(merged.api.zfs_handlers.pool.raid_level, Some("raidz1".to_string()));
        assert!(merged.features.debug_mode);
    }
} 