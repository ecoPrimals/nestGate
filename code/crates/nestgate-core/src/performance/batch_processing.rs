//! Batch Processing module

use crate::error::NestGateError;
//
// Intelligent batch processing for high-throughput operations with adaptive sizing.

use crate::error::CanonicalResult as Result;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

// Type aliases for batch processing
type PendingItemsQueue<T> = Arc<Mutex<VecDeque<BatchItem<T>>>>;
/// Type alias for ProcessingQueue
type ProcessingQueue<T> = Arc<Mutex<VecDeque<Batch<T>>>>;
/// Type alias for ProcessorFunction
type ProcessorFunction<T, R> = Arc<dyn Fn(Vec<T>) -> Result<Vec<R>> + Send + Sync>;

/// Batch processing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::BatchProcessingConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::BatchProcessingConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
/// Configuration for BatchProcessing
pub struct BatchProcessingConfig {
    /// Size of max batch
    pub max_batch_size: usize,
    /// Size of min batch
    pub min_batch_size: usize,
    /// Batch Timeout
    pub batch_timeout: Duration,
    /// Max Pending Items
    pub max_pending_items: usize,
    /// Enable Adaptive Sizing
    pub enable_adaptive_sizing: bool,
    /// Target Latency
    pub target_latency: Duration,
}
impl Default for BatchProcessingConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            max_batch_size: 100,
            min_batch_size: 10,
            batch_timeout: Duration::from_millis(100),
            max_pending_items: 10000,
            enable_adaptive_sizing: true,
            target_latency: Duration::from_millis(50),
        }
    }
}

/// Individual item in a batch
#[derive(Debug, Clone)]
/// Batchitem
pub struct BatchItem<T> {
    /// Data
    pub data: T,
    /// Timestamp
    pub timestamp: Instant,
    /// Priority
    pub priority: BatchPriority,
}
/// Batch priority levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// Batchpriority
pub enum BatchPriority {
    /// Low
    Low,
    /// Normal
    Normal,
    /// High
    High,
    /// Critical
    Critical,
}
/// A batch of items ready for processing
#[derive(Debug)]
/// Batch
pub struct Batch<T> {
    /// Items
    pub items: Vec<BatchItem<T>>,
    /// Timestamp when this was created
    pub created_at: Instant,
    /// Estimated Processing Time
    pub estimated_processing_time: Duration,
}
/// High-performance batch processor with intelligent sizing
pub struct BatchProcessor<T, R> {
    pending_items: PendingItemsQueue<T>,
    processing_queue: ProcessingQueue<T>,
    processor_fn: ProcessorFunction<T, R>,
    config: BatchProcessingConfig,
    metrics: Arc<BatchMetrics>,
}
/// Batch processing metrics
#[derive(Debug, Default)]
/// Batchmetrics
pub struct BatchMetrics {
    /// Batches Processed
    pub batches_processed: std::sync::atomic::AtomicU64,
    /// Items Processed
    pub items_processed: std::sync::atomic::AtomicU64,
    /// Size of average batch
    pub average_batch_size: std::sync::atomic::AtomicU64,
    /// Average Processing Time
    pub average_processing_time: std::sync::atomic::AtomicU64,
    /// Queue Depth
    pub queue_depth: std::sync::atomic::AtomicUsize,
}
impl<T: Send + Sync + 'static, R: Send + Sync + 'static> BatchProcessor<T, R> {
    /// Creates a new instance
    pub fn new(processor_fn: ProcessorFunction<T, R>, config: BatchProcessingConfig) -> Self {
        Self {
            pending_items: Arc::new(Mutex::new(VecDeque::new())),
            processing_queue: Arc::new(Mutex::new(VecDeque::new())),
            processor_fn,
            config,
            metrics: Arc::new(BatchMetrics::default()),
        }
    }

    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
                pub fn add_item(&self, item: T, priority: BatchPriority) -> Result<()>  {
        let batch_item = BatchItem {
            data: item,
            timestamp: Instant::now(),
            priority,
        };

        let mut pending = self.pending_items.lock().await;

        if pending.len() >= self.config.max_pending_items {
            return Err(crate::error::NestGateError::validation_error(
                "queue_full",
                &format!("Pending queue is full ({self.config.max_pending_items})"),
                Some(pending.len().to_string()),
            ));
        }

        pending.push_back(batch_item);
        Ok(())
    }

    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
        pub async fn process_batches(&self) -> Result<Vec<R>>  {
        // Create batches from pending items
        self.create_batches().await?;

        // Process all ready batches
        let mut results = Vec::new();
        let mut processing_queue = self.processing_queue.lock().await;

        while let Some(batch) = processing_queue.pop_front() {
            let batch_data: Vec<T> = batch.items.into_iter().map(|item| item.data).collect();
            match (self.processor_fn)(batch_data) {
                Ok(mut batch_results) => {
                    results.append(&mut batch_results);
                    self.metrics
                        .batches_processed
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }
                Err(e) => return Err(e),
            }
        }

        Ok(results)
    }

    /// Creates  Batches
    async fn create_batches(&self) -> Result<()> {
        let mut pending = self.pending_items.lock().await;
        let mut processing = self.processing_queue.lock().await;

        while !pending.is_empty() && pending.len() >= self.config.min_batch_size {
            let batch_size = std::cmp::min(self.config.max_batch_size, pending.len());
            let mut batch_items = Vec::with_capacity(batch_size);

            for _ in 0..batch_size {
                if let Some(item) = pending.pop_front() {
                    batch_items.push(item);
                }
            }

            let batch = Batch {
                items: batch_items,
                created_at: Instant::now(),
                estimated_processing_time: Duration::from_millis(50), // Estimate
            };

            processing.push_back(batch);
        }

        Ok(())
    }
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Batchprocessingconfigcanonical
pub type BatchProcessingConfigCanonical = crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using BatchProcessingConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

