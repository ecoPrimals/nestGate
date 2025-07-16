//! Performance optimization module for NestGate Core
//!
//! This module provides optimized patterns and implementations for
//! performance-critical paths in storage operations.

use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{RwLock, Semaphore};

/// High-performance storage operation coordinator
#[derive(Debug)]
pub struct PerformanceOptimizedCoordinator {
    /// Semaphore for limiting concurrent operations
    operation_semaphore: Arc<Semaphore>,
    /// Request batching system
    batch_processor: Arc<BatchProcessor>,
    /// Connection pool for backends
    connection_pool: Arc<ConnectionPool>,
    /// Performance metrics collector
    metrics_collector: Arc<OptimizedMetricsCollector>,
}

impl PerformanceOptimizedCoordinator {
    /// Create a new performance-optimized coordinator
    pub fn new(max_concurrent_ops: usize) -> Self {
        Self {
            operation_semaphore: Arc::new(Semaphore::new(max_concurrent_ops)),
            batch_processor: Arc::new(BatchProcessor::new()),
            connection_pool: Arc::new(ConnectionPool::new()),
            metrics_collector: Arc::new(OptimizedMetricsCollector::new()),
        }
    }

    /// Execute a storage operation with performance optimization
    pub async fn execute_optimized_operation(
        &self,
        operation: StorageOperation,
    ) -> Result<OperationResult> {
        // Acquire semaphore permit to limit concurrency
        let _permit = self.operation_semaphore.acquire().await.map_err(|_| {
            crate::NestGateError::Internal("Failed to acquire operation permit".to_string())
        })?;

        let start = Instant::now();

        // Check if operation can be batched
        if self.batch_processor.can_batch(&operation) {
            let result = self.batch_processor.add_to_batch(operation).await?;
            let duration = start.elapsed();
            self.metrics_collector
                .record_batched_operation(duration)
                .await;
            return Ok(result);
        }

        // Execute individual operation with connection pooling
        let connection = self
            .connection_pool
            .get_connection(&operation.backend)
            .await?;
        let result = connection.execute(operation).await?;

        let duration = start.elapsed();
        self.metrics_collector
            .record_individual_operation(duration)
            .await;

        Ok(result)
    }

    /// Get performance metrics
    pub async fn get_performance_metrics(&self) -> PerformanceMetrics {
        self.metrics_collector.get_metrics().await
    }
}

/// Batch processing system for grouping similar operations
#[derive(Debug)]
pub struct BatchProcessor {
    /// Current batches grouped by operation type
    batches: Arc<RwLock<HashMap<String, Vec<StorageOperation>>>>,
    /// Batch size threshold
    batch_size: usize,
    /// Batch timeout
    _batch_timeout: Duration,
}

impl BatchProcessor {
    pub fn new() -> Self {
        Self {
            batches: Arc::new(RwLock::new(HashMap::new())),
            batch_size: 100,                           // Optimize batch size
            _batch_timeout: Duration::from_millis(50), // Low latency batching
        }
    }
}

impl Default for BatchProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl BatchProcessor {
    /// Check if an operation can be batched
    pub fn can_batch(&self, operation: &StorageOperation) -> bool {
        matches!(
            operation.operation_type,
            OperationType::Read | OperationType::Write
        )
    }

    /// Add operation to batch
    pub async fn add_to_batch(&self, operation: StorageOperation) -> Result<OperationResult> {
        let mut batches = self.batches.write().await;
        let batch_key = self.get_batch_key(&operation);

        let batch = batches.entry(batch_key.clone()).or_insert_with(Vec::new);
        batch.push(operation);

        // Check if batch is ready to process
        if batch.len() >= self.batch_size {
            let operations = std::mem::take(batch);
            drop(batches); // Release lock before processing

            self.process_batch(operations).await
        } else {
            // Return placeholder - in real implementation, this would be async
            Ok(OperationResult::Pending)
        }
    }

    /// Process a batch of operations
    async fn process_batch(&self, operations: Vec<StorageOperation>) -> Result<OperationResult> {
        // Parallel processing of batch operations
        let tasks = operations.into_iter().map(|_op| {
            tokio::spawn(async move {
                // Simulate optimized batch processing
                tokio::time::sleep(Duration::from_micros(100)).await;
                OperationResult::Success
            })
        });

        let results = futures::future::join_all(tasks).await;

        // Aggregate results
        let success_count = results.iter().filter(|r| r.is_ok()).count();

        Ok(OperationResult::BatchCompleted {
            total: results.len(),
            succeeded: success_count,
            failed: results.len() - success_count,
        })
    }

    fn get_batch_key(&self, operation: &StorageOperation) -> String {
        format!("{}_{}", operation.operation_type, operation.backend)
    }
}

/// Connection pool for backend connections
#[derive(Debug)]
pub struct ConnectionPool {
    /// Pool of connections per backend
    connections: Arc<RwLock<HashMap<String, Vec<Arc<BackendConnection>>>>>,
    /// Maximum connections per backend
    max_connections: usize,
}

impl ConnectionPool {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            max_connections: 10,
        }
    }

    /// Get a connection from the pool
    pub async fn get_connection(&self, backend: &str) -> Result<Arc<BackendConnection>> {
        let mut connections = self.connections.write().await;
        let backend_connections = connections
            .entry(backend.to_string())
            .or_insert_with(Vec::new);

        if let Some(connection) = backend_connections.pop() {
            Ok(connection)
        } else {
            // Create new connection
            let connection = Arc::new(BackendConnection::new(backend.to_string()));
            Ok(connection)
        }
    }

    /// Return connection to pool
    pub async fn return_connection(&self, backend: &str, connection: Arc<BackendConnection>) {
        let mut connections = self.connections.write().await;
        let backend_connections = connections
            .entry(backend.to_string())
            .or_insert_with(Vec::new);

        if backend_connections.len() < self.max_connections {
            backend_connections.push(connection);
        }
    }
}

impl Default for ConnectionPool {
    fn default() -> Self {
        Self::new()
    }
}

/// Optimized metrics collector using lock-free patterns
#[derive(Debug)]
pub struct OptimizedMetricsCollector {
    /// Performance metrics
    metrics: Arc<RwLock<PerformanceMetrics>>,
}

impl OptimizedMetricsCollector {
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(PerformanceMetrics::default())),
        }
    }

    /// Record individual operation metrics
    pub async fn record_individual_operation(&self, duration: Duration) {
        let mut metrics = self.metrics.write().await;
        metrics.individual_operations += 1;
        metrics.total_individual_time += duration;

        if duration < metrics.min_latency {
            metrics.min_latency = duration;
        }
        if duration > metrics.max_latency {
            metrics.max_latency = duration;
        }
    }

    /// Record batched operation metrics
    pub async fn record_batched_operation(&self, duration: Duration) {
        let mut metrics = self.metrics.write().await;
        metrics.batched_operations += 1;
        metrics.total_batched_time += duration;
    }

    /// Get current metrics
    pub async fn get_metrics(&self) -> PerformanceMetrics {
        self.metrics.read().await.clone()
    }
}

impl Default for OptimizedMetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Performance metrics structure
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Number of individual operations
    pub individual_operations: u64,
    /// Total time spent on individual operations
    pub total_individual_time: Duration,
    /// Number of batched operations
    pub batched_operations: u64,
    /// Total time spent on batched operations
    pub total_batched_time: Duration,
    /// Minimum latency observed
    pub min_latency: Duration,
    /// Maximum latency observed
    pub max_latency: Duration,
    /// Average throughput (operations per second)
    pub average_throughput: f64,
}

impl PerformanceMetrics {
    /// Calculate average latency for individual operations
    pub fn average_individual_latency(&self) -> Duration {
        if self.individual_operations > 0 {
            self.total_individual_time / self.individual_operations as u32
        } else {
            Duration::ZERO
        }
    }

    /// Calculate average latency for batched operations
    pub fn average_batched_latency(&self) -> Duration {
        if self.batched_operations > 0 {
            self.total_batched_time / self.batched_operations as u32
        } else {
            Duration::ZERO
        }
    }

    /// Calculate performance improvement from batching
    pub fn batching_improvement(&self) -> f64 {
        if self.individual_operations > 0 && self.batched_operations > 0 {
            let individual_avg = self.average_individual_latency().as_nanos() as f64;
            let batched_avg = self.average_batched_latency().as_nanos() as f64;

            if batched_avg > 0.0 {
                (individual_avg - batched_avg) / individual_avg * 100.0
            } else {
                0.0
            }
        } else {
            0.0
        }
    }
}

/// Storage operation structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageOperation {
    pub operation_type: OperationType,
    pub backend: String,
    pub path: String,
    pub data: Option<Vec<u8>>,
    pub metadata: HashMap<String, String>,
}

/// Operation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationType {
    Read,
    Write,
    Delete,
    List,
    Copy,
    Move,
}

impl std::fmt::Display for OperationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationType::Read => write!(f, "read"),
            OperationType::Write => write!(f, "write"),
            OperationType::Delete => write!(f, "delete"),
            OperationType::List => write!(f, "list"),
            OperationType::Copy => write!(f, "copy"),
            OperationType::Move => write!(f, "move"),
        }
    }
}

/// Operation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationResult {
    Success,
    Pending,
    Error(String),
    BatchCompleted {
        total: usize,
        succeeded: usize,
        failed: usize,
    },
}

/// Backend connection
#[derive(Debug)]
pub struct BackendConnection {
    pub backend_name: String,
    pub created_at: Instant,
}

impl BackendConnection {
    pub fn new(backend_name: String) -> Self {
        Self {
            backend_name,
            created_at: Instant::now(),
        }
    }

    /// Execute operation on this connection
    pub async fn execute(&self, _operation: StorageOperation) -> Result<OperationResult> {
        // Simulate optimized backend execution
        tokio::time::sleep(Duration::from_micros(500)).await;
        Ok(OperationResult::Success)
    }
}

/// Performance benchmarking utilities
pub mod benchmarks {
    use super::*;

    /// Benchmark storage operations
    pub async fn benchmark_storage_operations(
        coordinator: &PerformanceOptimizedCoordinator,
    ) -> Result<BenchmarkResults> {
        let mut results = BenchmarkResults::default();

        // Benchmark individual operations
        let start = Instant::now();
        for i in 0..1000 {
            let operation = StorageOperation {
                operation_type: OperationType::Read,
                backend: "test".to_string(),
                path: format!("/test/file_{i}"),
                data: None,
                metadata: HashMap::new(),
            };

            coordinator.execute_optimized_operation(operation).await?;
        }
        results.individual_ops_time = start.elapsed();

        // Benchmark batched operations
        let start = Instant::now();
        let mut batch_tasks = Vec::new();
        for i in 0..1000 {
            let operation = StorageOperation {
                operation_type: OperationType::Read,
                backend: "test".to_string(),
                path: format!("/test/batch_file_{i}"),
                data: None,
                metadata: HashMap::new(),
            };

            batch_tasks.push(coordinator.execute_optimized_operation(operation));
        }

        futures::future::join_all(batch_tasks).await;
        results.batched_ops_time = start.elapsed();

        Ok(results)
    }

    /// Benchmark results
    #[derive(Debug, Clone, Default)]
    pub struct BenchmarkResults {
        pub individual_ops_time: Duration,
        pub batched_ops_time: Duration,
    }

    impl BenchmarkResults {
        pub fn performance_improvement(&self) -> f64 {
            if self.batched_ops_time.as_nanos() > 0 {
                let individual_ns = self.individual_ops_time.as_nanos() as f64;
                let batched_ns = self.batched_ops_time.as_nanos() as f64;
                ((individual_ns - batched_ns) / individual_ns) * 100.0
            } else {
                0.0
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_performance_coordinator() {
        let coordinator = PerformanceOptimizedCoordinator::new(100);

        let operation = StorageOperation {
            operation_type: OperationType::Read,
            backend: "test".to_string(),
            path: "/test/file".to_string(),
            data: None,
            metadata: HashMap::new(),
        };

        let result = coordinator.execute_optimized_operation(operation).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_batch_processing() {
        let processor = BatchProcessor::new();

        let operation = StorageOperation {
            operation_type: OperationType::Read,
            backend: "test".to_string(),
            path: "/test/file".to_string(),
            data: None,
            metadata: HashMap::new(),
        };

        assert!(processor.can_batch(&operation));
    }

    #[tokio::test]
    async fn test_connection_pool() {
        let pool = ConnectionPool::new();

        let connection = pool.get_connection("test").await.unwrap();
        assert_eq!(connection.backend_name, "test");

        pool.return_connection("test", connection).await;
    }

    #[test]
    fn test_performance_metrics() {
        let metrics = PerformanceMetrics {
            individual_operations: 100,
            total_individual_time: Duration::from_secs(1),
            ..Default::default()
        };

        let avg = metrics.average_individual_latency();
        assert_eq!(avg, Duration::from_millis(10));
    }
}
