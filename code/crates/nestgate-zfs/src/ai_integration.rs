//! ZFS AI Integration
//!
//! Integration between ZFS storage management and AI models for intelligent
//! tier optimization, predictive analytics, and automated decision making.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use tokio::time::interval;
use tracing::{debug, error, info, warn};
use serde::{Serialize, Deserialize};

use nestgate_core::{Result as CoreResult, NestGateError, StorageTier};

use crate::{
    config::ZfsConfig,
    pool::ZfsPoolManager,
    dataset::ZfsDatasetManager,
    automation::DatasetAnalyzer,
    migration::MigrationEngine,
    types::CompressionAlgorithm,
    error::ZfsError,
};

// Placeholder AI types (will be replaced with actual AI models integration)
#[derive(Debug, Clone)]
pub struct ModelManager {
    gpu_memory: u64,
    compute_capability: f64,
    cache_dir: std::path::PathBuf,
}

#[derive(Debug, Clone)]
pub struct ModelConfig {
    pub id: String,
    pub model_type: ModelType,
    pub format: ModelFormat,
    pub path: std::path::PathBuf,
    pub size: u64,
    pub priority: Priority,
    pub min_compute_capability: f64,
}

#[derive(Debug, Clone)]
pub enum ModelType {
    StorageOptimizer,
    WorkloadPredictor,
    AnomalyDetector,
}

#[derive(Debug, Clone)]
pub enum ModelFormat {
    ONNX,
}

#[derive(Debug, Clone)]
pub enum Priority {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone)]
pub struct TierOptimization {
    pub to_warm_count: u32,
    pub to_cold_count: u32,
    pub estimated_performance_improvement: f64,
}

/// AI-powered ZFS optimization service
#[derive(Debug)]
pub struct ZfsAiIntegration {
    /// AI model manager
    model_manager: Arc<ModelManager>,
    /// ZFS components
    pool_manager: Arc<ZfsPoolManager>,
    dataset_manager: Arc<ZfsDatasetManager>,
    dataset_analyzer: Arc<DatasetAnalyzer>,
    migration_engine: Arc<MigrationEngine>,
    
    /// Configuration
    config: ZfsAiConfig,
    
    /// Analytics data
    performance_history: Arc<RwLock<Vec<PerformanceSnapshot>>>,
    tier_statistics: Arc<RwLock<HashMap<StorageTier, TierAnalytics>>>,
    prediction_cache: Arc<RwLock<HashMap<String, TierPrediction>>>,
    
    /// Background tasks
    optimization_task: Option<tokio::task::JoinHandle<()>>,
    analytics_task: Option<tokio::task::JoinHandle<()>>,
}

/// Configuration for ZFS AI integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsAiConfig {
    /// Enable AI-powered tier optimization
    pub enable_tier_optimization: bool,
    /// Enable predictive analytics
    pub enable_predictive_analytics: bool,
    /// Enable anomaly detection
    pub enable_anomaly_detection: bool,
    /// Optimization interval in seconds
    pub optimization_interval: u64,
    /// Analytics collection interval in seconds
    pub analytics_interval: u64,
    /// Minimum confidence threshold for recommendations
    pub min_confidence_threshold: f64,
    /// Maximum models to deploy simultaneously
    pub max_concurrent_models: usize,
    /// Model cache directory
    pub model_cache_dir: String,
}

impl Default for ZfsAiConfig {
    fn default() -> Self {
        Self {
            enable_tier_optimization: true,
            enable_predictive_analytics: true,
            enable_anomaly_detection: true,
            optimization_interval: 3600, // 1 hour
            analytics_interval: 300,     // 5 minutes
            min_confidence_threshold: 0.7,
            max_concurrent_models: 3,
            model_cache_dir: "/var/cache/nestgate/ai-models".to_string(),
        }
    }
}

/// Performance snapshot for historical analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSnapshot {
    /// Timestamp of the snapshot
    pub timestamp: SystemTime,
    /// Tier-specific performance metrics
    pub tier_metrics: HashMap<StorageTier, TierPerformanceMetrics>,
    /// Overall system metrics
    pub system_metrics: SystemPerformanceMetrics,
}

/// Performance metrics for a specific tier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierPerformanceMetrics {
    /// Average latency in milliseconds
    pub avg_latency_ms: f64,
    /// Throughput in MB/s
    pub throughput_mbs: f64,
    /// IOPS (Input/Output Operations Per Second)
    pub iops: f64,
    /// Utilization percentage (0-100)
    pub utilization_percent: f64,
    /// Compression ratio
    pub compression_ratio: f64,
    /// Cache hit ratio
    pub cache_hit_ratio: f64,
}

/// System-wide performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemPerformanceMetrics {
    /// Total operations per second
    pub total_ops_per_second: f64,
    /// Total throughput in MB/s
    pub total_throughput_mbs: f64,
    /// Memory usage in bytes
    pub memory_usage_bytes: u64,
    /// CPU utilization percentage
    pub cpu_utilization_percent: f64,
    /// Network I/O in MB/s
    pub network_io_mbs: f64,
}

/// Analytics data for a storage tier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierAnalytics {
    /// Tier identifier
    pub tier: StorageTier,
    /// File count
    pub file_count: u64,
    /// Total size in bytes
    pub total_size_bytes: u64,
    /// Average file size
    pub avg_file_size_bytes: u64,
    /// Access pattern analysis
    pub access_patterns: AccessPatternAnalytics,
    /// Performance trends
    pub performance_trends: PerformanceTrends,
    /// Optimization opportunities
    pub optimization_opportunities: Vec<OptimizationOpportunity>,
}

/// Access pattern analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPatternAnalytics {
    /// Read/write ratio
    pub read_write_ratio: f64,
    /// Peak access hours
    pub peak_hours: Vec<u8>,
    /// Access frequency distribution
    pub frequency_distribution: HashMap<String, u64>,
    /// Sequential vs random access ratio
    pub sequential_ratio: f64,
}

/// Performance trend analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTrends {
    /// Latency trend (positive = increasing)
    pub latency_trend: f64,
    /// Throughput trend (positive = increasing)
    pub throughput_trend: f64,
    /// Utilization trend (positive = increasing)
    pub utilization_trend: f64,
    /// Prediction accuracy
    pub prediction_accuracy: f64,
}

/// Optimization opportunity identified by AI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationOpportunity {
    /// Type of optimization
    pub optimization_type: OptimizationType,
    /// Description of the opportunity
    pub description: String,
    /// Expected impact (percentage improvement)
    pub expected_impact: f64,
    /// Confidence level (0-1)
    pub confidence: f64,
    /// Implementation complexity
    pub complexity: OptimizationComplexity,
    /// Estimated time to implement
    pub implementation_time: Duration,
}

/// Types of optimization opportunities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationType {
    /// Tier migration recommendation
    TierMigration,
    /// Compression algorithm change
    CompressionOptimization,
    /// Cache configuration adjustment
    CacheOptimization,
    /// Dataset property tuning
    PropertyTuning,
    /// Storage layout optimization
    LayoutOptimization,
    /// Performance parameter adjustment
    PerformanceTuning,
}

/// Complexity level for optimization implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationComplexity {
    /// Low complexity, automated implementation
    Low,
    /// Medium complexity, semi-automated
    Medium,
    /// High complexity, manual intervention required
    High,
    /// Critical complexity, expert review required
    Critical,
}

/// Tier prediction result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierPrediction {
    /// File path
    pub file_path: String,
    /// Predicted optimal tier
    pub predicted_tier: StorageTier,
    /// Current tier
    pub current_tier: StorageTier,
    /// Confidence score
    pub confidence: f64,
    /// Reasoning
    pub reasoning: String,
    /// Expected performance improvement
    pub expected_improvement: f64,
    /// Prediction timestamp
    pub timestamp: SystemTime,
}

impl ModelManager {
    pub async fn new(gpu_memory: u64, compute_capability: f64, cache_dir: std::path::PathBuf) -> CoreResult<Self> {
        Ok(Self {
            gpu_memory,
            compute_capability,
            cache_dir,
        })
    }
    
    pub async fn start(&self) -> CoreResult<()> {
        info!("Starting AI model manager (placeholder)");
        Ok(())
    }
    
    pub async fn stop(&self) -> CoreResult<()> {
        info!("Stopping AI model manager (placeholder)");
        Ok(())
    }
    
    pub async fn deploy_model(&self, _config: ModelConfig) -> CoreResult<()> {
        info!("Deploying AI model (placeholder)");
        Ok(())
    }
    
    pub async fn optimize_tier_placement(&self) -> CoreResult<TierOptimization> {
        // Placeholder implementation
        Ok(TierOptimization {
            to_warm_count: 10,
            to_cold_count: 5,
            estimated_performance_improvement: 15.0,
        })
    }
}

impl ZfsAiIntegration {
    /// Create a new ZFS AI integration service
    pub async fn new(
        config: ZfsAiConfig,
        pool_manager: Arc<ZfsPoolManager>,
        dataset_manager: Arc<ZfsDatasetManager>,
        dataset_analyzer: Arc<DatasetAnalyzer>,
        migration_engine: Arc<MigrationEngine>,
    ) -> CoreResult<Self> {
        info!("Initializing ZFS AI integration");
        
        // Initialize AI model manager
        let model_manager = Arc::new(
            ModelManager::new(
                2 * 1024 * 1024 * 1024, // 2GB GPU memory
                7.5, // RTX 2070 compute capability
                std::path::PathBuf::from(&config.model_cache_dir),
            ).await
            .map_err(|e| NestGateError::Internal(format!("Failed to initialize AI model manager: {}", e)))?
        );
        
        Ok(Self {
            model_manager,
            pool_manager,
            dataset_manager,
            dataset_analyzer,
            migration_engine,
            config,
            performance_history: Arc::new(RwLock::new(Vec::new())),
            tier_statistics: Arc::new(RwLock::new(HashMap::new())),
            prediction_cache: Arc::new(RwLock::new(HashMap::new())),
            optimization_task: None,
            analytics_task: None,
        })
    }
    
    /// Start AI-powered optimization services
    pub async fn start(&mut self) -> CoreResult<()> {
        info!("Starting ZFS AI integration services");
        
        // Start AI model manager
        self.model_manager.start().await
            .map_err(|e| NestGateError::Internal(format!("Failed to start AI model manager: {}", e)))?;
        
        // Deploy optimization models
        self.deploy_optimization_models().await?;
        
        // Start background optimization task
        if self.config.enable_tier_optimization {
            self.start_optimization_task().await?;
        }
        
        // Start analytics collection task
        if self.config.enable_predictive_analytics {
            self.start_analytics_task().await?;
        }
        
        info!("ZFS AI integration services started successfully");
        Ok(())
    }
    
    /// Stop AI services
    pub async fn stop(&mut self) -> CoreResult<()> {
        info!("Stopping ZFS AI integration services");
        
        // Stop background tasks
        if let Some(task) = self.optimization_task.take() {
            task.abort();
        }
        
        if let Some(task) = self.analytics_task.take() {
            task.abort();
        }
        
        // Stop AI model manager
        self.model_manager.stop().await
            .map_err(|e| NestGateError::Internal(format!("Failed to stop AI model manager: {}", e)))?;
        
        info!("ZFS AI integration services stopped");
        Ok(())
    }
    
    /// Deploy AI models for optimization
    async fn deploy_optimization_models(&self) -> CoreResult<()> {
        info!("Deploying AI optimization models");
        
        // Deploy storage optimizer model
        if self.config.enable_tier_optimization {
            let storage_config = ModelConfig {
                id: "zfs-storage-optimizer".to_string(),
                model_type: ModelType::StorageOptimizer,
                format: ModelFormat::ONNX,
                path: std::path::PathBuf::from(&format!("{}/storage_optimizer.onnx", self.config.model_cache_dir)),
                size: 50 * 1024 * 1024, // 50MB
                priority: Priority::High,
                min_compute_capability: 7.0,
            };
            
            self.model_manager.deploy_model(storage_config).await
                .map_err(|e| NestGateError::Internal(format!("Failed to deploy storage optimizer: {}", e)))?;
        }
        
        // Deploy workload predictor model
        if self.config.enable_predictive_analytics {
            let predictor_config = ModelConfig {
                id: "zfs-workload-predictor".to_string(),
                model_type: ModelType::WorkloadPredictor,
                format: ModelFormat::ONNX,
                path: std::path::PathBuf::from(&format!("{}/workload_predictor.onnx", self.config.model_cache_dir)),
                size: 30 * 1024 * 1024, // 30MB
                priority: Priority::Medium,
                min_compute_capability: 7.0,
            };
            
            self.model_manager.deploy_model(predictor_config).await
                .map_err(|e| NestGateError::Internal(format!("Failed to deploy workload predictor: {}", e)))?;
        }
        
        // Deploy anomaly detector model
        if self.config.enable_anomaly_detection {
            let anomaly_config = ModelConfig {
                id: "zfs-anomaly-detector".to_string(),
                model_type: ModelType::AnomalyDetector,
                format: ModelFormat::ONNX,
                path: std::path::PathBuf::from(&format!("{}/anomaly_detector.onnx", self.config.model_cache_dir)),
                size: 25 * 1024 * 1024, // 25MB
                priority: Priority::Medium,
                min_compute_capability: 7.0,
            };
            
            self.model_manager.deploy_model(anomaly_config).await
                .map_err(|e| NestGateError::Internal(format!("Failed to deploy anomaly detector: {}", e)))?;
        }
        
        info!("AI optimization models deployed successfully");
        Ok(())
    }
    
    /// Start optimization background task
    async fn start_optimization_task(&mut self) -> CoreResult<()> {
        let model_manager = Arc::clone(&self.model_manager);
        let dataset_analyzer = Arc::clone(&self.dataset_analyzer);
        let migration_engine = Arc::clone(&self.migration_engine);
        let tier_statistics = Arc::clone(&self.tier_statistics);
        let prediction_cache = Arc::clone(&self.prediction_cache);
        let config = self.config.clone();
        
        let task = tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(config.optimization_interval));
            
            loop {
                interval.tick().await;
                
                if let Err(e) = Self::run_optimization_cycle(
                    &model_manager,
                    &dataset_analyzer,
                    &migration_engine,
                    &tier_statistics,
                    &prediction_cache,
                    &config,
                ).await {
                    error!("Optimization cycle failed: {}", e);
                }
            }
        });
        
        self.optimization_task = Some(task);
        Ok(())
    }
    
    /// Run a single optimization cycle
    async fn run_optimization_cycle(
        model_manager: &Arc<ModelManager>,
        dataset_analyzer: &Arc<DatasetAnalyzer>,
        migration_engine: &Arc<MigrationEngine>,
        tier_statistics: &Arc<RwLock<HashMap<StorageTier, TierAnalytics>>>,
        prediction_cache: &Arc<RwLock<HashMap<String, TierPrediction>>>,
        config: &ZfsAiConfig,
    ) -> CoreResult<()> {
        debug!("Running AI optimization cycle");
        
        // Run tier optimization using AI
        let optimization_result = model_manager.optimize_tier_placement().await
            .map_err(|e| NestGateError::Internal(format!("AI tier optimization failed: {}", e)))?;
        
        info!("AI tier optimization completed: {} to warm, {} to cold, {:.1}% improvement",
              optimization_result.to_warm_count,
              optimization_result.to_cold_count,
              optimization_result.estimated_performance_improvement);
        
        // TODO: Implement specific optimization actions based on AI recommendations
        // This would include:
        // 1. Identifying files for migration
        // 2. Queuing migration jobs
        // 3. Updating tier configurations
        // 4. Monitoring optimization results
        
        Ok(())
    }
    
    /// Start analytics collection task
    async fn start_analytics_task(&mut self) -> CoreResult<()> {
        let pool_manager = Arc::clone(&self.pool_manager);
        let dataset_manager = Arc::clone(&self.dataset_manager);
        let performance_history = Arc::clone(&self.performance_history);
        let tier_statistics = Arc::clone(&self.tier_statistics);
        let config = self.config.clone();
        
        let task = tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(config.analytics_interval));
            
            loop {
                interval.tick().await;
                
                if let Err(e) = Self::collect_analytics(
                    &pool_manager,
                    &dataset_manager,
                    &performance_history,
                    &tier_statistics,
                ).await {
                    error!("Analytics collection failed: {}", e);
                }
            }
        });
        
        self.analytics_task = Some(task);
        Ok(())
    }
    
    /// Collect performance analytics
    async fn collect_analytics(
        pool_manager: &Arc<ZfsPoolManager>,
        dataset_manager: &Arc<ZfsDatasetManager>,
        performance_history: &Arc<RwLock<Vec<PerformanceSnapshot>>>,
        tier_statistics: &Arc<RwLock<HashMap<StorageTier, TierAnalytics>>>,
    ) -> CoreResult<()> {
        debug!("Collecting performance analytics");
        
        // TODO: Implement actual analytics collection
        // This would include:
        // 1. Collecting tier performance metrics
        // 2. Analyzing access patterns
        // 3. Updating performance history
        // 4. Computing trends and predictions
        
        Ok(())
    }
    
    /// Get AI-powered tier recommendation for a file
    pub async fn get_ai_tier_recommendation(&self, file_path: &str) -> CoreResult<TierPrediction> {
        // Check cache first
        {
            let cache = self.prediction_cache.read().await;
            if let Some(prediction) = cache.get(file_path) {
                // Return cached prediction if it's recent (within 1 hour)
                if prediction.timestamp.elapsed().unwrap_or(Duration::from_secs(3600)) < Duration::from_secs(3600) {
                    return Ok(prediction.clone());
                }
            }
        }

        // Analyze file characteristics
        let file_analysis = self.analyze_file_characteristics(file_path).await?;
        
        // Get current tier
        let current_tier = self.determine_current_tier(file_path).await?;
        
        // Predict optimal tier using AI model
        let predicted_tier = self.predict_optimal_tier(&file_analysis, current_tier).await?;
        
        let prediction = TierPrediction {
            file_path: file_path.to_string(),
            predicted_tier,
            current_tier: self.determine_current_tier(file_path).await?,
            confidence: file_analysis.confidence,
            reasoning: file_analysis.reasoning,
            expected_improvement: file_analysis.expected_improvement,
            timestamp: SystemTime::now(),
        };

        // Cache the prediction
        {
            let mut cache = self.prediction_cache.write().await;
            cache.insert(file_path.to_string(), prediction.clone());
        }

        Ok(prediction)
    }

    /// Predict tier for a file (alias for get_ai_tier_recommendation)
    pub async fn predict_tier(&self, file_path: &str) -> CoreResult<Option<TierPrediction>> {
        match self.get_ai_tier_recommendation(file_path).await {
            Ok(prediction) => Ok(Some(prediction)),
            Err(e) => {
                warn!("Failed to get AI tier recommendation for {}: {}", file_path, e);
                Ok(None)
            }
        }
    }
    
    /// Get tier analytics
    pub async fn get_tier_analytics(&self) -> HashMap<StorageTier, TierAnalytics> {
        self.tier_statistics.read().await.clone()
    }
    
    /// Get performance history
    pub async fn get_performance_history(&self, limit: Option<usize>) -> Vec<PerformanceSnapshot> {
        let history = self.performance_history.read().await;
        if let Some(limit) = limit {
            history.iter().rev().take(limit).cloned().collect()
        } else {
            history.clone()
        }
    }
    
    /// Get optimization opportunities across all tiers
    pub async fn get_optimization_opportunities(&self) -> Vec<OptimizationOpportunity> {
        let tier_stats = self.tier_statistics.read().await;
        let mut opportunities = Vec::new();
        
        for (tier, analytics) in tier_stats.iter() {
            opportunities.extend(analytics.optimization_opportunities.clone());
        }
        
        // Sort by expected impact (highest first)
        opportunities.sort_by(|a, b| b.expected_impact.partial_cmp(&a.expected_impact).unwrap_or(std::cmp::Ordering::Equal));
        
        opportunities
    }

    /// Analyze file characteristics for AI prediction
    async fn analyze_file_characteristics(&self, file_path: &str) -> CoreResult<FileAnalysis> {
        // TODO: Implement actual file analysis using AI models
        // For now, provide mock analysis based on file extension and path
        
        let extension = std::path::Path::new(file_path)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
        
        let confidence = match extension {
            "log" | "tmp" | "cache" => 0.9, // High confidence for temporary files -> cold
            "jpg" | "png" | "mp4" | "avi" => 0.8, // High confidence for media -> warm/cold
            "db" | "sql" | "idx" => 0.85, // High confidence for databases -> hot/warm
            "txt" | "doc" | "pdf" => 0.7, // Medium confidence for documents
            _ => 0.6, // Lower confidence for unknown types
        };
        
        let reasoning = format!("File analysis based on extension '{}' and path patterns", extension);
        
        let expected_improvement = match extension {
            "log" | "tmp" | "cache" => 30.0, // High improvement moving temp files to cold
            "jpg" | "png" | "mp4" | "avi" => 20.0, // Good improvement for media files
            "db" | "sql" | "idx" => 15.0, // Moderate improvement for databases
            _ => 10.0, // Lower improvement for unknown types
        };
        
        Ok(FileAnalysis {
            confidence,
            reasoning,
            expected_improvement,
        })
    }

    /// Determine current tier of a file
    async fn determine_current_tier(&self, file_path: &str) -> CoreResult<StorageTier> {
        // TODO: Implement actual tier detection by querying ZFS dataset properties
        // For now, determine based on path patterns
        
        if file_path.contains("/hot/") || file_path.contains("/cache/") {
            Ok(StorageTier::Hot)
        } else if file_path.contains("/warm/") || file_path.contains("/active/") {
            Ok(StorageTier::Warm)
        } else if file_path.contains("/cold/") || file_path.contains("/archive/") {
            Ok(StorageTier::Cold)
        } else {
            // Default to warm tier
            Ok(StorageTier::Warm)
        }
    }

    /// Predict optimal tier using AI model
    async fn predict_optimal_tier(&self, analysis: &FileAnalysis, current_tier: StorageTier) -> CoreResult<StorageTier> {
        // TODO: Implement actual AI model prediction
        // For now, provide rule-based prediction
        
        // If confidence is low, don't recommend changes
        if analysis.confidence < self.config.min_confidence_threshold {
            return Ok(current_tier);
        }
        
        // Simple rule-based prediction for demonstration
        let predicted_tier = if analysis.reasoning.contains("tmp") || analysis.reasoning.contains("cache") || analysis.reasoning.contains("log") {
            StorageTier::Cold
        } else if analysis.reasoning.contains("db") || analysis.reasoning.contains("sql") || analysis.reasoning.contains("idx") {
            StorageTier::Hot
        } else if analysis.reasoning.contains("jpg") || analysis.reasoning.contains("png") || analysis.reasoning.contains("mp4") {
            StorageTier::Warm
        } else {
            current_tier // No change if uncertain
        };
        
        Ok(predicted_tier)
    }
}

/// File analysis result for AI prediction
#[derive(Debug, Clone)]
struct FileAnalysis {
    confidence: f64,
    reasoning: String,
    expected_improvement: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[tokio::test]
    async fn test_zfs_ai_config_default() {
        let config = ZfsAiConfig::default();
        
        assert!(config.enable_tier_optimization);
        assert!(config.enable_predictive_analytics);
        assert!(config.enable_anomaly_detection);
        assert_eq!(config.optimization_interval, 3600);
        assert_eq!(config.analytics_interval, 300);
        assert_eq!(config.min_confidence_threshold, 0.7);
        assert_eq!(config.max_concurrent_models, 3);
    }
    
    #[test]
    fn test_optimization_opportunity_creation() {
        let opportunity = OptimizationOpportunity {
            optimization_type: OptimizationType::TierMigration,
            description: "Move frequently accessed files to hot tier".to_string(),
            expected_impact: 25.0,
            confidence: 0.85,
            complexity: OptimizationComplexity::Low,
            implementation_time: Duration::from_secs(300),
        };
        
        assert_eq!(opportunity.expected_impact, 25.0);
        assert_eq!(opportunity.confidence, 0.85);
        assert!(matches!(opportunity.complexity, OptimizationComplexity::Low));
    }
} 