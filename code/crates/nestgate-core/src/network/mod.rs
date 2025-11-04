pub mod client;
pub mod native_async;
pub mod native_async_network;

// Re-export commonly used types for convenience
pub use client::{
    Endpoint, HeaderMap, Method, Port, Request, RequestBody, Response, Scheme, StatusCode,
    TimeoutMs,
};

#[cfg(test)]
mod client_tests;
