/// **CANONICAL ADAPTER STATISTICS**
/// 
/// Consolidated statistics and metrics for the universal adapter system.
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
// Removed unused imports for pedantic perfection
// Custom serialization for Instant
mod instant_serde {
    use super::*;
    use serde::{Deserializer, Serializer};

    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
        pub fn serialize<S>(instant: &Instant, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
     {
        // Convert to duration since a reference point and serialize that
        let duration_since_start = instant.elapsed();
        duration_since_start.serialize(serializer)
    }

    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
        pub fn deserialize<'de, D>(deserializer: D) -> Result<Instant, D::Error>
    where
        D: Deserializer<'de>,
     {
        // For deserialization, just use current time minus the duration
        let duration = Duration::deserialize(deserializer)?;
        Ok(Instant::now() - duration)
    }
}

/// Comprehensive adapter statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterStats {
    /// Number of active providers
    pub active_providers: usize,
    /// Total requests processed
    pub total_requests: u64,
    /// Successful requests
    pub successful_requests: u64,
    /// Failed requests
    pub failed_requests: u64,
    /// Average response time in milliseconds
    pub average_response_time_ms: f64,
    /// Peak response time in milliseconds
    pub peak_response_time_ms: f64,
    /// Total uptime
    pub uptime: Duration,
    /// Last reset timestamp
    #[serde(with = "instant_serde")]
    pub last_reset: Instant,
}
impl Default for AdapterStats {
    fn default() -> Self {
        Self {
            active_providers: 0,
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            average_response_time_ms: 0.0,
            peak_response_time_ms: 0.0,
            uptime: Duration::from_secs(0),
            last_reset: Instant::now(),
        }
    }
}

impl AdapterStats {
    /// Record a successful request
    pub fn record_success(&mut self, response_time: Duration) {
        self.total_requests += 1;
        self.successful_requests += 1;
        
        let response_ms = response_time.as_millis() as f64;
        
        // Update average response time
        if self.total_requests == 1 {
            self.average_response_time_ms = response_ms;
        } else {
            self.average_response_time_ms = 
                (self.average_response_time_ms * (self.total_requests - 1) as f64 + response_ms) 
                / self.f64::from(total_requests);
        }
        
        // Update peak response time
        if response_ms > self.peak_response_time_ms {
            self.peak_response_time_ms = response_ms;
        }
    }
    
    /// Record a failed request
    pub fn record_failure(&mut self, response_time: Duration) {
        self.total_requests += 1;
        self.failed_requests += 1;
        
        let response_ms = response_time.as_millis() as f64;
        
        // Update average response time (include failures)
        if self.total_requests == 1 {
            self.average_response_time_ms = response_ms;
        } else {
            self.average_response_time_ms = 
                (self.average_response_time_ms * (self.total_requests - 1) as f64 + response_ms) 
                / self.f64::from(total_requests);
        }
    }
    
    /// Calculate success rate as percentage
    pub const fn success_rate(&self) -> f64 {
        if self.total_requests == 0 {
            100.0
        } else {
            (self.f64::from(successful_requests) / self.f64::from(total_requests)) * 100.0
        }
    }
    
    /// Reset all statistics
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

impl AdapterStats {
    /// Create new adapter statistics
    pub const fn new() -> Self {
        Self {
            active_providers: 0,
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            average_response_time_ms: 0.0,
            peak_response_time_ms: 0.0,
            uptime: Duration::from_secs(0),
            last_reset: Instant::now(),
        }
    }
    
    /// Increment total requests (simplified for compatibility)
    pub fn increment_requests(&mut self) {
        self.total_requests += 1;
    }
    
    /// Increment successful requests
    pub fn increment_successful(&mut self) {
        self.successful_requests += 1;
    }
    
    /// Increment failed requests
    pub fn increment_failed(&mut self) {
        self.failed_requests += 1;
    }
    
    /// Update response time
    pub fn update_response_time(&mut self, response_time_ms: f64) {
        if response_time_ms > self.peak_response_time_ms {
            self.peak_response_time_ms = response_time_ms;
        }
        // Simple moving average (could be improved)
        self.average_response_time_ms = (self.average_response_time_ms + response_time_ms) / 2.0;
    }
    
    /// Get current statistics as a summary
    pub const fn summary(&self) -> StatsSummary {
        StatsSummary {
            requests_total: self.total_requests,
            requests_successful: self.successful_requests,
            requests_failed: self.failed_requests,
            capability_cache_hits: 0, // Not tracked in simplified structure
            capability_cache_misses: 0, // Not tracked in simplified structure
            fallback_activations: 0, // Not tracked in simplified structure
        }
    }
    
    /// Get uptime in seconds
    pub const fn uptime_seconds(&self) -> u64 {
        self.last_reset.elapsed().as_secs()
    }
}



/// Summary of adapter statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsSummary {
    pub requests_total: u64,
    pub requests_successful: u64,
    pub requests_failed: u64,
    pub capability_cache_hits: u64,
    pub capability_cache_misses: u64,
    pub fallback_activations: u64,
}
