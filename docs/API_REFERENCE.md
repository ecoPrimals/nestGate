# NestGate API Reference

## Overview

NestGate is a comprehensive, production-ready system for distributed storage, networking, and service management. This document provides detailed API reference for all public interfaces.

## Table of Contents

- [Core Modules](#core-modules)
- [Error Handling](#error-handling)
- [Memory Management](#memory-management)
- [Zero-Copy Operations](#zero-copy-operations)
- [Observability](#observability)
- [Configuration](#configuration)
- [Networking](#networking)
- [Storage](#storage)

## Core Modules

### `nestgate_core`

The core module provides fundamental types and utilities used throughout the system.

#### Memory Optimization

```rust
use nestgate_core::memory_optimization::{
    MemoryStats, ObjectPool, WeakCache, MemoryArena, MemoryProfiler
};

// Track memory usage
let stats = MemoryStats::new();
stats.record_allocation(1024);

// Use object pooling for expensive objects
let pool = ObjectPool::new(|| Vec::<u8>::new(), 100);
let obj = pool.acquire();
pool.release(obj);

// Weak reference cache to prevent memory leaks
let cache = WeakCache::new();
cache.insert("key", Arc::new("value"));

// Arena allocator for bulk allocations
let arena = MemoryArena::new(65536);
let ptr = arena.allocate(1024);
```

#### Enhanced Error Handling

```rust
use nestgate_core::enhanced_error_handling::{
    EnhancedError, ErrorContext, ErrorSeverity, CircuitBreaker, RetryStrategy
};

// Create rich error contexts
let error = EnhancedError::network_error(
    "Connection failed".to_string(),
    "http_client".to_string(),
);

// Circuit breaker for fault tolerance
let breaker = CircuitBreaker::new(5, Duration::from_secs(30));
let result = breaker.execute(|| {
    // Your operation here
    Ok("success")
});

// Configurable retry strategy
let strategy = RetryStrategy::new()
    .with_max_attempts(3)
    .with_base_delay(Duration::from_millis(100));

let result = strategy.execute(|| async {
    // Your async operation here
    Ok("success")
}).await;
```

#### Zero-Copy Enhancements

```rust
use nestgate_core::zero_copy_enhancements::{
    ZeroCopyStringPool, ZeroCopyConfigRegistry, ZeroCopyResponse
};

// String interning for memory efficiency
let mut pool = ZeroCopyStringPool::new();
let shared_string = pool.intern("common_string");

// Configuration sharing without cloning
let mut registry = ZeroCopyConfigRegistry::new();
let config = MyConfig { value: 42 };
let shared_config = registry.register("app_config".to_string(), config);

// Zero-copy HTTP responses
let response = ZeroCopyResponse::builder()
    .status(200)
    .header("Content-Type", "application/json")
    .body(b"Hello, World!")
    .build();
```

### Observability

```rust
use nestgate_core::observability::{
    MetricsRegistry, HealthCheckSystem, TracingSystem, AlertingSystem
};

// Metrics collection
let metrics = MetricsRegistry::new();
let counter = metrics.counter("requests_total");
counter.fetch_add(1, Ordering::Relaxed);

let histogram = metrics.histogram("request_duration");
histogram.lock().unwrap().observe(0.125);

// Health checking
let health = HealthCheckSystem::new();
health.register(DatabaseHealthCheck::new("database".to_string()));
let overall_health = health.check_all();

// Distributed tracing
let tracing = TracingSystem::new();
let span_id = tracing.start_span(
    "trace_123".to_string(),
    "process_request".to_string(),
    None
);
tracing.add_tag(&span_id, "user_id".to_string(), "user_456".to_string());
tracing.finish_span(&span_id);

// Alerting
let alerting = AlertingSystem::new();
let rule = AlertRule {
    name: "high_error_rate".to_string(),
    condition: AlertCondition::MetricThreshold {
        metric_name: "error_rate".to_string(),
        operator: ComparisonOperator::GreaterThan,
        threshold: 0.05,
        duration: Duration::from_secs(300),
    },
    severity: AlertSeverity::Critical,
    description: "Error rate is too high".to_string(),
    enabled: true,
};
alerting.add_rule(rule);
```

## Error Handling

### Error Types

#### `EnhancedError`

A comprehensive error type with rich context and recovery information.

**Fields:**
- `message: String` - Core error message
- `context: ErrorContext` - Rich error context
- `recovery_suggestions: Vec<String>` - Suggested recovery actions
- `error_code: Option<String>` - Programmatic error code
- `retryable: bool` - Whether the error is retryable

**Methods:**
- `configuration_error(message, component)` - Create configuration error
- `network_error(message, component)` - Create network error
- `storage_error(message, component)` - Create storage error
- `system_error(message, component)` - Create system error
- `should_retry(attempt)` - Check if error should be retried

#### `ErrorContext`

Rich context information for enhanced error diagnostics.

**Fields:**
- `component: String` - Component that generated the error
- `operation: String` - Operation being performed
- `metadata: HashMap<String, String>` - Additional metadata
- `severity: ErrorSeverity` - Error severity level
- `timestamp: Instant` - When the error occurred

### Circuit Breaker

Implements the circuit breaker pattern for handling cascading failures.

**Constructor:**
```rust
CircuitBreaker::new(failure_threshold: u32, recovery_timeout: Duration)
```

**Methods:**
- `execute<T, F>(operation: F) -> Result<T, EnhancedError>` - Execute with protection
- `state() -> CircuitState` - Get current state
- `failure_count() -> u64` - Get failure count

### Retry Strategy

Configurable retry strategy with exponential backoff and jitter.

**Methods:**
- `with_max_attempts(attempts)` - Set maximum attempts
- `with_base_delay(delay)` - Set base delay
- `with_backoff_multiplier(multiplier)` - Set backoff multiplier
- `execute<T, F, Fut>(operation) -> Result<T, EnhancedError>` - Execute with retries

## Memory Management

### Memory Statistics

Track memory allocation patterns and detect potential leaks.

**Methods:**
- `record_allocation(size)` - Record an allocation
- `record_deallocation(size)` - Record a deallocation
- `efficiency_ratio() -> f64` - Get efficiency ratio
- `has_potential_leaks() -> bool` - Check for potential leaks

### Object Pool

Generic object pool for reusing expensive-to-create objects.

**Constructor:**
```rust
ObjectPool::new<F>(factory: F, max_size: usize)
```

**Methods:**
- `acquire() -> T` - Get object from pool
- `release(obj: T)` - Return object to pool
- `stats() -> (u64, u64, f64)` - Get pool statistics

### Weak Cache

Cache using weak references to avoid memory leaks.

**Methods:**
- `get(key) -> Option<Arc<V>>` - Get value from cache
- `insert(key, value)` - Insert value into cache
- `cleanup()` - Clean up dead weak references

### Memory Arena

Arena allocator for reducing allocation overhead.

**Constructor:**
```rust
MemoryArena::new(chunk_size: usize)
```

**Methods:**
- `allocate(size) -> Option<*mut u8>` - Allocate memory
- `reset()` - Reset arena
- `allocation_count() -> usize` - Get allocation count

## Zero-Copy Operations

### Zero-Copy String Pool

Shared string pool for memory-efficient string sharing.

**Methods:**
- `intern(s: &str) -> Arc<str>` - Intern string for sharing
- `get_ref(s: &str) -> Option<&Arc<str>>` - Get reference to interned string
- `contains(s: &str) -> bool` - Check if string is interned
- `stats() -> &StringPoolStats` - Get pool statistics

### Zero-Copy Configuration Registry

Registry for zero-copy sharing of configuration objects.

**Methods:**
- `register(key, config) -> Arc<T>` - Register configuration
- `get(key) -> Option<Arc<T>>` - Get configuration reference
- `access_count(key) -> u64` - Get access count

### Zero-Copy Response Builder

Builder for HTTP responses with zero-copy optimizations.

**Methods:**
- `status(status: u16)` - Set response status
- `header(key, value)` - Add header
- `body(body: &[u8])` - Set response body
- `build() -> ZeroCopyResponse` - Build response

## Observability

### Metrics Registry

Central registry for application metrics.

**Methods:**
- `counter(name) -> Arc<AtomicU64>` - Register counter metric
- `gauge(name) -> Arc<AtomicU64>` - Register gauge metric
- `histogram(name) -> Arc<Mutex<Histogram>>` - Register histogram metric
- `export_prometheus() -> String` - Export in Prometheus format
- `summary() -> MetricsSummary` - Get metrics summary

### Health Check System

Comprehensive health checking for system components.

**Methods:**
- `register<T: HealthCheck>(check)` - Register health check
- `check_all() -> OverallHealth` - Run all health checks
- `get_check(name) -> Option<HealthCheckResult>` - Get specific check result
- `is_healthy() -> bool` - Check overall health

### Tracing System

Distributed tracing for request tracking.

**Methods:**
- `start_span(trace_id, operation_name, parent_span_id) -> String` - Start span
- `finish_span(span_id)` - Finish span
- `add_tag(span_id, key, value)` - Add tag to span
- `log(span_id, level, message, fields)` - Add log entry
- `get_span(span_id) -> Option<Span>` - Get span by ID
- `get_trace(trace_id) -> Vec<Span>` - Get all spans for trace

### Alerting System

Rule-based alerting for monitoring thresholds.

**Methods:**
- `add_rule(rule: AlertRule)` - Add alert rule
- `register_handler<T: AlertHandler>(handler)` - Register alert handler
- `evaluate_rules(metrics, health)` - Evaluate all rules
- `get_active_alerts() -> Vec<Alert>` - Get active alerts

## Configuration

### Environment Variables

All configuration can be controlled via environment variables:

#### Network Configuration
- `NESTGATE_API_BASE_URL` - API base URL
- `NESTGATE_BIND_ADDRESS` - Service bind address
- `NESTGATE_API_PORT` - API port
- `NESTGATE_TEST_ENDPOINT` - Test endpoint for benchmarks

#### Database Configuration
- `NESTGATE_DB_HOST` - Database host
- `NESTGATE_DB_PORT` - Database port
- `NESTGATE_DB_NAME` - Database name
- `NESTGATE_DATABASE_URL` - Complete database URL

#### Security Configuration
- `NESTGATE_TLS_ENABLED` - Enable TLS
- `NESTGATE_TLS_CERT_PATH` - TLS certificate path
- `NESTGATE_TLS_KEY_PATH` - TLS private key path
- `NESTGATE_JWT_SECRET` - JWT secret key

#### General Configuration
- `NESTGATE_ENVIRONMENT` - Environment (development, staging, production)
- `NESTGATE_LOG_LEVEL` - Log level (trace, debug, info, warn, error)
- `NESTGATE_METRICS_ENABLED` - Enable metrics collection

## Performance Characteristics

### Zero-Copy Operations
- String interning: O(1) lookup after initial O(1) hash
- Configuration sharing: O(1) Arc clone
- Response building: No copying for borrowed data

### Memory Management
- Object pooling: O(1) acquire/release
- Weak cache: O(1) lookup with automatic cleanup
- Arena allocation: O(1) allocation within chunks

### Error Handling
- Circuit breaker: O(1) state check and update
- Retry strategy: Configurable backoff with jitter
- Error aggregation: O(1) insertion, O(n) analysis

### Observability
- Metrics: Atomic operations for counters/gauges
- Health checks: Parallel execution with timeout
- Tracing: O(1) span operations with HashMap storage
- Alerting: O(n) rule evaluation per check cycle

## Thread Safety

All public APIs are designed to be thread-safe:

- **Metrics**: Use atomic operations and mutexes
- **Memory pools**: Thread-safe with internal locking
- **Caches**: Mutex-protected with atomic statistics
- **Health checks**: Concurrent execution safe
- **Tracing**: Thread-safe span management
- **Error handling**: Immutable error types, thread-safe handlers

## Error Codes

### Configuration Errors
- `CONFIG_ERROR` - General configuration error
- `CONFIG_MISSING_KEY` - Required configuration key missing
- `CONFIG_INVALID_VALUE` - Configuration value invalid

### Network Errors
- `NETWORK_ERROR` - General network error
- `NETWORK_TIMEOUT` - Network operation timeout
- `NETWORK_CONNECTION_FAILED` - Connection establishment failed

### Storage Errors
- `STORAGE_ERROR` - General storage error
- `STORAGE_PERMISSION_DENIED` - Storage permission denied
- `STORAGE_DISK_FULL` - Storage disk full

### System Errors
- `SYSTEM_ERROR` - General system error
- `SYSTEM_RESOURCE_EXHAUSTED` - System resources exhausted
- `SYSTEM_INTERNAL_ERROR` - Internal system error

## Best Practices

### Error Handling
1. Use specific error types for different failure modes
2. Include rich context in error messages
3. Implement circuit breakers for external dependencies
4. Use retry strategies with exponential backoff
5. Aggregate errors for analysis and alerting

### Memory Management
1. Use object pools for frequently allocated objects
2. Prefer weak references for caches to prevent leaks
3. Use arena allocators for bulk allocations
4. Monitor memory usage with statistics
5. Profile allocation patterns regularly

### Zero-Copy Optimization
1. Intern frequently used strings
2. Share configuration objects via Arc
3. Use Cow<'_, T> for optional copying
4. Prefer borrowing over cloning when possible
5. Use memory mapping for large files

### Observability
1. Instrument all critical paths with metrics
2. Implement health checks for all dependencies
3. Use distributed tracing for request correlation
4. Set up alerts for critical thresholds
5. Export metrics in standard formats (Prometheus)

## Examples

### Complete Service Setup

```rust
use nestgate_core::{
    observability::*,
    memory_optimization::*,
    enhanced_error_handling::*,
    zero_copy_enhancements::*,
};

// Initialize observability
let metrics = MetricsRegistry::new();
let health = HealthCheckSystem::new();
let tracing = TracingSystem::new();
let alerting = AlertingSystem::new();

// Setup memory optimization
let memory_stats = MemoryStats::new();
let string_pool = ZeroCopyStringPool::new();
let config_registry = ZeroCopyConfigRegistry::new();

// Configure error handling
let circuit_breaker = CircuitBreaker::new(5, Duration::from_secs(30));
let retry_strategy = RetryStrategy::new()
    .with_max_attempts(3)
    .with_base_delay(Duration::from_millis(100));

// Register health checks
health.register(DatabaseHealthCheck::new("database".to_string()));

// Setup alerting rules
let rule = AlertRule {
    name: "high_error_rate".to_string(),
    condition: AlertCondition::MetricThreshold {
        metric_name: "error_rate".to_string(),
        operator: ComparisonOperator::GreaterThan,
        threshold: 0.05,
        duration: Duration::from_secs(300),
    },
    severity: AlertSeverity::Critical,
    description: "Error rate is too high".to_string(),
    enabled: true,
};
alerting.add_rule(rule);

// Your application logic here...
```

## Version Information

- **API Version**: 1.0.0
- **Rust Version**: 1.70+
- **Stability**: Production Ready

## Support

For questions, issues, or contributions, please refer to the project repository and documentation. 