// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Zero-cost data types
//!
//! This module defines the core data structures used in zero-cost architecture,
//! designed for compile-time optimization and zero allocation patterns.

/// Zero-cost request structure
#[derive(Debug, Clone)]
/// Request parameters for `ZeroCost` operation
pub struct ZeroCostRequest {
    /// Unique identifier
    pub id: u64,
    /// Data
    pub data: Vec<u8>,
    /// Additional metadata key-value pairs
    pub metadata: ZeroCostMetadata,
}

/// Zero-cost response structure\
#[derive(Debug, Clone)]
/// Response data for `ZeroCost` operation
pub struct ZeroCostResponse {
    /// Unique identifier
    pub id: u64,
    /// Data
    pub data: Vec<u8>,
    /// Success
    pub success: bool,
}

/// Zero-cost metadata - no heap allocations
#[derive(Debug, Clone)]
/// Zerocostmetadata
pub struct ZeroCostMetadata {
    /// Timestamp
    pub timestamp: u64,
    /// Priority
    pub priority: RequestPriority,
    /// Source
    pub source: [u8; 32], // Fixed-size array to avoid allocations
}

/// Zero-cost performance metrics
#[derive(Debug, Clone)]
/// Zerocostmetrics
pub struct ZeroCostMetrics {
    /// Requests Processed
    pub requests_processed: u64,
    /// Average Latency Ns
    pub average_latency_ns: u64,
}

/// Zero-cost error enumeration
#[derive(Debug, Clone, PartialEq, Eq)]
/// Errors that can occur during `ZeroCost` operations
pub enum ZeroCostError {
    /// Cacheerror
    CacheError,
    /// Securityerror
    SecurityError,
    /// Storageerror
    StorageError,
    /// Invalidrequest
    InvalidRequest,
    /// Systemoverload
    SystemOverload,
}

/// Zero-cost performance metrics
#[derive(Debug, Clone, Default)]
/// Zerocostperformancemetrics
pub struct ZeroCostPerformanceMetrics {
    /// Throughput Ops Per Sec
    pub throughput_ops_per_sec: u64,
    /// Latency P95 Ns
    pub latency_p95_ns: u64,
    /// Memory Usage Bytes
    pub memory_usage_bytes: u64,
}

/// Zero-cost benchmark results
#[derive(Debug, Clone)]
/// Zerocostbenchmarkresults
pub struct ZeroCostBenchmarkResults {
    /// Traditional Latency Ns
    pub traditional_latency_ns: u64,
    /// Zero Cost Latency Ns
    pub zero_cost_latency_ns: u64,
    /// Improvement Percent
    pub improvement_percent: f64,
}

/// Request priority for metadata
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Requestpriority
pub enum RequestPriority {
    /// Low
    Low,
    /// Normal
    Normal,
    /// High
    High,
    /// Critical
    Critical,
}

impl Default for RequestPriority {
    /// Returns the default instance
    fn default() -> Self {
        Self::Normal
    }
}

impl std::fmt::Display for ZeroCostError {
    /// Fmt
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CacheError => write!(f, "Cache operation failed"),
            Self::SecurityError => write!(f, "Security validation failed"),
            Self::StorageError => write!(f, "Storage operation failed"),
            Self::InvalidRequest => write!(f, "Invalid request format"),
            Self::SystemOverload => write!(f, "System capacity exceeded"),
        }
    }
}

impl std::error::Error for ZeroCostError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_cost_types_creation() {
        let metadata = ZeroCostMetadata {
            timestamp: 1_234_567_890,
            priority: RequestPriority::High,
            source: [0u8; 32],
        };

        let request = ZeroCostRequest {
            id: 1,
            data: vec![1, 2, 3],
            metadata,
        };

        assert_eq!(request.id, 1);
        assert_eq!(request.data, vec![1, 2, 3]);
        assert_eq!(request.metadata.priority, RequestPriority::High);
    }

    #[test]
    fn test_error_display() {
        assert_eq!(
            ZeroCostError::CacheError.to_string(),
            "Cache operation failed"
        );
    }

    #[test]
    fn round5_zero_cost_error_display_security_storage_invalid_overload() {
        assert_eq!(
            ZeroCostError::SecurityError.to_string(),
            "Security validation failed"
        );
        assert_eq!(
            ZeroCostError::StorageError.to_string(),
            "Storage operation failed"
        );
        assert_eq!(
            ZeroCostError::InvalidRequest.to_string(),
            "Invalid request format"
        );
        assert_eq!(
            ZeroCostError::SystemOverload.to_string(),
            "System capacity exceeded"
        );
    }

    #[test]
    fn round5_request_priority_default_impl() {
        assert_eq!(RequestPriority::default(), RequestPriority::Normal);
    }

    #[test]
    fn test_metrics_default() {
        let metrics = ZeroCostPerformanceMetrics::default();
        assert_eq!(metrics.throughput_ops_per_sec, 0);
        assert_eq!(metrics.latency_p95_ns, 0);
        assert_eq!(metrics.memory_usage_bytes, 0);
    }
}
