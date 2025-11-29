//! Bulkhead module

use crate::error::NestGateError;
//
// Implements the Bulkhead pattern to isolate resources and prevent cascading
// failures by limiting concurrent access to critical resources.

use crate::{Result};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{RwLock, Semaphore, SemaphorePermit};

/// Bulkhead configuration
#[derive(Debug, Clone)]
/// Configuration for Bulkhead
pub struct BulkheadConfig {
    /// Maximum number of concurrent operations
    pub max_concurrent: usize,
    /// Maximum queue size for waiting operations
    pub max_queue_size: usize,
    /// Timeout for acquiring a permit
    pub acquire_timeout: Duration,
    /// Enable queue overflow rejection
    pub reject_on_queue_full: bool,
}
impl Default for BulkheadConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            max_concurrent: 10,
            max_queue_size: 100,
            acquire_timeout: Duration::from_secs(30),
            reject_on_queue_full: true,
        }
    }
}

/// Bulkhead implementation for resource isolation
#[derive(Clone)]
/// Bulkhead
pub struct Bulkhead {
    name: String,
    config: BulkheadConfig,
    semaphore: Arc<Semaphore>,
    metrics: Arc<RwLock<BulkheadMetrics>>,
}
/// Bulkhead metrics for monitoring
#[derive(Debug, Clone)]
/// Bulkheadmetrics
pub struct BulkheadMetrics {
    /// Total Requests
    pub total_requests: u64,
    /// Successful Acquisitions
    pub successful_acquisitions: u64,
    /// Rejections
    pub rejections: u64,
    /// Timeouts
    pub timeouts: u64,
    /// Current Concurrent
    pub current_concurrent: usize,
    /// Max Concurrent Reached
    pub max_concurrent_reached: usize,
    /// Average Wait Time
    pub average_wait_time: Duration,
    /// Total Wait Time
    pub total_wait_time: Duration,
}
/// Bulkhead status information
#[derive(Debug, Clone)]
/// Bulkheadstatus
pub struct BulkheadStatus {
    /// Name
    pub name: String,
    /// Available Permits
    pub available_permits: usize,
    /// Max Permits
    pub max_permits: usize,
    /// Current Utilization
    pub current_utilization: f64,
    /// Size of queue
    pub queue_size: usize,
    /// Size of max queue
    pub max_queue_size: usize,
    /// Is At Capacity
    pub is_at_capacity: bool,
}
/// Permit holder that automatically releases on drop
pub struct BulkheadPermit<'a> {
    _permit: SemaphorePermit<'a>,
    bulkhead_name: String,
    acquired_at: Instant,
    metrics: Arc<RwLock<BulkheadMetrics>>,
}
impl Drop for BulkheadPermit<'_> {
    /// Drop
    fn drop(&mut self) {
        // Record the operation duration when permit is dropped
        let duration = self.acquired_at.elapsed();

        tokio::spawn({
            let metrics = Arc::clone(&self.metrics);
            async move {
                let mut m = metrics.write().await;
                m.total_wait_time += duration;
                if m.successful_acquisitions > 0 {
                    m.average_wait_time = m.total_wait_time / m.successful_acquisitions as u32;
                }
            }
        );

        tracing::debug!(
            "Released bulkhead permit for '{}' after {:?}",
            self.bulkhead_name,
            duration
        );
    }
}

impl Bulkhead {
    /// Create a new bulkhead
    pub fn new(name: String, config: BulkheadConfig) -> Self {
        let semaphore = Arc::new(Semaphore::new(config.max_concurrent));
        let metrics = Arc::new(RwLock::new(BulkheadMetrics {
            total_requests: 0,
            successful_acquisitions: 0,
            rejections: 0,
            timeouts: 0,
            current_concurrent: 0,
            max_concurrent_reached: 0,
            average_wait_time: Duration::ZERO,
            total_wait_time: Duration::ZERO,
        }));

        Self {
            name,
            config,
            semaphore,
            metrics,
        }
    }

    /// Acquire a permit to access the protected resource
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn acquire_permit(&self) -> Result<BulkheadPermit<'_>>  {
        let start_time = Instant::now();

        // Update metrics
        {
            let mut metrics = self.metrics.write().await;
            metrics.total_requests += 1;
        }

        // Check if we should reject due to queue size
        if self.config.reject_on_queue_full {
            let available = self.semaphore.available_permits();
            let queue_size = self.config.max_concurrent - available;

            if queue_size >= self.config.max_queue_size {
                let mut metrics = self.metrics.write().await;
                metrics.rejections += 1;
                return Err(NestGateError::simple("Bulkhead queue is full"));
            }
        }

        // Try to acquire permit with timeout
        let permit_result =
            tokio::time::timeout(self.config.acquire_timeout, self.semaphore.acquire()).await;

        match permit_result {
            Ok(Ok(permit)) => {
                let wait_time = start_time.elapsed();

                // Update metrics
                {
                    let mut metrics = self.metrics.write().await;
                    metrics.successful_acquisitions += 1;
                    metrics.total_wait_time += wait_time;
                    metrics.average_wait_time =
                        metrics.total_wait_time / metrics.successful_acquisitions as u32;

                    let current_concurrent =
                        self.config.max_concurrent - self.semaphore.available_permits();
                    metrics.current_concurrent = current_concurrent;
                    if current_concurrent > metrics.max_concurrent_reached {
                        metrics.max_concurrent_reached = current_concurrent;
                    }
                }

                tracing::debug!(
                    "Acquired bulkhead permit for '{}' after {:?}",
                    self.name,
                    wait_time
                );

                Ok(BulkheadPermit {
                    _permit: permit,
                    bulkhead_name: self.name.clone(),
                    acquired_at: Instant::now(),
                    metrics: Arc::clone(&self.metrics),
                })
            }
            Ok(Err(_)) => {
                // Semaphore closed (shouldn't happen in normal operation)
                let mut metrics = self.metrics.write().await;
                metrics.rejections += 1;
                Err(NestGateError::simple("Bulkhead semaphore closed"))
            }
            Err(_) => {
                // Timeout
                let mut metrics = self.metrics.write().await;
                metrics.timeouts += 1;
                Err(NestGateError::simple("Timeout acquiring bulkhead permit"))
            }
        }
    }

    /// Try to acquire a permit without waiting
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
                pub fn try_acquire_permit(&self) -> Result<Option<BulkheadPermit<'_>>>  {
        match self.semaphore.try_acquire() {
            Ok(permit) => {
                // Update metrics
                tokio::spawn({
                    let metrics = Arc::clone(&self.metrics);
                    let max_concurrent = self.config.max_concurrent;
                    let available = self.semaphore.available_permits();
                    async move {
                        let mut m = metrics.write().await;
                        m.total_requests += 1;
                        m.successful_acquisitions += 1;

                        let current_concurrent = max_concurrent - available;
                        m.current_concurrent = current_concurrent;
                        if current_concurrent > m.max_concurrent_reached {
                            m.max_concurrent_reached = current_concurrent;
                        }
                    }
                );

                Ok(Some(BulkheadPermit {
                    _permit: permit,
                    bulkhead_name: self.name.clone(),
                    acquired_at: Instant::now(),
                    metrics: Arc::clone(&self.metrics),
                }))
            }
            Err(_) => Ok(None), // No permits available
        }
    }

    /// Get current bulkhead status
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
                pub fn get_status(&self) -> Result<BulkheadStatus>  {
        let available_permits = self.semaphore.available_permits();
        let current_concurrent = self.config.max_concurrent - available_permits;
        let utilization = (current_concurrent as f64 / self.config.max_concurrent as f64) * 100.0;

        Ok(BulkheadStatus {
            name: self.name.clone(),
            available_permits,
            max_permits: self.config.max_concurrent,
            current_utilization: utilization,
            queue_size: 0, // Queue size tracking would require additional state management
            max_queue_size: self.config.max_queue_size,
            is_at_capacity: available_permits == 0,
        })
    }

    /// Get bulkhead metrics
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn get_metrics(&self) -> Result<BulkheadMetrics>  {
        let metrics = self.metrics.read().await;
        Ok(metrics.clone())
    }

    /// Execute operation with bulkhead protection
    where
        F: std::future::Future<Output = Result<T>>,
    {
        let _permit = self.acquire_permit().await?;
        operation.await
    }

    /// Get utilization percentage
    pub fn get_utilization(&self) -> f64 {
        let available = self.semaphore.available_permits();
        let used = self.config.max_concurrent - available;
        (used as f64 / self.config.max_concurrent as f64) * 100.0
    }

    /// Check if bulkhead is at capacity
    pub fn is_at_capacity(&self) -> bool {
        self.semaphore.available_permits() == 0
    }

    /// Get available permits count
    pub fn available_permits(&self) -> usize {
        self.semaphore.available_permits()
    }

    /// Force add permits (for testing/emergency situations)
    pub fn force_add_permits(&self, permits: usize) {
        self.semaphore.add_permits(permits);
        tracing::warn!(
            "Forcefully added {} permits to bulkhead '{}'",
            permits,
            self.name
        );
    }

    /// Reset bulkhead metrics
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn reset_metrics(&self) -> Result<()>  {
        let mut metrics = self.metrics.write().await;
        *metrics = BulkheadMetrics {
            total_requests: 0,
            successful_acquisitions: 0,
            rejections: 0,
            timeouts: 0,
            current_concurrent: self.config.max_concurrent - self.semaphore.available_permits(),
            max_concurrent_reached: 0,
            average_wait_time: Duration::ZERO,
            total_wait_time: Duration::ZERO,
        };
        tracing::info!("Reset metrics for bulkhead '{}'", self.name);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Duration;

    #[tokio::test]
    async fn test_bulkhead_basic_functionality() {
        let config = BulkheadConfig {
            max_concurrent: 2,
            ..Default::default()
        };

        let bulkhead = Bulkhead::new("test".to_string(), config);

        // Should be able to acquire permits
        let permit1 = bulkhead.acquire_permit().await.expect("Operation failed");
        let permit2 = bulkhead.acquire_permit().await.expect("Operation failed");

        // Should be at capacity
        assert!(bulkhead.is_at_capacity());
        assert_eq!(bulkhead.available_permits(), 0);

        // Try acquire should fail
        assert!(bulkhead.try_acquire_permit().expect("Operation failed").is_none());

        // Drop one permit
        drop(permit1);

        // Should have one permit available
        assert_eq!(bulkhead.available_permits(), 1);
        assert!(!bulkhead.is_at_capacity());
    }

    #[tokio::test]
    async fn test_bulkhead_timeout() {
        let config = BulkheadConfig {
            max_concurrent: 1,
            acquire_timeout: Duration::from_millis(50),
            ..Default::default()
        };

        let bulkhead = Bulkhead::new("test".to_string(), config);

        // Acquire the only permit
        let _permit = bulkhead.acquire_permit().await.expect("Operation failed");

        // This should timeout
        let result = bulkhead.acquire_permit().await;
        assert!(result.is_err());

        let metrics = bulkhead.get_metrics().await.expect("Operation failed");
        assert_eq!(metrics.timeouts, 1);
    }

    #[tokio::test]
    async fn test_bulkhead_execute_with_protection() {
        let config = BulkheadConfig {
            max_concurrent: 1,
            ..Default::default()
        };

        let bulkhead = Bulkhead::new("test".to_string(), config);

        let result = bulkhead
            .execute_with_bulkhead(async { Ok("success".to_string()) })
            .await;

        assert!(result.is_ok());
        assert_eq!(result.expect("Operation failed"), "success");
    }
}
