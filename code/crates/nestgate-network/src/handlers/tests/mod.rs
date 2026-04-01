// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

mod load_balancer;
mod manager;
mod protocols;
mod trait_smoke;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use crate::types::{ConnectionInfo, NetworkConfig, ServiceInfo};

/// Helper to create test network config
pub(super) fn create_test_config() -> NetworkConfig {
    NetworkConfig::default()
}

/// Helper to create test connection info
pub(super) fn create_test_connection(id: &str, _active: bool) -> ConnectionInfo {
    use nestgate_core::constants::hardcoding::runtime_fallback_ports;
    ConnectionInfo::new(
        id.to_string(),
        SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            runtime_fallback_ports::HTTP,
        ),
    )
}

/// Helper to create test service info
pub(super) fn create_test_service(id: &str, name: &str, _healthy: bool) -> ServiceInfo {
    ServiceInfo::new(
        id.to_string(),
        name.to_string(),
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9000),
    )
}
