//! # Enhanced Error Handling
//!
//! Advanced error handling patterns and recovery strategies that extend
//! the existing NestGate error system with additional robustness and context.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::sync::atomic::{AtomicU64, Ordering};
use std::fmt;

/// **ERROR SEVERITY LEVELS**
///
/// Hierarchical error severity classification for better error management
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorSeverity {
    /// Trace-level errors (debugging information)
    Trace = 0,
    /// Debug-level errors (development information)
    Debug = 1,
    /// Info-level errors (informational)
    Info = 2,
    /// Warning-level errors (potential issues)
    Warning = 3,
    /// Error-level (recoverable errors)
    Error = 4,
    /// Critical errors (system stability at risk)
    Critical = 5,
    /// Fatal errors (system must shutdown)
    Fatal = 6,
}

impl fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorSeverity::Trace => write!(f, "TRACE"),
            ErrorSeverity::Debug => write!(f, "DEBUG"),
            ErrorSeverity::Info => write!(f, "INFO"),
            ErrorSeverity::Warning => write!(f, "WARNING"),
            ErrorSeverity::Error => write!(f, "ERROR"),
            ErrorSeverity::Critical => write!(f, "CRITICAL"),
            ErrorSeverity::Fatal => write!(f, "FATAL"),
        }
    }
}

/// **ERROR CONTEXT**
///
/// Rich context information for enhanced error diagnostics
#[derive(Debug, Clone)]
pub struct ErrorContext {
    /// Component that generated the error
    pub component: String,
    /// Operation being performed when error occurred
    pub operation: String,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
    /// Timestamp when error occurred
    pub timestamp: Instant,
    /// Error severity level
    pub severity: ErrorSeverity,
    /// Stack trace (if available)
    pub stack_trace: Option<String>,
    /// User-friendly error message
    pub user_message: Option<String>,
}

impl ErrorContext {
    #[must_use]
    pub fn new(component: String, operation: String) -> Self {
        Self {
            component,
            operation,
            metadata: HashMap::new(),
            timestamp: Instant::now(),
            severity: ErrorSeverity::Error,
            stack_trace: None,
            user_message: None,
        }
    }
    
    /// Add metadata to error context
    #[must_use]
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
    
    /// Set error severity
    #[must_use]
    pub fn with_severity(mut self, severity: ErrorSeverity) -> Self {
        self.severity = severity;
        self
    }
    
    /// Set user-friendly message
    #[must_use]
    pub fn with_user_message(mut self, message: String) -> Self {
        self.user_message = Some(message);
        self
    }
    
    /// Set stack trace
    #[must_use]
    pub fn with_stack_trace(mut self, trace: String) -> Self {
        self.stack_trace = Some(trace);
        self
    }
    
    /// Get elapsed time since error occurred
    pub fn elapsed(&self) -> Duration {
        self.timestamp.elapsed()
    }
}

/// **ENHANCED ERROR TYPE**
///
/// Advanced error type with rich context and recovery information
#[derive(Debug, Clone)]
pub struct EnhancedError {
    /// Core error message
    pub message: String,
    /// Error context
    pub context: ErrorContext,
    /// Recovery suggestions
    pub recovery_suggestions: Vec<String>,
    /// Related errors (error chain)
    pub related_errors: Vec<EnhancedError>,
    /// Error code for programmatic handling
    pub error_code: Option<String>,
    /// Whether this error is retryable
    pub retryable: bool,
    /// Maximum retry attempts recommended
    pub max_retries: Option<u32>,
}

impl EnhancedError {
    #[must_use]
    pub fn new(message: String, context: ErrorContext) -> Self {
        Self {
            message,
            context,
            recovery_suggestions: Vec::new(),
            related_errors: Vec::new(),
            error_code: None,
            retryable: false,
            max_retries: None,
        }
    }
    
    /// Create configuration error
    pub fn configuration_error(message: String, component: String) -> Self {
        let context = ErrorContext::new(component, "configuration".to_string())
            .with_severity(ErrorSeverity::Critical)
            .with_user_message("Configuration error - please check your settings".to_string());
        
        Self::new(message, context)
            .with_error_code("CONFIG_ERROR".to_string())
            .with_recovery_suggestion("Check configuration file syntax".to_string())
            .with_recovery_suggestion("Verify all required configuration keys are present".to_string())
    }
    
    /// Create network error
    pub fn network_error(message: String, component: String) -> Self {
        let context = ErrorContext::new(component, "network_operation".to_string())
            .with_severity(ErrorSeverity::Warning)
            .with_user_message("Network connectivity issue - retrying automatically".to_string());
        
        Self::new(message, context)
            .with_error_code("NETWORK_ERROR".to_string())
            .retryable()
            .with_max_retries(5)
            .with_recovery_suggestion("Check network connectivity".to_string())
            .with_recovery_suggestion("Verify firewall settings".to_string())
            .with_recovery_suggestion("Try again in a few moments".to_string())
    }
    
    /// Create storage error
    pub fn storage_error(message: String, component: String) -> Self {
        let context = ErrorContext::new(component, "storage_operation".to_string())
            .with_severity(ErrorSeverity::Error)
            .with_user_message("Storage operation failed - data may be temporarily unavailable".to_string());
        
        Self::new(message, context)
            .with_error_code("STORAGE_ERROR".to_string())
            .retryable()
            .with_max_retries(3)
            .with_recovery_suggestion("Check disk space availability".to_string())
            .with_recovery_suggestion("Verify storage permissions".to_string())
    }
    
    /// Create system error
    pub fn system_error(message: String, component: String) -> Self {
        let context = ErrorContext::new(component, "system_operation".to_string())
            .with_severity(ErrorSeverity::Critical)
            .with_user_message("System error occurred - please contact support if issue persists".to_string());
        
        Self::new(message, context)
            .with_error_code("SYSTEM_ERROR".to_string())
            .with_recovery_suggestion("Restart the application".to_string())
            .with_recovery_suggestion("Check system resources".to_string())
    }
    
    /// Add recovery suggestion
    #[must_use]
    pub fn with_recovery_suggestion(mut self, suggestion: String) -> Self {
        self.recovery_suggestions.push(suggestion);
        self
    }
    
    /// Set error code
    #[must_use]
    pub fn with_error_code(mut self, code: String) -> Self {
        self.error_code = Some(code);
        self
    }
    
    /// Mark error as retryable
    #[must_use]
    pub fn retryable(mut self) -> Self {
        self.retryable = true;
        self
    }
    
    /// Set maximum retry attempts
    #[must_use]
    pub fn with_max_retries(mut self, retries: u32) -> Self {
        self.max_retries = Some(retries);
        self
    }
    
    /// Add related error to chain
    #[must_use]
    pub fn with_related_error(mut self, error: EnhancedError) -> Self {
        self.related_errors.push(error);
        self
    }
    
    /// Check if error should be retried
    pub fn should_retry(&self, attempt: u32) -> bool {
        self.retryable && self.max_retries.map_or(true, |max| attempt < max)
    }
    
    /// Get user-friendly message
    pub fn user_message(&self) -> &str {
        self.context.user_message.as_deref().unwrap_or(&self.message)
    }
}

impl fmt::Display for EnhancedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}: {} ({}:{})", 
               self.context.severity,
               self.error_code.as_deref().unwrap_or("UNKNOWN"),
               self.message,
               self.context.component,
               self.context.operation)
    }
}

impl std::error::Error for EnhancedError {}

/// **CIRCUIT BREAKER**
///
/// Circuit breaker pattern for handling cascading failures
pub struct CircuitBreaker {
    failure_threshold: u32,
    recovery_timeout: Duration,
    failure_count: AtomicU64,
    last_failure_time: std::sync::Mutex<Option<Instant>>,
    state: std::sync::Mutex<CircuitState>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum CircuitState {
    Closed,   // Normal operation
    Open,     // Circuit is open, failing fast
    HalfOpen, // Testing if service has recovered
}

impl CircuitBreaker {
    pub fn new(failure_threshold: u32, recovery_timeout: Duration) -> Self {
        Self {
            failure_threshold,
            recovery_timeout,
            failure_count: AtomicU64::new(0),
            last_failure_time: std::sync::Mutex::new(None),
            state: std::sync::Mutex::new(CircuitState::Closed),
        }
    }
    
    /// Execute operation with circuit breaker protection
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn execute<T, F>(&self, operation: F) -> Result<T, EnhancedError>
    where
        F: FnOnce() -> Result<T, EnhancedError>,
     {
        // Check circuit state
        let state = match self.state.lock() {
            Ok(guard) => *guard,
            Err(_) => {
                tracing::error!("Circuit breaker state mutex poisoned");
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Circuit breaker state corrupted"
                )));
            }
        };
        
        match state {
            CircuitState::Open => {
                // Check if we should try half-open
                if let Some(last_failure) = match self.last_failure_time.lock() {
                    Ok(guard) => *guard,
                    Err(_) => {
                        tracing::error!("Circuit breaker last_failure_time mutex poisoned");
                        return Err(Box::new(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "Circuit breaker timing corrupted"
                        )));
                    }
                } {
                    if last_failure.elapsed() >= self.recovery_timeout {
                        *self.state.lock().unwrap_or_else(|poisoned| {
                            // Mutex was poisoned, but we can recover by accessing the underlying data
                            poisoned.into_inner()
                        }) = CircuitState::HalfOpen;
                    } else {
                        return Err(EnhancedError::system_error(
                            "Circuit breaker is open".to_string(),
                            "circuit_breaker".to_string(),
                        ));
                    }
                }
            }
            CircuitState::Closed | CircuitState::HalfOpen => {
                // Proceed with operation
            }
        }
        
        // Execute operation
        match operation() {
            Ok(result) => {
                // Success - reset circuit breaker
                self.failure_count.store(0, Ordering::Relaxed);
                *self.state.lock().unwrap_or_else(|poisoned| {
                    // Mutex was poisoned, but we can recover by accessing the underlying data
                    poisoned.into_inner()
                }) = CircuitState::Closed;
                Ok(result)
            }
            Err(error) => {
                // Failure - update circuit breaker
                let failures = self.failure_count.fetch_add(1, Ordering::Relaxed) + 1;
                *self.last_failure_time.lock().unwrap_or_else(|poisoned| {
                    // Mutex was poisoned, but we can recover by accessing the underlying data
                    poisoned.into_inner()
                }) = Some(Instant::now());
                
                if failures >= self.failure_threshold as u64 {
                    *self.state.lock().unwrap_or_else(|poisoned| {
                        // Mutex was poisoned, but we can recover by accessing the underlying data
                        poisoned.into_inner()
                    }) = CircuitState::Open;
                }
                
                Err(error)
            }
        }
    }
    
    /// Get current circuit breaker state
    pub fn state(&self) -> CircuitState {
        *self.state.lock().expect("Failed to acquire lock")
    }
    
    /// Get current failure count
    pub fn failure_count(&self) -> u64 {
        self.failure_count.load(Ordering::Relaxed)
    }
}

/// **RETRY STRATEGY**
///
/// Configurable retry strategy with backoff and jitter
pub struct RetryStrategy {
    max_attempts: u32,
    base_delay: Duration,
    max_delay: Duration,
    backoff_multiplier: f64,
    jitter: bool,
}

impl RetryStrategy {
    pub fn new() -> Self {
        Self {
            max_attempts: 3,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(60),
            backoff_multiplier: 2.0,
            jitter: true,
        }
    }
    
    /// Set maximum retry attempts
    #[must_use]
    pub fn with_max_attempts(mut self, attempts: u32) -> Self {
        self.max_attempts = attempts;
        self
    }
    
    /// Set base delay between retries
    #[must_use]
    pub fn with_base_delay(mut self, delay: Duration) -> Self {
        self.base_delay = delay;
        self
    }
    
    /// Set maximum delay between retries
    #[must_use]
    pub fn with_max_delay(mut self, delay: Duration) -> Self {
        self.max_delay = delay;
        self
    }
    
    /// Set backoff multiplier
    #[must_use]
    pub fn with_backoff_multiplier(mut self, multiplier: f64) -> Self {
        self.backoff_multiplier = multiplier;
        self
    }
    
    /// Enable or disable jitter
    #[must_use]
    pub fn with_jitter(mut self, jitter: bool) -> Self {
        self.jitter = jitter;
        self
    }
    
    /// Execute operation with retry strategy
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn execute<T, F, Fut>(&self, mut operation: F) -> Result<T, EnhancedError>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T, EnhancedError>>,
     {
        let mut last_error = None;
        
        for attempt in 1..=self.max_attempts {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(error) => {
                    if !error.should_retry(attempt) {
                        return Err(error);
                    }
                    
                    last_error = Some(error);
                    
                    // Don't delay after the last attempt
                    if attempt < self.max_attempts {
                        let delay = self.calculate_delay(attempt);
                        tokio::time::sleep(delay).await;
                    }
                }
            }
        }
        
        Err(last_error.unwrap_or_else(|| {
            EnhancedError::system_error(
                "All retry attempts exhausted".to_string(),
                "retry_strategy".to_string(),
            )
        }))
    }
    
    fn calculate_delay(&self, attempt: u32) -> Duration {
        let delay_ms = (self.base_delay.as_millis() as f64) 
            * self.backoff_multiplier.powi((attempt - 1) as i32);
        
        let delay_ms = delay_ms.min(self.max_delay.as_millis() as f64);
        
        let delay_ms = if self.jitter {
            // Add ±25% jitter
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let jitter_factor = rng.gen_range(0.75..=1.25);
            delay_ms * jitter_factor
        } else {
            delay_ms
        };
        
        Duration::from_millis(delay_ms as u64)
    }
}

impl Default for RetryStrategy {
    fn default() -> Self {
        Self::new()
    }
}

/// **ERROR AGGREGATOR**
///
/// Collects and aggregates errors for batch processing and analysis
pub struct ErrorAggregator {
    errors: std::sync::Mutex<Vec<EnhancedError>>,
    max_errors: usize,
    stats: ErrorStats,
}

#[derive(Debug, Default)]
pub struct ErrorStats {
    pub total_errors: AtomicU64,
    pub errors_by_severity: HashMap<ErrorSeverity, AtomicU64>,
    pub errors_by_component: std::sync::Mutex<HashMap<String, AtomicU64>>,
}

impl ErrorAggregator {
    #[must_use]
    pub fn new(max_errors: usize) -> Self {
        let mut errors_by_severity = HashMap::new();
        errors_by_severity.insert(ErrorSeverity::Trace, AtomicU64::new(0));
        errors_by_severity.insert(ErrorSeverity::Debug, AtomicU64::new(0));
        errors_by_severity.insert(ErrorSeverity::Info, AtomicU64::new(0));
        errors_by_severity.insert(ErrorSeverity::Warning, AtomicU64::new(0));
        errors_by_severity.insert(ErrorSeverity::Error, AtomicU64::new(0));
        errors_by_severity.insert(ErrorSeverity::Critical, AtomicU64::new(0));
        errors_by_severity.insert(ErrorSeverity::Fatal, AtomicU64::new(0));
        
        Self {
            errors: std::sync::Mutex::new(Vec::new()),
            max_errors,
            stats: ErrorStats {
                total_errors: AtomicU64::new(0),
                errors_by_severity,
                errors_by_component: std::sync::Mutex::new(HashMap::new()),
            },
        }
    }
    
    /// Add error to aggregator
    pub fn add_error(&self, error: EnhancedError) {
        // Update statistics
        self.stats.total_errors.fetch_add(1, Ordering::Relaxed);
        
        if let Some(counter) = self.stats.errors_by_severity.get(&error.context.severity) {
            counter.fetch_add(1, Ordering::Relaxed);
        }
        
        {
            let mut component_stats = self.stats.errors_by_component.lock().expect("Failed to acquire lock");
            component_stats.entry(error.context.component.clone())
                .or_insert_with(|| AtomicU64::new(0))
                .fetch_add(1, Ordering::Relaxed);
        }
        
        // Store error
        let mut errors = self.errors.lock().expect("Failed to acquire lock");
        errors.push(error);
        
        // Trim if over max capacity
        if errors.len() > self.max_errors {
            errors.remove(0);
        }
    }
    
    /// Get all errors
    pub fn get_errors(&self) -> Vec<EnhancedError> {
        self.errors.lock().expect("Failed to acquire lock").clone()
    }
    
    /// Get errors by severity
    pub fn get_errors_by_severity(&self, severity: ErrorSeverity) -> Vec<EnhancedError> {
        self.errors.lock().expect("Failed to acquire lock")
            .iter()
            .filter(|e| e.context.severity == severity)
            .cloned()
            .collect()
    }
    
    /// Get error statistics
    pub fn get_stats(&self) -> &ErrorStats {
        &self.stats
    }
    
    /// Clear all errors
    pub fn clear(&self) {
        self.errors.lock().expect("Failed to acquire lock").clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_context_creation() {
        let context = ErrorContext::new("test_component".to_string(), "test_operation".to_string())
            .with_metadata("key".to_string(), "value".to_string())
            .with_severity(ErrorSeverity::Warning)
            .with_user_message("User friendly message".to_string());
        
        assert_eq!(context.component, "test_component");
        assert_eq!(context.operation, "test_operation");
        assert_eq!(context.severity, ErrorSeverity::Warning);
        assert_eq!(context.metadata.get("key"), Some(&"value".to_string()));
        assert_eq!(context.user_message, Some("User friendly message".to_string()));
    }
    
    #[test]
    fn test_enhanced_error_creation() {
        let error = EnhancedError::network_error(
            "Connection failed".to_string(),
            "http_client".to_string(),
        );
        
        assert_eq!(error.message, "Connection failed");
        assert_eq!(error.context.component, "http_client");
        assert_eq!(error.error_code, Some("NETWORK_ERROR".to_string()));
        assert!(error.retryable);
        assert_eq!(error.max_retries, Some(5));
        assert!(!error.recovery_suggestions.is_empty());
    }
    
    #[test]
    fn test_circuit_breaker() {
        let circuit_breaker = CircuitBreaker::new(2, Duration::from_millis(100));
        
        // First failure
        let result1 = circuit_breaker.execute(|| {
            Err(EnhancedError::system_error("Test error".to_string(), "test".to_string()))
        });
        assert!(result1.is_err());
        assert_eq!(circuit_breaker.state(), CircuitState::Closed);
        
        // Second failure - should open circuit
        let result2 = circuit_breaker.execute(|| {
            Err(EnhancedError::system_error("Test error".to_string(), "test".to_string()))
        });
        assert!(result2.is_err());
        assert_eq!(circuit_breaker.state(), CircuitState::Open);
        
        // Third attempt - should fail fast
        let result3 = circuit_breaker.execute(|| {
            Ok("success")
        });
        assert!(result3.is_err());
    }
    
    #[test]
    fn test_error_aggregator() {
        let aggregator = ErrorAggregator::new(10);
        
        let error1 = EnhancedError::network_error("Error 1".to_string(), "component1".to_string());
        let error2 = EnhancedError::configuration_error("Error 2".to_string(), "component2".to_string());
        
        aggregator.add_error(error1);
        aggregator.add_error(error2);
        
        assert_eq!(aggregator.get_stats().total_errors.load(Ordering::Relaxed), 2);
        assert_eq!(aggregator.get_errors().len(), 2);
        
        let warning_errors = aggregator.get_errors_by_severity(ErrorSeverity::Warning);
        assert_eq!(warning_errors.len(), 1);
    }
    
    #[tokio::test]
    async fn test_retry_strategy() {
        let strategy = RetryStrategy::new()
            .with_max_attempts(3)
            .with_base_delay(Duration::from_millis(10));
        
        let mut attempt_count = 0;
        let result = strategy.execute(|| {
            attempt_count += 1;
            async move {
                if attempt_count < 3 {
                    Err(EnhancedError::network_error("Retry test".to_string(), "test".to_string()))
                } else {
                    Ok("success")
                }
            }
        }).await;
        
        assert!(result.is_ok());
        assert_eq!(attempt_count, 3);
    }
} 