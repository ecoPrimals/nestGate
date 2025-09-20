//! **CANONICAL DEFAULTS**
//!
//! Default values for canonical configuration and network settings.

pub mod network {
    //! Network-related canonical defaults

    /// Default API base URL
    pub const DEFAULT_API_BASE_URL: &str = "http://localhost:8080";

    /// Default WebSocket URL  
    pub const DEFAULT_WEBSOCKET_URL: &str = "ws://localhost:8080/ws";

    /// Default metrics URL
    pub const DEFAULT_METRICS_URL: &str = "http://localhost:9090";

    /// Default web UI URL
    pub const DEFAULT_WEB_UI_URL: &str = "http://localhost:3000";

    /// Localhost constant
    pub const LOCALHOST: &str = "127.0.0.1";

    /// Default bind address
    pub const DEFAULT_BIND_ADDRESS: &str = "0.0.0.0";

    /// Default API port
    pub const DEFAULT_API_PORT: u16 = 8080;

    /// Default internal port
    pub const DEFAULT_INTERNAL_PORT: u16 = 8081;

    /// Default metrics port
    pub const DEFAULT_METRICS_PORT: u16 = 9090;

    /// Build API URL from environment or default
    pub const fn build_api_url() -> String {
        std::env::var("NESTGATE_API_URL").unwrap_or_else(|_| DEFAULT_API_BASE_URL.to_string())
    }

    /// Build WebSocket URL from environment or default
    pub const fn build_websocket_url() -> String {
        std::env::var("NESTGATE_WS_URL").unwrap_or_else(|_| DEFAULT_WEBSOCKET_URL.to_string())
    }

    /// Build metrics URL from environment or default
    pub const fn build_metrics_url() -> String {
        std::env::var("NESTGATE_METRICS_URL").unwrap_or_else(|_| DEFAULT_METRICS_URL.to_string())
    }

    /// Build generic endpoint from environment
    pub const fn build_endpoint() -> String {
        build_api_url()
    }

    pub mod limits {
        //! Network limits and constraints

        /// Maximum concurrent requests
        pub const MAX_CONCURRENT_REQUESTS: usize = 1000;

        /// Maximum request size in bytes
        pub const MAX_REQUEST_SIZE: usize = 1024 * 1024; // 1MB

        /// Connection timeout in milliseconds
        pub const CONNECTION_TIMEOUT_MS: u64 = 5000;
    }
}

pub mod performance {
    //! Performance-related defaults

    /// Network buffer size
    pub const NETWORK_BUFFER_SIZE: usize = 8192;

    /// Default buffer size
    pub const DEFAULT_BUFFER_SIZE: usize = 4096;

    /// Maximum connections
    pub const MAX_CONNECTIONS: usize = 1000;
}

pub mod timeouts {
    //! Timeout constants

    /// Default timeout in milliseconds
    pub const DEFAULT_TIMEOUT_MS: u64 = 5000;

    /// Connection timeout
    pub const CONNECTION_TIMEOUT_MS: u64 = 3000;

    /// Request timeout
    pub const REQUEST_TIMEOUT_MS: u64 = 10000;
}
