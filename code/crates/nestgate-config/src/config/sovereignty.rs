// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// **SOVEREIGNTY CONFIGURATION SYSTEM**
//
// This module provides environment-driven configuration that eliminates hardcoded
// infrastructure assumptions, ensuring users maintain full control over their systems.

use super::sovereignty_config::SovereigntyRuntimeConfig;

/// Sovereignty-compliant configuration helpers
///
/// IMPORTANT: This module now delegates to `SovereigntyRuntimeConfig` for all
/// environment variable lookups. All methods use `SovereigntyRuntimeConfig::from_env()`
/// to eliminate direct env::var calls from production code.
pub struct SovereigntyConfig;

impl SovereigntyConfig {
    /// Get API endpoint from environment or default
    pub fn api_endpoint() -> String {
        SovereigntyRuntimeConfig::from_env().api_endpoint()
    }

    /// Get API port from environment or default
    pub fn api_port() -> u16 {
        SovereigntyRuntimeConfig::from_env().api_port()
    }

    /// Get bind address from environment or default
    pub fn bind_address() -> String {
        SovereigntyRuntimeConfig::from_env().bind_address()
    }

    /// Get WebSocket endpoint from environment or default
    pub fn websocket_endpoint() -> String {
        SovereigntyRuntimeConfig::from_env().websocket_endpoint()
    }

    /// Get discovery endpoint from environment or default
    pub fn discovery_endpoint() -> String {
        SovereigntyRuntimeConfig::from_env().discovery_endpoint()
    }

    /// Get orchestration endpoint from environment or default
    pub fn orchestration_endpoint() -> String {
        SovereigntyRuntimeConfig::from_env().orchestration_endpoint()
    }

    /// Get test endpoint for development/testing
    pub fn test_endpoint() -> String {
        SovereigntyRuntimeConfig::from_env().test_endpoint()
    }
}

/// Migration helpers for replacing hardcoded values
pub mod migration_helpers {
    use super::SovereigntyConfig;

    /// Replace hardcoded localhost:8080 with environment-driven value
    pub fn replace_localhost_8080() -> String {
        SovereigntyConfig::api_endpoint()
    }

    /// Replace hardcoded 127.0.0.1:8080 with environment-driven value
    pub fn replace_127_0_0_1_8080() -> String {
        format!(
            "{}:{}",
            SovereigntyConfig::bind_address(),
            SovereigntyConfig::api_port()
        )
    }

    /// Replace hardcoded ws://localhost:8080 with environment-driven value
    pub fn replace_ws_localhost_8080() -> String {
        SovereigntyConfig::websocket_endpoint()
    }

    /// Get environment-driven port string
    pub fn api_port_string() -> String {
        SovereigntyConfig::api_port().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sovereignty_config_api_endpoint() {
        let endpoint = SovereigntyConfig::api_endpoint();
        assert!(endpoint.starts_with("http://"));
        assert!(endpoint.contains(":"));
    }

    #[test]
    fn test_sovereignty_config_api_port() {
        let port = SovereigntyConfig::api_port();
        assert!(port > 0);
        // Port is u16, always <= 65535
    }

    #[test]
    fn test_sovereignty_config_bind_address() {
        let address = SovereigntyConfig::bind_address();
        assert!(!address.is_empty());
    }

    #[test]
    fn test_sovereignty_config_websocket_endpoint() {
        let endpoint = SovereigntyConfig::websocket_endpoint();
        assert!(endpoint.starts_with("ws://"));
    }

    #[test]
    fn test_sovereignty_config_discovery_endpoint() {
        let endpoint = SovereigntyConfig::discovery_endpoint();
        assert!(endpoint.contains("/discovery"));
    }

    #[test]
    fn test_sovereignty_config_orchestration_endpoint() {
        let endpoint = SovereigntyConfig::orchestration_endpoint();
        assert!(endpoint.starts_with("http://"));
    }

    #[test]
    fn test_sovereignty_config_test_endpoint() {
        let endpoint = SovereigntyConfig::test_endpoint();
        assert!(endpoint.starts_with("http://"));
    }

    #[test]
    fn test_migration_helpers_replace_localhost_8080() {
        let endpoint = migration_helpers::replace_localhost_8080();
        assert!(endpoint.starts_with("http://"));
    }

    #[test]
    fn test_migration_helpers_replace_127_0_0_1_8080() {
        let endpoint = migration_helpers::replace_127_0_0_1_8080();
        assert!(endpoint.contains(":"));
    }

    #[test]
    fn test_migration_helpers_replace_ws_localhost_8080() {
        let endpoint = migration_helpers::replace_ws_localhost_8080();
        assert!(endpoint.starts_with("ws://"));
    }

    #[test]
    fn test_migration_helpers_api_port_string() {
        let port_str = migration_helpers::api_port_string();
        assert!(port_str.parse::<u16>().is_ok());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 8)]
    async fn test_concurrent_sovereignty_config_access() {
        let handles: Vec<_> = (0..100)
            .map(|_| {
                tokio::spawn(async move {
                    let _ = SovereigntyConfig::api_endpoint();
                    let _ = SovereigntyConfig::api_port();
                    let _ = SovereigntyConfig::bind_address();
                    let _ = SovereigntyConfig::websocket_endpoint();
                    let _ = SovereigntyConfig::discovery_endpoint();
                    let _ = SovereigntyConfig::orchestration_endpoint();
                    let _ = SovereigntyConfig::test_endpoint();
                })
            })
            .collect();

        for handle in handles {
            handle.await.expect("Task should complete successfully");
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 8)]
    async fn test_concurrent_migration_helpers() {
        let handles: Vec<_> = (0..100)
            .map(|_| {
                tokio::spawn(async move {
                    let _ = migration_helpers::replace_localhost_8080();
                    let _ = migration_helpers::replace_127_0_0_1_8080();
                    let _ = migration_helpers::replace_ws_localhost_8080();
                    let _ = migration_helpers::api_port_string();
                })
            })
            .collect();

        for handle in handles {
            handle.await.expect("Task should complete successfully");
        }
    }
}
