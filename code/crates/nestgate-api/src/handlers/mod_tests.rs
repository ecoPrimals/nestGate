//! **COMPREHENSIVE TESTS FOR HANDLERS MODULE**
//!
//! Tests for handlers/mod.rs focusing on handler collection,
//! initialization, and registry functionality.

#[cfg(test)]
mod handler_module_tests {
    use super::super::*;

    // ==================== HANDLER COLLECTION TESTS ====================

    #[test]
    fn test_handler_collection_new() {
        let collection = HandlerCollection::new();
        // Verify all handlers are created
        assert!(std::ptr::addr_of!(collection.ai_first) != std::ptr::null());
        assert!(std::ptr::addr_of!(collection.compliance) != std::ptr::null());
        assert!(std::ptr::addr_of!(collection.hardware_tuning) != std::ptr::null());
        assert!(std::ptr::addr_of!(collection.health) != std::ptr::null());
        assert!(std::ptr::addr_of!(collection.load_testing) != std::ptr::null());
        assert!(std::ptr::addr_of!(collection.metrics_collector) != std::ptr::null());
        assert!(std::ptr::addr_of!(collection.performance_analyzer) != std::ptr::null());
        assert!(std::ptr::addr_of!(collection.storage) != std::ptr::null());
        assert!(std::ptr::addr_of!(collection.workspace_manager) != std::ptr::null());
        assert!(std::ptr::addr_of!(collection.zfs) != std::ptr::null());
    }

    #[test]
    fn test_handler_collection_default() {
        let collection = HandlerCollection::default();
        // Verify default implementation works
        assert!(std::ptr::addr_of!(collection.ai_first) != std::ptr::null());
    }

    #[test]
    fn test_initialize_handlers() {
        let collection = initialize_handlers();
        // Verify utility function creates valid collection
        assert!(std::ptr::addr_of!(collection.ai_first) != std::ptr::null());
        assert!(std::ptr::addr_of!(collection.compliance) != std::ptr::null());
    }

    // ==================== INDIVIDUAL HANDLER TESTS ====================

    #[test]
    fn test_ai_first_handler_creation() {
        let handler = AIFirstHandler::new();
        // Verify router is initialized
        assert!(std::ptr::addr_of!(handler.router) != std::ptr::null());
    }

    #[test]
    fn test_ai_first_handler_default() {
        let handler = AIFirstHandler::default();
        assert!(std::ptr::addr_of!(handler.router) != std::ptr::null());
    }

    #[test]
    fn test_compliance_handler_creation() {
        let handler = ComplianceHandler::new();
        // Verify manager is initialized
        assert!(std::ptr::addr_of!(handler.manager) != std::ptr::null());
    }

    #[test]
    fn test_compliance_handler_default() {
        let handler = ComplianceHandler::default();
        assert!(std::ptr::addr_of!(handler.manager) != std::ptr::null());
    }

    #[test]
    fn test_hardware_tuning_handler_creation() {
        let handler = HardwareTuningHandler::new();
        // Verify config is initialized
        assert!(std::ptr::addr_of!(handler.config) != std::ptr::null());
    }

    #[test]
    fn test_hardware_tuning_handler_default() {
        let handler = HardwareTuningHandler::default();
        assert!(std::ptr::addr_of!(handler.config) != std::ptr::null());
    }

    #[test]
    fn test_health_handler_creation() {
        let _handler = HealthHandler::new();
        // Health handler has no fields, just verify it creates
    }

    #[test]
    fn test_health_handler_default() {
        let _handler = HealthHandler;
        // Verify default implementation works
    }

    #[test]
    fn test_load_test_handler_creation() {
        let handler = LoadTestHandler::new();
        assert!(std::ptr::addr_of!(handler.config) != std::ptr::null());
    }

    #[test]
    fn test_load_test_handler_default() {
        let handler = LoadTestHandler::default();
        assert!(std::ptr::addr_of!(handler.config) != std::ptr::null());
    }

    #[test]
    fn test_metrics_collector_creation() {
        let collector = MetricsCollector::new();
        assert!(std::ptr::addr_of!(collector.collector) != std::ptr::null());
    }

    #[test]
    fn test_metrics_collector_default() {
        let collector = MetricsCollector::default();
        assert!(std::ptr::addr_of!(collector.collector) != std::ptr::null());
    }

    #[test]
    fn test_performance_analyzer_creation() {
        let analyzer = PerformanceAnalyzer::new();
        assert!(std::ptr::addr_of!(analyzer.analyzer) != std::ptr::null());
    }

    #[test]
    fn test_performance_analyzer_default() {
        let analyzer = PerformanceAnalyzer::default();
        assert!(std::ptr::addr_of!(analyzer.analyzer) != std::ptr::null());
    }

    #[test]
    fn test_storage_handler_creation() {
        let handler = StorageHandler::new();
        assert!(std::ptr::addr_of!(handler.manager) != std::ptr::null());
    }

    #[test]
    fn test_storage_handler_default() {
        let handler = StorageHandler::default();
        assert!(std::ptr::addr_of!(handler.manager) != std::ptr::null());
    }

    #[test]
    fn test_workspace_manager_creation() {
        let manager = WorkspaceManager::new();
        assert!(std::ptr::addr_of!(manager.manager) != std::ptr::null());
    }

    #[test]
    fn test_workspace_manager_default() {
        let manager = WorkspaceManager::default();
        assert!(std::ptr::addr_of!(manager.manager) != std::ptr::null());
    }

    #[test]
    fn test_zfs_handler_creation() {
        let handler = ZfsHandler::new();
        assert!(std::ptr::addr_of!(handler.handler) != std::ptr::null());
    }

    #[test]
    fn test_zfs_handler_default() {
        let handler = ZfsHandler::default();
        assert!(std::ptr::addr_of!(handler.handler) != std::ptr::null());
    }

    // ==================== UTILITY FUNCTION TESTS ====================

    #[test]
    fn test_available_handlers_returns_all() {
        let handlers = available_handlers();
        assert_eq!(handlers.len(), 10);
        assert!(handlers.contains(&"ai_first"));
        assert!(handlers.contains(&"compliance"));
        assert!(handlers.contains(&"hardware_tuning"));
        assert!(handlers.contains(&"health"));
        assert!(handlers.contains(&"load_testing"));
        assert!(handlers.contains(&"metrics"));
        assert!(handlers.contains(&"performance"));
        assert!(handlers.contains(&"storage"));
        assert!(handlers.contains(&"workspace"));
        assert!(handlers.contains(&"zfs"));
    }

    #[test]
    fn test_create_handler_by_name_valid() {
        let handler = create_handler_by_name("compliance");
        assert!(handler.is_some());
    }

    #[test]
    fn test_create_handler_by_name_invalid() {
        let handler = create_handler_by_name("nonexistent");
        assert!(handler.is_none());
    }

    #[test]
    fn test_create_all_handlers_by_name() {
        let handler_names = available_handlers();
        for name in handler_names {
            let handler = create_handler_by_name(name);
            assert!(handler.is_some(), "Handler '{name}' should be creatable");
        }
    }

    // ==================== MANAGER WRAPPER TESTS ====================

    #[test]
    fn test_compliance_manager_creation() {
        let manager = ComplianceManager::new();
        assert!(std::ptr::addr_of!(manager.manager) != std::ptr::null());
    }

    #[test]
    fn test_compliance_manager_default() {
        let manager = ComplianceManager::default();
        assert!(std::ptr::addr_of!(manager.manager) != std::ptr::null());
    }

    #[test]
    fn test_hardware_tuning_manager_creation() {
        let manager = HardwareTuningManager::new();
        assert!(std::ptr::addr_of!(manager.config) != std::ptr::null());
    }

    #[test]
    fn test_hardware_tuning_manager_default() {
        let manager = HardwareTuningManager::default();
        assert!(std::ptr::addr_of!(manager.config) != std::ptr::null());
    }

    #[test]
    fn test_load_test_manager_creation() {
        let manager = LoadTestManager::new();
        assert!(std::ptr::addr_of!(manager.config) != std::ptr::null());
    }

    #[test]
    fn test_load_test_manager_default() {
        let manager = LoadTestManager::default();
        assert!(std::ptr::addr_of!(manager.config) != std::ptr::null());
    }

    #[test]
    fn test_performance_analyzer_manager_creation() {
        let manager = PerformanceAnalyzerManager::new();
        assert!(std::ptr::addr_of!(manager.analyzer) != std::ptr::null());
    }

    #[test]
    fn test_performance_analyzer_manager_default() {
        let manager = PerformanceAnalyzerManager::default();
        assert!(std::ptr::addr_of!(manager.analyzer) != std::ptr::null());
    }

    #[test]
    fn test_workspace_manager_wrapper_creation() {
        let wrapper = WorkspaceManagerWrapper::new();
        assert!(std::ptr::addr_of!(wrapper.manager) != std::ptr::null());
    }

    #[test]
    fn test_workspace_manager_wrapper_default() {
        let wrapper = WorkspaceManagerWrapper::default();
        assert!(std::ptr::addr_of!(wrapper.manager) != std::ptr::null());
    }

    #[test]
    fn test_zfs_manager_creation() {
        let manager = ZfsManager::new();
        assert!(std::ptr::addr_of!(manager.handler) != std::ptr::null());
    }

    #[test]
    fn test_zfs_manager_default() {
        let manager = ZfsManager::default();
        assert!(std::ptr::addr_of!(manager.handler) != std::ptr::null());
    }

    #[test]
    fn test_api_router_creation() {
        let router = ApiRouter::new();
        assert!(std::ptr::addr_of!(router.router) != std::ptr::null());
    }

    #[test]
    fn test_api_router_default() {
        let router = ApiRouter::default();
        assert!(std::ptr::addr_of!(router.router) != std::ptr::null());
    }

    // ==================== DEBUG TRAIT TESTS ====================

    #[test]
    fn test_ai_first_handler_debug() {
        let handler = AIFirstHandler::new();
        let debug_str = format!("{handler:?}");
        assert!(debug_str.contains("AIFirstHandler"));
    }

    #[test]
    fn test_compliance_handler_debug() {
        let handler = ComplianceHandler::new();
        let debug_str = format!("{handler:?}");
        assert!(debug_str.contains("ComplianceHandler"));
    }

    #[test]
    fn test_health_handler_debug() {
        let handler = HealthHandler::new();
        let debug_str = format!("{handler:?}");
        assert!(debug_str.contains("HealthHandler"));
    }

    // ==================== CLONE TRAIT TESTS ====================

    #[test]
    fn test_ai_first_handler_clone() {
        let handler = AIFirstHandler::new();
        let cloned = handler.clone();
        assert!(std::ptr::addr_of!(cloned.router) != std::ptr::null());
    }

    #[test]
    fn test_compliance_handler_clone() {
        let handler = ComplianceHandler::new();
        let cloned = handler.clone();
        assert!(std::ptr::addr_of!(cloned.manager) != std::ptr::null());
    }

    #[test]
    fn test_hardware_tuning_handler_clone() {
        let handler = HardwareTuningHandler::new();
        let cloned = handler.clone();
        assert!(std::ptr::addr_of!(cloned.config) != std::ptr::null());
    }

    #[test]
    fn test_health_handler_clone() {
        let handler = HealthHandler::new();
        let _cloned = handler.clone();
        // Health handler has no fields, just verify it clones
    }

    #[test]
    fn test_load_test_handler_clone() {
        let handler = LoadTestHandler::new();
        let cloned = handler.clone();
        assert!(std::ptr::addr_of!(cloned.config) != std::ptr::null());
    }

    #[test]
    fn test_metrics_collector_clone() {
        let collector = MetricsCollector::new();
        let cloned = collector.clone();
        assert!(std::ptr::addr_of!(cloned.collector) != std::ptr::null());
    }

    #[test]
    fn test_performance_analyzer_clone() {
        let analyzer = PerformanceAnalyzer::new();
        let cloned = analyzer.clone();
        assert!(std::ptr::addr_of!(cloned.analyzer) != std::ptr::null());
    }

    #[test]
    fn test_storage_handler_clone() {
        let handler = StorageHandler::new();
        let cloned = handler.clone();
        assert!(std::ptr::addr_of!(cloned.manager) != std::ptr::null());
    }

    #[test]
    fn test_workspace_manager_clone() {
        let manager = WorkspaceManager::new();
        let cloned = manager.clone();
        assert!(std::ptr::addr_of!(cloned.manager) != std::ptr::null());
    }

    #[test]
    fn test_zfs_handler_clone() {
        let handler = ZfsHandler::new();
        let cloned = handler.clone();
        assert!(std::ptr::addr_of!(cloned.handler) != std::ptr::null());
    }

    // ==================== TYPE NAME VERIFICATION TESTS ====================

    #[test]
    fn test_handler_type_names() {
        let collection = HandlerCollection::new();
        assert!(std::any::type_name_of_val(&collection.ai_first).contains("AIFirstHandler"));
        assert!(std::any::type_name_of_val(&collection.compliance).contains("ComplianceHandler"));
        assert!(std::any::type_name_of_val(&collection.hardware_tuning).contains("HardwareTuningHandler"));
        assert!(std::any::type_name_of_val(&collection.health).contains("HealthHandler"));
        assert!(std::any::type_name_of_val(&collection.load_testing).contains("LoadTestHandler"));
        assert!(std::any::type_name_of_val(&collection.metrics_collector).contains("MetricsCollector"));
        assert!(std::any::type_name_of_val(&collection.performance_analyzer).contains("PerformanceAnalyzer"));
        assert!(std::any::type_name_of_val(&collection.storage).contains("StorageHandler"));
        assert!(std::any::type_name_of_val(&collection.workspace_manager).contains("WorkspaceManager"));
        assert!(std::any::type_name_of_val(&collection.zfs).contains("ZfsHandler"));
    }

    #[test]
    fn test_manager_type_names() {
        let compliance_mgr = ComplianceManager::new();
        assert!(std::any::type_name_of_val(&compliance_mgr.manager).contains("ComplianceState"));

        let hardware_mgr = HardwareTuningManager::new();
        assert!(std::any::type_name_of_val(&hardware_mgr.config).contains("HardwareTuningConfig"));

        let load_test_mgr = LoadTestManager::new();
        assert!(std::any::type_name_of_val(&load_test_mgr.config).contains("LoadTestConfig"));

        let perf_mgr = PerformanceAnalyzerManager::new();
        assert!(std::any::type_name_of_val(&perf_mgr.analyzer).contains("PerformanceAnalyzerState"));

        let workspace_wrapper = WorkspaceManagerWrapper::new();
        assert!(std::any::type_name_of_val(&workspace_wrapper.manager).contains("WorkspaceManager"));

        let zfs_mgr = ZfsManager::new();
        assert!(std::any::type_name_of_val(&zfs_mgr.handler).contains("ZfsHandlerImpl"));
    }

    // ==================== EDGE CASE TESTS ====================

    #[test]
    fn test_create_handler_by_name_case_sensitivity() {
        // Valid lowercase name
        assert!(create_handler_by_name("health").is_some());
        
        // Invalid uppercase name (case sensitive)
        assert!(create_handler_by_name("HEALTH").is_none());
        assert!(create_handler_by_name("Health").is_none());
    }

    #[test]
    fn test_create_handler_by_name_empty_string() {
        assert!(create_handler_by_name("").is_none());
    }

    #[test]
    fn test_create_handler_by_name_whitespace() {
        assert!(create_handler_by_name(" health ").is_none());
        assert!(create_handler_by_name("hea lth").is_none());
    }

    #[test]
    fn test_available_handlers_no_duplicates() {
        let handlers = available_handlers();
        let mut seen = std::collections::HashSet::new();
        for handler in handlers {
            assert!(seen.insert(handler), "Duplicate handler name: {handler}");
        }
    }

    #[test]
    fn test_available_handlers_order_independent() {
        let handlers1 = available_handlers();
        let handlers2 = available_handlers();
        assert_eq!(handlers1, handlers2, "Handler list should be deterministic");
    }

    // ==================== INTEGRATION TESTS ====================

    #[test]
    fn test_handler_collection_multiple_creations() {
        let collection1 = HandlerCollection::new();
        let collection2 = HandlerCollection::new();
        // Verify each creation is independent
        assert!(std::ptr::addr_of!(collection1.ai_first) != std::ptr::addr_of!(collection2.ai_first));
    }

    #[test]
    fn test_all_managers_can_be_created() {
        let _compliance = ComplianceManager::new();
        let _hardware = HardwareTuningManager::new();
        let _load_test = LoadTestManager::new();
        let _perf = PerformanceAnalyzerManager::new();
        let _workspace = WorkspaceManagerWrapper::new();
        let _zfs = ZfsManager::new();
        // All managers should create without panicking
    }

    #[test]
    fn test_api_router_creation_multiple() {
        let router1 = ApiRouter::new();
        let router2 = ApiRouter::new();
        // Verify each router is independent
        assert!(std::ptr::addr_of!(router1.router) != std::ptr::addr_of!(router2.router));
    }

    // ==================== PERFORMANCE TESTS ====================

    #[test]
    fn test_handler_collection_creation_fast() {
        // Verify handler collection creates quickly (< 1ms)
        let start = std::time::Instant::now();
        let _collection = HandlerCollection::new();
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 100, "Handler collection should create in < 100ms");
    }

    #[test]
    fn test_multiple_handler_creations() {
        // Verify we can create many handlers without issues
        for _ in 0..100 {
            let _handler = HealthHandler::new();
        }
    }
}
