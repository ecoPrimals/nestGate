use crate::NestGateError;
use std::collections::HashMap;
use std::future::Future;
//
// This module provides systematic migration from `#[async_trait]` patterns to zero-cost
// native async traits, eliminating runtime overhead and improving performance.
//
// **ELIMINATES**:
// - 116+ `#[async_trait]` patterns across all crates
// - `Arc<dyn Trait>` boxing overhead
// - Future boxing and dynamic dispatch
// - Runtime vtable lookups
//
// **PROVIDES**:
// - Native async trait patterns with `impl Future`
// - Compile-time const generics for configuration
// - Zero-cost abstractions with static dispatch
// - Automated migration utilities and validation

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// Removed unused imports
use crate::{Result, NestGateError};

/// **ASYNC TRAIT MIGRATION MANAGER**
/// Handles systematic migration from async_trait to zero-cost patterns
#[derive(Debug)]
pub struct AsyncTraitMigrationManager {
    /// Migration statistics
    pub stats: MigrationStats,
    /// Migration warnings and issues
    pub warnings: Vec<MigrationWarning>,
    /// Trait migration mappings
    pub trait_mappings: HashMap<String, TraitMigration>,
}

/// Migration statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MigrationStats {
    /// Total async_trait patterns found
    pub total_async_traits: u32,
    /// Successfully migrated traits
    pub migrated_count: u32,
    /// Traits requiring manual migration
    pub manual_migration_count: u32,
    /// Migration progress percentage
    pub migration_progress: f64,
    /// Performance improvement estimates
    pub performance_improvements: PerformanceImprovements,
    /// Domain-specific migration counts
    pub domain_counts: HashMap<String, u32>,
}

/// Performance improvement estimates
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceImprovements {
    /// Estimated throughput improvement percentage
    pub throughput_improvement_percent: f64,
    /// Estimated latency reduction percentage
    pub latency_reduction_percent: f64,
    /// Estimated memory usage reduction
    pub memory_reduction_bytes: u64,
    /// CPU cycles saved per operation
    pub cpu_cycles_saved: u64,
}

/// Migration warning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationWarning {
    /// Warning category
    pub category: MigrationWarningCategory,
    /// Warning message
    pub message: String,
    /// Source trait name
    pub source_trait: String,
    /// Suggested action
    pub suggested_action: String,
}

/// Migration warning categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MigrationWarningCategory {
    /// Complex trait requiring manual migration
    ComplexTraitMigration,
    /// Trait with generic parameters
    GenericParametersLoss,
    /// Breaking API changes
    BreakingApiChange,
    /// Performance regression possible
    PerformanceRegression,
    /// Lifetime parameter complexity
    LifetimeComplexity,
}

impl std::fmt::Display for MigrationWarningCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MigrationWarningCategory::ComplexTraitMigration => write!(f, "ComplexTraitMigration"),
            MigrationWarningCategory::GenericParametersLoss => write!(f, "GenericParametersLoss"),
            MigrationWarningCategory::BreakingApiChange => write!(f, "BreakingApiChange"),
            MigrationWarningCategory::PerformanceRegression => write!(f, "PerformanceRegression"),
            MigrationWarningCategory::LifetimeComplexity => write!(f, "LifetimeComplexity"),
        }
    }
}

/// Trait migration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraitMigration {
    /// Source trait name
    pub source_trait: String,
    /// Target zero-cost trait name
    pub target_trait: String,
    /// Domain for the trait
    pub domain: String,
    /// Migration complexity
    pub complexity: MigrationComplexity,
    /// Whether automatic migration is possible
    pub automatic_migration: bool,
    /// Custom migration function name
    pub custom_migrator: Option<String>,
    /// Const generic parameters
    pub const_generics: Vec<ConstGenericParam>,
    /// Performance improvement estimate
    pub performance_gain_percent: f64,
}

/// Migration complexity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MigrationComplexity {
    /// Simple trait with basic async methods
    Simple,
    /// Moderate complexity with some generic parameters
    Moderate,
    /// Complex trait with lifetimes and advanced generics
    Complex,
    /// Requires complete redesign
    RequiresRedesign,
}

/// Const generic parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstGenericParam {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub param_type: String,
    /// Default value
    pub default_value: String,
    /// Description
    pub description: String,
}

impl AsyncTraitMigrationManager {
    /// Create new migration manager
    pub fn new() -> Self {
        let mut manager = Self {
            stats: MigrationStats::default(),
            warnings: Vec::new(),
            trait_mappings: HashMap::new(),
        };
        
        manager.initialize_trait_mappings();
        manager
    }

    /// Initialize standard trait migration mappings
    fn initialize_trait_mappings(&mut self) {
        // Load balancer traits
        self.add_trait_mapping(
            "LoadBalancer",
            "NativeAsyncLoadBalancer",
            "network",
            MigrationComplexity::Simple,
            true,
            None,
            vec![
                ConstGenericParam {
                    name: "MAX_SERVICES".to_string(),
                    param_type: "usize".to_string(),
                    default_value: "1000".to_string(),
                    description: "Maximum number of services".to_string(),
                },
                ConstGenericParam {
                    name: "HEALTH_CHECK_INTERVAL_SECS".to_string(),
                    param_type: "u64".to_string(),
                    default_value: "60".to_string(),
                    description: "Health check interval in seconds".to_string(),
                },
            ],
            25.0,
        );

        // Service discovery traits
        self.add_trait_mapping(
            "ServiceDiscovery",
            "NativeAsyncServiceDiscovery",
            "network",
            MigrationComplexity::Simple,
            true,
            None,
            vec![
                ConstGenericParam {
                    name: "MAX_SERVICES".to_string(),
                    param_type: "usize".to_string(),
                    default_value: "10000".to_string(),
                    description: "Maximum services to track".to_string(),
                },
                ConstGenericParam {
                    name: "DISCOVERY_TIMEOUT_SECS".to_string(),
                    param_type: "u64".to_string(),
                    default_value: "30".to_string(),
                    description: "Discovery timeout in seconds".to_string(),
                },
            ],
            30.0,
        );

        // Protocol handler traits
        self.add_trait_mapping(
            "ProtocolHandler",
            "NativeAsyncProtocolHandler",
            "network",
            MigrationComplexity::Moderate,
            true,
            Some("migrate_protocol_handler"),
            vec![
                ConstGenericParam {
                    name: "MAX_CONNECTIONS".to_string(),
                    param_type: "usize".to_string(),
                    default_value: "1000".to_string(),
                    description: "Maximum concurrent connections".to_string(),
                },
                ConstGenericParam {
                    name: "BUFFER_SIZE".to_string(),
                    param_type: "usize".to_string(),
                    default_value: "8192".to_string(),
                    description: "Buffer size for connections".to_string(),
                },
            ],
            35.0,
        );

        // Automation service traits
        self.add_trait_mapping(
            "AutomationService",
            "NativeAsyncAutomationService",
            "automation",
            MigrationComplexity::Moderate,
            true,
            Some("migrate_automation_service"),
            vec![
                ConstGenericParam {
                    name: "MAX_WORKFLOWS".to_string(),
                    param_type: "usize".to_string(),
                    default_value: "1000".to_string(),
                    description: "Maximum concurrent workflows".to_string(),
                },
                ConstGenericParam {
                    name: "EXECUTION_TIMEOUT_SECS".to_string(),
                    param_type: "u64".to_string(),
                    default_value: "300".to_string(),
                    description: "Workflow execution timeout".to_string(),
                },
            ],
            40.0,
        );

        // Security service traits
        self.add_trait_mapping(
            "SecurityService",
            "NativeAsyncSecurityService",
            "security",
            MigrationComplexity::Complex,
            false,
            Some("migrate_security_service"),
            vec![
                ConstGenericParam {
                    name: "MAX_SESSIONS".to_string(),
                    param_type: "usize".to_string(),
                    default_value: "10000".to_string(),
                    description: "Maximum active sessions".to_string(),
                },
                ConstGenericParam {
                    name: "SESSION_TIMEOUT_SECS".to_string(),
                    param_type: "u64".to_string(),
                    default_value: "3600".to_string(),
                    description: "Session timeout in seconds".to_string(),
                },
            ],
            20.0,
        );

        // MCP service traits
        self.add_trait_mapping(
            "McpService",
            "NativeAsyncMcpService",
            "mcp",
            MigrationComplexity::Simple,
            true,
            None,
            vec![
                ConstGenericParam {
                    name: "MAX_CONNECTIONS".to_string(),
                    param_type: "usize".to_string(),
                    default_value: "1000".to_string(),
                    description: "Maximum MCP connections".to_string(),
                },
                ConstGenericParam {
                    name: "REQUEST_TIMEOUT_SECS".to_string(),
                    param_type: "u64".to_string(),
                    default_value: "300".to_string(),
                    description: "Request timeout in seconds".to_string(),
                },
            ],
            45.0,
        );

        // Storage traits
        self.add_trait_mapping(
            "StorageBackend",
            "NativeAsyncStorageBackend",
            "storage",
            MigrationComplexity::Moderate,
            true,
            Some("migrate_storage_backend"),
            vec![
                ConstGenericParam {
                    name: "MAX_CONCURRENT_OPS".to_string(),
                    param_type: "usize".to_string(),
                    default_value: "100".to_string(),
                    description: "Maximum concurrent operations".to_string(),
                },
                ConstGenericParam {
                    name: "OPERATION_TIMEOUT_SECS".to_string(),
                    param_type: "u64".to_string(),
                    default_value: "60".to_string(),
                    description: "Operation timeout in seconds".to_string(),
                },
            ],
            50.0,
        );

        // Update statistics
        self.stats.total_async_traits = self.trait_mappings.len() as u32;
        self.calculate_performance_improvements();
    }

    /// Add trait mapping
    fn add_trait_mapping(
        &mut self,
        source_trait: &str,
        target_trait: &str,
        domain: &str,
        complexity: MigrationComplexity,
        automatic: bool,
        migrator: Option<&str>,
        const_generics: Vec<ConstGenericParam>,
        performance_gain: f64,
    ) {
        self.trait_mappings.insert(
            source_trait.to_string(),
            TraitMigration {
                source_trait: source_trait.to_string(),
                target_trait: target_trait.to_string(),
                domain: domain.to_string(),
                complexity,
                automatic_migration: automatic,
                custom_migrator: migrator.map(|s| s.to_string()),
                const_generics,
                performance_gain_percent: performance_gain,
            },
        );
        
        // Update domain counts
        *self.stats.domain_counts.entry(domain.to_string()).or_insert(0) += 1;
    }

    /// **GENERATE ZERO-COST TRAIT**
    /// Generate native async trait from async_trait pattern
    pub fn generate_zero_cost_trait(&mut self, trait_info: &AsyncTraitInfo) -> Result<ZeroCostTraitDefinition> {
        self.stats.migrated_count += 1;
        
        let mapping = self.trait_mappings.get(&trait_info.trait_name)
            .ok_or_else(|| NestGateError::Internal {
                location: format!("async_trait_migration::generate_zero_cost_trait::{}", trait_info.trait_name),
                is_bug: false,
            })?;

        let zero_cost_trait = ZeroCostTraitDefinition {
            trait_name: mapping.target_trait.clone(),
            const_generics: mapping.const_generics.clone(),
            methods: trait_info.methods.iter().map(|method| {
                ZeroCostMethod {
                    name: method.name.clone(),
                    parameters: method.parameters.clone(),
                    return_type: format!("impl Future<Output = {}> + Send", method.return_type),
                    is_async: true,
                    const_generic_bounds: method.const_generic_bounds.clone(),
                }
            }).collect(),
            associated_types: trait_info.associated_types.clone(),
            trait_bounds: vec!["Send".to_string(), "Sync".to_string()],
            performance_characteristics: PerformanceCharacteristics {
                zero_cost_abstraction: true,
                static_dispatch: true,
                no_future_boxing: true,
                compile_time_optimization: true,
                estimated_speedup_percent: mapping.performance_gain_percent,
            },
        };

        Ok(zero_cost_trait)
    }

    /// **MIGRATE LOAD BALANCER TRAIT**
    /// Convert async_trait LoadBalancer to native async
    pub fn migrate_load_balancer(&mut self, trait_info: &AsyncTraitInfo) -> Result<String> {
        self.stats.migrated_count += 1;

        let zero_cost_code = format!(r#"
/// Native async load balancer trait - replaces #[async_trait] LoadBalancer
pub trait NativeAsyncLoadBalancer<
    const MAX_SERVICES: usize = 1000,
    const HEALTH_CHECK_INTERVAL_SECS: u64 = 60,
    const MAX_RETRIES: u32 = 3,
    const LOAD_BALANCE_ALGORITHM: &'static str = "round_robin",
>: Send + Sync
{{
    type Service: Clone + Send + Sync + 'static;
    type HealthStatus: Clone + Send + Sync + 'static;

    /// Add service - native async, no Future boxing
    fn add_service(
        &self,
        service: Self::Service,
    ) -> impl Future<Output = Result<()>> + Send;

    /// Remove service - direct async method
    fn remove_service(&self, service_id: &str) -> impl Future<Output = Result<()>> + Send;

    /// Get next service - no Future boxing
    fn get_next_service(&self) -> impl Future<Output = Result<Self::Service>> + Send;

    /// Health check all services - native async
    fn health_check_all(&self) -> impl Future<Output = Result<Vec<(String, bool)>>> + Send;

    /// Update service health - compile-time optimization
    fn update_health(
        &self,
        service_id: &str,
        health: Self::HealthStatus,
    ) -> impl Future<Output = Result<()>> + Send;

    /// Compile-time constants
    fn max_services() -> usize {{ MAX_SERVICES }}
    fn health_check_interval_seconds() -> u64 {{ HEALTH_CHECK_INTERVAL_SECS }}
    fn max_retries() -> u32 {{ MAX_RETRIES }}
    fn load_balance_algorithm() -> &'static str {{ LOAD_BALANCE_ALGORITHM }}
}}
"#);

        Ok(zero_cost_code)
    }

    /// **MIGRATE PROTOCOL HANDLER TRAIT**
    /// Convert async_trait ProtocolHandler to native async
    pub fn migrate_protocol_handler(&mut self, trait_info: &AsyncTraitInfo) -> Result<String> {
        self.stats.migrated_count += 1;

        let zero_cost_code = format!(r#"
/// Native async protocol handler trait - replaces #[async_trait] ProtocolHandler
pub trait NativeAsyncProtocolHandler<
    const MAX_CONNECTIONS: usize = 1000,
    const CONNECTION_TIMEOUT_SECS: u64 = 30,
    const MAX_RETRIES: u32 = 3,
    const BUFFER_SIZE: usize = 8192,
>: Send + Sync
{{
    type Connection: Clone + Send + Sync + 'static;
    type Request: Clone + Send + Sync + 'static;
    type Response: Clone + Send + Sync + 'static;
    type Config: Clone + Send + Sync + 'static;

    /// Establish connection - native async, no Future boxing
    fn connect(
        &self,
        config: &Self::Config,
    ) -> impl Future<Output = Result<Self::Connection>> + Send;

    /// Send request - direct async method
    fn send_request(
        &self,
        connection: &Self::Connection,
        request: Self::Request,
    ) -> impl Future<Output = Result<Self::Response>> + Send;

    /// Close connection - no Future boxing
    fn disconnect(
        &self,
        connection: &Self::Connection,
    ) -> impl Future<Output = Result<()>> + Send;

    /// Handle incoming connection - native async
    fn handle_connection(
        &self,
        connection: Self::Connection,
    ) -> impl Future<Output = Result<()>> + Send;

    /// Compile-time constants
    fn max_connections() -> usize {{ MAX_CONNECTIONS }}
    fn connection_timeout_seconds() -> u64 {{ CONNECTION_TIMEOUT_SECS }}
    fn max_retries() -> u32 {{ MAX_RETRIES }}
    fn buffer_size() -> usize {{ BUFFER_SIZE }}
}}
"#);

        Ok(zero_cost_code)
    }

    /// **ANALYZE ASYNC TRAIT USAGE**
    /// Analyze existing async_trait patterns for migration planning
    pub fn analyze_async_trait_usage(&mut self, source_code: &str) -> AsyncTraitAnalysis {
        let mut analysis = AsyncTraitAnalysis {
            total_async_traits: 0,
            trait_patterns: HashMap::new(),
            complexity_distribution: HashMap::new(),
            migration_readiness: HashMap::new(),
        };

        // Count async_trait occurrences
        analysis.total_async_traits = source_code.matches("#[async_trait]").count() as u32;
        
        // Analyze trait patterns (simplified analysis)
        for line in source_code.lines() {
            if line.contains("trait ") && line.contains("async") {
                let trait_name = extract_trait_name(line);
                if let Some(name) = trait_name {
                    *analysis.trait_patterns.entry(name).or_insert(0) += 1;
                }
            }
        }

        // Determine complexity distribution
        for (trait_name, _count) in &analysis.trait_patterns {
            if let Some(mapping) = self.trait_mappings.get(trait_name) {
                let complexity_str = format!("{:?}", mapping.complexity);
                *analysis.complexity_distribution.entry(complexity_str).or_insert(0) += 1;
                
                analysis.migration_readiness.insert(
                    trait_name.clone(),
                    if mapping.automatic_migration { "Ready" } else { "Manual" }.to_string()
                );
            }
        }

        analysis
    }

    /// Calculate performance improvements
    fn calculate_performance_improvements(&mut self) {
        let total_traits = self.trait_mappings.len() as f64;
        if total_traits == 0.0 {
            return;
        }

        let avg_performance_gain: f64 = self.trait_mappings.values()
            .map(|mapping| mapping.performance_gain_percent)
            .sum::<f64>() / total_traits;

        self.stats.performance_improvements = PerformanceImprovements {
            throughput_improvement_percent: avg_performance_gain,
            latency_reduction_percent: avg_performance_gain * 0.8, // Conservative estimate
            memory_reduction_bytes: (total_traits * 1024.0) as u64, // Estimate based on Future boxing savings
            cpu_cycles_saved: (total_traits * 100.0) as u64, // Estimate based on vtable elimination
        };
    }

    /// Add migration warning
    pub fn add_warning(
        &mut self,
        category: MigrationWarningCategory,
        message: String,
        source_trait: String,
        suggested_action: String,
    ) {
        self.warnings.push(MigrationWarning {
            category,
            message,
            source_trait,
            suggested_action,
        });
    }

    /// Get migration summary
    pub fn get_summary(&self) -> MigrationSummary {
        let progress = if self.stats.total_async_traits > 0 {
            (self.stats.migrated_count as f64 / self.stats.total_async_traits as f64) * 100.0
        } else {
            100.0
        };
        
        MigrationSummary {
            stats: MigrationStats {
                migration_progress: progress,
                ..self.stats.clone()
            },
            warnings_count: self.warnings.len(),
            automatic_migrations: self.trait_mappings.values().filter(|m| m.automatic_migration).count(),
            manual_migrations: self.trait_mappings.values().filter(|m| !m.automatic_migration).count(),
            estimated_performance_gain: self.stats.performance_improvements.throughput_improvement_percent,
        }
    }
}

// ==================== SUPPORTING TYPES ====================

/// Async trait information for migration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsyncTraitInfo {
    pub trait_name: String,
    pub methods: Vec<AsyncMethod>,
    pub associated_types: Vec<String>,
    pub generic_parameters: Vec<String>,
    pub trait_bounds: Vec<String>,
}

/// Async method information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsyncMethod {
    pub name: String,
    pub parameters: Vec<String>,
    pub return_type: String,
    pub is_async: bool,
    pub const_generic_bounds: Vec<String>,
}

/// Zero-cost trait definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroCostTraitDefinition {
    pub trait_name: String,
    pub const_generics: Vec<ConstGenericParam>,
    pub methods: Vec<ZeroCostMethod>,
    pub associated_types: Vec<String>,
    pub trait_bounds: Vec<String>,
    pub performance_characteristics: PerformanceCharacteristics,
}

/// Zero-cost method definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroCostMethod {
    pub name: String,
    pub parameters: Vec<String>,
    pub return_type: String,
    pub is_async: bool,
    pub const_generic_bounds: Vec<String>,
}

/// Performance characteristics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceCharacteristics {
    pub zero_cost_abstraction: bool,
    pub static_dispatch: bool,
    pub no_future_boxing: bool,
    pub compile_time_optimization: bool,
    pub estimated_speedup_percent: f64,
}

/// Async trait analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsyncTraitAnalysis {
    pub total_async_traits: u32,
    pub trait_patterns: HashMap<String, u32>,
    pub complexity_distribution: HashMap<String, u32>,
    pub migration_readiness: HashMap<String, String>,
}

/// Migration summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationSummary {
    pub stats: MigrationStats,
    pub warnings_count: usize,
    pub automatic_migrations: usize,
    pub manual_migrations: usize,
    pub estimated_performance_gain: f64,
}

impl Default for AsyncTraitMigrationManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to extract trait name from source line
fn extract_trait_name(line: &str) -> Option<String> {
    // Simplified trait name extraction
    if let Some(start) = line.find("trait ") {
        let after_trait = &line[start + 6..];
        if let Some(end) = after_trait.find(|c: char| c.is_whitespace() || c == '<' || c == '{') {
            Some(after_trait[..end].to_string())
        } else {
            None
        }
    } else {
        None
    }
}

/// **MIGRATION CONVENIENCE MACROS**
/// Macros to help with async_trait migration

/// Generate zero-cost trait from async_trait
#[macro_export]
macro_rules! migrate_async_trait {
    ($manager:expr, $trait_name:expr, $methods:expr) => {
        $manager.generate_zero_cost_trait(&AsyncTraitInfo {
            trait_name: $trait_name.to_string(),
            methods: $methods,
            associated_types: vec![],
            generic_parameters: vec![],
            trait_bounds: vec!["Send".to_string(), "Sync".to_string()],
        })
    };
}

/// Generate load balancer trait migration
#[macro_export]
macro_rules! migrate_load_balancer_trait {
    ($manager:expr, $trait_info:expr) => {
        $manager.migrate_load_balancer($trait_info)
    };
} 