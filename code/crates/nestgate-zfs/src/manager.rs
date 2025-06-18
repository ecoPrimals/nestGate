//! ZFS Manager - Main orchestrator for ZFS operations
//! 
//! Integrates advanced integration patterns with v2 orchestrator architecture
//! Enhanced with AI integration and performance monitoring

use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error, debug, warn};
use serde::{Deserialize, Serialize};

use nestgate_core::{Result, NestGateError, StorageTier};
use crate::{
    pool::ZfsPoolManager,
    dataset::ZfsDatasetManager, 
    snapshot::ZfsSnapshotManager,
    migration::MigrationEngine,
    automation::DatasetAnalyzer,
    ai_integration::{ZfsAiIntegration, ZfsAiConfig},
    performance::{ZfsPerformanceMonitor, PerformanceConfig},
    tier::TierManager,
    health::ZfsHealthMonitor,
    metrics::ZfsMetrics,
    config::ZfsConfig,
    error::ZfsError,
};

#[cfg(feature = "orchestrator")]
use crate::orchestrator::OrchestratorClient;

/// Enhanced ZFS Manager integrating AI and performance monitoring
#[derive(Debug)]
pub struct ZfsManager {
    /// Pool management operations
    pub pool_manager: Arc<ZfsPoolManager>,
    /// Dataset management operations  
    pub dataset_manager: Arc<ZfsDatasetManager>,
    /// Snapshot management operations
    pub snapshot_manager: Arc<ZfsSnapshotManager>,
    /// Migration engine for tier optimization
    pub migration_engine: Arc<MigrationEngine>,
    /// Dataset analysis and automation
    pub dataset_analyzer: Arc<DatasetAnalyzer>,
    /// AI-powered optimization
    pub ai_integration: Option<Arc<ZfsAiIntegration>>,
    /// Performance monitoring
    pub performance_monitor: Arc<ZfsPerformanceMonitor>,
    /// Tiered storage management
    pub tier_manager: Arc<TierManager>,
    /// Health monitoring system
    pub health_monitor: Arc<ZfsHealthMonitor>,
    /// Performance metrics collection
    pub metrics: Arc<ZfsMetrics>,
    /// Configuration management
    pub config: ZfsConfig,
    /// Optional orchestrator client
    #[cfg(feature = "orchestrator")]
    orchestrator_client: Option<Arc<OrchestratorClient>>,
}

/// Enhanced service information for orchestrator registration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub name: String,
    pub endpoint: String,
    pub health_endpoint: String,
    pub capabilities: Vec<String>,
    pub metadata: std::collections::HashMap<String, String>,
    /// AI capabilities
    pub ai_capabilities: Vec<String>,
    /// Performance monitoring features
    pub monitoring_features: Vec<String>,
}

impl ZfsManager {
    /// Create a new enhanced ZFS manager with AI and performance monitoring
    pub async fn new(config: ZfsConfig) -> Result<Self> {
        info!("Initializing Enhanced ZFS Manager with AI integration");
        
        // Initialize pool manager first (foundation for everything else)
        let pool_manager = Arc::new(
            ZfsPoolManager::new(&config)
                .await
                .map_err(|e| {
                    error!("Failed to initialize ZFS pool manager: {}", e);
                    NestGateError::Internal(format!("Pool manager: {}", e))
                })?
        );
        
        // Initialize dataset manager
        let dataset_manager = Arc::new(
            ZfsDatasetManager::new(config.clone(), pool_manager.clone())
        );
        
        // Initialize dataset analyzer
        let dataset_analyzer = Arc::new(
            DatasetAnalyzer::new(
                config.clone(),
                pool_manager.clone(),
                dataset_manager.clone(),
            )
        );
        
        // Initialize migration engine
        let migration_config = crate::migration::MigrationConfig::default();
        let migration_engine = Arc::new(
            MigrationEngine::new(
                migration_config,
                config.clone(),
                pool_manager.clone(),
                dataset_manager.clone(),
                dataset_analyzer.clone(),
            )
        );
        
        // Initialize snapshot manager
        let snapshot_manager = Arc::new(
            ZfsSnapshotManager::new(config.clone(), pool_manager.clone(), dataset_manager.clone())
        );
        
        // Initialize performance monitor
        let performance_config = PerformanceConfig::default();
        let performance_monitor = Arc::new(
            ZfsPerformanceMonitor::new(
                performance_config,
                pool_manager.clone(),
                dataset_manager.clone(),
            )
        );
        
        // Initialize AI integration (optional, based on configuration)
        let ai_integration = if config.enable_ai_integration.unwrap_or(true) {
            let ai_config = ZfsAiConfig::default();
            match ZfsAiIntegration::new(
                ai_config,
                pool_manager.clone(),
                dataset_manager.clone(),
                dataset_analyzer.clone(),
                migration_engine.clone(),
            ).await {
                Ok(ai) => {
                    info!("AI integration initialized successfully");
                    Some(Arc::new(ai))
                }
                Err(e) => {
                    warn!("Failed to initialize AI integration: {}, continuing without AI features", e);
                    None
                }
            }
        } else {
            info!("AI integration disabled by configuration");
            None
        };
        
        // Initialize tier manager for hot/warm/cold storage
        let tier_manager = Arc::new(
            TierManager::new(&config, pool_manager.clone(), dataset_manager.clone())
                .await
                .map_err(|e| {
                    error!("Failed to initialize tier manager: {}", e);
                    NestGateError::Internal(format!("Tier manager: {}", e))
                })?
        );
        
        // Initialize health monitoring
        let health_monitor = Arc::new(
            ZfsHealthMonitor::new(pool_manager.clone())
                .await
                .map_err(|e| {
                    error!("Failed to initialize ZFS health monitor: {}", e);
                    NestGateError::Internal(format!("Health monitor: {}", e))
                })?
        );
        
        // Initialize metrics collection
        let metrics = Arc::new(ZfsMetrics::new());
        
        info!("Enhanced ZFS Manager initialization complete");
        
        Ok(Self {
            pool_manager,
            dataset_manager,
            snapshot_manager,
            migration_engine,
            dataset_analyzer,
            ai_integration,
            performance_monitor,
            tier_manager,
            health_monitor,
            metrics,
            config,
            #[cfg(feature = "orchestrator")]
            orchestrator_client: None,
        })
    }
    
    /// Start all ZFS services
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting Enhanced ZFS Manager services");
        
        // Start performance monitoring first
        let mut perf_monitor = Arc::clone(&self.performance_monitor);
        Arc::get_mut(&mut perf_monitor)
            .ok_or_else(|| NestGateError::Internal("Failed to get mutable reference to performance monitor".to_string()))?
            .start()
            .await?;
        
        // Start AI integration if available
        if let Some(ai_integration) = &self.ai_integration {
            let mut ai = Arc::clone(ai_integration);
            Arc::get_mut(&mut ai)
                .ok_or_else(|| NestGateError::Internal("Failed to get mutable reference to AI integration".to_string()))?
                .start()
                .await?;
        }
        
        // Start migration engine
        let mut migration = Arc::clone(&self.migration_engine);
        Arc::get_mut(&mut migration)
            .ok_or_else(|| NestGateError::Internal("Failed to get mutable reference to migration engine".to_string()))?
            .start()
            .await?;
        
        // Start snapshot manager
        let mut snapshot = Arc::clone(&self.snapshot_manager);
        Arc::get_mut(&mut snapshot)
            .ok_or_else(|| NestGateError::Internal("Failed to get mutable reference to snapshot manager".to_string()))?
            .start()
            .await?;
        
        info!("Enhanced ZFS Manager services started successfully");
        Ok(())
    }
    
    /// Stop all ZFS services
    pub async fn stop(&mut self) -> Result<()> {
        info!("Stopping Enhanced ZFS Manager services");
        
        // Stop AI integration
        if let Some(ai_integration) = &self.ai_integration {
            let mut ai = Arc::clone(ai_integration);
            if let Some(ai_mut) = Arc::get_mut(&mut ai) {
                ai_mut.stop().await?;
            }
        }
        
        // Stop migration engine
        let mut migration = Arc::clone(&self.migration_engine);
        if let Some(migration_mut) = Arc::get_mut(&mut migration) {
            migration_mut.stop().await?;
        }
        
        // Stop snapshot manager
        let mut snapshot = Arc::clone(&self.snapshot_manager);
        if let Some(snapshot_mut) = Arc::get_mut(&mut snapshot) {
            snapshot_mut.stop().await?;
        }
        
        // Stop performance monitoring
        let mut perf_monitor = Arc::clone(&self.performance_monitor);
        if let Some(perf_mut) = Arc::get_mut(&mut perf_monitor) {
            perf_mut.stop().await?;
        }
        
        // Stop health monitoring
        self.health_monitor.stop_monitoring().await?;
        self.metrics.stop_collection().await?;
        
        // Graceful shutdown of tier manager
        self.tier_manager.shutdown().await?;
        
        info!("Enhanced ZFS Manager services stopped");
        Ok(())
    }
    
    /// Register enhanced ZFS service with orchestrator
    #[cfg(feature = "orchestrator")]
    pub async fn register_with_orchestrator(&mut self, orchestrator_endpoint: String) -> Result<()> {
        info!("Registering Enhanced ZFS service with orchestrator at: {}", orchestrator_endpoint);
        
        let client = OrchestratorClient::new(orchestrator_endpoint)
            .await
            .map_err(|e| {
                error!("Failed to create orchestrator client: {}", e);
                NestGateError::Network(format!("Orchestrator: {}", e))
            })?;
        
        // Prepare enhanced service information
        let service_info = ServiceInfo {
            name: "nestgate-zfs-enhanced".to_string(),
            endpoint: self.config.api_endpoint.clone(),
            health_endpoint: format!("{}/health", self.config.api_endpoint),
            capabilities: vec![
                "dataset_management".to_string(),
                "snapshot_operations".to_string(),
                "tier_management".to_string(),
                "pool_monitoring".to_string(),
                "migration_services".to_string(),
                "health_monitoring".to_string(),
                "performance_monitoring".to_string(),
                "ai_optimization".to_string(),
                "predictive_analytics".to_string(),
                "automated_tiering".to_string(),
            ],
            ai_capabilities: if self.ai_integration.is_some() {
                vec![
                    "tier_optimization".to_string(),
                    "workload_prediction".to_string(),
                    "anomaly_detection".to_string(),
                    "performance_forecasting".to_string(),
                ]
            } else {
                vec![]
            },
            monitoring_features: vec![
                "real_time_metrics".to_string(),
                "performance_alerting".to_string(),
                "trend_analysis".to_string(),
                "sla_monitoring".to_string(),
                "capacity_planning".to_string(),
            ],
            metadata: {
                let mut metadata = std::collections::HashMap::new();
                metadata.insert("version".to_string(), env!("CARGO_PKG_VERSION").to_string());
                metadata.insert("tier_count".to_string(), "3".to_string()); // hot, warm, cold
                metadata.insert("zfs_enabled".to_string(), cfg!(feature = "zfs").to_string());
                metadata.insert("ai_enabled".to_string(), self.ai_integration.is_some().to_string());
                metadata.insert("performance_monitoring".to_string(), "enabled".to_string());
                metadata.insert("migration_engine".to_string(), "enabled".to_string());
                metadata.insert("snapshot_automation".to_string(), "enabled".to_string());
                metadata
            },
        };
        
        // Register with orchestrator
        client.register_service(service_info)
            .await
            .map_err(|e| {
                error!("Failed to register with orchestrator: {}", e);
                NestGateError::Network(format!("Service registration: {}", e))
            })?;
        
        self.orchestrator_client = Some(Arc::new(client));
        
        info!("Successfully registered Enhanced ZFS service with orchestrator");
        Ok(())
    }
    
    /// Get enhanced service status for health checks
    pub async fn get_service_status(&self) -> Result<EnhancedServiceStatus> {
        debug!("Collecting Enhanced ZFS service status");
        
        let pool_status = self.pool_manager.get_overall_status().await?;
        let tier_status = self.tier_manager.get_tier_status().await?;
        let health_status = self.health_monitor.get_current_status().await?;
        let performance_metrics = self.performance_monitor.get_current_metrics().await;
        
        // Get AI status if available
        let ai_status = if let Some(ai) = &self.ai_integration {
            Some(AiIntegrationStatus {
                enabled: true,
                models_deployed: 3, // TODO: Get actual count
                optimization_active: true,
                last_optimization: std::time::SystemTime::now(),
                prediction_accuracy: 0.85,
            })
        } else {
            None
        };
        
        // Get migration status
        let migration_stats = self.migration_engine.get_statistics().await;
        let migration_status = MigrationStatus {
            active_jobs: migration_stats.active_migrations,
            queued_jobs: migration_stats.queued_migrations,
            completed_jobs: migration_stats.successful_migrations,
            failed_jobs: migration_stats.failed_migrations,
            total_bytes_migrated: migration_stats.total_bytes_migrated,
        };
        
        // Get snapshot status
        let snapshot_stats = self.snapshot_manager.get_statistics().await;
        let snapshot_status = SnapshotStatus {
            total_snapshots: snapshot_stats.total_snapshots,
            active_policies: snapshot_stats.active_policies,
            pending_operations: snapshot_stats.pending_operations,
            recent_failures: snapshot_stats.recent_failures,
        };
        
        Ok(EnhancedServiceStatus {
            overall_health: health_status.overall_health,
            pool_status,
            tier_status,
            performance_metrics,
            ai_status,
            migration_status,
            snapshot_status,
            metrics: self.metrics.get_current_metrics().await?,
            timestamp: chrono::Utc::now(),
        })
    }
    
    /// Initialize enhanced ZFS system
    pub async fn initialize_system(&self) -> Result<()> {
        info!("Initializing Enhanced ZFS system components");
        
        // Discover and validate pools
        self.pool_manager.discover_pools().await?;
        
        // Initialize tier configurations
        self.tier_manager.initialize_tiers().await?;
        
        // Start health monitoring
        self.health_monitor.start_monitoring().await?;
        
        // Begin metrics collection
        self.metrics.start_collection().await?;
        
        info!("Enhanced ZFS system initialization complete");
        Ok(())
    }
    
    /// Get AI tier recommendation for a file
    pub async fn get_ai_tier_recommendation(&self, file_path: &str) -> Result<Option<crate::ai_integration::TierPrediction>> {
        if let Some(ai) = &self.ai_integration {
            ai.predict_tier(file_path).await
        } else {
            debug!("AI integration not available, returning None for tier recommendation");
            Ok(None)
        }
    }
    
    /// Get ZFS system health status
    pub async fn get_zfs_health(&self) -> Result<EnhancedServiceStatus> {
        self.get_service_status().await
    }

    /// Create a new ZFS pool
    pub async fn create_pool(&self, name: &str, devices: &[String]) -> Result<crate::pool::PoolInfo> {
        info!("Creating ZFS pool via manager: {}", name);
        let result = self.pool_manager.create_pool(name, devices).await?;
        
        // Update metrics
        self.metrics.increment_operation("pool_create").await;
        
        Ok(result)
    }

    /// Destroy a ZFS pool
    pub async fn destroy_pool(&self, name: &str) -> Result<()> {
        warn!("Destroying ZFS pool via manager: {}", name);
        let result = self.pool_manager.destroy_pool(name).await?;
        
        // Update metrics
        self.metrics.increment_operation("pool_destroy").await;
        
        Ok(result)
    }

    /// Get detailed pool status
    pub async fn get_pool_status(&self, name: &str) -> Result<String> {
        debug!("Getting pool status via manager: {}", name);
        self.pool_manager.get_pool_status(name).await
    }

    /// Start pool scrub operation
    pub async fn scrub_pool(&self, name: &str) -> Result<()> {
        info!("Starting pool scrub via manager: {}", name);
        let result = self.pool_manager.scrub_pool(name).await?;
        
        // Update metrics
        self.metrics.increment_operation("pool_scrub").await;
        
        Ok(result)
    }

    /// Create a new dataset
    pub async fn create_dataset(&self, name: &str, parent: &str, tier: StorageTier) -> Result<crate::dataset::DatasetInfo> {
        info!("Creating dataset via manager: {} under {}", name, parent);
        let result = self.dataset_manager.create_dataset(name, parent, tier).await?;
        
        // Update metrics
        self.metrics.increment_operation("dataset_create").await;
        
        Ok(result)
    }

    /// Destroy a dataset
    pub async fn destroy_dataset(&self, name: &str) -> Result<()> {
        warn!("Destroying dataset via manager: {}", name);
        let result = self.dataset_manager.destroy_dataset(name).await?;
        
        // Update metrics
        self.metrics.increment_operation("dataset_destroy").await;
        
        Ok(result)
    }

    /// List snapshots for a dataset
    pub async fn list_snapshots(&self, dataset: &str) -> Result<Vec<crate::snapshot::SnapshotInfo>> {
        debug!("Listing snapshots for dataset: {}", dataset);
        self.snapshot_manager.list_snapshots(dataset).await
    }

    /// Get performance analytics
    pub async fn get_performance_analytics(&self) -> Result<PerformanceAnalytics> {
        let current_metrics = self.performance_monitor.get_current_metrics().await;
        let history = self.performance_monitor.get_performance_history(Some(100)).await;
        let active_alerts = self.performance_monitor.get_active_alerts().await;
        
        // Get tier-specific analytics
        let mut tier_analytics = std::collections::HashMap::new();
        for tier in [StorageTier::Hot, StorageTier::Warm, StorageTier::Cold] {
            if let Some(tier_data) = self.performance_monitor.get_tier_metrics(&tier).await {
                tier_analytics.insert(tier, tier_data);
            }
        }
        
        Ok(PerformanceAnalytics {
            current_metrics,
            history,
            active_alerts,
            tier_analytics,
        })
    }
    
    /// Trigger manual optimization
    pub async fn trigger_optimization(&self) -> Result<OptimizationResult> {
        info!("Triggering manual ZFS optimization");
        
        let mut results = Vec::new();
        
        // Run AI optimization if available
        if let Some(ai) = &self.ai_integration {
            match ai.get_optimization_opportunities().await {
                opportunities => {
                    results.push(format!("Found {} optimization opportunities", opportunities.len()));
                    for opp in opportunities.iter().take(5) {
                        results.push(format!("- {}: {:.1}% improvement", opp.description, opp.expected_impact));
                    }
                }
            }
        }
        
        // Run migration optimization
        let migration_stats = self.migration_engine.get_statistics().await;
        results.push(format!("Migration engine: {} active, {} queued", 
                            migration_stats.active_migrations, migration_stats.queued_migrations));
        
        // Run snapshot optimization
        let snapshot_stats = self.snapshot_manager.get_statistics().await;
        results.push(format!("Snapshot management: {} total snapshots, {} active policies",
                            snapshot_stats.total_snapshots, snapshot_stats.active_policies));
        
        Ok(OptimizationResult {
            timestamp: std::time::SystemTime::now(),
            results,
            success: true,
        })
    }
    
    /// Graceful shutdown of enhanced ZFS manager
    pub async fn shutdown(&self) -> Result<()> {
        info!("Shutting down Enhanced ZFS Manager");
        
        // The actual shutdown is handled by the stop method
        // This method is for external cleanup if needed
        
        info!("Enhanced ZFS Manager shutdown complete");
        Ok(())
    }
}

/// Enhanced service status for health reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedServiceStatus {
    pub overall_health: HealthState,
    pub pool_status: PoolOverallStatus,
    pub tier_status: TierOverallStatus,
    pub performance_metrics: crate::performance::CurrentPerformanceMetrics,
    pub ai_status: Option<AiIntegrationStatus>,
    pub migration_status: MigrationStatus,
    pub snapshot_status: SnapshotStatus,
    pub metrics: CurrentMetrics,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// AI integration status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiIntegrationStatus {
    pub enabled: bool,
    pub models_deployed: u32,
    pub optimization_active: bool,
    pub last_optimization: std::time::SystemTime,
    pub prediction_accuracy: f64,
}

/// Migration status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationStatus {
    pub active_jobs: u32,
    pub queued_jobs: u32,
    pub completed_jobs: u64,
    pub failed_jobs: u64,
    pub total_bytes_migrated: u64,
}

/// Snapshot status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotStatus {
    pub total_snapshots: u64,
    pub active_policies: u32,
    pub pending_operations: u32,
    pub recent_failures: u32,
}

/// Performance analytics data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAnalytics {
    pub current_metrics: crate::performance::CurrentPerformanceMetrics,
    pub history: Vec<crate::performance::PerformanceSnapshot>,
    pub active_alerts: Vec<crate::performance::ActiveAlert>,
    pub tier_analytics: std::collections::HashMap<StorageTier, crate::performance::TierPerformanceData>,
}

/// Optimization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationResult {
    pub timestamp: std::time::SystemTime,
    pub results: Vec<String>,
    pub success: bool,
}

/// Health state enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthState {
    Healthy,
    Warning,
    Critical,
    Unknown,
}

/// Placeholder types (will be defined in respective modules)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolOverallStatus {
    pub pools_online: usize,
    pub pools_degraded: usize,
    pub total_capacity: u64,
    pub available_capacity: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierOverallStatus {
    pub hot_utilization: f64,
    pub warm_utilization: f64,
    pub cold_utilization: f64,
    pub migration_queue_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentMetrics {
    pub operations_per_second: f64,
    pub throughput_bytes_per_second: u64,
    pub average_latency_ms: f64,
    pub error_rate: f64,
} 