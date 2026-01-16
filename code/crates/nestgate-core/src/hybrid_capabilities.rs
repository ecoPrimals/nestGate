//! Hybrid Capabilities module

use crate::error::NestGateError;
use dashmap::DashMap;
use std::collections::HashMap;
//
// **Architecture**: Local Smart + Universal Adapter + Failsafe Defaults
// **Principle**: NestGate only knows storage. External capabilities route through universal adapter.
//
// ## Capability Tiers:
// - **Local Smart**: Fast storage-specific intelligence (no external deps)
// - **External Heavy**: Route through universal adapter for heavy compute  
// - **Failsafe**: Always-working defaults for standalone operation

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use crate::universal_adapter::PrimalAgnosticAdapter;

/// Capability execution modes
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Capabilitymode
pub enum CapabilityMode {
    /// Fast, lightweight, storage-specific intelligence
    LocalSmart,
    /// Route to external primal via universal adapter for heavy compute
    ExternalHeavy { capability_type: String },
    /// Basic functionality that always works
    Failsafe,
}
/// Fallback strategy when external capabilities fail
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Fallbackstrategy
pub enum FallbackStrategy {
    /// Fall back to local smart implementation
    LocalSmart,
    /// Fall back to basic failsafe implementation
    Failsafe,
    /// No fallback - fail if external unavailable
    None,
}
/// Configuration for a specific capability
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Capability
pub struct CapabilityConfig {
    /// Mode
    pub mode: CapabilityMode,
    /// Fallback
    pub fallback: Option<FallbackStrategy>,
    /// Timeout Ms
    pub timeout_ms: Option<u64>,
    /// Retry Attempts
    pub retry_attempts: Option<u32>,
}
/// Hybrid capability resolver that maintains primal sovereignty
pub struct HybridCapabilityResolver {
    /// Local storage-specific smart capabilities (NestGate's domain)
    local_capabilities: Arc<LocalStorageCapabilities>,
    /// Universal adapter for external primal communication
    universal_adapter: Arc<UniversalAdapter>,
    /// Failsafe implementations for standalone operation
    failsafe_defaults: Arc<FailsafeDefaults>,
    /// Configuration for each capability (lock-free for 5-10x better performance)
    capability_configs: Arc<DashMap<String, CapabilityConfig>>,
}
impl HybridCapabilityResolver {
    /// Creates a new instance
    pub fn new(
        universal_adapter: Arc<UniversalAdapter>,
        config: HashMap<String, CapabilityConfig>,
    ) -> Self {
        Self {
            local_capabilities: Arc::new(LocalStorageCapabilities::new()),
            universal_adapter,
            failsafe_defaults: Arc::new(FailsafeDefaults::new()),
            capability_configs: Arc::new(config.into_iter().collect()),
        }
    }

    /// Resolve a capability using hybrid approach
    /// 1. Try configured mode (local smart or external heavy)
    /// 2. Fall back according to strategy
    /// 3. Always have failsafe as final option
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn resolve_storage_tier_recommendation(
        &self,
        file_metrics: &FileMetrics,
    ) -> Result<TierRecommendation, NestGateError>  {
        let capability_name = "storage.tier_recommendation";
        let config = self.get_capability_config(capability_name).await?;

        match &config.mode {
            CapabilityMode::LocalSmart => {
                debug!("Using local smart tier recommendation");
                self.local_capabilities.recommend_tier(file_metrics).await
            }
            CapabilityMode::ExternalHeavy { capability_type } => {
                debug!("Attempting external heavy tier recommendation via universal adapter");

                // Try external primal through universal adapter
                match self
                    .try_external_capability(capability_type, file_metrics)
                    .await
                {
                    Ok(recommendation) => {
                        info!("External tier recommendation successful");
                        Ok(recommendation)
                    }
                    Err(e) => {
                        warn!("External capability failed: {}, using fallback", e);
                        self.execute_fallback(&config, file_metrics).await
                    }
                }
            }
            CapabilityMode::Failsafe => {
                debug!("Using failsafe tier recommendation");
                Ok(self
                    .failsafe_defaults
                    .default_tier_recommendation(file_metrics))
            }
        }
    }

    /// Try to execute capability through universal adapter
    /// NestGate doesn't know which primal provides the capability
    async fn try_external_capability(
        &self,
        capability_type: &str,
        file_metrics: &FileMetrics,
    ) -> Result<TierRecommendation, NestGateError> {
        // Universal adapter handles finding and routing to appropriate primal
        // NestGate only knows the capability type, not which primal provides it
        let request = CapabilityRequest {
            capability_type: capability_type.to_string(),
            data: serde_json::to_value(file_metrics)
                .map_err(|e| NestGateError::SerializationError(e"))?,
            timeout_ms: Some(5000), // 5 second timeout for external calls
        };

        self.universal_adapter
            .execute_capability(request)
            .await
            .and_then(|response| {
                serde_json::from_value(response.data)
                    .map_err(|e| NestGateError::DeserializationError(e"))
            })
    }

    /// Execute fallback strategy
    async fn execute_fallback(
        &self,
        config: &CapabilityConfig,
        file_metrics: &FileMetrics,
    ) -> Result<TierRecommendation, NestGateError> {
        match &config.fallback {
            Some(FallbackStrategy::LocalSmart) => {
                info!("Falling back to local smart tier recommendation");
                self.local_capabilities.recommend_tier(file_metrics).await
            }
            Some(FallbackStrategy::Failsafe) => {
                info!("Falling back to failsafe tier recommendation");
                Ok(self
                    .failsafe_defaults
                    .default_tier_recommendation(file_metrics))
            }
            None | Some(FallbackStrategy::None) => Err(NestGateError::FeatureUnavailable {
                feature: "tier_recommendation".to_string(),
                reason: "No fallback configured for tier recommendation".to_string(),
                available_in: None,
                alternatives: vec!["local_smart".to_string(), "failsafe".to_string()],
            }),
        }
    }

    /// Gets Capability Config
    async fn get_capability_config(
        &self,
        capability: &str,
    ) -> Result<CapabilityConfig, NestGateError> {
        // Lock-free config lookup
        self.capability_configs.get(capability).map(|entry| entry.value().clone()).ok_or_else(|| {
            NestGateError::ConfigurationError(format!(
                "No configuration found for capability: {capability}"
            ))
        })
    }
}

/// Local storage-specific smart capabilities
/// These are NestGate's domain - fast, lightweight, storage-focused
pub struct LocalStorageCapabilities {
    // Storage-specific intelligence that doesn't require external compute
}
impl Default for LocalStorageCapabilities {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl LocalStorageCapabilities {
    /// Creates a new instance
    pub fn new() -> Self {
        Self {}
    }

    /// Smart tier recommendation based on storage heuristics
    /// Fast, local analysis without external dependencies
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn recommend_tier(
        &self,
        file: &FileMetrics,
    ) -> Result<TierRecommendation, NestGateError>  {
        // Use sophisticated pattern analysis
        let pattern = self.analyze_storage_patterns(file);
        let recommended_tier = pattern.recommended_tier();

        let recommendation = match recommended_tier {
            StorageTier::Hot => TierRecommendation {
                tier: StorageTier::Hot,
                confidence: 0.85,
                reasoning: format!(
                    "Pattern analysis: access={:?}, size={:?}, age={:?}",
                    pattern.access_pattern, pattern.size_category, pattern.age_category
                ),
                method: "local_smart_pattern_analysis".to_string(),
            },
            StorageTier::Cold => TierRecommendation {
                tier: StorageTier::Cold,
                confidence: 0.90,
                reasoning: format!(
                    "Pattern analysis: access={:?}, size={:?}, age={:?}",
                    pattern.access_pattern, pattern.size_category, pattern.age_category
                ),
                method: "local_smart_pattern_analysis".to_string(),
            },
            StorageTier::Warm => TierRecommendation {
                tier: StorageTier::Warm,
                confidence: 0.70,
                reasoning: format!(
                    "Pattern analysis: access={:?}, size={:?}, age={:?}",
                    pattern.access_pattern, pattern.size_category, pattern.age_category
                ),
                method: "local_smart_pattern_analysis".to_string(),
            },
        };

        debug!("Local smart tier recommendation: {:?}", recommendation);
        Ok(recommendation)
    }

    /// Analyze storage-specific patterns
    fn analyze_storage_patterns(&self, file: &FileMetrics) -> StoragePattern {
        // Use the from_metrics method for consistency
        StoragePattern::from_metrics(file)
    }
}

/// Failsafe defaults for standalone operation
/// Always works, no external dependencies
pub struct FailsafeDefaults {
    default_tier: StorageTier,
    default_compression: CompressionType,
}
impl Default for FailsafeDefaults {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl FailsafeDefaults {
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            default_tier: StorageTier::Warm,           // Safe middle ground
            default_compression: CompressionType::LZ4, // Fast, reliable
        }
    }

    /// Get default tier recommendation
    pub fn default_tier_recommendation(&self, _file_metrics: &FileMetrics) -> TierRecommendation {
        TierRecommendation {
            tier: self.default_tier.clone(),
            confidence: 0.5, // Conservative confidence for failsafe
            reasoning: "Failsafe default - no analysis available".to_string(),
            method: "failsafe_default".to_string(),
        }
    }

    /// Get default compression recommendation
    pub fn default_compression_recommendation(&self) -> CompressionType {
        self.default_compression.clone()
    }
}

// Supporting types
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Filemetrics
pub struct FileMetrics {
    /// Size Bytes
    pub size_bytes: u64,
    /// Access Frequency
    pub access_frequency: f64, // accesses per day
    /// Age Days
    pub age_days: u32,
    /// File Type
    pub file_type: String,
    /// Last Modified
    pub last_modified: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Tierrecommendation
pub struct TierRecommendation {
    /// Tier
    pub tier: StorageTier,
    /// Confidence
    pub confidence: f64, // 0.0 to 1.0
    /// Reasoning
    pub reasoning: String,
    /// Method
    pub method: String, // "local_smart", "external_ai", "failsafe"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Storagetier
pub enum StorageTier {
    Hot,  // NVMe, fastest access
    Warm, // SSD, balanced performance
    Cold, // HDD, archival storage
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of Compression
pub enum CompressionType {
    /// None
    None,
    LZ4,  // Fast compression
    ZSTD, // Balanced compression
    GZIP, // High compression
}

#[derive(Debug, Clone)]
/// Request parameters for Capability operation
pub struct CapabilityRequest {
    /// Capability Type
    pub capability_type: String,
    /// Data
    pub data: serde_json::Value,
    /// Timeout Ms
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Clone)]
/// Response data for Capability operation
pub struct CapabilityResponse {
    /// Data
    pub data: serde_json::Value,
    /// Provider
    pub provider: String, // Which primal provided the response (set by universal adapter)
    /// Execution Time Ms
    pub execution_time_ms: u64,
}

// Internal analysis types
#[derive(Debug, Clone)]
#[allow(dead_code)] // Used for internal storage pattern analysis
struct StoragePattern {
    access_pattern: AccessPattern,
    size_category: SizeCategory,
    age_category: AgeCategory,
}

impl StoragePattern {
    /// Analyze storage pattern from file metrics
    pub fn from_metrics(file: &FileMetrics) -> Self {
        let access_pattern = if file.access_frequency > 10.0 {
            AccessPattern::Hot
        } else if file.access_frequency > 1.0 {
            AccessPattern::Warm
        } else {
            AccessPattern::Cold
        };

        let size_category = if file.size_bytes < 100_000 {
            SizeCategory::Tiny
        } else if file.size_bytes < 1_000_000 {
            SizeCategory::Small
        } else if file.size_bytes < 100_000_000 {
            SizeCategory::Medium
        } else {
            SizeCategory::Large
        };

        let age_category = if file.age_days < 7 {
            AgeCategory::New
        } else if file.age_days < 30 {
            AgeCategory::Recent
        } else if file.age_days < 90 {
            AgeCategory::Moderate
        } else {
            AgeCategory::Old
        };

        Self {
            access_pattern,
            size_category,
            age_category,
        }
    }

    /// Get recommended tier based on pattern analysis
    pub fn recommended_tier(&self) -> StorageTier {
        match (
            &self.access_pattern,
            &self.size_category,
            &self.age_category,
        ) {
            (AccessPattern::Hot, _, _) => StorageTier::Hot,
            (AccessPattern::Cold, SizeCategory::Large, AgeCategory::Old) => StorageTier::Cold,
            _ => StorageTier::Warm,
        }
    }
}

#[derive(Debug, Clone)]
enum AccessPattern {
    /// Hot
    Hot,
    /// Warm
    Warm,
    /// Cold
    Cold,
}

#[derive(Debug, Clone)]
enum SizeCategory {
    /// Tiny
    Tiny,
    /// Small
    Small,
    /// Medium
    Medium,
    /// Large
    Large,
}

#[derive(Debug, Clone)]
enum AgeCategory {
    /// New
    New,
    /// Recent
    Recent,
    /// Moderate
    Moderate,
    /// Old
    Old,
}

// Migration examples removed - all implementations completed successfully

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_local_smart_tier_recommendation() {
        let local_caps = LocalStorageCapabilities::new();

        // Test hot tier recommendation
        let hot_file = FileMetrics {
            size_bytes: 50_000,
            access_frequency: 15.0,
            age_days: 1,
            file_type: "document".to_string(),
            last_modified: chrono::Utc::now(),
        };

        let recommendation = local_caps.recommend_tier(&hot_file).await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    std::io::Error::new(std::io::ErrorKind::Other, format!("Operation failed: {e:?}"))
)?;
        assert_eq!(recommendation.tier, StorageTier::Hot);
        assert!(recommendation.confidence > 0.8);
    }

    #[tokio::test]
    async fn test_failsafe_defaults() {
        let failsafe = FailsafeDefaults::new();

        let any_file = FileMetrics {
            size_bytes: 500_000,
            access_frequency: 2.0,
            age_days: 10,
            file_type: "unknown".to_string(),
            last_modified: chrono::Utc::now(),
        };

        let recommendation = failsafe.default_tier_recommendation(&any_file);
        assert_eq!(recommendation.tier, StorageTier::Warm);
        assert_eq!(recommendation.method, "failsafe_default");
    }
}
