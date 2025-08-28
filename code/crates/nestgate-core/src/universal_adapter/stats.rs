/// **CANONICAL ADAPTER STATISTICS**
/// 
/// Consolidated statistics and metrics for the universal adapter system.
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

// Custom serialization for Instant
mod instant_serde {
    use super::*;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(instant: &Instant, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Convert to duration since a reference point and serialize that
        let duration_since_start = instant.elapsed();
        duration_since_start.serialize(serializer)
    }

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
                / self.total_requests as f64;
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
                / self.total_requests as f64;
        }
    }
    
    /// Calculate success rate as percentage
    pub fn success_rate(&self) -> f64 {
        if self.total_requests == 0 {
            100.0
        } else {
            (self.successful_requests as f64 / self.total_requests as f64) * 100.0
        }
    }
    
    /// Reset all statistics
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
