/// Middleware Configuration Types
/// Basic enums and supporting types split from the main config file
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Middleware types supported by NestGate
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MiddlewareType {
    /// Authentication middleware
    Auth,
    /// Authorization middleware
    Authorization,
    /// Rate limiting middleware
    RateLimit,
    /// CORS middleware
    Cors,
    /// Logging middleware
    Logging,
    /// Compression middleware
    Compression,
    /// Cache middleware
    Cache,
    /// Security headers middleware
    Security,
    /// Request validation middleware
    Validation,
    /// Response transformation middleware
    Transform,
    /// Error handling middleware
    ErrorHandler,
    /// Custom middleware
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiddlewareConfiguration {
    /// Middleware-specific settings
    pub settings: HashMap<String, serde_json::Value>,
    /// Enable/disable this middleware
    pub enabled: bool,
    /// Priority order
    pub priority: u32,
    /// Conditional execution rules
    pub conditions: Vec<MiddlewareCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiddlewareCondition {
    /// Condition type
    pub condition_type: ConditionType,
    /// Condition value
    pub value: String,
    /// Negation flag
    pub negate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionType {
    /// Path pattern matching
    Path,
    /// HTTP method matching
    Method,
    /// Header presence/value
    Header,
    /// Query parameter presence/value
    QueryParam,
    /// User agent matching
    UserAgent,
    /// IP address range
    IpRange,
    /// Custom condition
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HttpResponseFormat {
    /// JSON error response
    Json,
    /// Plain text error response
    Text,
    /// HTML error response
    Html,
    /// Custom format
    Custom(String),
}

// Default implementations
impl Default for MiddlewareConfiguration {
    fn default() -> Self {
        Self {
            settings: HashMap::new(),
            enabled: true,
            priority: 50,
            conditions: Vec::new(),
        }
    }
}
