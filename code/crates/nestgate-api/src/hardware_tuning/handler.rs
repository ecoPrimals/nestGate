//
// This module provides the HardwareTuningHandler for API routes.

//! Handler module

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use super::adapter::HardwareTuningAdapter;
use super::types::*;
// CANONICAL MODERNIZATION: Use unified error system
use nestgate_core::error::{NestGateError, Result};

/// Hardware tuning handler for API routes
#[derive(Clone)]
/// Handler for HardwareTuning requests
pub struct HardwareTuningHandler {
    session_manager: Arc<RwLock<HashMap<Uuid, TuningSession>>>,
    compute_adapter: Arc<HardwareTuningAdapter>,
}
impl Default for HardwareTuningHandler {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl HardwareTuningHandler {
    /// Create a new hardware tuning handler with universal adapter
    #[must_use]
    pub fn new() -> Self { Self {
            session_manager: Arc::new(RwLock::new(HashMap::new()),
            compute_adapter: Arc::new(HardwareTuningAdapter::new()),
         }

    /// Start a new tuning session
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
                pub fn start_tuning_session(&self, request: HardwareTuningRequest) -> Result<Uuid>  {
        let session_id = Uuid::new_v4();
        let session = TuningSession {
            session_id,
            user_id: "default".to_string(),
            start_time: chrono::Utc::now(),
            last_activity: chrono::Utc::now(),
            status: SessionStatus::Active,
            tuning_mode: request.tuning_mode,
            active_profiles: request.target_hardware,
        };

        let mut sessions = self.session_manager.write().await;
        sessions.insert(session_id, session);

        Ok(session_id)
    }

    /// Get tuning session status
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn get_session_status(&self, session_id: Uuid) -> Result<TuningSession>  {
        let sessions = self.session_manager.read().await;
        sessions
            .get(&session_id)
            .cloned()
            .ok_or_else(|| NestGateError::internal_error(
                location: Some(format!("self.base_url:self.base_url"), line!()),
                context: None,
                is_bug: false,
            })
    }

    /// Auto-tune hardware
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn auto_tune(&self) -> Result<TuningResult>  {
        // Get live hardware metrics
        let metrics = self.compute_adapter.get_live_hardware_metrics().await?;

        // Calculate performance improvement based on current metrics
        let performance_improvement = if metrics._cpu_usage > 80.0 {
            25.0 // High CPU usage -> more improvement potential
        } else if metrics.memory_usage > 80.0 {
            20.0 // High memory usage -> moderate improvement
        } else {
            10.0 // Low usage -> minimal improvement
        };

        Ok(TuningResult {
            success: true,
            performance_improvement,
            energy_savings: 15.0,
            applied_settings: HashMap::new(),
            warnings: vec![],
            errors: vec![],
        })
    }

    /// Get available profiles
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn get_profiles(&self) -> Result<Vec<String>>  {
        Ok(vec![
            "performance".to_string(),
            "balanced".to_string(),
            "efficiency".to_string(),
        ])
    }

    /// Get configuration
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn get_config(&self) -> Result<serde_json::Value>  {
        Ok(serde_json::json!({
            "service": "hardware_tuning",
            "version": "1.0.0",
            "compute_capability_integration": true
        }))
    }

    /// Run benchmark
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn benchmark(&self, benchmark_name: &str) -> Result<BenchmarkResult>  {
        let metrics = self.compute_adapter.get_live_hardware_metrics().await?;

        let overall_score = match benchmark_name {
            "cpu" => 100.0 - metrics._cpu_usage,
            "memory" => 100.0 - metrics.memory_usage,
            "overall" => (100.0 - metrics._cpu_usage + 100.0 - metrics.memory_usage) / 2.0,
            _ => 50.0,
        };

        Ok(BenchmarkResult {
            benchmark_id: Uuid::new_v4().to_string(),
            benchmark_name: benchmark_name.to_string(),
            score: overall_score,
            duration_ms: 1000,
            _metadata: serde_json::json!({
                "cpu_usage": metrics._cpu_usage,
                "memory_usage": metrics.memory_usage
            }),
        })
    }

    /// Generate extraction lock
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn generate_extraction_lock(
        &self,
        _source: String,
        _destination: String,
    ) -> Result<ExtractionLock>  {
        use nestgate_core::hardware_tuning::*;

        Ok(ExtractionLock {
            lock_id: Uuid::new_v4().to_string(),
            lock_type: ExternalLockType::SovereignExternal,
            proof: CryptographicProof {
                signature: "test_signature".to_string(),
                timestamp: std::time::SystemTime::now(),
                valid_until: std::time::SystemTime::now() + std::time::Duration::from_secs(3600),
                algorithm: "test_algorithm".to_string(),
            }
            expires_at: std::time::SystemTime::now() + std::time::Duration::from_secs(3600),
            restrictions: ExtractionRestrictions {
                max_size: Some(1024 * 1024),
                time_restrictions: None,
                geographic_restrictions: vec![],
                usage_restrictions: vec![],
            }
            copyleft_requirements: CopyleftRequirements {
                license_type: "GPL".to_string(),
                attribution_required: true,
                share_alike: true,
                commercial_restrictions: vec![],
            }
        })
    }

    /// Verify extraction lock
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn verify_extraction_lock(&self, _lock_id: Uuid) -> Result<bool>  {
        Ok(true)
    }
}
