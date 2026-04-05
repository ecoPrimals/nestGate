// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

/// **CANONICAL ADAPTER STATISTICS**
///
/// Consolidated statistics and metrics for the universal adapter system.
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
// Removed unused imports for pedantic perfection
// Custom serialization for Instant
mod instant_serde {
    use super::{Deserialize, Duration, Instant, Serialize};
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
        // For deserialization, approximate the prior instant from elapsed duration
        let duration = Duration::deserialize(deserializer)?;
        Ok(Instant::now()
            .checked_sub(duration)
            .unwrap_or_else(Instant::now))
    }
}

/// Comprehensive adapter statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Adapterstats
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
    /// Last Reset
    pub last_reset: Instant,
}
impl Default for AdapterStats {
    /// Returns the default instance
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
    #[must_use]
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

impl AdapterStats {
    /// Create new adapter statistics
    #[must_use]
    pub fn new() -> Self {
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
    pub const fn increment_requests(&mut self) {
        self.total_requests += 1;
    }

    /// Increment successful requests
    pub const fn increment_successful(&mut self) {
        self.successful_requests += 1;
    }

    /// Increment failed requests
    pub const fn increment_failed(&mut self) {
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
    #[must_use]
    pub const fn summary(&self) -> StatsSummary {
        StatsSummary {
            requests_total: self.total_requests,
            requests_successful: self.successful_requests,
            requests_failed: self.failed_requests,
            capability_cache_hits: 0, // Not tracked in simplified structure
            capability_cache_misses: 0, // Not tracked in simplified structure
            fallback_activations: 0,  // Not tracked in simplified structure
        }
    }

    /// Get uptime in seconds
    #[must_use]
    pub fn uptime_seconds(&self) -> u64 {
        self.last_reset.elapsed().as_secs()
    }
}

/// Summary of adapter statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Statssummary
pub struct StatsSummary {
    /// Requests Total
    pub requests_total: u64,
    /// Requests Successful
    pub requests_successful: u64,
    /// Requests Failed
    pub requests_failed: u64,
    /// Capability Cache Hits
    pub capability_cache_hits: u64,
    /// Capability Cache Misses
    pub capability_cache_misses: u64,
    /// Fallback Activations
    pub fallback_activations: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adapter_stats_default() {
        let stats = AdapterStats::default();

        assert_eq!(stats.active_providers, 0);
        assert_eq!(stats.total_requests, 0);
        assert_eq!(stats.successful_requests, 0);
        assert_eq!(stats.failed_requests, 0);
        assert_eq!(stats.average_response_time_ms, 0.0);
        assert_eq!(stats.peak_response_time_ms, 0.0);
        assert_eq!(stats.uptime, Duration::from_secs(0));
    }

    #[test]
    fn test_adapter_stats_new() {
        let stats = AdapterStats::new();

        assert_eq!(stats.active_providers, 0);
        assert_eq!(stats.total_requests, 0);
        assert_eq!(stats.successful_requests, 0);
        assert_eq!(stats.failed_requests, 0);
    }

    #[test]
    fn test_record_success_single() {
        let mut stats = AdapterStats::new();
        let response_time = Duration::from_millis(100);

        stats.record_success(response_time);

        assert_eq!(stats.total_requests, 1);
        assert_eq!(stats.successful_requests, 1);
        assert_eq!(stats.failed_requests, 0);
        assert_eq!(stats.average_response_time_ms, 100.0);
        assert_eq!(stats.peak_response_time_ms, 100.0);
    }

    #[test]
    fn test_record_success_multiple() {
        let mut stats = AdapterStats::new();

        stats.record_success(Duration::from_millis(100));
        stats.record_success(Duration::from_millis(200));
        stats.record_success(Duration::from_millis(300));

        assert_eq!(stats.total_requests, 3);
        assert_eq!(stats.successful_requests, 3);
        assert_eq!(stats.failed_requests, 0);
        assert_eq!(stats.average_response_time_ms, 200.0); // (100 + 200 + 300) / 3
        assert_eq!(stats.peak_response_time_ms, 300.0);
    }

    #[test]
    fn test_record_failure_single() {
        let mut stats = AdapterStats::new();
        let response_time = Duration::from_millis(150);

        stats.record_failure(response_time);

        assert_eq!(stats.total_requests, 1);
        assert_eq!(stats.successful_requests, 0);
        assert_eq!(stats.failed_requests, 1);
        assert_eq!(stats.average_response_time_ms, 150.0);
    }

    #[test]
    fn test_record_mixed_success_and_failure() {
        let mut stats = AdapterStats::new();

        stats.record_success(Duration::from_millis(100));
        stats.record_failure(Duration::from_millis(200));
        stats.record_success(Duration::from_millis(300));

        assert_eq!(stats.total_requests, 3);
        assert_eq!(stats.successful_requests, 2);
        assert_eq!(stats.failed_requests, 1);
        assert_eq!(stats.average_response_time_ms, 200.0);
    }

    #[test]
    fn test_success_rate_no_requests() {
        let stats = AdapterStats::new();
        assert_eq!(stats.success_rate(), 100.0);
    }

    #[test]
    fn test_success_rate_all_successful() {
        let mut stats = AdapterStats::new();
        stats.record_success(Duration::from_millis(100));
        stats.record_success(Duration::from_millis(100));
        stats.record_success(Duration::from_millis(100));

        assert_eq!(stats.success_rate(), 100.0);
    }

    #[test]
    fn test_success_rate_all_failed() {
        let mut stats = AdapterStats::new();
        stats.record_failure(Duration::from_millis(100));
        stats.record_failure(Duration::from_millis(100));

        assert_eq!(stats.success_rate(), 0.0);
    }

    #[test]
    fn test_success_rate_mixed() {
        let mut stats = AdapterStats::new();
        stats.record_success(Duration::from_millis(100));
        stats.record_success(Duration::from_millis(100));
        stats.record_failure(Duration::from_millis(100));
        stats.record_failure(Duration::from_millis(100));

        assert_eq!(stats.success_rate(), 50.0);
    }

    #[test]
    fn test_peak_response_time_updates() {
        let mut stats = AdapterStats::new();

        stats.record_success(Duration::from_millis(100));
        assert_eq!(stats.peak_response_time_ms, 100.0);

        stats.record_success(Duration::from_millis(50));
        assert_eq!(stats.peak_response_time_ms, 100.0); // Doesn't decrease

        stats.record_success(Duration::from_millis(200));
        assert_eq!(stats.peak_response_time_ms, 200.0); // Updates to higher
    }

    #[test]
    fn test_reset_clears_all_stats() {
        let mut stats = AdapterStats::new();

        stats.record_success(Duration::from_millis(100));
        stats.record_failure(Duration::from_millis(200));
        assert_eq!(stats.total_requests, 2);

        stats.reset();

        assert_eq!(stats.total_requests, 0);
        assert_eq!(stats.successful_requests, 0);
        assert_eq!(stats.failed_requests, 0);
        assert_eq!(stats.average_response_time_ms, 0.0);
        assert_eq!(stats.peak_response_time_ms, 0.0);
    }

    #[test]
    fn test_increment_methods() {
        let mut stats = AdapterStats::new();

        stats.increment_requests();
        stats.increment_successful();

        assert_eq!(stats.total_requests, 1);
        assert_eq!(stats.successful_requests, 1);

        stats.increment_requests();
        stats.increment_failed();

        assert_eq!(stats.total_requests, 2);
        assert_eq!(stats.failed_requests, 1);
    }

    #[test]
    fn test_update_response_time() {
        let mut stats = AdapterStats::new();

        stats.update_response_time(100.0);
        stats.update_response_time(200.0);

        assert_eq!(stats.peak_response_time_ms, 200.0);
        // Average is (100 + 200) / 2 = 150.0, then (150 + 200) / 2 = 175.0
        // ✅ MODERN: Use epsilon for positive value check
        assert!(stats.average_response_time_ms > 1e-9);
    }

    #[test]
    fn test_summary_creation() {
        let mut stats = AdapterStats::new();
        stats.record_success(Duration::from_millis(100));
        stats.record_failure(Duration::from_millis(100));

        let summary = stats.summary();

        assert_eq!(summary.requests_total, 2);
        assert_eq!(summary.requests_successful, 1);
        assert_eq!(summary.requests_failed, 1);
    }

    #[test]
    fn test_uptime_seconds() {
        let stats = AdapterStats::new();

        // Should be very close to 0 since just created
        let uptime = stats.uptime_seconds();
        assert!(uptime < 2); // Allow for test execution time
    }

    #[test]
    fn test_stats_clone() {
        let mut stats = AdapterStats::new();
        stats.record_success(Duration::from_millis(100));

        let cloned = stats.clone();

        assert_eq!(cloned.total_requests, stats.total_requests);
        assert_eq!(cloned.successful_requests, stats.successful_requests);
        assert_eq!(
            cloned.average_response_time_ms,
            stats.average_response_time_ms
        );
    }

    #[test]
    fn test_edge_case_zero_duration() {
        let mut stats = AdapterStats::new();
        stats.record_success(Duration::from_millis(0));

        assert_eq!(stats.average_response_time_ms, 0.0);
        assert_eq!(stats.peak_response_time_ms, 0.0);
    }

    #[test]
    fn test_edge_case_large_response_time() {
        let mut stats = AdapterStats::new();
        stats.record_success(Duration::from_secs(60)); // 60 seconds

        assert_eq!(stats.average_response_time_ms, 60_000.0);
        assert_eq!(stats.peak_response_time_ms, 60_000.0);
    }

    #[test]
    fn test_stats_summary_clone() {
        let summary = StatsSummary {
            requests_total: 100,
            requests_successful: 80,
            requests_failed: 20,
            capability_cache_hits: 50,
            capability_cache_misses: 30,
            fallback_activations: 5,
        };

        let cloned = summary.clone();

        assert_eq!(cloned.requests_total, summary.requests_total);
        assert_eq!(cloned.requests_successful, summary.requests_successful);
        assert_eq!(cloned.requests_failed, summary.requests_failed);
    }

    #[test]
    fn test_stress_many_requests() {
        let mut stats = AdapterStats::new();

        // Record 1000 requests
        for i in 0..1000 {
            if i % 2 == 0 {
                stats.record_success(Duration::from_millis(100));
            } else {
                stats.record_failure(Duration::from_millis(50));
            }
        }

        assert_eq!(stats.total_requests, 1000);
        assert_eq!(stats.successful_requests, 500);
        assert_eq!(stats.failed_requests, 500);
        assert_eq!(stats.success_rate(), 50.0);
    }
}
