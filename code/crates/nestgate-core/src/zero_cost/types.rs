//! Zero-cost data types
//!
//! This module defines the core data structures used in zero-cost architecture,
//! designed for compile-time optimization and zero allocation patterns.

/// Zero-cost request structure
#[derive(Debug, Clone)]
pub struct ZeroCostRequest {
    pub id: u64,
    pub data: Vec<u8>,
    pub metadata: ZeroCostMetadata,
}

/// Zero-cost response structure  
#[derive(Debug, Clone)]
pub struct ZeroCostResponse {
    pub id: u64,
    pub data: Vec<u8>,
    pub success: bool,
}

/// Zero-cost metadata - no heap allocations
#[derive(Debug, Clone)]
pub struct ZeroCostMetadata {
    pub timestamp: u64,
    pub priority: RequestPriority,
    pub source: [u8; 32], // Fixed-size array to avoid allocations
}

/// Zero-cost performance metrics
#[derive(Debug, Clone)]
pub struct ZeroCostMetrics {
    pub requests_processed: u64,
    pub average_latency_ns: u64,
}

/// Zero-cost error enumeration
#[derive(Debug, Clone, PartialEq)]
pub enum ZeroCostError {
    CacheError,
    SecurityError,
    StorageError,
    InvalidRequest,
    SystemOverload,
}

/// Zero-cost performance metrics
#[derive(Debug, Clone, Default)]
pub struct ZeroCostPerformanceMetrics {
    pub throughput_ops_per_sec: u64,
    pub latency_p95_ns: u64,
    pub memory_usage_bytes: u64,
}

/// Zero-cost benchmark results
#[derive(Debug, Clone)]
pub struct ZeroCostBenchmarkResults {
    pub traditional_latency_ns: u64,
    pub zero_cost_latency_ns: u64,
    pub improvement_percent: f64,
}

/// Request priority for metadata
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestPriority {
    Low,
    Normal,
    High,
    Critical,
}

impl Default for RequestPriority {
    fn default() -> Self {
        Self::Normal
    }
}

impl std::fmt::Display for ZeroCostError {
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
            timestamp: 1234567890,
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
    fn test_metrics_default() {
        let metrics = ZeroCostPerformanceMetrics::default();
        assert_eq!(metrics.throughput_ops_per_sec, 0);
        assert_eq!(metrics.latency_p95_ns, 0);
        assert_eq!(metrics.memory_usage_bytes, 0);
    }
}
