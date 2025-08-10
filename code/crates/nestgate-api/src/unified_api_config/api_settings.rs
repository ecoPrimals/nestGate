//! **UNIFIED API CONFIGURATION - SETTINGS MODULE**
//!
//! Contains all individual API configuration setting structs.
//! This file has been restructured into focused sub-modules for better maintainability
//! and adherence to the 1000-line file size limit.
//!
//! Originally 902 lines, now split into focused modules under `settings/`.

// Re-export all settings from the organized sub-modules
pub use settings::*;

// Declare the settings sub-module
pub mod settings;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_server_settings_default() {
        let settings = ApiHttpServerSettings::default();
        assert!(settings.enable_cors);
        assert_eq!(settings.max_concurrent_connections, 1000);
        assert!(settings.enable_compression);
        assert!(settings.enable_http2);
    }

    #[test]
    fn test_streaming_settings_default() {
        let settings = ApiStreamingSettings::default();
        assert!(settings.enable_mcp_streaming);
        assert_eq!(settings.max_concurrent_streams, 100);
        assert_eq!(settings.stream_buffer_size, 64 * 1024);
    }

    #[test]
    fn test_performance_settings_default() {
        let settings = ApiPerformanceSettings::default();
        assert!(settings.enable_request_caching);
        assert!(settings.enable_response_compression);
        assert!(settings.enable_connection_pooling);
        assert_eq!(settings.connection_pool_size, 100);
    }

    #[test]
    fn test_circuit_breaker_settings_default() {
        let settings = ApiCircuitBreakerSettings::default();
        assert!(settings.enable_circuit_breaker);
        assert_eq!(settings.failure_threshold, 5);
        assert_eq!(settings.success_threshold, 3);
        assert!(settings.enable_exponential_backoff);
    }

    #[test]
    fn test_auth_settings_default() {
        let settings = ApiAuthSettings::default();
        assert!(settings.enable_auth);
        assert_eq!(settings.auth_provider, "jwt");
        assert!(settings.enable_refresh_tokens);
        assert!(settings.enable_api_key_auth);
    }

    #[test]
    fn test_all_settings_can_be_created() {
        // Test that all settings types can be instantiated
        let _http = ApiHttpServerSettings::default();
        let _streaming = ApiStreamingSettings::default();
        let _sse = ApiSseSettings::default();
        let _performance = ApiPerformanceSettings::default();
        let _circuit_breaker = ApiCircuitBreakerSettings::default();
        let _retry = ApiRetryPolicySettings::default();
        let _pool = ApiConnectionPoolSettings::default();
        let _mesh = ApiServiceMeshSettings::default();
        let _primal = ApiPrimalSettings::default();
        let _auth = ApiAuthSettings::default();
        let _health = ApiHealthSettings::default();
        let _storage = ApiStorageSettings::default();
        let _stream_retry = ApiStreamRetrySettings::default();
        let _discovery = ApiServiceDiscoverySettings::default();
        let _rpc = ApiRpcTimeoutSettings::default();
        
        // If we get here, all settings types are properly defined
        assert!(true);
    }
}
