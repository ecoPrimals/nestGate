//! Network module

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
pub use traits::{HealthStatus, Metrics, NetworkTraitsConfig as TraitsConfig, Service};

#[cfg(test)]
mod client_tests;

#[cfg(test)]
mod network_edge_cases;
#[cfg(test)]
mod network_error_path_tests; // Nov 23, 2025 - P1 test expansion // Nov 23, 2025 - P1-5 edge case tests
