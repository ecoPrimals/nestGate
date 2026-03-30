// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

    /// Default operation timeout (seconds)
    pub const DEFAULT_TIMEOUT_SECS: u64 = 30;

    /// Default API port
    ///
    /// # Environment Variable
    /// Override with `NESTGATE_API_PORT`
    ///
    /// # Usage
    /// ```rust,ignore
    /// use nestgate_core::constants::ports;
    /// let port = ports::api_server_port(); // Environment-aware
    /// ```
    pub const DEFAULT_API_PORT: u16 = 8080;

    /// Default bind address
    pub const DEFAULT_BIND_ADDRESS: &str = crate::constants::hardcoding::addresses::LOCALHOST_IPV4;

    /// Localhost address
    pub const LOCALHOST: &str = crate::constants::hardcoding::addresses::LOCALHOST_IPV4;

    /// Request timeout in seconds
    pub const REQUEST_TIMEOUT_SECS: u64 = 30;

    /// Connection timeout in seconds
    pub const CONNECTION_TIMEOUT_SECS: u64 = 10;

    /// Maximum number of connections in the pool
    pub const MAX_CONNECTIONS: usize = 1000;

    /// Keep-alive timeout
    pub const KEEP_ALIVE_TIMEOUT_SECS: u64 = 75;
