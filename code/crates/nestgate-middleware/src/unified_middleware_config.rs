/// **UNIFIED MIDDLEWARE CONFIGURATION**
/// Consolidates all fragmented middleware configuration structs into the StandardDomainConfig pattern.
/// This eliminates configuration fragmentation across the middleware crate.
///
/// **ELIMINATES**:
/// - Scattered middleware configuration types
/// - Duplicate middleware settings across modules
/// - Fragmented condition and validation logic
///
/// **PROVIDES**:
/// - Single source of truth for all middleware configuration
/// - Consistent configuration patterns with base unified configs
/// - Extensible architecture for middleware-specific settings
use nestgate_core::unified_config_consolidation::StandardDomainConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// Re-export types from config/types.rs for backward compatibility
pub use crate::config::types::{
    ConditionType, HttpResponseFormat, MiddlewareCondition, MiddlewareConfiguration, MiddlewareType,
};

/// **UNIFIED MIDDLEWARE EXTENSIONS**
/// Consolidates all middleware-specific configuration patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedMiddlewareExtensions {
    /// Authentication middleware settings
    pub auth: MiddlewareAuthSettings,
    /// Authorization middleware settings
    pub authorization: MiddlewareAuthorizationSettings,
    /// Rate limiting middleware settings
    pub rate_limit: MiddlewareRateLimitSettings,
    /// CORS middleware settings
    pub cors: MiddlewareCorsSettings,
    /// Logging middleware settings
    pub logging: MiddlewareLoggingSettings,
    /// Compression middleware settings
    pub compression: MiddlewareCompressionSettings,
    /// Cache middleware settings
    pub cache: MiddlewareCacheSettings,
    /// Security headers middleware settings
    pub security: MiddlewareSecuritySettings,
    /// Request validation middleware settings
    pub validation: MiddlewareValidationSettings,
    /// Response transformation middleware settings
    pub transform: MiddlewareTransformSettings,
    /// Error handling middleware settings
    pub error_handler: MiddlewareErrorHandlerSettings,
    /// Custom middleware configurations
    pub custom: HashMap<String, MiddlewareConfiguration>,
    /// Global middleware chain settings
    pub chain: MiddlewareChainSettings,
}

/// Authentication middleware settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiddlewareAuthSettings {
    /// Enable authentication middleware
    pub enabled: bool,
    /// Authentication providers
    pub providers: Vec<AuthProvider>,
    /// Token validation settings
    pub token_validation: TokenValidationSettings,
    /// Session management settings
    pub session: SessionSettings,
    /// JWT settings
    pub jwt: JwtSettings,
}

/// Authorization middleware settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiddlewareAuthorizationSettings {
    /// Enable authorization middleware
    pub enabled: bool,
    /// Authorization engine (RBAC, ABAC, etc.)
    pub engine: String,
    /// Policy sources
    pub policy_sources: Vec<String>,
    /// Role-based access control settings
    pub rbac: RbacSettings,
    /// Attribute-based access control settings
    pub abac: AbacSettings,
}

/// Rate limiting middleware settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiddlewareRateLimitSettings {
    /// Enable rate limiting middleware
    pub enabled: bool,
    /// Global rate limits
    pub global_limits: RateLimitConfig,
    /// Per-user rate limits
    pub per_user_limits: RateLimitConfig,
    /// Per-IP rate limits
    pub per_ip_limits: RateLimitConfig,
    /// Rate limit storage backend
    pub storage_backend: String,
    /// Rate limit algorithms
    pub algorithms: Vec<RateLimitAlgorithm>,
}

/// CORS middleware settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiddlewareCorsSettings {
    /// Enable CORS middleware
    pub enabled: bool,
    /// Allowed origins
    pub allowed_origins: Vec<String>,
    /// Allowed methods
    pub allowed_methods: Vec<String>,
    /// Allowed headers
    pub allowed_headers: Vec<String>,
    /// Exposed headers
    pub exposed_headers: Vec<String>,
    /// Allow credentials
    pub allow_credentials: bool,
    /// Max age for preflight requests
    pub max_age: Duration,
}

/// Logging middleware settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiddlewareLoggingSettings {
    /// Enable logging middleware
    pub enabled: bool,
    /// Log format (JSON, text, etc.)
    pub format: String,
    /// Log level
    pub level: String,
    /// Fields to include in logs
    pub fields: Vec<String>,
    /// Fields to exclude from logs
    pub excluded_fields: Vec<String>,
    /// Request/response body logging
    pub log_bodies: bool,
    /// Log sampling rate
    pub sampling_rate: f64,
}

/// Compression middleware settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiddlewareCompressionSettings {
    /// Enable compression middleware
    pub enabled: bool,
    /// Compression algorithms
    pub algorithms: Vec<String>,
    /// Minimum response size for compression
    pub min_size: usize,
    /// Compression level
    pub level: u32,
    /// MIME types to compress
    pub mime_types: Vec<String>,
}

/// Cache middleware settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiddlewareCacheSettings {
    /// Enable cache middleware
    pub enabled: bool,
    /// Cache backend
    pub backend: String,
    /// Default TTL
    pub default_ttl: Duration,
    /// Cache key patterns
    pub key_patterns: Vec<CacheKeyPattern>,
    /// Cache invalidation rules
    pub invalidation_rules: Vec<CacheInvalidationRule>,
    /// Cache size limits
    pub size_limits: CacheSizeLimits,
}

/// Security headers middleware settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiddlewareSecuritySettings {
    /// Enable security middleware
    pub enabled: bool,
    /// Security headers to add
    pub headers: HashMap<String, String>,
    /// Content Security Policy
    pub csp: ContentSecurityPolicySettings,
    /// HSTS settings
    pub hsts: HstsSettings,
    /// X-Frame-Options settings
    pub frame_options: FrameOptionsSettings,
}

/// Request validation middleware settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiddlewareValidationSettings {
    /// Enable validation middleware
    pub enabled: bool,
    /// Schema validation settings
    pub schema_validation: SchemaValidationSettings,
    /// Input sanitization settings
    pub sanitization: SanitizationSettings,
    /// Size limits
    pub size_limits: ValidationSizeLimits,
    /// Custom validators
    pub custom_validators: Vec<CustomValidator>,
}

/// Response transformation middleware settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiddlewareTransformSettings {
    /// Enable transformation middleware
    pub enabled: bool,
    /// Response transformers
    pub transformers: Vec<ResponseTransformer>,
    /// Content type transformations
    pub content_type_transforms: HashMap<String, String>,
    /// Header transformations
    pub header_transforms: Vec<HeaderTransform>,
}

/// Error handling middleware settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiddlewareErrorHandlerSettings {
    /// Enable error handling middleware
    pub enabled: bool,
    /// Error response format
    pub response_format: HttpResponseFormat,
    /// Error logging settings
    pub logging: ErrorLoggingSettings,
    /// Custom error handlers
    pub custom_handlers: HashMap<String, ErrorHandler>,
    /// Error transformation rules
    pub transform_rules: Vec<ErrorTransformRule>,
}

/// Middleware chain settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiddlewareChainSettings {
    /// Global middleware execution order
    pub execution_order: Vec<MiddlewareType>,
    /// Per-route middleware overrides
    pub route_overrides: HashMap<String, Vec<MiddlewareType>>,
    /// Middleware timeout settings
    pub timeouts: MiddlewareTimeoutSettings,
    /// Error handling strategy for chain
    pub error_strategy: ChainErrorStrategy,
}

// Supporting types for middleware configuration

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthProvider {
    pub name: String,
    pub provider_type: String,
    pub config: HashMap<String, serde_json::Value>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenValidationSettings {
    pub validate_signature: bool,
    pub validate_expiration: bool,
    pub validate_audience: bool,
    pub validate_issuer: bool,
    pub clock_skew: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSettings {
    pub enabled: bool,
    pub store_type: String,
    pub ttl: Duration,
    pub secure: bool,
    pub http_only: bool,
    pub same_site: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtSettings {
    pub secret_key: String,
    pub algorithm: String,
    pub issuer: String,
    pub audience: Vec<String>,
    pub expiration: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RbacSettings {
    pub enabled: bool,
    pub roles_source: String,
    pub permissions_source: String,
    pub cache_ttl: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbacSettings {
    pub enabled: bool,
    pub policy_engine: String,
    pub attribute_sources: Vec<String>,
    pub evaluation_cache_ttl: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub requests_per_second: u32,
    pub burst_size: u32,
    pub window_size: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitAlgorithm {
    pub name: String,
    pub config: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheKeyPattern {
    pub pattern: String,
    pub ttl: Duration,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheInvalidationRule {
    pub trigger: String,
    pub patterns: Vec<String>,
    pub strategy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheSizeLimits {
    pub max_entries: usize,
    pub max_memory: usize,
    pub eviction_policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentSecurityPolicySettings {
    pub enabled: bool,
    pub policy: String,
    pub report_only: bool,
    pub report_uri: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HstsSettings {
    pub enabled: bool,
    pub max_age: Duration,
    pub include_subdomains: bool,
    pub preload: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameOptionsSettings {
    pub enabled: bool,
    pub policy: String, // DENY, SAMEORIGIN, ALLOW-FROM
    pub allowed_origins: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaValidationSettings {
    pub enabled: bool,
    pub schema_format: String, // JSON Schema, OpenAPI, etc.
    pub schemas: HashMap<String, String>,
    pub strict_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanitizationSettings {
    pub enabled: bool,
    pub html_sanitization: bool,
    pub sql_injection_protection: bool,
    pub xss_protection: bool,
    pub custom_sanitizers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationSizeLimits {
    pub max_request_size: usize,
    pub max_header_size: usize,
    pub max_field_count: usize,
    pub max_nesting_depth: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomValidator {
    pub name: String,
    pub validator_type: String,
    pub config: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseTransformer {
    pub name: String,
    pub transformer_type: String,
    pub conditions: Vec<MiddlewareCondition>,
    pub config: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderTransform {
    pub action: String, // add, remove, modify
    pub header_name: String,
    pub value: Option<String>,
    pub conditions: Vec<MiddlewareCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorLoggingSettings {
    pub enabled: bool,
    pub log_level: String,
    pub include_stack_trace: bool,
    pub include_request_details: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorHandler {
    pub handler_type: String,
    pub config: HashMap<String, serde_json::Value>,
    pub conditions: Vec<MiddlewareCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorTransformRule {
    pub from_status: u16,
    pub to_status: u16,
    pub transform_body: bool,
    pub conditions: Vec<MiddlewareCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiddlewareTimeoutSettings {
    pub default_timeout: Duration,
    pub per_middleware_timeouts: HashMap<String, Duration>,
    pub total_chain_timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChainErrorStrategy {
    /// Stop execution on first error
    StopOnError,
    /// Continue execution, collect errors
    ContinueOnError,
    /// Skip failed middleware, continue chain
    SkipOnError,
}

/// **UNIFIED MIDDLEWARE CONFIGURATION**
/// The single source of truth for all middleware configuration across the system
pub type UnifiedMiddlewareConfig = StandardDomainConfig<UnifiedMiddlewareExtensions>;

/// Extension trait for UnifiedMiddlewareConfig factory methods
pub trait UnifiedMiddlewareConfigExt {
    /// Create development configuration optimized for local development
    fn development() -> Self;
    /// Create production configuration optimized for high-load production
    fn production() -> Self;
    /// Create security-focused configuration with all security middleware enabled
    fn security_focused() -> Self;
    /// Create performance-focused configuration with minimal middleware overhead
    fn performance_focused() -> Self;
    /// Create testing configuration optimized for integration tests
    fn testing() -> Self;
}

impl UnifiedMiddlewareConfigExt for UnifiedMiddlewareConfig {
    /// Create development configuration optimized for local development
    fn development() -> Self {
        Self::new(UnifiedMiddlewareExtensions::default())
    }

    /// Create production configuration optimized for high-load production
    fn production() -> Self {
        let mut extensions = UnifiedMiddlewareExtensions::default();
        extensions.security.enabled = true;
        extensions.compression.enabled = true;
        extensions.cache.enabled = true;
        Self::new(extensions)
    }

    /// Create security-focused configuration with all security middleware enabled
    fn security_focused() -> Self {
        let mut extensions = UnifiedMiddlewareExtensions::default();
        extensions.auth.enabled = true;
        extensions.authorization.enabled = true;
        extensions.security.enabled = true;
        extensions.validation.enabled = true;
        Self::new(extensions)
    }

    /// Create performance-focused configuration with minimal middleware overhead
    fn performance_focused() -> Self {
        let mut extensions = UnifiedMiddlewareExtensions::default();
        extensions.compression.enabled = true;
        extensions.cache.enabled = true;
        extensions.logging.sampling_rate = 0.1; // Reduced logging
        Self::new(extensions)
    }

    /// Create testing configuration optimized for integration tests
    fn testing() -> Self {
        let mut extensions = UnifiedMiddlewareExtensions::default();
        extensions.auth.enabled = false;
        extensions.authorization.enabled = false;
        Self::new(extensions)
    }
}

impl Default for UnifiedMiddlewareExtensions {
    fn default() -> Self {
        Self {
            auth: MiddlewareAuthSettings::default(),
            authorization: MiddlewareAuthorizationSettings::default(),
            rate_limit: MiddlewareRateLimitSettings::default(),
            cors: MiddlewareCorsSettings::default(),
            logging: MiddlewareLoggingSettings::default(),
            compression: MiddlewareCompressionSettings::default(),
            cache: MiddlewareCacheSettings::default(),
            security: MiddlewareSecuritySettings::default(),
            validation: MiddlewareValidationSettings::default(),
            transform: MiddlewareTransformSettings::default(),
            error_handler: MiddlewareErrorHandlerSettings::default(),
            custom: HashMap::new(),
            chain: MiddlewareChainSettings::default(),
        }
    }
}

// Default implementations for all settings structs
impl Default for MiddlewareAuthSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            providers: Vec::new(),
            token_validation: TokenValidationSettings::default(),
            session: SessionSettings::default(),
            jwt: JwtSettings::default(),
        }
    }
}

impl Default for MiddlewareAuthorizationSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            engine: "rbac".to_string(),
            policy_sources: Vec::new(),
            rbac: RbacSettings::default(),
            abac: AbacSettings::default(),
        }
    }
}

impl Default for MiddlewareRateLimitSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            global_limits: RateLimitConfig::default(),
            per_user_limits: RateLimitConfig::default(),
            per_ip_limits: RateLimitConfig::default(),
            storage_backend: "memory".to_string(),
            algorithms: Vec::new(),
        }
    }
}

impl Default for MiddlewareCorsSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            allowed_origins: vec!["*".to_string()],
            allowed_methods: vec![
                "GET".to_string(),
                "POST".to_string(),
                "PUT".to_string(),
                "DELETE".to_string(),
            ],
            allowed_headers: vec!["Content-Type".to_string(), "Authorization".to_string()],
            exposed_headers: Vec::new(),
            allow_credentials: false,
            max_age: Duration::from_secs(3600),
        }
    }
}

impl Default for MiddlewareLoggingSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            format: "json".to_string(),
            level: "info".to_string(),
            fields: vec![
                "timestamp".to_string(),
                "method".to_string(),
                "path".to_string(),
                "status".to_string(),
            ],
            excluded_fields: Vec::new(),
            log_bodies: false,
            sampling_rate: 1.0,
        }
    }
}

impl Default for MiddlewareCompressionSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            algorithms: vec!["gzip".to_string(), "deflate".to_string()],
            min_size: 1024,
            level: 6,
            mime_types: vec![
                "application/json".to_string(),
                "text/html".to_string(),
                "text/css".to_string(),
            ],
        }
    }
}

impl Default for MiddlewareCacheSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            backend: "memory".to_string(),
            default_ttl: Duration::from_secs(300),
            key_patterns: Vec::new(),
            invalidation_rules: Vec::new(),
            size_limits: CacheSizeLimits::default(),
        }
    }
}

impl Default for MiddlewareSecuritySettings {
    fn default() -> Self {
        Self {
            enabled: true,
            headers: HashMap::new(),
            csp: ContentSecurityPolicySettings::default(),
            hsts: HstsSettings::default(),
            frame_options: FrameOptionsSettings::default(),
        }
    }
}

impl Default for MiddlewareValidationSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            schema_validation: SchemaValidationSettings::default(),
            sanitization: SanitizationSettings::default(),
            size_limits: ValidationSizeLimits::default(),
            custom_validators: Vec::new(),
        }
    }
}

impl Default for MiddlewareTransformSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            transformers: Vec::new(),
            content_type_transforms: HashMap::new(),
            header_transforms: Vec::new(),
        }
    }
}

impl Default for MiddlewareErrorHandlerSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            response_format: HttpResponseFormat::Json,
            logging: ErrorLoggingSettings::default(),
            custom_handlers: HashMap::new(),
            transform_rules: Vec::new(),
        }
    }
}

impl Default for MiddlewareChainSettings {
    fn default() -> Self {
        Self {
            execution_order: vec![
                MiddlewareType::Logging,
                MiddlewareType::Security,
                MiddlewareType::Cors,
                MiddlewareType::Auth,
                MiddlewareType::Authorization,
                MiddlewareType::RateLimit,
                MiddlewareType::Validation,
                MiddlewareType::Compression,
                MiddlewareType::Cache,
                MiddlewareType::Transform,
                MiddlewareType::ErrorHandler,
            ],
            route_overrides: HashMap::new(),
            timeouts: MiddlewareTimeoutSettings::default(),
            error_strategy: ChainErrorStrategy::StopOnError,
        }
    }
}

// Additional default implementations for supporting types
impl Default for TokenValidationSettings {
    fn default() -> Self {
        Self {
            validate_signature: true,
            validate_expiration: true,
            validate_audience: true,
            validate_issuer: true,
            clock_skew: Duration::from_secs(60),
        }
    }
}

impl Default for SessionSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            store_type: "memory".to_string(),
            ttl: Duration::from_secs(3600),
            secure: true,
            http_only: true,
            same_site: "strict".to_string(),
        }
    }
}

impl Default for JwtSettings {
    fn default() -> Self {
        Self {
            secret_key: "default-secret".to_string(),
            algorithm: "HS256".to_string(),
            issuer: "nestgate".to_string(),
            audience: vec!["nestgate".to_string()],
            expiration: Duration::from_secs(3600),
        }
    }
}

impl Default for RbacSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            roles_source: "database".to_string(),
            permissions_source: "database".to_string(),
            cache_ttl: Duration::from_secs(300),
        }
    }
}

impl Default for AbacSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            policy_engine: "opa".to_string(),
            attribute_sources: Vec::new(),
            evaluation_cache_ttl: Duration::from_secs(300),
        }
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_second: 100,
            burst_size: 200,
            window_size: Duration::from_secs(60),
        }
    }
}

impl Default for CacheSizeLimits {
    fn default() -> Self {
        Self {
            max_entries: 10000,
            max_memory: 100 * 1024 * 1024, // 100MB
            eviction_policy: "lru".to_string(),
        }
    }
}

impl Default for ContentSecurityPolicySettings {
    fn default() -> Self {
        Self {
            enabled: true,
            policy: "default-src 'self'".to_string(),
            report_only: false,
            report_uri: None,
        }
    }
}

impl Default for HstsSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            max_age: Duration::from_secs(31536000), // 1 year
            include_subdomains: true,
            preload: false,
        }
    }
}

impl Default for FrameOptionsSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            policy: "DENY".to_string(),
            allowed_origins: Vec::new(),
        }
    }
}

impl Default for SchemaValidationSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            schema_format: "json-schema".to_string(),
            schemas: HashMap::new(),
            strict_mode: false,
        }
    }
}

impl Default for SanitizationSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            html_sanitization: true,
            sql_injection_protection: true,
            xss_protection: true,
            custom_sanitizers: Vec::new(),
        }
    }
}

impl Default for ValidationSizeLimits {
    fn default() -> Self {
        Self {
            max_request_size: 10 * 1024 * 1024, // 10MB
            max_header_size: 8192,
            max_field_count: 1000,
            max_nesting_depth: 10,
        }
    }
}

impl Default for ErrorLoggingSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            log_level: "error".to_string(),
            include_stack_trace: true,
            include_request_details: true,
        }
    }
}

impl Default for MiddlewareTimeoutSettings {
    fn default() -> Self {
        Self {
            default_timeout: Duration::from_secs(30),
            per_middleware_timeouts: HashMap::new(),
            total_chain_timeout: Duration::from_secs(60),
        }
    }
}
