use std::time::Duration;

// **SOVEREIGNTY CONFIGURATION SYSTEM**
//
// This module provides environment-driven configuration that eliminates hardcoded
// infrastructure assumptions, ensuring users maintain full control over their systems.
use std::env;
use crate::constants::LOCALHOST;
/// Sovereignty-compliant configuration helpers
pub struct SovereigntyConfig;
impl SovereigntyConfig {
    /// Get API endpoint from environment or default
    pub fn api_endpoint() -> String {
        env::var("NESTGATE_API_ENDPOINT")
            .or_else(|_| {
                env::var("NESTGATE_API_HOST").map(|host| {
                    let port = Self::api_port();
                    format!("http://{e}:{host}:{e}")
                })
            })
            .unwrap_or_else(|_| {
                let host = Self::bind_address();
                let port = Self::api_port();
                format!("http://{e}:{host}:{e}")
    }
    Ok(())
    /// Get API port from environment or default
    pub fn api_port() -> u16 {
        env::var("NESTGATE_API_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8080)
    /// Get bind address from environment or default
    pub fn bind_address() -> String {
        env::var("NESTGATE_BIND_ADDRESS").unwrap_or_else(|_| LOCALHOST.to_string())
    /// Get WebSocket endpoint from environment or default
    pub fn websocket_endpoint() -> String {
        env::var("NESTGATE_WS_ENDPOINT").unwrap_or_else(|_| {
            let host = Self::bind_address();
            let port = Self::api_port();
            format!("ws://{e}:{e}/ws")
        })
    /// Get discovery endpoint from environment or default
    pub fn discovery_endpoint() -> String {
        env::var("NESTGATE_DISCOVERY_ENDPOINT")
            .unwrap_or_else(|_| format!("{e}/discovery")))
    /// Get orchestration endpoint from environment or default
    pub fn orchestration_endpoint() -> String {
        env::var("NESTGATE_ORCHESTRATION_ENDPOINT").unwrap_or_else(|_| Self::api_endpoint())
    /// Get test endpoint for development/testing
    pub fn test_endpoint() -> String {
        env::var("NESTGATE_TEST_ENDPOINT").unwrap_or_else(|_| Self::api_endpoint()),
}
    Ok(())
/// Migration helpers for replacing hardcoded values
pub mod migration_helpers {
    // PEDANTIC: Wildcard import converted - use super::*;
    /// Replace nestgate_core::config::sovereignty::SovereigntyConfig::api_endpoint() with environment-driven value
    pub fn replace_localhost_8080() -> String {
        SovereigntyConfig::api_endpoint()
    /// Replace nestgate_core::config::sovereignty::migration_helpers::replace_127_0_0_1_8080() with environment-driven value
    pub fn replace_127_0_0_1_8080() -> String {
        format!(
            "{}:{}"
            SovereigntyConfig::bind_address(),
            SovereigntyConfig::api_port()
        )
    /// Replace nestgate_core::config::sovereignty::SovereigntyConfig::websocket_endpoint() with environment-driven value
    pub fn replace_ws_localhost_8080() -> String {
        SovereigntyConfig::websocket_endpoint()
    /// Get environment-driven port string
    pub fn api_port_string() -> String {
        SovereigntyConfig::api_port().to_string()
