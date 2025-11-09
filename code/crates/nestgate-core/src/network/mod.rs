pub mod client;
pub mod native_async;
pub mod native_async_network;
pub mod traits; // Canonical network service traits

// Re-export commonly used types for convenience
pub use client::{
    Endpoint, HeaderMap, Method, Port, Request, RequestBody, Response, Scheme, StatusCode,
    TimeoutMs,
};

// Re-export canonical Service trait for module-wide use
pub use traits::{Service, HealthStatus, Metrics, NetworkTraitsConfig as TraitsConfig};

#[cfg(test)]
mod client_tests;
