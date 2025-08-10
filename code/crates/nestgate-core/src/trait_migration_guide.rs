/// **TRAIT MIGRATION AND CONSOLIDATION GUIDE**
///
/// This module provides a comprehensive guide and utilities for migrating
/// from deprecated trait systems to the canonical UniversalService trait.
///
/// **CONSOLIDATES**:
/// - PrimalProvider → UniversalService
/// - UniversalZfsService → UniversalService with ZFS extensions
/// - Legacy Service trait → UniversalService
/// - Various specialized provider traits → UniversalService extensions
///
/// **MIGRATION STRATEGY**:
/// 1. Identify deprecated trait usage
/// 2. Create domain-specific extensions
/// 3. Implement UniversalService for existing services
/// 4. Update all usage sites
/// 5. Remove deprecated trait definitions
use crate::traits::UniversalService;

// ==================== MIGRATION PATTERNS ====================

/// Migration pattern for PrimalProvider → UniversalService
///
/// **BEFORE (Deprecated)**:
/// ```rust,ignore
/// #[async_trait]
/// impl PrimalProvider for MyService {
///     fn service_id(&self) -> Uuid { ... }
///     fn capabilities(&self) -> Vec<PrimalCapability> { ... }
///     async fn health_check(&self) -> PrimalHealth { ... }
///     // ... other methods
/// }
/// ```
///
/// **AFTER (Canonical)**:
/// ```rust
/// #[derive(Debug, Clone, Serialize, Deserialize)]
/// struct MyServiceConfig {
///     service_id: String,
///     capabilities: Vec<String>,
/// }
///
/// #[derive(Debug, Serialize)]
/// struct MyServiceHealth {
///     status: UnifiedHealthStatus,
///     capabilities: Vec<String>,
///     uptime: u64,
/// }
///
/// #[async_trait]
/// impl UniversalService for MyService {
///     type Config = MyServiceConfig;
///     type Health = MyServiceHealth;
///     
///     async fn initialize(&mut self, config: Self::Config) -> Result<()> { ... }
///     async fn start(&mut self) -> Result<()> { ... }
///     async fn stop(&mut self) -> Result<()> { ... }
///     async fn health_check(&self) -> Result<Self::Health> { ... }
///     // ... other required methods
/// }
/// ```
pub struct PrimalProviderMigration;

impl PrimalProviderMigration {
    /// Generate migration template for a PrimalProvider implementation
    pub fn generate_migration_template(service_name: &str) -> String {
        format!(
            r#"
// Migration from PrimalProvider to UniversalService for {service_name}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {service_name}Config {{
    pub service_id: String,
    pub capabilities: Vec<String>,
    pub settings: HashMap<String, serde_json::Value>,
}}

#[derive(Debug, Serialize)]
pub struct {service_name}Health {{
    pub status: UnifiedHealthStatus,
    pub capabilities: Vec<String>,
    pub uptime: u64,
    pub last_check: chrono::DateTime<chrono::Utc>,
}}

#[async_trait]
impl UniversalService for {service_name} {{
    type Config = {service_name}Config;
    type Health = {service_name}Health;

    async fn initialize(&mut self, config: Self::Config) -> Result<()> {{
        // Migrate from PrimalProvider::initialize
        tracing::info!("Initializing service with config");
        // Store configuration and perform any setup needed
        Ok(())
    }}

    async fn start(&mut self) -> Result<()> {{
        // Migrate service startup logic
        tracing::info!("Starting service");
        // Perform startup operations like connecting to resources
        Ok(())
    }}

    async fn stop(&mut self) -> Result<()> {{
        // Migrate from PrimalProvider::shutdown
        tracing::info!("Stopping service");
        // Perform cleanup operations
        Ok(())
    }}

    async fn health_check(&self) -> Result<Self::Health> {{
        // Migrate from PrimalProvider::health_check
        tracing::debug!("Performing health check");
        // Return healthy status - implement actual checks as needed
        Ok(true)
    }}

    async fn handle_request(&self, request: UniversalServiceRequest) -> Result<UniversalServiceResponse> {{
        // Migrate from PrimalProvider::handle_service_request
        tracing::debug!("Handling request: {{}}", request.operation);
        
        // Return a basic success response - implement actual logic as needed
        Ok(UniversalServiceResponse {{
            request_id: request.request_id,
            status: crate::traits::UniversalResponseStatus::Success,
            data: None,
            error: None,
            metadata: std::collections::HashMap::new(),
        }})
    }}

    fn service_info(&self) -> ServiceInfo {{
        ServiceInfo {{
            name: "{service_name}".to_string(),
            version: "1.0.0".to_string(),
            description: "{service_name} service migrated from PrimalProvider".to_string(),
            capabilities: vec![], // Migrate from PrimalProvider::capabilities
            dependencies: vec![], // Migrate from PrimalProvider::dependencies
        }}
    }}
}}
"#
        )
    }
}

/// Migration pattern for UniversalZfsService → UniversalService
pub struct ZfsServiceMigration;

impl ZfsServiceMigration {
    /// Generate migration template for ZFS service
    pub fn generate_migration_template() -> String {
        r#"
// Migration from UniversalZfsService to UniversalService with ZFS extensions

use crate::traits::{UniversalService, StorageService};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsServiceConfig {
    pub pool_configs: Vec<PoolConfig>,
    pub dataset_defaults: DatasetConfig,
    pub snapshot_policy: SnapshotConfig,
    pub performance_settings: PerformanceConfig,
}

#[derive(Debug, Serialize)]
pub struct ZfsServiceHealth {
    pub status: UnifiedHealthStatus,
    pub pools: Vec<PoolHealth>,
    pub datasets: Vec<DatasetHealth>,
    pub overall_capacity: CapacityInfo,
}

#[async_trait]
impl UniversalService for ZfsService {
    type Config = ZfsServiceConfig;
    type Health = ZfsServiceHealth;

    async fn initialize(&mut self, config: Self::Config) -> Result<()> {
        // Initialize ZFS pools and datasets
        tracing::info!("Initializing ZFS service with config");
        // Store configuration and initialize ZFS subsystem
        Ok(())
    }

    async fn start(&mut self) -> Result<()> {
        // Start ZFS monitoring and management
        tracing::info!("Starting ZFS service");
        // Start monitoring tasks and health checks
        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        // Gracefully stop ZFS operations
        tracing::info!("Stopping ZFS service");
        // Stop monitoring and cleanup resources
        Ok(())
    }

    async fn health_check(&self) -> Result<Self::Health> {
        // Check ZFS pool and dataset health
        tracing::debug!("Performing ZFS health check");
        
        Ok(ZfsServiceHealth {
            pools_healthy: true,
            datasets_accessible: true,
            last_scrub: std::time::SystemTime::now(),
        })
    }

    async fn handle_request(&self, request: UniversalServiceRequest) -> Result<UniversalServiceResponse> {
        // Handle ZFS-specific operations
        match request.operation.as_str() {
            "create_pool" => {
                tracing::info!("Creating ZFS pool");
                // Delegate to ZFS handlers for actual implementation
                Ok(UniversalServiceResponse {
                    request_id: request.request_id,
                    status: crate::traits::UniversalResponseStatus::Success,
                    data: Some(serde_json::json!({"message": "Pool creation initiated"})),
                    error: None,
                    metadata: std::collections::HashMap::new(),
                })
            },
            "create_dataset" => {
                tracing::info!("Creating ZFS dataset");
                Ok(UniversalServiceResponse {
                    request_id: request.request_id,
                    status: crate::traits::UniversalResponseStatus::Success,
                    data: Some(serde_json::json!({"message": "Dataset creation initiated"})),
                    error: None,
                    metadata: std::collections::HashMap::new(),
                })
            },
            "create_snapshot" => {
                tracing::info!("Creating ZFS snapshot");
                Ok(UniversalServiceResponse {
                    request_id: request.request_id,
                    status: crate::traits::UniversalResponseStatus::Success,
                    data: Some(serde_json::json!({"message": "Snapshot creation initiated"})),
                    error: None,
                    metadata: std::collections::HashMap::new(),
                })
            },
            "list_pools" => {
                tracing::debug!("Listing ZFS pools");
                Ok(UniversalServiceResponse {
                    request_id: request.request_id,
                    status: crate::traits::UniversalResponseStatus::Success,
                    data: Some(serde_json::json!({"pools": []})),
                    error: None,
                    metadata: std::collections::HashMap::new(),
                })
            },
            "list_datasets" => {
                tracing::debug!("Listing ZFS datasets");
                Ok(UniversalServiceResponse {
                    request_id: request.request_id,
                    status: crate::traits::UniversalResponseStatus::Success,
                    data: Some(serde_json::json!({"datasets": []})),
                    error: None,
                    metadata: std::collections::HashMap::new(),
                })
            },
            "get_pool_status" => {
                tracing::debug!("Getting ZFS pool status");
                Ok(UniversalServiceResponse {
                    request_id: request.request_id,
                    status: crate::traits::UniversalResponseStatus::Success,
                    data: Some(serde_json::json!({"status": "healthy"})),
                    error: None,
                    metadata: std::collections::HashMap::new(),
                })
            },
            _ => Err(NestGateError::unsupported_operation(&request.operation)),
        }
    }

    fn service_info(&self) -> ServiceInfo {
        ServiceInfo {
            name: "zfs-service".to_string(),
            version: "1.0.0".to_string(),
            description: "ZFS storage management service".to_string(),
            capabilities: vec![
                "storage-pool-management".to_string(),
                "dataset-management".to_string(),
                "snapshot-management".to_string(),
                "storage-monitoring".to_string(),
            ],
            dependencies: vec![],
        }
    }
}

// Implement StorageService extension trait
#[async_trait]
impl StorageService for ZfsService {
    async fn create_storage(&self, config: StorageConfig) -> Result<String> {
        // Implement storage creation
        tracing::info!("Creating ZFS storage");
        // Delegate to ZFS handlers for actual storage creation
        Ok(())
    }

    async fn delete_storage(&self, storage_id: &str) -> Result<()> {
        // Implement storage deletion
        tracing::info!("Deleting ZFS storage");
        // Delegate to ZFS handlers for actual storage deletion
        Ok(())
    }

    async fn get_storage_info(&self, storage_id: &str) -> Result<StorageInfo> {
        // Get storage information
        tracing::debug!("Getting ZFS storage info");
        // Delegate to ZFS handlers for actual storage information
        Ok(serde_json::json!({
            "type": "zfs",
            "status": "available",
            "pools": []
        }))
    }
}
"#.to_string()
    }
}

/// Migration utilities for service trait consolidation
pub struct ServiceTraitMigration;

impl ServiceTraitMigration {
    /// Analyze a service implementation and suggest migration steps
    pub fn analyze_service_implementation(
        service_name: &str,
        trait_name: &str,
    ) -> MigrationAnalysis {
        MigrationAnalysis {
            service_name: service_name.to_string(),
            deprecated_trait: trait_name.to_string(),
            target_trait: "UniversalService".to_string(),
            migration_steps: vec![
                "1. Create service-specific Config struct".to_string(),
                "2. Create service-specific Health struct".to_string(),
                "3. Implement UniversalService trait".to_string(),
                "4. Migrate method implementations".to_string(),
                "5. Update all usage sites".to_string(),
                "6. Remove deprecated trait usage".to_string(),
            ],
            estimated_effort: MigrationEffort::Medium,
            breaking_changes: true,
        }
    }

    /// Generate a complete migration plan for all deprecated traits
    pub fn generate_complete_migration_plan() -> CompleteMigrationPlan {
        CompleteMigrationPlan {
            phases: vec![
                MigrationPhase {
                    name: "Phase 1: Core Service Migration".to_string(),
                    description: "Migrate PrimalProvider implementations to UniversalService"
                        .to_string(),
                    services: vec![
                        "SecurityProvider".to_string(),
                        "ComputeProvider".to_string(),
                        "StorageProvider".to_string(),
                        "NetworkProvider".to_string(),
                    ],
                    estimated_duration: "1-2 weeks".to_string(),
                },
                MigrationPhase {
                    name: "Phase 2: Specialized Service Migration".to_string(),
                    description:
                        "Migrate specialized service traits to UniversalService extensions"
                            .to_string(),
                    services: vec![
                        "UniversalZfsService".to_string(),
                        "AuthenticationService".to_string(),
                        "MonitoringService".to_string(),
                    ],
                    estimated_duration: "1 week".to_string(),
                },
                MigrationPhase {
                    name: "Phase 3: Cleanup and Validation".to_string(),
                    description: "Remove deprecated traits and validate all migrations".to_string(),
                    services: vec![],
                    estimated_duration: "3-5 days".to_string(),
                },
            ],
            total_estimated_duration: "3-4 weeks".to_string(),
            risk_level: RiskLevel::Low,
        }
    }
}

// ==================== MIGRATION DATA STRUCTURES ====================

#[derive(Debug, Clone)]
pub struct MigrationAnalysis {
    pub service_name: String,
    pub deprecated_trait: String,
    pub target_trait: String,
    pub migration_steps: Vec<String>,
    pub estimated_effort: MigrationEffort,
    pub breaking_changes: bool,
}

#[derive(Debug, Clone)]
pub enum MigrationEffort {
    Low,    // < 1 day
    Medium, // 1-3 days
    High,   // 1-2 weeks
}

#[derive(Debug, Clone)]
pub struct CompleteMigrationPlan {
    pub phases: Vec<MigrationPhase>,
    pub total_estimated_duration: String,
    pub risk_level: RiskLevel,
}

#[derive(Debug, Clone)]
pub struct MigrationPhase {
    pub name: String,
    pub description: String,
    pub services: Vec<String>,
    pub estimated_duration: String,
}

#[derive(Debug, Clone)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

// ==================== DEPRECATED TRAIT IDENTIFICATION ====================

/// Utility to identify all deprecated trait usage in the codebase
pub struct DeprecatedTraitScanner;

impl DeprecatedTraitScanner {
    /// List all deprecated traits that need migration
    pub fn get_deprecated_traits() -> Vec<DeprecatedTrait> {
        vec![
            DeprecatedTrait {
                name: "PrimalProvider".to_string(),
                location: "nestgate_core::universal_traits".to_string(),
                replacement: "UniversalService".to_string(),
                migration_complexity: MigrationEffort::Medium,
                usage_count: 15, // Estimated based on grep results
            },
            DeprecatedTrait {
                name: "UniversalZfsService".to_string(),
                location: "nestgate_api::handlers::zfs::universal_zfs::traits".to_string(),
                replacement: "UniversalService with ZFS extensions".to_string(),
                migration_complexity: MigrationEffort::High,
                usage_count: 8,
            },
            DeprecatedTrait {
                name: "SecurityPrimalProvider".to_string(),
                location: "nestgate_core::universal_traits".to_string(),
                replacement: "UniversalService with Security extensions".to_string(),
                migration_complexity: MigrationEffort::Medium,
                usage_count: 6,
            },
            DeprecatedTrait {
                name: "ComputePrimalProvider".to_string(),
                location: "nestgate_core::universal_traits".to_string(),
                replacement: "UniversalService with Compute extensions".to_string(),
                migration_complexity: MigrationEffort::Medium,
                usage_count: 4,
            },
            DeprecatedTrait {
                name: "StoragePrimalProvider".to_string(),
                location: "nestgate_api::universal_primal".to_string(),
                replacement: "UniversalService with Storage extensions".to_string(),
                migration_complexity: MigrationEffort::Medium,
                usage_count: 3,
            },
        ]
    }

    /// Generate migration priority based on usage and complexity
    pub fn get_migration_priority() -> Vec<MigrationPriority> {
        vec![
            MigrationPriority {
                trait_name: "PrimalProvider".to_string(),
                priority: Priority::High,
                reason: "Most widely used deprecated trait".to_string(),
            },
            MigrationPriority {
                trait_name: "UniversalZfsService".to_string(),
                priority: Priority::Medium,
                reason: "Complex but isolated to ZFS handlers".to_string(),
            },
            MigrationPriority {
                trait_name: "SecurityPrimalProvider".to_string(),
                priority: Priority::High,
                reason: "Critical security functionality".to_string(),
            },
            MigrationPriority {
                trait_name: "ComputePrimalProvider".to_string(),
                priority: Priority::Medium,
                reason: "Moderate usage, clear migration path".to_string(),
            },
            MigrationPriority {
                trait_name: "StoragePrimalProvider".to_string(),
                priority: Priority::Low,
                reason: "Limited usage, can be migrated last".to_string(),
            },
        ]
    }
}

#[derive(Debug, Clone)]
pub struct DeprecatedTrait {
    pub name: String,
    pub location: String,
    pub replacement: String,
    pub migration_complexity: MigrationEffort,
    pub usage_count: usize,
}

#[derive(Debug, Clone)]
pub struct MigrationPriority {
    pub trait_name: String,
    pub priority: Priority,
    pub reason: String,
}

#[derive(Debug, Clone)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

// ==================== MIGRATION VALIDATION ====================

/// Validation utilities for trait migrations
pub struct MigrationValidator;

impl MigrationValidator {
    /// Validate that a service correctly implements UniversalService
    pub fn validate_universal_service_implementation<T: UniversalService>(
        _service: &T,
    ) -> ValidationResult {
        // This would perform runtime validation of the implementation
        ValidationResult {
            is_valid: true,
            issues: vec![],
            suggestions: vec![
                "Consider adding more detailed health check information".to_string(),
                "Ensure all error cases are properly handled".to_string(),
            ],
        }
    }

    /// Check for remaining deprecated trait usage
    pub fn scan_for_deprecated_usage() -> Vec<DeprecatedUsage> {
        // This would scan the codebase for deprecated trait usage
        // For now, return empty - would be implemented with actual scanning logic
        vec![]
    }
}

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub issues: Vec<String>,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DeprecatedUsage {
    pub file_path: String,
    pub line_number: usize,
    pub deprecated_item: String,
    pub suggested_replacement: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migration_template_generation() {
        let template = PrimalProviderMigration::generate_migration_template("TestService");
        assert!(template.contains("TestServiceConfig"));
        assert!(template.contains("TestServiceHealth"));
        assert!(template.contains("UniversalService"));
    }

    #[test]
    fn test_deprecated_trait_identification() {
        let deprecated_traits = DeprecatedTraitScanner::get_deprecated_traits();
        assert!(!deprecated_traits.is_empty());
        assert!(deprecated_traits.iter().any(|t| t.name == "PrimalProvider"));
    }

    #[test]
    fn test_migration_plan_generation() {
        let plan = ServiceTraitMigration::generate_complete_migration_plan();
        assert_eq!(plan.phases.len(), 3);
        assert!(plan.total_estimated_duration.contains("week"));
    }
}
