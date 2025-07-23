//! Tier Prediction Module
//!
//! AI-powered tier prediction for optimal storage placement

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;
// Removed unused tracing import

use crate::Result;
use tracing::debug;
use tracing::info;
use tracing::warn;

/// Storage tier types for prediction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TierType {
    Hot,
    Warm,
    Cold,
}

impl std::fmt::Display for TierType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TierType::Hot => write!(f, "Hot"),
            TierType::Warm => write!(f, "Warm"),
            TierType::Cold => write!(f, "Cold"),
        }
    }
}

/// Confidence levels for predictions
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Confidence {
    Low,
    Medium,
    High,
    VeryHigh,
}

/// File analysis result for prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileAnalysis {
    pub file_path: String,
    pub size_bytes: u64,
    pub created_at: SystemTime,
    pub modified_at: SystemTime,
    pub accessed_at: SystemTime,
    pub file_type: String,
}

/// Access pattern data for prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPattern {
    pub accesses_last_24h: u32,
    pub accesses_last_week: u32,
    pub accesses_last_month: u32,
    pub total_accesses: u64,
    pub last_access: SystemTime,
}

/// Tier prediction result with confidence and alternatives
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierPrediction {
    pub recommended_tier: TierType,
    pub confidence: Confidence,
    pub reasoning: String,
    pub alternative_tiers: Vec<TierType>,
    pub prediction_score: f64,
}

/// Machine learning model for tier prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelType {
    /// Simple rule-based predictor
    RuleBased,
    /// Frequency-based predictor
    FrequencyBased,
    /// Machine learning predictor (future)
    MachineLearning,
}

/// Prediction metrics and statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionMetrics {
    pub total_predictions: u64,
    pub accuracy_rate: f64,
    pub average_confidence: f64,
    pub model_version: String,
    pub last_updated: SystemTime,
}

/// AI-powered tier predictor
#[derive(Debug)]
pub struct TierPredictor {
    model_type: ModelType,
    metrics: PredictionMetrics,
    rules: HashMap<String, TierType>,
    frequency_thresholds: FrequencyThresholds,
}

/// Frequency-based thresholds for tier prediction
#[derive(Debug, Clone)]
struct FrequencyThresholds {
    hot_tier_accesses_per_day: u32,
    warm_tier_accesses_per_week: u32,
    cold_tier_max_accesses_per_month: u32,
}

impl Default for FrequencyThresholds {
    fn default() -> Self {
        Self {
            hot_tier_accesses_per_day: 10,
            warm_tier_accesses_per_week: 5,
            cold_tier_max_accesses_per_month: 2,
        }
    }
}

impl TierPredictor {
    /// Create a new tier predictor
    pub fn new() -> Self {
        info!("🧠 Initializing TierPredictor");

        let mut rules = HashMap::new();
        // Default rules for file types
        rules.insert("*.log".to_string(), TierType::Cold);
        rules.insert("*.tmp".to_string(), TierType::Cold);
        rules.insert("*.cache".to_string(), TierType::Hot);
        rules.insert("*.db".to_string(), TierType::Hot);
        rules.insert("*.sql".to_string(), TierType::Warm);
        rules.insert("*.backup".to_string(), TierType::Cold);
        rules.insert("*.archive".to_string(), TierType::Cold);
        rules.insert("*.config".to_string(), TierType::Warm);

        Self {
            model_type: ModelType::FrequencyBased,
            metrics: PredictionMetrics {
                total_predictions: 0,
                accuracy_rate: 0.0,
                average_confidence: 0.0,
                model_version: "1.0.0".to_string(),
                last_updated: SystemTime::now(),
            },
            rules,
            frequency_thresholds: FrequencyThresholds::default(),
        }
    }

    /// Predict optimal tier for a file based on analysis and access patterns
    pub async fn predict_tier(
        &self,
        analysis: &FileAnalysis,
        patterns: &AccessPattern,
    ) -> Result<TierPrediction> {
        debug!("Predicting tier for file: {}", analysis.file_path);

        let prediction = match self.model_type {
            ModelType::RuleBased => self.predict_rule_based(analysis).await?,
            ModelType::FrequencyBased => self.predict_frequency_based(analysis, patterns).await?,
            ModelType::MachineLearning => self.predict_ml_based(analysis, patterns).await?,
        };

        info!(
            "Predicted {} tier for {} with {} confidence",
            prediction.recommended_tier,
            analysis.file_path,
            match prediction.confidence {
                Confidence::Low => "low",
                Confidence::Medium => "medium",
                Confidence::High => "high",
                Confidence::VeryHigh => "very high",
            }
        );

        Ok(prediction)
    }

    /// Rule-based prediction using file patterns
    async fn predict_rule_based(&self, analysis: &FileAnalysis) -> Result<TierPrediction> {
        let file_path = &analysis.file_path;
        let file_extension = std::path::Path::new(file_path)
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| format!("*.{ext}"))
            .unwrap_or_else(|| "*".to_string());

        let tier = self
            .rules
            .get(&file_extension)
            .or_else(|| self.rules.get("*"))
            .copied()
            .unwrap_or(TierType::Warm);

        let confidence = match tier {
            TierType::Hot => Confidence::High,
            TierType::Warm => Confidence::Medium,
            TierType::Cold => Confidence::High,
        };

        Ok(TierPrediction {
            recommended_tier: tier,
            confidence,
            reasoning: format!("Rule-based prediction for file type: {file_extension}"),
            alternative_tiers: vec![],
            prediction_score: self.confidence_to_score(&confidence),
        })
    }

    /// Frequency-based prediction using access patterns
    async fn predict_frequency_based(
        &self,
        analysis: &FileAnalysis,
        patterns: &AccessPattern,
    ) -> Result<TierPrediction> {
        let daily_accesses = patterns.accesses_last_24h;
        let weekly_accesses = patterns.accesses_last_week;
        let monthly_accesses = patterns.accesses_last_month;

        let (tier, confidence, reasoning) = if daily_accesses
            >= self.frequency_thresholds.hot_tier_accesses_per_day
        {
            (
                TierType::Hot,
                Confidence::VeryHigh,
                format!("High frequency access: {daily_accesses} accesses in last 24h"),
            )
        } else if weekly_accesses >= self.frequency_thresholds.warm_tier_accesses_per_week {
            (
                TierType::Warm,
                Confidence::High,
                format!("Moderate frequency access: {weekly_accesses} accesses in last week"),
            )
        } else if monthly_accesses <= self.frequency_thresholds.cold_tier_max_accesses_per_month {
            (
                TierType::Cold,
                Confidence::High,
                format!("Low frequency access: {monthly_accesses} accesses in last month"),
            )
        } else {
            (
                TierType::Warm,
                Confidence::Medium,
                "Default to warm tier for moderate usage".to_string(),
            )
        };

        // Consider file size in the prediction
        let adjusted_tier = if analysis.size_bytes > {
            use nestgate_core::config::StorageConstants;
            StorageConstants::from_environment()
                .file_sizes
                .archive_threshold
        } && tier == TierType::Hot
        {
            // Large files (>1GB) may be better in warm tier even with high access
            warn!(
                "Large file {} adjusted from Hot to Warm tier",
                analysis.file_path
            );
            TierType::Warm
        } else {
            tier
        };

        Ok(TierPrediction {
            recommended_tier: adjusted_tier,
            confidence,
            reasoning,
            alternative_tiers: self.get_alternative_tiers(adjusted_tier),
            prediction_score: self.confidence_to_score(&confidence),
        })
    }

    /// ML-based prediction (placeholder for future implementation)
    async fn predict_ml_based(
        &self,
        analysis: &FileAnalysis,
        patterns: &AccessPattern,
    ) -> Result<TierPrediction> {
        // For now, fall back to frequency-based prediction
        warn!("ML-based prediction not yet implemented, falling back to frequency-based");
        self.predict_frequency_based(analysis, patterns).await
    }

    /// Get alternative tier recommendations
    fn get_alternative_tiers(&self, primary_tier: TierType) -> Vec<TierType> {
        match primary_tier {
            TierType::Hot => vec![TierType::Warm],
            TierType::Warm => vec![TierType::Hot, TierType::Cold],
            TierType::Cold => vec![TierType::Warm],
        }
    }

    /// Convert confidence level to numerical score
    fn confidence_to_score(&self, confidence: &Confidence) -> f64 {
        match confidence {
            Confidence::Low => 0.25,
            Confidence::Medium => 0.50,
            Confidence::High => 0.75,
            Confidence::VeryHigh => 0.95,
        }
    }

    /// Update prediction metrics after a prediction
    pub async fn update_metrics(&mut self, was_accurate: bool) -> Result<()> {
        self.metrics.total_predictions += 1;

        // Update accuracy rate using exponential moving average
        let alpha = 0.1; // Learning rate
        if self.metrics.total_predictions == 1 {
            self.metrics.accuracy_rate = if was_accurate { 1.0 } else { 0.0 };
        } else {
            let new_accuracy = if was_accurate { 1.0 } else { 0.0 };
            self.metrics.accuracy_rate =
                (1.0 - alpha) * self.metrics.accuracy_rate + alpha * new_accuracy;
        }

        self.metrics.last_updated = SystemTime::now();

        debug!(
            "Updated prediction metrics: {} predictions, {:.2}% accuracy",
            self.metrics.total_predictions,
            self.metrics.accuracy_rate * 100.0
        );

        Ok(())
    }

    /// Get current prediction metrics
    pub fn get_metrics(&self) -> &PredictionMetrics {
        &self.metrics
    }

    /// Update frequency thresholds for tier prediction
    pub fn update_thresholds(
        &mut self,
        hot_daily: Option<u32>,
        warm_weekly: Option<u32>,
        cold_monthly: Option<u32>,
    ) {
        if let Some(threshold) = hot_daily {
            self.frequency_thresholds.hot_tier_accesses_per_day = threshold;
        }
        if let Some(threshold) = warm_weekly {
            self.frequency_thresholds.warm_tier_accesses_per_week = threshold;
        }
        if let Some(threshold) = cold_monthly {
            self.frequency_thresholds.cold_tier_max_accesses_per_month = threshold;
        }

        info!("Updated tier prediction thresholds");
    }

    /// Add or update a rule for file type prediction
    pub fn add_rule(&mut self, pattern: String, tier: TierType) {
        self.rules.insert(pattern.clone(), tier);
        debug!("Added prediction rule: {} -> {:?}", pattern, tier);
    }

    /// Remove a rule
    pub fn remove_rule(&mut self, pattern: &str) -> Option<TierType> {
        let removed = self.rules.remove(pattern);
        if removed.is_some() {
            debug!("Removed prediction rule: {}", pattern);
        }
        removed
    }

    /// Get all current rules
    pub fn get_rules(&self) -> &HashMap<String, TierType> {
        &self.rules
    }
}

impl Default for TierPredictor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tier_predictor_creation() {
        let predictor = TierPredictor::new();
        assert_eq!(predictor.metrics.total_predictions, 0);
        assert!(!predictor.rules.is_empty());
    }

    #[tokio::test]
    async fn test_rule_based_prediction() {
        let predictor = TierPredictor::new();

        let analysis = FileAnalysis {
            file_path: "test.log".to_string(),
            size_bytes: 1024,
            created_at: SystemTime::now(),
            modified_at: SystemTime::now(),
            accessed_at: SystemTime::now(),
            file_type: "log".to_string(),
        };

        let prediction = predictor.predict_rule_based(&analysis).await.unwrap();
        assert_eq!(prediction.recommended_tier, TierType::Cold);
    }

    #[tokio::test]
    async fn test_frequency_based_prediction() {
        let predictor = TierPredictor::new();

        let analysis = FileAnalysis {
            file_path: "test.db".to_string(),
            size_bytes: 1024,
            created_at: SystemTime::now(),
            modified_at: SystemTime::now(),
            accessed_at: SystemTime::now(),
            file_type: "database".to_string(),
        };

        // High frequency access pattern
        let patterns = AccessPattern {
            accesses_last_24h: 15, // Above hot tier threshold
            accesses_last_week: 50,
            accesses_last_month: 200,
            total_accesses: 500,
            last_access: SystemTime::now(),
        };

        let prediction = predictor
            .predict_frequency_based(&analysis, &patterns)
            .await
            .unwrap();
        assert_eq!(prediction.recommended_tier, TierType::Hot);
    }

    #[tokio::test]
    async fn test_metrics_update() {
        let mut predictor = TierPredictor::new();

        predictor.update_metrics(true).await.unwrap();
        assert_eq!(predictor.metrics.total_predictions, 1);
        assert_eq!(predictor.metrics.accuracy_rate, 1.0);

        predictor.update_metrics(false).await.unwrap();
        assert_eq!(predictor.metrics.total_predictions, 2);
        assert!(predictor.metrics.accuracy_rate < 1.0);
    }

    #[tokio::test]
    async fn test_rule_management() {
        let mut predictor = TierPredictor::new();

        predictor.add_rule("*.test".to_string(), TierType::Hot);
        assert_eq!(predictor.rules.get("*.test"), Some(&TierType::Hot));

        let removed = predictor.remove_rule("*.test");
        assert_eq!(removed, Some(TierType::Hot));
        assert_eq!(predictor.rules.get("*.test"), None);
    }
}
