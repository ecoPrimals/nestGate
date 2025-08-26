//
// **CANONICAL MODERNIZATION COMPLETE** - Automated optimization and management system
// with zero-cost abstractions and compile-time optimization.

pub mod analysis;
pub mod manager;
pub mod types;

// Re-export core types with canonical imports
pub use nestgate_core::error::{NestGateError, Result};
pub use nestgate_core::unified_enums::StorageTier;

// **CANONICAL MODERNIZATION**: Use direct config import instead of aliasing
pub use nestgate_core::config::NestGateCanonicalUnifiedConfig;

// Type alias for backward compatibility
pub type NestGateFinalConfig = nestgate_core::config::NestGateCanonicalUnifiedConfig;

// **AUTOMATION CORE TYPES** - Use correct module paths
pub use crate::types::prediction::FileAnalysis;
pub use manager::IntelligentDatasetManager as AutomationManager;
pub use types::{optimization::OptimizationResult, prediction::TierPrediction as PredictionModel};

// Add missing types for compatibility
pub struct OptimizationRecommendation {
    pub description: String,
    pub priority: u8,
    pub estimated_savings: Option<u64>,
}

pub struct OptimizationMetrics {
    pub files_processed: usize,
    pub bytes_saved: u64,
    pub time_taken: std::time::Duration,
}

pub enum OptimizationType {
    Compression,
    Deduplication,
    TierMigration,
    Cleanup,
}

// Add StoragePrediction type
pub struct StoragePrediction {
    pub recommended_tier: StorageTier,
    pub confidence: f32,
    pub reasoning: String,
}

// **CANONICAL AUTOMATION PROVIDER**
/// Zero-cost automation provider with compile-time optimization
pub struct CanonicalAutomationProvider {
    config: NestGateCanonicalUnifiedConfig,
}

impl CanonicalAutomationProvider {
    pub fn new(config: NestGateCanonicalUnifiedConfig) -> Self {
        Self { config }
    }
}
