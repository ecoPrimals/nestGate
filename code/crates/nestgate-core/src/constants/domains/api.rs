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
/// Apidomainconstants
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
/// Apiversions
pub struct ApiVersions {
    /// Current
    pub current: &'static str,
    /// V1
    pub v1: &'static str,
    /// V2
    pub v2: &'static str,
    /// Prefix
    pub prefix: &'static str,
}

#[derive(Debug, Clone)]
/// Apistatuscodes
pub struct ApiStatusCodes {
    /// Ok
    pub ok: u16,
    /// Created
    pub created: u16,
    /// Accepted
    pub accepted: u16,
    /// No Content
    pub no_content: u16,
    /// Bad Request
    pub bad_request: u16,
    /// Unauthorized
    pub unauthorized: u16,
    /// Forbidden
    pub forbidden: u16,
    /// Not Found
    pub not_found: u16,
    /// Method Not Allowed
    pub method_not_allowed: u16,
    /// Conflict
    pub conflict: u16,
    /// Unprocessable Entity
    pub unprocessable_entity: u16,
    /// Too Many Requests
    pub too_many_requests: u16,
    /// Internal Server Error
    pub internal_server_error: u16,
    /// Bad Gateway
    pub bad_gateway: u16,
    /// Service Unavailable
    pub service_unavailable: u16,
    /// Gateway Timeout
    pub gateway_timeout: u16,
}

#[derive(Debug, Clone)]
/// Apiheaders
pub struct ApiHeaders {
    /// Content Type
    pub content_type: &'static str,
    /// Authorization
    pub authorization: &'static str,
    /// Accept
    pub accept: &'static str,
    /// User Agent
    pub user_agent: &'static str,
    /// X Api Version
    pub x_api_version: &'static str,
    /// X Request identifier
    pub x_request_id: &'static str,
    /// X Rate Limit
    pub x_rate_limit: &'static str,
    /// X Rate Limit Remaining
    pub x_rate_limit_remaining: &'static str,
    /// Cache Control
    pub cache_control: &'static str,
    /// Etag
    pub etag: &'static str,
}

#[derive(Debug, Clone)]
/// Apicontenttypes
pub struct ApiContentTypes {
    /// Json
    pub json: &'static str,
    /// Xml
    pub xml: &'static str,
    /// Html
    pub html: &'static str,
    /// Text
    pub text: &'static str,
    /// Binary
    pub binary: &'static str,
    /// Form Urlencoded
    pub form_urlencoded: &'static str,
    /// Multipart Form
    pub multipart_form: &'static str,
}

#[derive(Debug, Clone)]
/// Apiendpoints
pub struct ApiEndpoints {
    /// Base
    pub base: &'static str,
    /// Health
    pub health: &'static str,
    /// Metrics
    pub metrics: &'static str,
    /// Version
    pub version: &'static str,
    /// Docs
    pub docs: &'static str,
    /// Zfs
    pub zfs: &'static str,
    /// Storage
    pub storage: &'static str,
    /// Admin
    pub admin: &'static str,
}

#[derive(Debug, Clone)]
/// Apilimits
pub struct ApiLimits {
    /// Size of max request
    pub max_request_size: usize,
    /// Size of max response
    pub max_response_size: usize,
    /// Size of max header
    pub max_header_size: usize,
    /// Max Uri Length
    pub max_uri_length: usize,
    /// Request Timeout Secs
    pub request_timeout_secs: u64,
    /// Rate Limit Per Minute
    pub rate_limit_per_minute: u32,
    /// Burst Limit
    pub burst_limit: u32,
}

impl Default for ApiDomainConstants {
    /// Returns the default instance
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
