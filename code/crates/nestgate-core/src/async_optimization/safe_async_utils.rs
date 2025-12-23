//! **SAFE ASYNC UTILITIES**
//!
//! High-performance async utilities using 100% safe Rust.
//! Provides timeout, retry, and concurrency primitives without unsafe code.
//!
//! ## Features
//!
//! - **100% Safe**: Zero unsafe code
//! - **High Performance**: Optimized for async workloads
//! - **Ergonomic**: Easy-to-use APIs
//! - **Composable**: Works with any Future
//!
//! ## Example
//!
//! ```rust
//! use nestgate_core::async_optimization::safe_async_utils::*;
//! use std::time::Duration;
//!
//! async fn example() {
//!     // Timeout any future safely
//!     let result = timeout(Duration::from_secs(5), async {
//!         // Your async operation
//!         42
//!     }).await;
//!     
//!     // Retry with exponential backoff
//!     let result = retry_with_backoff(3, || async {
//!         // Your fallible operation
//!         Ok::<_, std::io::Error>(())
//!     }).await;
//! }
//! ```

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::time::Sleep;

/// Timeout wrapper for any future
///
/// Returns `Ok(value)` if future completes in time,
/// `Err(Elapsed)` if timeout expires.
pub async fn timeout<F, T>(duration: Duration, future: F) -> Result<T, tokio::time::error::Elapsed>
where
    F: Future<Output = T>,
{
    tokio::time::timeout(duration, future).await
}

/// Retry operation with exponential backoff
///
/// Retries the operation up to `max_attempts` times with exponential backoff.
///
/// ## Sleep Usage
///
/// This function uses `tokio::time::sleep` for retry delays, which is **acceptable** because:
/// - Sleep is the **correct** mechanism for retry backoff (not a workaround)
/// - It yields to the executor (non-blocking, concurrent-safe)
/// - It's cancellation-safe (works with `select!` and `timeout`)
/// - There's no alternative event to wait for (the delay IS the requirement)
///
/// For more complex retry needs, consider the `tokio-retry` crate.
pub async fn retry_with_backoff<F, Fut, T, E>(max_attempts: usize, mut operation: F) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    let mut attempt = 0;
    let mut delay = Duration::from_millis(100);

    loop {
        attempt += 1;
        
        match operation().await {
            Ok(value) => return Ok(value),
            Err(e) if attempt >= max_attempts => return Err(e),
            Err(_) => {
                // Exponential backoff: 100ms, 200ms, 400ms, 800ms, ...
                // Sleep is appropriate here - this IS the intended behavior
                tokio::time::sleep(delay).await;
                delay = delay.saturating_mul(2);
            }
        }
    }
}

/// Retry operation with linear backoff
///
/// Retries the operation up to `max_attempts` times with fixed delay between attempts.
///
/// ## Sleep Usage
///
/// Uses `tokio::time::sleep` for retry delays, which is **appropriate** for backoff logic.
/// The delay between retries is the intended behavior, not a workaround.
pub async fn retry_with_linear_backoff<F, Fut, T, E>(
    max_attempts: usize,
    delay: Duration,
    mut operation: F,
) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    let mut attempt = 0;

    loop {
        attempt += 1;
        
        match operation().await {
            Ok(value) => return Ok(value),
            Err(e) if attempt >= max_attempts => return Err(e),
            Err(_) => {
                // Fixed delay between retries
                // Sleep is the correct mechanism here
                tokio::time::sleep(delay).await;
            }
        }
    }
}

/// Race multiple futures, returning the first to complete
///
/// This is a safe alternative to manual future racing.
pub async fn race<F1, F2, T>(future1: F1, future2: F2) -> T
where
    F1: Future<Output = T>,
    F2: Future<Output = T>,
{
    tokio::select! {
        result = future1 => result,
        result = future2 => result,
    }
}

/// Execute futures concurrently with a limit
///
/// Runs up to `limit` futures concurrently, spawning new ones as others complete.
pub async fn concurrent_with_limit<F, Fut, T>(
    futures: Vec<F>,
    limit: usize,
) -> Vec<T>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    use futures::stream::{FuturesUnordered, StreamExt};
    
    let mut results = Vec::with_capacity(futures.len());
    let mut pending = FuturesUnordered::new();
    let mut futures = futures.into_iter();

    // Start initial batch
    for _ in 0..limit.min(futures.len()) {
        if let Some(f) = futures.next() {
            pending.push(tokio::spawn(f()));
        }
    }

    // Process results and spawn new tasks
    while let Some(result) = pending.next().await {
        if let Ok(value) = result {
            results.push(value);
        }
        
        // Spawn next task if available
        if let Some(f) = futures.next() {
            pending.push(tokio::spawn(f()));
        }
    }

    results
}

/// Safe pinning helper
///
/// Safely pin a value to the stack for use with async operations.
#[macro_export]
macro_rules! pin {
    ($val:expr) => {
        std::pin::pin!($val)
    };
}

/// Yield to the executor
///
/// Allows other tasks to run. Useful in CPU-intensive async operations.
pub async fn yield_now() {
    tokio::task::yield_now().await;
}

/// Sleep for a duration
///
/// Async sleep that yields to the executor.
///
/// ## When to Use Sleep
///
/// `tokio::time::sleep` is **appropriate** when:
/// - Implementing retry/backoff logic (delay IS the requirement)
/// - Rate limiting (intentional delays between operations)
/// - Timeouts (combined with `select!`)
/// - Simulating delays in tests
///
/// ## When NOT to Use Sleep
///
/// **AVOID** `sleep` for:
/// - Waiting for conditions (use `Notify`, channels, or `watch` instead)
/// - Polling for state changes (use event-driven patterns)
/// - Synchronizing operations (use proper sync primitives)
///
/// ## Properties
///
/// - **Non-blocking**: Yields to executor, doesn't block thread
/// - **Cancellation-safe**: Works correctly with `select!` and `timeout`
/// - **Precise**: More accurate than spinning or polling
pub async fn sleep(duration: Duration) {
    tokio::time::sleep(duration).await;
}

/// Interval timer
///
/// Creates a stream that yields at regular intervals.
pub fn interval(period: Duration) -> tokio::time::Interval {
    tokio::time::interval(period)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_timeout_success() {
        let result = timeout(Duration::from_secs(1), async {
            tokio::time::sleep(Duration::from_millis(100)).await;
            42
        })
        .await;

        assert_eq!(result.unwrap(), 42);
    }

    #[tokio::test]
    async fn test_timeout_expired() {
        let result = timeout(Duration::from_millis(100), async {
            tokio::time::sleep(Duration::from_secs(1)).await;
            42
        })
        .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_retry_success_first_try() {
        let mut attempts = 0;
        let result = retry_with_backoff(3, || async {
            attempts += 1;
            Ok::<_, ()>(42)
        })
        .await;

        assert_eq!(result.unwrap(), 42);
        assert_eq!(attempts, 1);
    }

    #[tokio::test]
    async fn test_retry_success_after_failures() {
        let mut attempts = 0;
        let result = retry_with_backoff(3, || async {
            attempts += 1;
            if attempts < 3 {
                Err(())
            } else {
                Ok(42)
            }
        })
        .await;

        assert_eq!(result.unwrap(), 42);
        assert_eq!(attempts, 3);
    }

    #[tokio::test]
    async fn test_retry_exhausted() {
        let mut attempts = 0;
        let result = retry_with_backoff(3, || async {
            attempts += 1;
            Err::<i32, _>(())
        })
        .await;

        assert!(result.is_err());
        assert_eq!(attempts, 3);
    }

    #[tokio::test]
    async fn test_race() {
        let result = race(
            async {
                tokio::time::sleep(Duration::from_millis(100)).await;
                1
            },
            async {
                tokio::time::sleep(Duration::from_millis(200)).await;
                2
            },
        )
        .await;

        assert_eq!(result, 1); // First to complete
    }

    #[tokio::test]
    async fn test_concurrent_with_limit() {
        let futures: Vec<_> = (0..10)
            .map(|i| move || async move { i * 2 })
            .collect();

        let results = concurrent_with_limit(futures, 3).await;

        assert_eq!(results.len(), 10);
        assert!(results.contains(&0));
        assert!(results.contains(&18));
    }
}

