//! Logging Module - Simplified Wrapper Around `tracing`
//!
//! **ARCHITECTURE NOTE**: NestGate uses `tracing` crate directly for logging.
//! We are a **storage primal**, not a logging infrastructure provider.
//!
//! # Deleted Stubs
//!
//! **Previously had these stub implementations** (DELETED):
//! - `aggregator.rs` - Log aggregation (not our domain)
//! - `storage.rs` - Log storage (not our domain)
//! - `search.rs` - Log search (not our domain)
//! - `analysis.rs` - Log analysis (not our domain)
//! - `ingestion.rs` - Log ingestion (not our domain)
//! - `destinations.rs` - Log routing (not our domain)
//! - `alerts.rs` - Log alerting (not our domain)
//!
//! # Modern Approach
//!
//! ```rust
//! // Just use `tracing` directly
//! use tracing::{debug, error, info, trace, warn};
//!
//! info!("Dataset created: {}", name);
//! error!("Failed to create snapshot: {}", err);
//! debug!("Cache hit for key: {}", key);
//! ```
//!
//! For structured logging:
//! ```rust
//! use tracing::{info, instrument};
//!
//! #[instrument]
//! async fn create_dataset(name: &str) -> Result<Dataset> {
//!     info!(name = %name, "Creating dataset");
//!     // Implementation...
//! }
//! ```
//!
//! # Log Aggregation
//!
//! For production deployments, use external tools:
//! - **Grafana Loki** - Log aggregation and search
//! - **ELK Stack** - Elasticsearch, Logstash, Kibana
//! - **Promtail** - Log shipping
//! - **Fluentd** - Log collection and forwarding
//!
//! NestGate emits logs in structured format (JSON) that these tools can consume.

// Re-export tracing macros for convenience
pub use tracing::{debug, error, info, trace, warn, instrument, event, span, Level};

// Configuration and types (keep these)
pub mod config;
pub mod error;
pub mod traits;
pub mod types;

/// Initialize tracing subscriber for NestGate
///
/// This sets up structured logging with JSON formatting for production.
pub fn init() {
    use tracing_subscriber::{fmt, EnvFilter};
    
    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info"))
        )
        .json()
        .init();
}

/// Initialize tracing subscriber for development
///
/// This uses human-readable formatting for local development.
pub fn init_dev() {
    use tracing_subscriber::{fmt, EnvFilter};
    
    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("debug"))
        )
        .pretty()
        .init();
}

/// Initialize tracing subscriber for testing
///
/// This captures logs for test assertions.
#[cfg(test)]
pub fn init_test() {
    use tracing_subscriber::{fmt, EnvFilter};
    
    let _ = fmt()
        .with_env_filter(EnvFilter::new("trace"))
        .with_test_writer()
        .try_init();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_logging_macros_work() {
        init_test();
        
        info!("Test log message");
        debug!("Debug message with data: {}", 42);
        warn!("Warning message");
        error!("Error message");
    }
    
    #[test]
    fn test_structured_logging() {
        init_test();
        
        let dataset_name = "test-dataset";
        let size_bytes = 1024u64;
        
        info!(
            dataset = %dataset_name,
            size_bytes = size_bytes,
            "Created dataset"
        );
    }
}
