use crate::error::NestGateError;
// Advanced Async Optimization Module
//
// This module provides optimized async patterns and utilities to improve
// performance of async operations throughout NestGate.

use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::sleep;

/// Type alias for batch processor function with idiomatic error handling
type BatchProcessorFn<T, R, E> =
    Box<dyn Fn(Vec<T>) -> Pin<Box<dyn Future<Output = Result<Vec<R>, E>> + Send>> + Send + Sync>;
/// Type alias for async operation function with idiomatic error handling
type AsyncOperationFn<T, E> =
    Box<dyn Fn() -> Pin<Box<dyn Future<Output = Result<T, E>> + Send>> + Send + Sync>;
/// Async batch processor for high-throughput operations
pub struct AsyncBatchProcessor<T, R, E = NestGateError> {
    batch_size: usize,
    processor: BatchProcessorFn<T, R, E>,
}
impl<T, R, E> AsyncBatchProcessor<T, R, E>
where
    T: Clone + Send + 'static,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    /// Create a new batch processor with optimized settings
    pub fn new<F, Fut>(batch_size: usize, _timeout: Duration, processor: F) -> Self
    where
        F: Fn(Vec<T>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Vec<R>, E>> + Send + 'static,
    {
        Self {
            batch_size,
            processor: Box::new(move |items| Box::pin(processor(items))),
        }
    }

    /// Process a batch of items with optimized error handling
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn process_batch(&self, items: Vec<T>) -> Result<Vec<R>, E>  {
        if items.len() <= self.batch_size {
            (self.processor)(items).await
        } else {
            // Split large batches for better memory management
            let mut results = Vec::new();
            for chunk in items.chunks(self.batch_size) {
                let chunk_results = (self.processor)(chunk.to_vec()).await?;
                results.extend(chunk_results);
            }
            Ok(results)
        }
    }
}

/// Zero-allocation async iterator for streaming operations
pub struct AsyncStreamProcessor<T> {
    items: Vec<T>,
    current_index: usize,
    delay: Duration,
}
impl<T> AsyncStreamProcessor<T> {
    /// Create new stream processor
    pub fn new(items: Vec<T>, delay: Duration) -> Self {
        Self {
            items,
            current_index: 0,
            delay,
        }
    }

    /// Get next item asynchronously without allocation
    #[must_use]
    pub fn next(&mut self) -> Option<&T> {
        if self.current_index >= self.items.len() {
            return None;
        }

        if self.delay > Duration::ZERO {
            sleep(self.delay).await;
        }

        let item = &self.items[self.current_index];
        self.current_index += 1;
        Some(item)
    }

    /// Process all remaining items with a closure
    pub async fn process_remaining<F, Fut>(&mut self, mut processor: F)
    where
        F: FnMut(&T) -> Fut,
        Fut: Future<Output = ()>,
    {
        while let Some(item) = self.next().await {
            processor(item).await;
        }
    }
}

/// **IDIOMATIC**: Timeout future with explicit error types
pub struct TimeoutFuture<F, E = NestGateError> {
    future: F,
    #[allow(dead_code)]
    timeout: Duration,
    _error_type: std::marker::PhantomData<E>,
}
impl<F, E> Future for TimeoutFuture<F, E>
where
    F: Future,
    E: From<NestGateError>,
{
    type Output = Result<F::Output, E>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Implementation would use timeout logic here
        // For now, just forward to the inner future
        let future = unsafe { self.as_mut().map_unchecked_mut(|s| &mut s.future) };
        match future.poll(cx) {
            Poll::Ready(result) => Poll::Ready(Ok(result)),
            Poll::Pending => Poll::Pending,
        }
    }
}

/// Async operation executor with intelligent caching and idiomatic errors
pub struct AsyncOperationExecutor<T, E = NestGateError> {
    cache: Arc<RwLock<Option<T>>>,
    cache_duration: Duration,
    last_update: Arc<RwLock<Option<std::time::Instant>>>,
}
impl<T, E> AsyncOperationExecutor<T, E>
where
    T: Clone + Send + Sync + 'static,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    /// Create new executor with caching
    where
        F: Fn() -> Pin<Box<dyn Future<Output = Result<T, E>> + Send>> + Send + Sync + 'static,
    {
        Self {
            cache: Arc::new(RwLock::new(None)),
            cache_duration,
            last_update: Arc::new(RwLock::new(None)),
        }
    }

    /// Execute with intelligent caching
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn execute(&self) -> Result<T, E>  {
        // Check cache validity
        let last_update = *self.last_update.read().await;
        let cache_valid = last_update
            .map(|time| time.elapsed() < self.cache_duration)
            .unwrap_or(false);

        if cache_valid {
            if let Some(cached) = self.cache.read().await.clone() {
                return Ok(cached);
            }
        }

        // Execute operation
        let result = (self.b_operation)().await?;

        // Update cache
        *self.cache.write().await = Some(result.clone());
        *self.last_update.write().await = Some(std::time::Instant::now());

        Ok(result)
    }
}

/// Optimized async retry mechanism with exponential backoff
pub struct OptimizedAsyncRetry<F, T, E> {
    max_retries: usize,
    initial_delay: Duration,
    max_delay: Duration,
    backoff_multiplier: f64,
    _phantom: std::marker::PhantomData<(T, E)>,
}
impl<F, T, E> OptimizedAsyncRetry<F, T, E>
where
    F: Fn() -> Pin<Box<dyn Future<Output = Result<T, E>> + Send>>,
    E: std::fmt::Debug,
{
    /// Create new optimized retry mechanism
    pub fn new(
        max_retries: usize,
        initial_delay: Duration,
        max_delay: Duration,
    ) -> Self {
        Self {
            operation,
            max_retries,
            initial_delay,
            max_delay,
            backoff_multiplier: 2.0,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Execute operation with optimized retry logic
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn execute(&self) -> Result<T, E>  {
        let mut current_delay = self.initial_delay;

        for attempt in 0..=self.max_retries {
            match (self.b_operation)().await {
                Ok(result) => return Ok(result),
                Err(error) => {
                    if attempt == self.max_retries {
                        return Err(error);
                    }

                    tracing::debug!(
                        "Retry attempt {} failed, retrying in {:?}: {:?}",
                        attempt + 1,
                        current_delay,
                        error
                    );

                    sleep(current_delay).await;

                    // Exponential backoff with max delay cap
                    current_delay = std::cmp::min(
                        Duration::from_millis(
                            (current_delay.as_millis() as f64 * self.backoff_multiplier) as u64,
                        ),
                        self.max_delay,
                    );
                }
            }
        }

        unreachable!()
    }
}

/// Optimized async semaphore for resource limiting
pub struct OptimizedAsyncSemaphore {
    semaphore: tokio::sync::Semaphore,
    max_permits: usize,
}
impl OptimizedAsyncSemaphore {
    /// Create new optimized semaphore
    pub fn new(max_permits: usize) -> Self {
        Self {
            semaphore: tokio::sync::Semaphore::new(max_permits),
            max_permits,
        }
    }

    /// Acquire permit with optimized waiting
    pub async fn acquire(&self) -> tokio::sync::SemaphorePermit<'_> {
        self.semaphore
            .acquire()
            .await
            .expect("Semaphore should not be closed")
    }

    /// Try to acquire permit without waiting
    pub fn try_acquire(&self) -> Option<tokio::sync::SemaphorePermit<'_>> {
        self.semaphore.try_acquire().ok()
    }

    /// Get available permits
    pub fn available_permits(&self) -> usize {
        self.semaphore.available_permits()
    }

    /// Get utilization percentage
    pub fn utilization_percent(&self) -> f64 {
        let used = self.max_permits - self.semaphore.available_permits();
        (used as f64 / self.max_permits as f64) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Duration;

    #[tokio::test]
    async fn test_batch_processor() {
        let processor =
            AsyncBatchProcessor::new(10, Duration::from_secs(1), |items: Vec<i32>| async move {
                Ok::<Vec<i32>, NestGateError>(items.into_iter().map(|x| x * 2).collect())
            );

        let items: Vec<i32> = (1..=25).collect();
        let results = processor.process_batch(items).await;

        assert!(results.is_ok());
        let results = results.unwrap();
        assert_eq!(results.len(), 25);
        assert_eq!(results[0], 2);
        assert_eq!(results[24], 50);
    }

    #[tokio::test]
    async fn test_async_executor_caching() {
        let call_count = Arc::new(std::sync::atomic::AtomicU32::new(0));
        let call_count_clone = call_count.clone();

        let executor = AsyncOperationExecutor::new(
            move || {
                call_count_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                Box::pin(async move { Ok::<i32, NestGateError>(42) })
            },
            Duration::from_millis(100),
        );

        // First call should execute
        let result1 = executor.execute().await;
        assert!(result1.is_ok());
        assert_eq!(result1.unwrap(), 42);

        // Second call should use cache
        let result2 = executor.execute().await;
        assert!(result2.is_ok());
        assert_eq!(result2.unwrap(), 42);

        // Verify caching worked (only one call made)
        assert_eq!(call_count.load(std::sync::atomic::Ordering::SeqCst), 1);
    }
}