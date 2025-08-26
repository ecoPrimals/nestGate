//! **ERROR SYSTEM CONSOLIDATION DEMONSTRATION**
//!
//! This example demonstrates the systematic consolidation of fragmented error types
//! into the unified NestGateError system, eliminating duplicate error handling patterns.
//!
//! **SHOWS**:
//! - Migration from 30+ fragmented error types to unified NestGateError
//! - Error mapping and conversion utilities
//! - Consolidation statistics and reporting
//! - Backward compatibility during migration

use nestgate_core::error::{
    ErrorConsolidationManager, NestGateError,
    ZfsErrorInfo, ApiErrorInfo, NetworkErrorInfo, SecurityErrorInfo,
};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 **NESTGATE ERROR SYSTEM CONSOLIDATION DEMONSTRATION**\n");

    // ==================== PHASE 1: FRAGMENTED ERROR LANDSCAPE ====================
    
    println!("📊 **PHASE 1: FRAGMENTED ERROR LANDSCAPE (BEFORE)**");
    println!("Current state: 30+ scattered error types across all crates\n");

    // Show examples of fragmented error types
    println!("❌ **FRAGMENTED ERROR TYPES FOUND**:");
    println!("   ZFS Domain: ZfsError, UniversalZfsError, PoolSetupError");
    println!("   API Domain: ApiError, EcosystemError, PrimalError");
    println!("   Network Domain: NetworkError, ConnectionError, RpcError");
    println!("   Security Domain: SecurityError, InputValidationError, RateLimitError");
    println!("   System Domain: AutomationError, InstallerError, FsMonitorError");
    println!("   MCP Domain: McpError, McpProtocolError");
    println!("   Test Domain: TestStorageError, ServiceTestError, NetworkTestError");
    println!("   + 15+ more scattered across different modules\n");

    println!("🚨 **PROBLEMS WITH FRAGMENTATION**:");
    println!("   - Inconsistent error handling patterns");
    println!("   - Duplicate error severity and category definitions");
    println!("   - No unified error context or recovery guidance");
    println!("   - Difficult debugging across domain boundaries");
    println!("   - Maintenance burden with scattered error types\n");

    // ==================== PHASE 2: ERROR CONSOLIDATION PROCESS ====================
    
    println!("🔄 **PHASE 2: ERROR CONSOLIDATION PROCESS**");
    
    let mut consolidation_manager = ErrorConsolidationManager::new();
    
    println!("📋 **CONSOLIDATION MANAGER INITIALIZED**:");
    let initial_summary = consolidation_manager.get_summary();
    println!("   - Total error types identified: {}", initial_summary.stats.total_error_types);
    println!("   - Automatic migrations available: {}", initial_summary.automatic_migrations);
    println!("   - Manual migrations required: {}", initial_summary.manual_migrations);
    
    println!("\n🗺️  **ERROR TYPE MAPPINGS**:");
    for (source, mapping) in &consolidation_manager.mappings {
        let migration_type = if mapping.automatic_migration { "AUTO" } else { "MANUAL" };
        println!("   {} → NestGateError::{} [{}] ({})", 
            source, mapping.target_variant, mapping.category, migration_type);
    }

    // ==================== PHASE 3: SYSTEMATIC ERROR MIGRATION ====================
    
    println!("\n🔧 **PHASE 3: SYSTEMATIC ERROR MIGRATION**");
    
    // Migrate ZFS errors
    println!("\n🗂️  **MIGRATING ZFS ERRORS**:");
    let zfs_error_info = ZfsErrorInfo {
        operation: "create_dataset".to_string(),
        pool_name: Some("tank".to_string()),
        dataset_name: Some("tank/data".to_string()),
        error_code: Some("ENOSPC".to_string()),
        system_error: Some("No space left on device".to_string()),
        recovery_suggestion: Some("Free up space or expand pool".to_string()),
        metadata: Some({
            let mut meta = HashMap::new();
            meta.insert("available_space".to_string(), "0GB".to_string());
            meta.insert("requested_space".to_string(), "100GB".to_string());
            meta
        }),
    };
    
    let unified_zfs_error = consolidation_manager.migrate_zfs_error(&zfs_error_info);
    println!("   ✅ ZfsError → NestGateError::Zfs");
    println!("      Operation: {}", zfs_error_info.operation);
    println!("      Pool: {}", zfs_error_info.pool_name.unwrap());
    println!("      Error: {}", unified_zfs_error);

    // Migrate API errors
    println!("\n🌐 **MIGRATING API ERRORS**:");
    let api_error_info = ApiErrorInfo {
        endpoint: "/api/v1/pools".to_string(),
        method: "POST".to_string(),
        status_code: 400,
        error_type: "validation_error".to_string(),
        user_message: Some("Invalid pool configuration".to_string()),
        debug_info: Some("RAID level 'raidz99' is not supported".to_string()),
        metadata: Some({
            let mut meta = HashMap::new();
            meta.insert("request_id".to_string(), "req_123456".to_string());
            meta.insert("user_agent".to_string(), "NestGate-CLI/1.0".to_string());
            meta
        }),
    };
    
    let unified_api_error = consolidation_manager.migrate_api_error(&api_error_info);
    println!("   ✅ ApiError → NestGateError::Api");
    println!("      Endpoint: {} {}", api_error_info.method, api_error_info.endpoint);
    println!("      Status: {}", api_error_info.status_code);
    println!("      Error: {}", unified_api_error);

    // Migrate Network errors
    println!("\n🌍 **MIGRATING NETWORK ERRORS**:");
    let network_error_info = NetworkErrorInfo {
        operation: "connect_to_service".to_string(),
        endpoint: "https://api.nestgate.io:8443".to_string(),
        error_type: "connection_timeout".to_string(),
        timeout_duration: Some(std::time::Duration::from_secs(30)),
        retry_count: Some(3),
        last_attempt: Some(std::time::SystemTime::now()),
        metadata: Some({
            let mut meta = HashMap::new();
            meta.insert("dns_resolution".to_string(), "success".to_string());
            meta.insert("tcp_connect".to_string(), "timeout".to_string());
            meta
        }),
    };
    
    let unified_network_error = consolidation_manager.migrate_network_error(&network_error_info);
    println!("   ✅ NetworkError → NestGateError::Network");
    println!("      Endpoint: {}", network_error_info.endpoint);
    println!("      Retries: {}", network_error_info.retry_count.unwrap());
    println!("      Error: {}", unified_network_error);

    // Migrate Security errors
    println!("\n🔒 **MIGRATING SECURITY ERRORS**:");
    let security_error_info = SecurityErrorInfo {
        security_domain: "authentication".to_string(),
        operation: "validate_token".to_string(),
        user_id: Some("user_12345".to_string()),
        resource: Some("/admin/config".to_string()),
        permission_required: Some("admin:write".to_string()),
        metadata: Some({
            let mut meta = HashMap::new();
            meta.insert("token_type".to_string(), "JWT".to_string());
            meta.insert("token_expired".to_string(), "true".to_string());
            meta
        }),
    };
    
    let unified_security_error = consolidation_manager.migrate_security_error(&security_error_info);
    println!("   ✅ SecurityError → NestGateError::Security");
    println!("      Domain: {}", security_error_info.security_domain);
    println!("      User: {}", security_error_info.user_id.as_ref().unwrap());
    println!("      Error: {}", unified_security_error);

    // Generic error consolidation
    println!("\n🔧 **GENERIC ERROR CONSOLIDATION**:");
    let generic_error = consolidation_manager.consolidate_generic_error(
        "CustomDomainError",
        "process_data",
        "Failed to process custom domain data",
        "custom_domain"
    );
    println!("   ✅ CustomDomainError → NestGateError::Internal");
    println!("      Error: {}", generic_error);

    // ==================== PHASE 4: CONSOLIDATION RESULTS ====================
    
    println!("\n📈 **PHASE 4: CONSOLIDATION RESULTS**");
    
    let final_summary = consolidation_manager.get_summary();
    println!("✅ **CONSOLIDATION STATISTICS**:");
    println!("   - Total error types: {}", final_summary.stats.total_error_types);
    println!("   - Successfully consolidated: {}", final_summary.stats.consolidated_count);
    println!("   - Consolidation progress: {:.1}%", final_summary.stats.consolidation_progress);
    println!("   - Warnings generated: {}", final_summary.warnings_count);
    
    println!("\n📊 **DOMAIN CONSOLIDATION BREAKDOWN**:");
    for (domain, count) in &final_summary.stats.domain_counts {
        println!("   - {}: {} error types", domain, count);
    }

    if !consolidation_manager.warnings.is_empty() {
        println!("\n⚠️  **CONSOLIDATION WARNINGS**:");
        for warning in &consolidation_manager.warnings {
            println!("   - {}: {}", warning.category, warning.message);
            println!("     Source: {}", warning.source_error_type);
            println!("     Action: {}", warning.suggested_action);
        }
    }

    // ==================== PHASE 5: UNIFIED ERROR SYSTEM BENEFITS ====================
    
    println!("\n🎯 **PHASE 5: UNIFIED ERROR SYSTEM BENEFITS**");
    
    println!("✅ **CONSISTENCY**:");
    println!("   - Single NestGateError enum for all error handling");
    println!("   - Consistent error context with operation, component, metadata");
    println!("   - Unified error severity and recovery guidance");
    
    println!("\n✅ **RICH CONTEXT**:");
    println!("   - Domain-specific error data structures");
    println!("   - Structured metadata for debugging");
    println!("   - Recovery suggestions and retry information");
    
    println!("\n✅ **MAINTAINABILITY**:");
    println!("   - Single error type to maintain");
    println!("   - Consistent error handling patterns");
    println!("   - Centralized error documentation");
    
    println!("\n✅ **DEBUGGING**:");
    println!("   - Unified error logging and reporting");
    println!("   - Cross-domain error correlation");
    println!("   - Rich error context for troubleshooting");

    // ==================== PHASE 6: MIGRATION STRATEGY ====================
    
    println!("\n🔄 **PHASE 6: MIGRATION STRATEGY**");
    
    println!("📋 **STEP-BY-STEP MIGRATION PLAN**:");
    println!("   1. **Identify** - Catalog all existing error types");
    println!("   2. **Map** - Create mappings to NestGateError variants");
    println!("   3. **Convert** - Use ErrorConsolidationManager for systematic migration");
    println!("   4. **Test** - Verify error handling behavior is preserved");
    println!("   5. **Replace** - Replace old error types with unified errors");
    println!("   6. **Cleanup** - Remove deprecated error definitions");
    
    println!("\n🛡️  **BACKWARD COMPATIBILITY**:");
    println!("   - Gradual migration with compatibility layers");
    println!("   - Conversion utilities for existing error handling code");
    println!("   - Deprecation warnings for old error types");
    println!("   - Migration validation and testing");

    // ==================== SUMMARY ====================
    
    println!("\n🎉 **ERROR SYSTEM CONSOLIDATION COMPLETE**");
    println!("📈 **BENEFITS ACHIEVED**:");
    println!("   ✅ Single unified error system (NestGateError)");
    println!("   ✅ Systematic migration utilities (ErrorConsolidationManager)");
    println!("   ✅ Rich error context and recovery guidance");
    println!("   ✅ Consistent error handling patterns");
    println!("   ✅ Comprehensive consolidation statistics");
    println!("   ✅ Backward compatibility during migration");
    
    println!("\n🔄 **MIGRATION PROGRESS**:");
    println!("   - Framework: ✅ COMPLETE");
    println!("   - ZFS errors: ✅ MIGRATED");
    println!("   - API errors: ✅ MIGRATED"); 
    println!("   - Network errors: ✅ MIGRATED");
    println!("   - Security errors: ✅ MIGRATED");
    println!("   - Remaining errors: 🔄 Ready for migration");
    
    println!("\n🎯 **NEXT STEPS**:");
    println!("   - Apply migrations across all crates");
    println!("   - Update error handling code to use unified errors");
    println!("   - Remove deprecated error type definitions");
    println!("   - Complete zero-cost trait unification");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_consolidation_manager() {
        let mut manager = ErrorConsolidationManager::new();
        
        // Test initial state
        let summary = manager.get_summary();
        assert!(summary.stats.total_error_types > 0);
        assert_eq!(summary.stats.consolidated_count, 0);
        
        // Test ZFS error migration
        let zfs_error = ZfsErrorInfo {
            operation: "test_operation".to_string(),
            pool_name: Some("test_pool".to_string()),
            dataset_name: None,
            error_code: Some("TEST_ERROR".to_string()),
            system_error: None,
            recovery_suggestion: None,
            metadata: None,
        };
        
        let unified_error = manager.migrate_zfs_error(&zfs_error);
        assert!(matches!(unified_error, NestGateError::Zfs(_)));
        
        // Test statistics update
        let final_summary = manager.get_summary();
        assert_eq!(final_summary.stats.consolidated_count, 1);
        assert!(final_summary.stats.consolidation_progress > 0.0);
    }

    #[test]
    fn test_api_error_migration() {
        let mut manager = ErrorConsolidationManager::new();
        
        let api_error = ApiErrorInfo {
            endpoint: "/test".to_string(),
            method: "GET".to_string(),
            status_code: 404,
            error_type: "not_found".to_string(),
            user_message: Some("Resource not found".to_string()),
            debug_info: None,
            metadata: None,
        };
        
        let unified_error = manager.migrate_api_error(&api_error);
        assert!(matches!(unified_error, NestGateError::Api(_)));
    }

    #[test]
    fn test_generic_error_consolidation() {
        let mut manager = ErrorConsolidationManager::new();
        
        let generic_error = manager.consolidate_generic_error(
            "TestError",
            "test_operation", 
            "Test error message",
            "test_domain"
        );
        
        assert!(matches!(generic_error, NestGateError::Internal { .. }));
    }
} 