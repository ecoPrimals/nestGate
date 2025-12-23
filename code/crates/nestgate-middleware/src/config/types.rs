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
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust
/// // OLD (deprecated):
/// use crate::network::config::MiddlewareConfiguration;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::MiddlewareConfiguration; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
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
    /// Returns the default instance
    fn default() -> Self { Self {
            settings: HashMap::new(),
            enabled: true,
            priority: 50,
            conditions: Vec::new(),
         }
}


// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
pub type MiddlewareConfigurationCanonical = crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using MiddlewareConfiguration (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_middleware_type_all_variants() {
        let types = vec![
            MiddlewareType::Auth,
            MiddlewareType::Authorization,
            MiddlewareType::RateLimit,
            MiddlewareType::Cors,
            MiddlewareType::Logging,
            MiddlewareType::Compression,
            MiddlewareType::Cache,
            MiddlewareType::Security,
            MiddlewareType::Validation,
            MiddlewareType::Transform,
            MiddlewareType::ErrorHandler,
        ];
        assert_eq!(types.len(), 11);
    }

    #[test]
    fn test_middleware_type_custom() {
        let custom = MiddlewareType::Custom("my_mw".to_string());
        assert!(matches!(custom, MiddlewareType::Custom(_)));
    }

    #[test]
    fn test_middleware_type_equality() {
        assert_eq!(MiddlewareType::Auth, MiddlewareType::Auth);
        assert_ne!(MiddlewareType::Auth, MiddlewareType::Logging);
    }

    #[test]
    fn test_middleware_type_clone() {
        let mw = MiddlewareType::Cache;
        let cloned = mw.clone();
        assert_eq!(mw, cloned);
    }

    #[test]
    fn test_middleware_type_serialization() {
        let mw = MiddlewareType::Auth;
        let json = serde_json::to_string(&mw).expect("Configuration error");
        let deserialized: MiddlewareType = serde_json::from_str(&json).expect("Configuration error");
        assert_eq!(mw, deserialized);
    }

    #[test]
    fn test_middleware_configuration_default() {
        let config = MiddlewareConfiguration::default();
        assert!(config.enabled);
        assert_eq!(config.priority, 50);
        assert!(config.settings.is_empty());
        assert!(config.conditions.is_empty());
    }

    #[test]
    fn test_middleware_configuration_custom() {
        let mut settings = HashMap::new();
        settings.insert("key".to_string(), serde_json::json!("value"));
        
        let config = MiddlewareConfiguration {
            settings,
            enabled: false,
            priority: 100,
            conditions: vec![],
        };
        
        assert!(!config.enabled);
        assert_eq!(config.priority, 100);
        assert_eq!(config.settings.len(), 1);
    }

    #[test]
    fn test_middleware_condition_creation() {
        let condition = MiddlewareCondition {
            condition_type: ConditionType::Path,
            value: "/api/*".to_string(),
            negate: false,
        };
        
        assert!(matches!(condition.condition_type, ConditionType::Path));
        assert!(!condition.negate);
    }

    #[test]
    fn test_condition_type_variants() {
        let types = vec![
            ConditionType::Path,
            ConditionType::Method,
            ConditionType::Header,
            ConditionType::QueryParam,
            ConditionType::UserAgent,
            ConditionType::IpRange,
        ];
        assert_eq!(types.len(), 6);
    }

    #[test]
    fn test_http_response_format_variants() {
        let formats = vec![
            HttpResponseFormat::Json,
            HttpResponseFormat::Text,
            HttpResponseFormat::Html,
        ];
        assert_eq!(formats.len(), 3);
    }

    #[test]
    fn test_http_response_format_custom() {
        let format = HttpResponseFormat::Custom("xml".to_string());
        assert!(matches!(format, HttpResponseFormat::Custom(_)));
    }
}
