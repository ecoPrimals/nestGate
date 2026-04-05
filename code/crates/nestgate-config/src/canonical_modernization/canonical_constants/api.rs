// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

    /// API version identifier
    pub const CURRENT_API_VERSION: &str = "v1";

    /// HTTP status messages
    ///
    /// HTTP 200 OK status message
    pub const STATUS_OK: &str = "OK";
    /// HTTP 404 Not Found status message
    pub const STATUS_NOT_FOUND: &str = "Not Found";
    /// HTTP 500 Internal Server Error status message
    pub const STATUS_INTERNAL_ERROR: &str = "Internal Server Error";
    /// HTTP 401 Unauthorized status message
    pub const STATUS_UNAUTHORIZED: &str = "Unauthorized";
    /// HTTP 400 Bad Request status message
    pub const STATUS_BAD_REQUEST: &str = "Bad Request";

    /// Content types
    ///
    /// JSON content type (application/json)
    pub const CONTENT_TYPE_JSON: &str = "application/json";
    /// HTML content type (text/html)
    pub const CONTENT_TYPE_HTML: &str = "text/html";
    /// Plain text content type (text/plain)
    pub const CONTENT_TYPE_PLAIN: &str = "text/plain";

    /// **PERFORMANCE ANALYSIS CONSTANTS**
    ///
    /// High impact level for performance issues
    pub const IMPACT_HIGH: &str = "High";
    /// Medium impact level for performance issues
    pub const IMPACT_MEDIUM: &str = "Medium";
    /// Low impact level for performance issues
    pub const IMPACT_LOW: &str = "Low";

    /// Performance analysis recommendation titles
    ///
    /// Recommendation to expand storage capacity
    pub const TITLE_EXPAND_STORAGE: &str = "Expand Storage Capacity";
    /// Recommendation to schedule pool defragmentation
    pub const TITLE_SCHEDULE_DEFRAG: &str = "Schedule Pool Defragmentation";
    /// Recommendation to optimize cache configuration
    pub const TITLE_OPTIMIZE_CACHE: &str = "Optimize Cache Configuration";
    /// Recommendation to consider hardware upgrade
    pub const TITLE_UPGRADE_HARDWARE: &str = "Consider Hardware Upgrade";

    /// API rate limiting
    ///
    /// Default rate limit (1000 requests per minute)
    pub const DEFAULT_RATE_LIMIT: u32 = 1000;
    /// Burst limit for rate limiting (100 requests)
    pub const BURST_LIMIT: u32 = 100;

    /// Maximum request size in bytes (10MB)
    pub const MAX_REQUEST_SIZE: usize = 10 * 1024 * 1024;
    /// Maximum response size in bytes (50MB)
    pub const MAX_RESPONSE_SIZE: usize = 50 * 1024 * 1024;
