/// NestGate API Configuration Management
/// Handles REST API configuration, endpoint management, and service exposure
/// **ECOSYSTEM UNIFICATION**: This module now uses the unified type system from nestgate-core
/// to eliminate API config fragmentation and ensure consistency.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// 🚀 ECOSYSTEM UNIFICATION: Import unified types
use nestgate_core::unified_types::{UnifiedConfig, UnifiedNetworkConfig, UnifiedSecurityConfig, UnifiedMonitoringConfig, UnifiedServiceConfig};

// **DEPRECATED IMPLEMENTATIONS REMOVED**
// All deprecated Default implementations for removed types have been eliminated
// Use unified types from nestgate_core::unified_types instead

// 🚀 MODERN UNIFIED CONFIGURATION: Pure unified types

/// API manager for handling REST API operations using unified configuration
pub struct ApiManager {
    config: UnifiedConfig,
    api_stats: ApiStatistics,
    }

#[derive(Debug, Default)]
pub struct ApiStatistics {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub rate_limited_requests: u64,
    pub average_response_time: Duration,
    pub peak_concurrent_requests: u32,
    pub current_concurrent_requests: u32,
    pub requests_by_endpoint: HashMap<String, u64>,
    pub requests_by_status: HashMap<u16, u64>,
    pub last_request_time: Option<std::time::SystemTime>,
    }

impl ApiManager {
    pub fn new(config: UnifiedConfig) -> Self {
        Self {
            config,
            api_stats: ApiStatistics::default(),
    }
    }

    pub fn get_config(&self) -> &UnifiedConfig {
        &self.config
    }

    pub fn get_api_statistics(&self) -> &ApiStatistics {
        &self.api_stats
    }

    /// Record an API request
    pub fn record_request(&mut self, endpoint: String, status_code: u16, response_time: Duration) {
        self.api_stats.total_requests += 1;

        if status_code >= 200 && status_code < 400 {
            self.api_stats.successful_requests += 1;
        } else {
            self.api_stats.failed_requests += 1;
    }

        // Update average response time (simple moving average)
        if self.api_stats.total_requests == 1 {
            self.api_stats.average_response_time = response_time;
        } else {
            let total_time = self.api_stats.average_response_time.as_millis() as u64
                * (self.api_stats.total_requests - 1) + response_time.as_millis() as u64;
            self.api_stats.average_response_time =
                Duration::from_millis(total_time / self.api_stats.total_requests);
    }

        // Update counters
        *self.api_stats.requests_by_endpoint.entry(endpoint).or_insert(0) += 1;
        *self.api_stats.requests_by_status.entry(status_code).or_insert(0) += 1;

        self.api_stats.last_request_time = Some(std::time::SystemTime::now());
    }

    /// Record rate limiting
    pub fn record_rate_limit(&mut self) {
        self.api_stats.rate_limited_requests += 1;
        self.record_request("rate_limited".to_string(), 429, Duration::ZERO);
    }

    /// Calculate success rate
    pub fn success_rate(&self) -> f64 {
        if self.api_stats.total_requests == 0 {
            0.0
        } else {
            (self.api_stats.successful_requests as f64 / self.api_stats.total_requests as f64) * 100.0
    }
    }

    /// Get most popular endpoint
    pub fn most_popular_endpoint(&self) -> Option<(String, u64)> {
        self.api_stats.requests_by_endpoint
            .iter()
            .max_by_key(|(_, count)| *count)
            .map(|(endpoint, count)| (endpoint.clone(), *count))
    }
    }