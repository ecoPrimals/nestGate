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
type ProcessingQueue<T> = Arc<Mutex<VecDeque<Batch<T>>>>;
type ProcessorFunction<T, R> = Arc<dyn Fn(Vec<T>) -> Result<Vec<R>> + Send + Sync>;

/// Batch processing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchProcessingConfig {
    pub max_batch_size: usize,
    pub min_batch_size: usize,
    pub batch_timeout: Duration,
    pub max_pending_items: usize,
    pub enable_adaptive_sizing: bool,
    pub target_latency: Duration,
}
impl Default for BatchProcessingConfig {
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
pub struct BatchItem<T> {
    pub data: T,
    pub timestamp: Instant,
    pub priority: BatchPriority,
}
/// Batch priority levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BatchPriority {
    Low,
    Normal,
    High,
    Critical,
}
/// A batch of items ready for processing
#[derive(Debug)]
pub struct Batch<T> {
    pub items: Vec<BatchItem<T>>,
    pub created_at: Instant,
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
pub struct BatchMetrics {
    pub batches_processed: std::sync::atomic::AtomicU64,
    pub items_processed: std::sync::atomic::AtomicU64,
    pub average_batch_size: std::sync::atomic::AtomicU64,
    pub average_processing_time: std::sync::atomic::AtomicU64,
    pub queue_depth: std::sync::atomic::AtomicUsize,
}
impl<T: Send + Sync + 'static, R: Send + Sync + 'static> BatchProcessor<T, R> {
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
        #[must_use]
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
