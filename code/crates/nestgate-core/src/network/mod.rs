pub mod client;
pub mod native_async;
pub mod native_async_network;

// Re-export commonly used types for convenience
pub use client::{
    Port, TimeoutMs, Method, StatusCode, Endpoint, Scheme,
    Request, RequestBody, Response, HeaderMap,
};

#[cfg(test)]
mod client_tests;
