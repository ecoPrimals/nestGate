// NestGate Storage Metrics - Performance and Strategy Tracking
//
// Collects metrics for:
// - Compression ratios achieved
// - Entropy distributions
// - Strategy usage counts
// - Performance timings
// - Storage efficiency

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use anyhow::Result;

use super::analysis::DataAnalysis;
use super::pipeline::Pipeline;

/// Metrics collector for storage operations
#[derive(Clone)]
pub struct MetricsCollector {
    inner: Arc<Mutex<MetricsInner>>,
}

struct MetricsInner {
    // Cumulative metrics
    total_bytes_stored: u64,
    total_bytes_saved: u64,  // Via compression/dedup
    total_operations: u64,
    
    // Strategy counters
    strategy_counts: HashMap<String, u64>,
    
    // Compression ratios (histogram buckets)
    compression_ratios: Vec<f64>,
    
    // Entropy distribution
    entropy_samples: Vec<f64>,
    
    // Performance timings (milliseconds)
    operation_times: Vec<f64>,
}

impl MetricsCollector {
    /// Create new metrics collector
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(MetricsInner {
                total_bytes_stored: 0,
                total_bytes_saved: 0,
                total_operations: 0,
                strategy_counts: HashMap::new(),
                compression_ratios: Vec::new(),
                entropy_samples: Vec::new(),
                operation_times: Vec::new(),
            })),
        }
    }
    
    /// Record a storage operation
    pub async fn record(
        &self,
        analysis: &DataAnalysis,
        pipeline: &Pipeline,
        result: &StorageResult,
    ) -> Result<()> {
        let mut inner = self.inner.lock().unwrap();
        
        // Update cumulative metrics
        inner.total_bytes_stored += result.original_size as u64;
        inner.total_operations += 1;
        
        // Calculate savings
        if result.stored_size < result.original_size {
            let saved = result.original_size - result.stored_size;
            inner.total_bytes_saved += saved as u64;
        }
        
        // Record strategy
        let strategy_name = pipeline.strategy_name();
        *inner.strategy_counts.entry(strategy_name).or_insert(0) += 1;
        
        // Record compression ratio
        if let Some(ratio) = result.compression_ratio {
            inner.compression_ratios.push(ratio);
            
            // Keep last 10000 samples (rolling window)
            if inner.compression_ratios.len() > 10000 {
                inner.compression_ratios.remove(0);
            }
        }
        
        // Record entropy
        inner.entropy_samples.push(analysis.entropy);
        if inner.entropy_samples.len() > 10000 {
            inner.entropy_samples.remove(0);
        }
        
        // Record timing
        if let Some(duration_ms) = result.duration_ms {
            inner.operation_times.push(duration_ms);
            if inner.operation_times.len() > 10000 {
                inner.operation_times.remove(0);
            }
        }
        
        Ok(())
    }
    
    /// Get current metrics snapshot
    pub fn snapshot(&self) -> MetricsSnapshot {
        let inner = self.inner.lock().unwrap();
        
        // Calculate compression ratio stats
        let (avg_ratio, p50_ratio, p95_ratio, p99_ratio) = if !inner.compression_ratios.is_empty() {
            let mut sorted = inner.compression_ratios.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            
            let avg = sorted.iter().sum::<f64>() / sorted.len() as f64;
            let p50 = sorted[sorted.len() / 2];
            let p95 = sorted[sorted.len() * 95 / 100];
            let p99 = sorted[sorted.len() * 99 / 100];
            
            (avg, p50, p95, p99)
        } else {
            (0.0, 0.0, 0.0, 0.0)
        };
        
        // Calculate entropy stats
        let (avg_entropy, p50_entropy) = if !inner.entropy_samples.is_empty() {
            let mut sorted = inner.entropy_samples.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            
            let avg = sorted.iter().sum::<f64>() / sorted.len() as f64;
            let p50 = sorted[sorted.len() / 2];
            
            (avg, p50)
        } else {
            (0.0, 0.0)
        };
        
        // Calculate timing stats
        let (avg_time, p50_time, p95_time) = if !inner.operation_times.is_empty() {
            let mut sorted = inner.operation_times.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            
            let avg = sorted.iter().sum::<f64>() / sorted.len() as f64;
            let p50 = sorted[sorted.len() / 2];
            let p95 = sorted[sorted.len() * 95 / 100];
            
            (avg, p50, p95)
        } else {
            (0.0, 0.0, 0.0)
        };
        
        MetricsSnapshot {
            total_bytes_stored: inner.total_bytes_stored,
            total_bytes_saved: inner.total_bytes_saved,
            total_operations: inner.total_operations,
            savings_percent: if inner.total_bytes_stored > 0 {
                (inner.total_bytes_saved as f64 / inner.total_bytes_stored as f64) * 100.0
            } else {
                0.0
            },
            strategy_counts: inner.strategy_counts.clone(),
            compression_ratio_avg: avg_ratio,
            compression_ratio_p50: p50_ratio,
            compression_ratio_p95: p95_ratio,
            compression_ratio_p99: p99_ratio,
            entropy_avg: avg_entropy,
            entropy_p50: p50_entropy,
            operation_time_avg_ms: avg_time,
            operation_time_p50_ms: p50_time,
            operation_time_p95_ms: p95_time,
        }
    }
    
    /// Reset all metrics
    pub fn reset(&self) {
        let mut inner = self.inner.lock().unwrap();
        inner.total_bytes_stored = 0;
        inner.total_bytes_saved = 0;
        inner.total_operations = 0;
        inner.strategy_counts.clear();
        inner.compression_ratios.clear();
        inner.entropy_samples.clear();
        inner.operation_times.clear();
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Snapshot of current metrics
#[derive(Debug, Clone)]
pub struct MetricsSnapshot {
    pub total_bytes_stored: u64,
    pub total_bytes_saved: u64,
    pub total_operations: u64,
    pub savings_percent: f64,
    pub strategy_counts: HashMap<String, u64>,
    pub compression_ratio_avg: f64,
    pub compression_ratio_p50: f64,
    pub compression_ratio_p95: f64,
    pub compression_ratio_p99: f64,
    pub entropy_avg: f64,
    pub entropy_p50: f64,
    pub operation_time_avg_ms: f64,
    pub operation_time_p50_ms: f64,
    pub operation_time_p95_ms: f64,
}

/// Result of a storage operation
#[derive(Debug, Clone)]
pub struct StorageResult {
    pub original_size: usize,
    pub stored_size: usize,
    pub compression_ratio: Option<f64>,
    pub duration_ms: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::analysis::{DataAnalysis, DataFormat};
    use crate::storage::pipeline::Pipeline;
    
    #[tokio::test]
    async fn test_metrics_collection() {
        let metrics = MetricsCollector::new();
        
        // Record some operations
        for i in 0..100 {
            let analysis = DataAnalysis {
                entropy: 2.0 + (i as f64 * 0.05),
                format: DataFormat::Text,
                compressibility_estimate: 0.8,
                is_text: true,
                is_binary: false,
                size: 1000,
                has_repetition: true,
            };
            
            let pipeline = Pipeline::new_for_test("zstd");
            
            let result = StorageResult {
                original_size: 1000,
                stored_size: 200,
                compression_ratio: Some(5.0),
                duration_ms: Some(10.0 + i as f64),
            };
            
            metrics.record(&analysis, &pipeline, &result).await.unwrap();
        }
        
        // Get snapshot
        let snapshot = metrics.snapshot();
        
        assert_eq!(snapshot.total_operations, 100);
        assert_eq!(snapshot.total_bytes_stored, 100_000);
        assert_eq!(snapshot.total_bytes_saved, 80_000);
        assert_eq!(snapshot.savings_percent, 80.0);
        
        assert!(snapshot.compression_ratio_avg > 0.0);
        assert!(snapshot.entropy_avg > 2.0);
        assert!(snapshot.operation_time_avg_ms > 0.0);
    }
    
    #[tokio::test]
    async fn test_strategy_counting() {
        let metrics = MetricsCollector::new();
        
        let analysis = DataAnalysis {
            entropy: 2.0,
            format: DataFormat::Text,
            compressibility_estimate: 0.8,
            is_text: true,
            is_binary: false,
            size: 1000,
            has_repetition: true,
        };
        
        // Record operations with different strategies
        for _ in 0..10 {
            let pipeline = Pipeline::new_for_test("zstd");
            let result = StorageResult {
                original_size: 1000,
                stored_size: 200,
                compression_ratio: Some(5.0),
                duration_ms: Some(10.0),
            };
            metrics.record(&analysis, &pipeline, &result).await.unwrap();
        }
        
        for _ in 0..5 {
            let pipeline = Pipeline::new_for_test("lz4");
            let result = StorageResult {
                original_size: 1000,
                stored_size: 300,
                compression_ratio: Some(3.33),
                duration_ms: Some(5.0),
            };
            metrics.record(&analysis, &pipeline, &result).await.unwrap();
        }
        
        let snapshot = metrics.snapshot();
        assert_eq!(snapshot.strategy_counts.get("zstd").copied().unwrap_or(0), 10);
        assert_eq!(snapshot.strategy_counts.get("lz4").copied().unwrap_or(0), 5);
    }
}

