// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Network module

pub mod client;
pub mod native_async;
/// Native async network implementation for high-performance networking
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
mod error_path_tests_comprehensive; // Dec 10, 2025 - Comprehensive error path coverage
#[cfg(test)]
mod network_edge_cases;
#[cfg(test)]
mod network_error_path_tests; // Nov 23, 2025 - P1 test expansion // Nov 23, 2025 - P1-5 edge case tests
