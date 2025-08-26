use std::collections::HashMap;
///
/// This module contains all monitoring-related configuration types including metrics,
/// logging, alerting, and dashboards.
/// Split from unified_types/mod.rs for better maintainability and 2000-line compliance.
use crate::idiomatic_evolution::{SmartDefault, SafeResultExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// ==================== UNIFIED MONITORING CONFIGURATION ====================

/// Unified Monitoring Configuration - consolidates all monitoring settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)] // ✅ Now using derive(Default)
pub struct UnifiedMonitoringConfig {
    pub metrics: MetricsConfig,
    pub logging: LoggingConfig,
    pub tracing: TracingConfig,
    pub alerting: AlertingConfig,
    pub dashboards: DashboardConfig,
    pub enabled: bool,
    pub collection_interval_seconds: u64,
    pub retention_days: u32,
}

impl SmartDefault for UnifiedMonitoringConfig {
    fn smart_default() -> Self {
        // Can use derive(Default) since all fields implement Default
        Self::default()
    }
    
    fn can_derive_default() -> bool {
        true // All fields implement Default, so we can use derive
    }
}

// ==================== METRICS CONFIGURATION ====================

/// Metrics collection and export configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    /// Enable metrics collection
    pub enabled: bool,
    /// Metrics collection interval
    pub collection_interval: Duration,
    /// Metrics export format
    pub export_format: MetricsFormat,
    /// Metrics export endpoints
    pub export_endpoints: Vec<String>,
    /// Custom metrics definitions
    pub custom_metrics: Vec<CustomMetric>,
    /// Metrics retention period
    pub retention_period: Duration,
    /// Enable high-cardinality metrics
    pub high_cardinality: bool,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            collection_interval: Duration::from_secs(30),
            export_format: MetricsFormat::Prometheus,
            export_endpoints: vec!["http://localhost:9090".to_string()],
            custom_metrics: vec![],
            retention_period: Duration::from_secs(86400 * 7), // 7 days
            high_cardinality: false,
        }
    }
}

/// Metrics export formats
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MetricsFormat {
    /// Prometheus format
    Prometheus,
    /// OpenTelemetry format
    OpenTelemetry,
    /// StatsD format
    StatsD,
    /// InfluxDB format
    InfluxDb,
    /// Custom format
    Custom(String),
}

/// Custom metric definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomMetric {
    /// Metric name
    pub name: String,
    /// Metric type
    pub metric_type: MetricType,
    /// Metric description
    pub description: String,
    /// Metric labels
    pub labels: Vec<String>,
    /// Collection function or query
    pub collection_source: String,
}

/// Types of metrics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MetricType {
    /// Counter metric (monotonically increasing)
    Counter,
    /// Gauge metric (can go up and down)
    Gauge,
    /// Histogram metric (distribution of values)
    Histogram,
    /// Summary metric (quantiles)
    Summary,
}

// ==================== LOGGING CONFIGURATION ====================

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Enable logging
    pub enabled: bool,
    /// Log level
    pub level: LogLevel,
    /// Log format
    pub format: LogFormat,
    /// Log outputs
    pub outputs: Vec<LogOutput>,
    /// Structured logging fields
    pub structured_fields: Vec<String>,
    /// Log rotation configuration
    pub rotation: LogRotationConfig,
    /// Log filtering rules
    pub filters: Vec<LogFilter>,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            level: LogLevel::Info,
            format: LogFormat::Json,
            outputs: vec![
                LogOutput::Stdout,
                LogOutput::File("/var/log/nestgate.log".to_string()),
            ],
            structured_fields: vec![
                "timestamp".to_string(),
                "level".to_string(),
                "service".to_string(),
                "trace_id".to_string(),
            ],
            rotation: LogRotationConfig::default(),
            filters: vec![],
        }
    }
}

/// Log levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    /// Trace level (most verbose)
    Trace,
    /// Debug level
    Debug,
    /// Info level
    Info,
    /// Warn level
    Warn,
    /// Error level (least verbose)
    Error,
}

/// Log formats
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LogFormat {
    /// JSON structured format
    Json,
    /// Plain text format
    Text,
    /// Logfmt format
    Logfmt,
    /// Custom format
    Custom(String),
}

/// Log output destinations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogOutput {
    /// Standard output
    Stdout,
    /// Standard error
    Stderr,
    /// File output
    File(String),
    /// Syslog output
    Syslog,
    /// Remote logging service
    Remote(String),
    /// Database output
    Database(String),
}

/// Log rotation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogRotationConfig {
    /// Enable log rotation
    pub enabled: bool,
    /// Maximum file size before rotation
    pub max_file_size: u64,
    /// Maximum number of rotated files to keep
    pub max_files: u32,
    /// Rotation schedule
    pub schedule: RotationSchedule,
    /// Compress rotated files
    pub compress: bool,
}

impl Default for LogRotationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_file_size: 100 * 1024 * 1024, // 100MB
            max_files: 10,
            schedule: RotationSchedule::Daily,
            compress: true,
        }
    }
}

/// Log rotation schedules
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RotationSchedule {
    /// Rotate hourly
    Hourly,
    /// Rotate daily
    Daily,
    /// Rotate weekly
    Weekly,
    /// Rotate monthly
    Monthly,
    /// No scheduled rotation (size-based only)
    None,
}

/// Log filtering rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogFilter {
    /// Filter name
    pub name: String,
    /// Field to filter on
    pub field: String,
    /// Filter operation
    pub operation: FilterOperation,
    /// Filter value
    pub value: String,
    /// Action to take when filter matches
    pub action: FilterAction,
}

/// Filter operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FilterOperation {
    /// Exact match
    Equals,
    /// Contains substring
    Contains,
    /// Regex match
    Regex,
    /// Greater than (for numeric values)
    GreaterThan,
    /// Less than (for numeric values)
    LessThan,
}

/// Filter actions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FilterAction {
    /// Include the log entry
    Include,
    /// Exclude the log entry
    Exclude,
    /// Modify the log level
    ModifyLevel(LogLevel),
    /// Add additional fields
    AddFields(std::collections::HashMap<String, String>),
}

// ==================== TRACING CONFIGURATION ====================

/// Distributed tracing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracingConfig {
    /// Enable tracing
    pub enabled: bool,
    /// Tracing backend
    pub backend: TracingBackend,
    /// Sampling configuration
    pub sampling: SamplingConfig,
    /// Trace export endpoints
    pub export_endpoints: Vec<String>,
    /// Custom trace attributes
    pub custom_attributes: std::collections::HashMap<String, String>,
    /// Trace retention period
    pub retention_period: Duration,
}

impl Default for TracingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            backend: TracingBackend::Jaeger,
            sampling: SamplingConfig::default(),
            export_endpoints: vec!["http://localhost:14268/api/traces".to_string()],
            custom_attributes: std::collections::HashMap::new(),
            retention_period: Duration::from_secs(86400 * 3), // 3 days
        }
    }
}

/// Tracing backends
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TracingBackend {
    /// Jaeger tracing
    Jaeger,
    /// Zipkin tracing
    Zipkin,
    /// OpenTelemetry
    OpenTelemetry,
    /// AWS X-Ray
    XRay,
    /// Custom backend
    Custom(String),
}

/// Trace sampling configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamplingConfig {
    /// Sampling strategy
    pub strategy: SamplingStrategy,
    /// Sampling rate (0.0 to 1.0)
    pub rate: f64,
    /// Per-operation sampling rules
    pub per_operation_rules: Vec<OperationSamplingRule>,
}

impl Default for SamplingConfig {
    fn default() -> Self {
        Self {
            strategy: SamplingStrategy::Probabilistic,
            rate: 0.1, // 10% sampling
            per_operation_rules: vec![],
        }
    }
}

/// Sampling strategies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SamplingStrategy {
    /// Always sample
    Always,
    /// Never sample
    Never,
    /// Probabilistic sampling
    Probabilistic,
    /// Rate limiting sampling
    RateLimited,
    /// Adaptive sampling
    Adaptive,
}

/// Per-operation sampling rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationSamplingRule {
    /// Operation name pattern
    pub operation_pattern: String,
    /// Service name pattern
    pub service_pattern: String,
    /// Sampling rate for this rule
    pub rate: f64,
    /// Maximum traces per second
    pub max_traces_per_second: Option<u32>,
}

// ==================== ALERTING CONFIGURATION ====================

/// Alerting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertingConfig {
    /// Enable alerting
    pub enabled: bool,
    /// Alert rules
    pub rules: Vec<AlertRule>,
    /// Notification channels
    pub notification_channels: Vec<NotificationChannel>,
    /// Alert grouping configuration
    pub grouping: AlertGroupingConfig,
    /// Alert routing rules
    pub routing_rules: Vec<AlertRoutingRule>,
}

impl Default for AlertingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            rules: vec![],
            notification_channels: vec![],
            grouping: AlertGroupingConfig::default(),
            routing_rules: vec![],
        }
    }
}

/// Alert rule definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    /// Rule name
    pub name: String,
    /// Rule description
    pub description: String,
    /// Query or condition
    pub condition: String,
    /// Alert severity
    pub severity: AlertSeverity,
    /// Evaluation interval
    pub evaluation_interval: Duration,
    /// Duration condition must be true before firing
    pub for_duration: Duration,
    /// Custom labels for this alert
    pub labels: std::collections::HashMap<String, String>,
    /// Custom annotations for this alert
    pub annotations: std::collections::HashMap<String, String>,
}

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum AlertSeverity {
    /// Critical alerts require immediate attention
    Critical,
    /// High priority alerts
    High,
    /// Medium priority alerts
    Medium,
    /// Low priority alerts
    Low,
    /// Informational alerts
    Info,
}

/// Notification channel configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationChannel {
    /// Channel name
    pub name: String,
    /// Channel type
    pub channel_type: ChannelType,
    /// Channel configuration
    pub config: std::collections::HashMap<String, String>,
    /// Enable this channel
    pub enabled: bool,
}

/// Notification channel types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChannelType {
    /// Email notifications
    Email,
    /// Slack notifications
    Slack,
    /// PagerDuty notifications
    PagerDuty,
    /// Webhook notifications
    Webhook,
    /// SMS notifications
    Sms,
    /// Custom notification channel
    Custom(String),
}

/// Alert grouping configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertGroupingConfig {
    /// Enable alert grouping
    pub enabled: bool,
    /// Fields to group alerts by
    pub group_by: Vec<String>,
    /// Group wait time before sending first notification
    pub group_wait: Duration,
    /// Group interval for sending additional notifications
    pub group_interval: Duration,
    /// Repeat interval for ongoing alerts
    pub repeat_interval: Duration,
}

impl Default for AlertGroupingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            group_by: vec!["alertname".to_string(), "service".to_string()],
            group_wait: Duration::from_secs(30),
            group_interval: Duration::from_secs(300), // 5 minutes
            repeat_interval: Duration::from_secs(3600), // 1 hour
        }
    }
}

/// Alert routing rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRoutingRule {
    /// Rule name
    pub name: String,
    /// Matching conditions
    pub matchers: Vec<AlertMatcher>,
    /// Target notification channels
    pub channels: Vec<String>,
    /// Continue processing other rules
    pub continue_processing: bool,
}

/// Alert matcher for routing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertMatcher {
    /// Label name to match
    pub label: String,
    /// Match operation
    pub operation: MatchOperation,
    /// Value to match against
    pub value: String,
}

/// Match operations for alert routing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MatchOperation {
    /// Exact match
    Equals,
    /// Regex match
    Regex,
    /// Not equals
    NotEquals,
    /// Not regex match
    NotRegex,
}

// ==================== DASHBOARD CONFIGURATION ====================

/// Dashboard configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    /// Enable dashboards
    pub enabled: bool,
    /// Dashboard provider
    pub provider: DashboardProvider,
    /// Dashboard definitions
    pub dashboards: Vec<Dashboard>,
    /// Auto-refresh interval
    pub refresh_interval: Duration,
    /// Dashboard access configuration
    pub access: DashboardAccessConfig,
}

impl Default for DashboardConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            provider: DashboardProvider::Grafana,
            dashboards: vec![],
            refresh_interval: Duration::from_secs(30),
            access: DashboardAccessConfig::default(),
        }
    }
}

/// Dashboard providers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DashboardProvider {
    /// Grafana dashboards
    Grafana,
    /// Kibana dashboards
    Kibana,
    /// Custom dashboard provider
    Custom(String),
}

/// Dashboard definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dashboard {
    /// Dashboard name
    pub name: String,
    /// Dashboard description
    pub description: String,
    /// Dashboard panels
    pub panels: Vec<DashboardPanel>,
    /// Dashboard tags
    pub tags: Vec<String>,
    /// Dashboard variables
    pub variables: std::collections::HashMap<String, String>,
}

/// Dashboard panel definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardPanel {
    /// Panel title
    pub title: String,
    /// Panel type
    pub panel_type: PanelType,
    /// Data source query
    pub query: String,
    /// Panel position and size
    pub layout: PanelLayout,
    /// Panel-specific configuration
    pub config: std::collections::HashMap<String, serde_json::Value>,
}

/// Dashboard panel types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PanelType {
    /// Line graph
    Graph,
    /// Single stat
    SingleStat,
    /// Table
    Table,
    /// Heatmap
    Heatmap,
    /// Alert list
    AlertList,
    /// Custom panel type
    Custom(String),
}

/// Panel layout configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelLayout {
    /// X position
    pub x: u32,
    /// Y position
    pub y: u32,
    /// Width
    pub width: u32,
    /// Height
    pub height: u32,
}

/// Dashboard access configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardAccessConfig {
    /// Enable authentication
    pub require_auth: bool,
    /// Allowed roles
    pub allowed_roles: Vec<String>,
    /// Public dashboards
    pub public_dashboards: Vec<String>,
    /// Dashboard sharing settings
    pub sharing: DashboardSharingConfig,
}

impl Default for DashboardAccessConfig {
    fn default() -> Self {
        Self {
            require_auth: true,
            allowed_roles: vec!["admin".to_string(), "operator".to_string()],
            public_dashboards: vec![],
            sharing: DashboardSharingConfig::default(),
        }
    }
}

/// Dashboard sharing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardSharingConfig {
    /// Enable dashboard sharing
    pub enabled: bool,
    /// Share link expiration
    pub link_expiration: Duration,
    /// Require password for shared links
    pub require_password: bool,
    /// Allow anonymous access to shared links
    pub allow_anonymous: bool,
}

impl Default for DashboardSharingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            link_expiration: Duration::from_secs(86400 * 7), // 7 days
            require_password: true,
            allow_anonymous: false,
        }
    }
}
