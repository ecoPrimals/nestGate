// **API DOMAIN CONSTANTS**
//! Api functionality and utilities.
// API-related constants extracted from the consolidated constants system.
// This module consolidates all API-related constants:
//! - HTTP status codes, headers
//! - API versioning, endpoints
//! - Content types, methods

// ==================== API DOMAIN CONSTANTS ====================

/// **API DOMAIN CONSTANTS**
///
/// Consolidates all API-related constants:
/// - HTTP status codes, headers
/// - API versioning, endpoints
/// - Content types, methods
#[derive(Debug, Clone)]
pub struct ApiDomainConstants {
    /// API versioning
    pub versions: ApiVersions,

    /// HTTP status codes
    pub status_codes: ApiStatusCodes,

    /// HTTP headers
    pub headers: ApiHeaders,

    /// Content types
    pub content_types: ApiContentTypes,

    /// API endpoints
    pub endpoints: ApiEndpoints,

    /// Request/response limits
    pub limits: ApiLimits,
}
#[derive(Debug, Clone)]
pub struct ApiVersions {
    pub current: &'static str,
    pub v1: &'static str,
    pub v2: &'static str,
    pub prefix: &'static str,
}

#[derive(Debug, Clone)]
pub struct ApiStatusCodes {
    pub ok: u16,
    pub created: u16,
    pub accepted: u16,
    pub no_content: u16,
    pub bad_request: u16,
    pub unauthorized: u16,
    pub forbidden: u16,
    pub not_found: u16,
    pub method_not_allowed: u16,
    pub conflict: u16,
    pub unprocessable_entity: u16,
    pub too_many_requests: u16,
    pub internal_server_error: u16,
    pub bad_gateway: u16,
    pub service_unavailable: u16,
    pub gateway_timeout: u16,
}

#[derive(Debug, Clone)]
pub struct ApiHeaders {
    pub content_type: &'static str,
    pub authorization: &'static str,
    pub accept: &'static str,
    pub user_agent: &'static str,
    pub x_api_version: &'static str,
    pub x_request_id: &'static str,
    pub x_rate_limit: &'static str,
    pub x_rate_limit_remaining: &'static str,
    pub cache_control: &'static str,
    pub etag: &'static str,
}

#[derive(Debug, Clone)]
pub struct ApiContentTypes {
    pub json: &'static str,
    pub xml: &'static str,
    pub html: &'static str,
    pub text: &'static str,
    pub binary: &'static str,
    pub form_urlencoded: &'static str,
    pub multipart_form: &'static str,
}

#[derive(Debug, Clone)]
pub struct ApiEndpoints {
    pub base: &'static str,
    pub health: &'static str,
    pub metrics: &'static str,
    pub version: &'static str,
    pub docs: &'static str,
    pub zfs: &'static str,
    pub storage: &'static str,
    pub admin: &'static str,
}

#[derive(Debug, Clone)]
pub struct ApiLimits {
    pub max_request_size: usize,
    pub max_response_size: usize,
    pub max_header_size: usize,
    pub max_uri_length: usize,
    pub request_timeout_secs: u64,
    pub rate_limit_per_minute: u32,
    pub burst_limit: u32,
}

impl Default for ApiDomainConstants {
    fn default() -> Self {
        Self {
            versions: ApiVersions {
                current: "v1",
                v1: "v1",
                v2: "v2",
                prefix: "/api",
            },
            status_codes: ApiStatusCodes {
                ok: 200,
                created: 201,
                accepted: 202,
                no_content: 204,
                bad_request: 400,
                unauthorized: 401,
                forbidden: 403,
                not_found: 404,
                method_not_allowed: 405,
                conflict: 409,
                unprocessable_entity: 422,
                too_many_requests: 429,
                internal_server_error: 500,
                bad_gateway: 502,
                service_unavailable: 503,
                gateway_timeout: 504,
            },
            headers: ApiHeaders {
                content_type: "Content-Type",
                authorization: "Authorization",
                accept: "Accept",
                user_agent: "User-Agent",
                x_api_version: "X-API-Version",
                x_request_id: "X-Request-ID",
                x_rate_limit: "X-RateLimit-Limit",
                x_rate_limit_remaining: "X-RateLimit-Remaining",
                cache_control: "Cache-Control",
                etag: "ETag",
            },
            content_types: ApiContentTypes {
                json: "application/json",
                xml: "application/xml",
                html: "text/html",
                text: "text/plain",
                binary: "application/octet-stream",
                form_urlencoded: "application/x-www-form-urlencoded",
                multipart_form: "multipart/form-data",
            },
            endpoints: ApiEndpoints {
                base: "/api/v1",
                health: "/health",
                metrics: "/metrics",
                version: "/version",
                docs: "/docs",
                zfs: "/zfs",
                storage: "/storage",
                admin: "/admin",
            },
            limits: ApiLimits {
                max_request_size: 10 * 1024 * 1024,  // 10MB
                max_response_size: 10 * 1024 * 1024, // 10MB
                max_header_size: 8192,               // 8KB
                max_uri_length: 2048,
                request_timeout_secs: 30,
                rate_limit_per_minute: 1000,
                burst_limit: 100,
            },
        }
    }
}

// ==================== CONVENIENCE EXPORTS ====================

/// Convenience module for easy access to API constants
pub mod api_defaults {
    use super::*;
    /// Get default API domain constants
    #[must_use]
    pub fn constants() -> ApiDomainConstants {
        ApiDomainConstants::default()
    }

    /// Current API version
    pub const VERSION: &str = "v1";

    /// API base path
    pub const BASE_PATH: &str = "/api/v1";

    /// Default content type
    pub const DEFAULT_CONTENT_TYPE: &str = "application/json";
}
