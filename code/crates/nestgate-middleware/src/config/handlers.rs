/// Configuration for request processing, response handling, and chain management
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use super::types::{HttpResponseFormat, MiddlewareType};

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RequestProcessingSettings {
    /// Request parsing configuration
    pub parsing: RequestParsingSettings,
    /// Request validation configuration
    pub validation: RequestValidationSettings,
    /// Request transformation
    pub transformation: RequestTransformationSettings,
    /// Request routing
    pub routing: RequestRoutingSettings,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestParsingSettings {
    /// Maximum request size
    pub max_size: usize,
    /// Request timeout
    pub timeout: Duration,
    /// Supported content types
    pub supported_content_types: Vec<String>,
    /// Character encoding
    pub default_encoding: String,
    /// Parse query strings
    pub parse_query: bool,
    /// Parse form data
    pub parse_form: bool,
    /// Parse JSON
    pub parse_json: bool,
}
    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RequestValidationSettings {
    /// Enable request validation
    pub enabled: bool,
    /// Validation rules
    pub rules: Vec<ValidationRule>,
    /// Schema validation
    pub schema_validation: bool,
    /// Custom validators
    pub custom_validators: HashMap<String, ValidatorConfig>,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    /// Field path
    pub field: String,
    /// Validation type
    pub validation_type: ValidationType,
    /// Error message
    pub message: Option<String>,
    /// Required field
    pub required: bool,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationType {
    /// String
    String {
        min_length: Option<usize>,
        max_length: Option<usize>,
        pattern: Option<String>,
    }
    /// Number
    Number {
        min: Option<f64>,
        max: Option<f64>,
    }
    Email,
    Url,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorConfig {
    /// Validator implementation
    pub implementation: String,
    /// Validator parameters
    pub parameters: HashMap<String, serde_json::Value>,
}
    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RequestTransformationSettings {
    /// Enable request transformation
    pub enabled: bool,
    /// Transformation rules
    pub rules: Vec<TransformationRule>,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationRule {
    /// Rule name
    pub name: String,
    /// Source field
    pub source: String,
    /// Target field
    pub target: String,
    /// Transformation type
    pub transformation: TransformationType,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransformationType {
    Map(HashMap<String, String>),
    Format(String),
    Custom(String),
}
    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RequestRoutingSettings {
    /// Enable custom routing
    pub enabled: bool,
    /// Routing rules
    pub rules: Vec<RoutingRule>,
    /// Default route
    pub default_route: Option<String>,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingRule {
    /// Path pattern
    pub pattern: String,
    /// Target handler
    pub handler: String,
    /// HTTP methods
    pub methods: Vec<String>,
    /// Middleware overrides
    pub middleware: Option<Vec<MiddlewareType>>,
}

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResponseHandlingSettings {
    /// Response formatting
    pub formatting: ResponseFormattingSettings,
    /// Error handling
    pub error_handling: ErrorHandlingSettings,
    /// Response transformation
    pub transformation: ResponseTransformationSettings,
    /// Response headers
    pub headers: ResponseHeadersSettings,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseFormattingSettings {
    /// Default response format
    pub default_format: ResponseFormat,
    /// Content negotiation
    pub content_negotiation: bool,
    /// Pretty print JSON
    pub pretty_json: bool,
    /// Include metadata
    pub include_metadata: bool,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseFormat {
    Json,
    Xml,
    Html,
    Text,
    Custom(String),
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorHandlingSettings {
    /// Include stack traces
    pub include_stack_trace: bool,
    /// Error response format
    pub format: HttpResponseFormat,
    /// Custom error handlers
    pub custom_handlers: HashMap<u16, ErrorHandlerConfig>,
    /// Log errors
    pub log_errors: bool,
    /// Error templates
    pub templates: HashMap<u16, String>,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorHandlerConfig {
    /// Handler implementation
    pub handler: String,
    /// Handler configuration
    pub config: HashMap<String, serde_json::Value>,
}
    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResponseTransformationSettings {
    /// Enable response transformation
    pub enabled: bool,
    /// Transformation rules
    pub rules: Vec<ResponseTransformationRule>,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseTransformationRule {
    /// Rule name
    pub name: String,
    /// Content type pattern
    pub content_type: Option<String>,
    /// Status code pattern
    pub status_code: Option<u16>,
    /// Transformation
    pub transformation: ResponseTransformationType,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseTransformationType {
    Filter(Vec<String>),
    Map(HashMap<String, String>),
    Template(String),
    Custom(String),
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseHeadersSettings {
    /// Default headers
    pub default_headers: HashMap<String, String>,
    /// Header rules
    pub header_rules: Vec<HeaderRule>,
    /// Remove headers
    pub remove_headers: Vec<String>,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderRule {
    /// Header name
    pub name: String,
    /// Header value
    pub value: String,
    /// Condition
    pub condition: Option<HeaderCondition>,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderCondition {
    /// Condition type
    pub condition_type: HeaderConditionType,
    /// Condition value
    pub value: String,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HeaderConditionType {
    Path,
    Method,
    StatusCode,
    ContentType,
    Custom(String),
}

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainManagementSettings {
    /// Execution order
    pub execution_order: Vec<MiddlewareType>,
    /// Chain optimization
    pub optimization: ChainOptimizationSettings,
    /// Error propagation
    pub error_propagation: ErrorPropagationSettings,
    /// Parallel execution
    pub parallel_execution: ParallelExecutionSettings,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainOptimizationSettings {
    /// Enable optimization
    pub enabled: bool,
    /// Skip disabled middleware
    pub skip_disabled: bool,
    /// Cache chain decisions
    pub cache_decisions: bool,
    /// Lazy initialization
    pub lazy_init: bool,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorPropagationSettings {
    /// Stop on first error
    pub stop_on_error: bool,
    /// Error aggregation
    pub aggregate_errors: bool,
    /// Rollback on error
    pub rollback_on_error: bool,
}
    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ParallelExecutionSettings {
    /// Enable parallel execution
    pub enabled: bool,
    /// Parallel middleware groups
    pub groups: Vec<ParallelGroup>,
    /// Synchronization points
    pub sync_points: Vec<MiddlewareType>,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelGroup {
    /// Group name
    pub name: String,
    /// Middleware in this group
    pub middleware: Vec<MiddlewareType>,
    /// Maximum concurrency
    pub max_concurrency: usize,
}

// ==================== SECTION ====================

impl Default for RequestParsingSettings {
    /// Returns the default instance
    fn default() -> Self { Self {
            max_size: 1024 * 1024 * 10, // 10MB
            timeout: Duration::from_secs(30),
            supported_content_types: vec![
                "application/json".to_string(),
                "application/x-www-form-urlencoded".to_string(),
                "multipart/form-data".to_string(),
                "text/plain".to_string(),
            ],
            default_encoding: "utf-8".to_string(),
            parse_query: true,
            parse_form: true,
            parse_json: true,
         }
}

impl Default for ResponseFormattingSettings {
    /// Returns the default instance
    fn default() -> Self { Self {
            default_format: ResponseFormat::Json,
            content_negotiation: true,
            pretty_json: false,
            include_metadata: false,
         }
}

impl Default for ErrorHandlingSettings {
    /// Returns the default instance
    fn default() -> Self { Self {
            include_stack_trace: false,
            format: HttpResponseFormat::Json,
            custom_handlers: HashMap::new(),
            log_errors: true,
            templates: HashMap::new(),
         }
}

impl Default for ResponseHeadersSettings {
    /// Returns the default instance
    fn default() -> Self { Self {
            default_headers: {
                let mut headers = HashMap::new();
                headers.insert("Content-Type".to_string(), "application/json".to_string());
                headers.insert(
                    "X-Powered-By".to_string(),
                    "NestGateMiddleware/2.0".to_string(),
                );
                headers
            , header_rules: Vec::new(),
            remove_headers: Vec::new() }
    }
}

impl Default for ChainManagementSettings {
    /// Returns the default instance
    fn default() -> Self { Self {
            execution_order: vec![
                MiddlewareType::Security,
                MiddlewareType::Auth,
                MiddlewareType::RateLimit,
                MiddlewareType::Cors,
                MiddlewareType::Logging,
                MiddlewareType::ErrorHandler,
            ],
            optimization: ChainOptimizationSettings::default(),
            error_propagation: ErrorPropagationSettings::default(),
            parallel_execution: ParallelExecutionSettings::default(),
         }
}

impl Default for ChainOptimizationSettings {
    /// Returns the default instance
    fn default() -> Self { Self {
            enabled: true,
            skip_disabled: true,
            cache_decisions: true,
            lazy_init: true,
         }
}

impl Default for ErrorPropagationSettings {
    /// Returns the default instance
    fn default() -> Self { Self {
            stop_on_error: true,
            aggregate_errors: false,
            rollback_on_error: false,
         }
}
