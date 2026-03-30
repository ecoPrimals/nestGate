// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]

//! **NETWORK HANDLERS AND PROTOCOLS**
//!
//! This module provides protocol handlers and network management functionality,
//! including connection handling, service discovery, and load balancing.

mod load_balancer;
mod manager;
mod protocols;
mod service_trait;

#[cfg(test)]
mod tests;

pub use load_balancer::{LoadBalancer, LoadBalancingStrategy};
pub use manager::NetworkServiceManager;
pub use protocols::{HttpProtocolHandler, HttpRequest, HttpResponse, TcpProtocolHandler};
pub use service_trait::NetworkService;
