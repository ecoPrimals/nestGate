/// Unified Fuzz Configuration System for NestGate
/// This module provides standardized fuzz configuration that replaces the
/// fragmented fuzz config structs scattered across the fuzz test suite.
/// **PROBLEM SOLVED**: Eliminates duplicate fuzz config structs with
/// inconsistent fields and approaches.
use arbitrary::Arbitrary;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
// Import the standardized config pattern
use crate::unified_config_consolidation::StandardDomainConfig;
use crate::unified_enums::service_types::UnifiedServiceType};

// ==================== SECTION ====================

/// Fuzz-specific configuration extensions
/// Domain-specific fields for comprehensive fuzz testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuzzExtensions {
    /// Configuration parsing fuzzing
    pub config_parsing: FuzzConfigParsingSettings,
    /// API endpoint fuzzing
    pub api_endpoints: FuzzApiEndpointSettings,
    /// ZFS command fuzzing
    pub zfs_commands: FuzzZfsCommandSettings,
    /// Network protocol fuzzing
    pub network_protocols: FuzzNetworkProtocolSettings,
    /// Path validation fuzzing
    pub path_validation: FuzzPathValidationSettings,
    /// Serialization fuzzing
    pub serialization: FuzzSerializationSettings,
    /// Management manifest fuzzing
    pub management_manifests: FuzzBioMeOSManifestSettings,
    /// Universal adapter fuzzing
    pub universal_adapter: FuzzUniversalAdapterSettings,
    }
/// Configuration parsing fuzz settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuzzConfigParsingSettings {
    /// Configuration format to test
    pub config_format: ConfigFormat,
    /// Configuration data structure
    pub config_data: FuzzConfigData,
    /// Malicious content vectors
    pub malicious_content: Vec<MaliciousContent>,
    /// Enable resource exhaustion testing
    pub enable_resource_exhaustion: bool,
    /// Maximum parsing time allowed (seconds)
    pub max_parsing_time: u64,
    /// Maximum memory usage (MB)
    pub max_memory_mb: u64,
    }
/// API endpoint fuzz settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuzzApiEndpointSettings {
    /// HTTP request configurations
    pub http_requests: Vec<FuzzHttpRequest>,
    /// Attack vectors to test
    pub attack_vectors: Vec<AttackVector>,
    /// Enable parameter injection testing
    pub enable_parameter_injection: bool,
    /// Enable header injection testing
    pub enable_header_injection: bool,
    /// Maximum request body size
    pub max_request_body_size: usize,
    }
/// ZFS command fuzz settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuzzZfsCommandSettings {
    /// ZFS commands to test
    pub zfs_commands: Vec<FuzzZfsCommand>,
    /// Enable pool name validation testing
    pub enable_pool_validation: bool,
    /// Enable dataset name validation testing
    pub enable_dataset_validation: bool,
    /// Enable property validation testing
    pub enable_property_validation: bool,
    /// Maximum command length
    pub max_command_length: usize,
    }
/// Network protocol fuzz settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuzzNetworkProtocolSettings {
    /// Network packets to test
    pub network_packets: Vec<FuzzNetworkPacket>,
    /// Protocol types to test
    pub protocol_types: Vec<NetworkProtocolType>,
    /// Enable malformed packet testing
    pub enable_malformed_packets: bool,
    /// Maximum packet size
    pub max_packet_size: usize,
    }
/// Path validation fuzz settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuzzPathValidationSettings {
    /// File paths to test
    pub file_paths: Vec<String>,
    /// Enable path traversal testing
    pub enable_path_traversal: bool,
    /// Enable null byte injection testing
    pub enable_null_byte_injection: bool,
    /// Maximum path length
    pub max_path_length: usize,
    }
/// Serialization fuzz settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuzzSerializationSettings {
    /// Serialization formats to test
    pub serialization_formats: Vec<SerializationFormat>,
    /// Enable deserialization bomb testing
    pub enable_deserialization_bombs: bool,
    /// Enable type confusion testing
    pub enable_type_confusion: bool,
    /// Maximum serialized data size
    pub max_serialized_size: usize,
    }
/// Management manifest fuzz settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuzzBioMeOSManifestSettings {
    /// Manifest structures to test
    pub manifests: Vec<FuzzBioMeOSManifest>,
    /// Enable manifest validation testing
    pub enable_manifest_validation: bool,
    /// Enable dependency injection testing
    pub enable_dependency_injection: bool,
    /// Maximum manifest size
    pub max_manifest_size: usize,
    }
/// Universal adapter fuzz settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuzzUniversalAdapterSettings {
    /// Provider configurations to test
    pub provider_configs: Vec<FuzzProviderConfig>,
    /// Enable discovery mechanism testing
    pub enable_discovery_testing: bool,
    /// Enable capability testing
    pub enable_capability_testing: bool,
    /// Maximum configuration complexity
    pub max_config_complexity: usize,
    }
// ==================== SECTION ====================

/// Configuration formats for fuzzing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigFormat {
    Yaml,
    Json,
    Toml,
    RawString(String),
    }
/// Fuzzable configuration data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuzzConfigData {
    pub database: FuzzDatabaseConfig,
    pub logging: FuzzLoggingConfig,
    pub zfs: FuzzZfsConfigData,
    pub api: FuzzApiConfigData,
    pub network: FuzzNetworkConfigData,
    pub security: FuzzSecurityConfigData,
    pub raw_fields: HashMap<String, String>,
    }
/// Database configuration for fuzzing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuzzDatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub timeout_seconds: u64,
    pub password: String,
    pub connection_pool_size: u16,
    pub ssl_mode: String,
    }
/// Logging configuration for fuzzing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuzzLoggingConfig {
    pub level: String,
    pub max_size_mb: u64,
    pub format: String,
    pub rotation_policy: String,
    pub retention_days: u32,
    }
/// ZFS configuration data for fuzzing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuzzZfsConfigData {
    pub pool_name: String,
    pub auto_discovery: bool,
    pub health_check_interval: u64,
    pub properties: HashMap<String, String>,
    pub compression: String,
    pub deduplication: bool,
    }
/// API configuration data for fuzzing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuzzApiConfigData {
    pub host: String,
    pub port: u16,
    pub cors_origins: Vec<String>,
    pub rate_limit: u32,
    pub timeout_seconds: u64,
    }
/// Network configuration data for fuzzing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuzzNetworkConfigData {
    pub bind_endpoint: String,
    pub port_range: (u16, u16),
    pub max_connections: u32,
    pub buffer_size: usize,
    pub protocol_version: String,
    }
/// Security configuration data for fuzzing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuzzSecurityConfigData {
    pub enable_tls: bool,
    pub cipher_suites: Vec<String>,
    pub cert_validation: bool,
    pub auth_methods: Vec<String>,
    pub session_timeout: u64,
    }
/// Malicious content types for testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MaliciousContent {
    /// YAML bombs - extremely nested structures
    YamlBomb(u32),
    /// JSON with extreme nesting
    JsonBomb(u32),
    /// Billion laughs attack
    BillionLaughs,
    /// Unicode corruption
    UnicodeCorruption(Vec<u8>),
    /// Null byte injection
    NullByteInjection,
    /// Path traversal in file paths
    PathTraversal(String),
    /// SQL injection in database URLs
    SqlInjection(String),
    /// Command injection in paths
    CommandInjection(String),
    /// Extremely long strings
    ExtremelyLongString(usize),
    /// Invalid UTF-8 sequences
    InvalidUtf8(Vec<u8>),
    /// XML external entity attacks
    XxeAttack(String),
    /// Server-side template injection
    SstiAttack(String),
    /// LDAP injection
    LdapInjection(String),
    /// NoSQL injection
    NoSqlInjection(String),
    }
/// HTTP request structure for API fuzzing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuzzHttpRequest {
    pub method: HttpMethod,
    pub headers: HashMap<String, String>,
    pub query_params: HashMap<String, String>,
    pub body: FuzzRequestBody,
    pub attack_vectors: Vec<AttackVector>,
    }
/// HTTP methods for fuzzing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
    Trace,
    Connect,
    Malformed(String),
    }
/// Request body types for fuzzing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FuzzRequestBody {
    Json(String),
    FormData(HashMap<String, String>),
    Raw(Vec<u8>),
    Multipart(Vec<MultipartField>),
    Empty,
    }
/// Multipart field for request body fuzzing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultipartField {
    pub name: String,
    pub filename: Option<String>,
    pub content_type: String,
    pub data: Vec<u8>,
    }
/// Attack vectors for comprehensive security testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttackVector {
    /// Path traversal attacks
    PathTraversal(String),
    /// SQL injection in parameters
    SqlInjection(String),
    /// XSS in parameters
    XssInjection(String),
    /// Command injection
    CommandInjection(String),
    /// LDAP injection
    LdapInjection(String),
    /// XML external entity
    XxeInjection(String),
    /// Server-side template injection
    SstiInjection(String),
    /// HTTP header injection
    HeaderInjection(String, String),
    /// Extremely long parameters
    ExtremelyLongParam(String, usize),
    /// Invalid JSON structure
    InvalidJson(String),
    /// Buffer overflow attempts
    BufferOverflow(Vec<u8>),
    /// Unicode exploits
    UnicodeExploit(String),
    /// Null byte injection
    NullByteInjection(String),
    /// CRLF injection
    CrlfInjection(String),
    /// HTTP request smuggling
    RequestSmuggling(String),
    /// Deserialization attacks
    DeserializationAttack(String),
    }
/// ZFS command structure for fuzzing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuzzZfsCommand {
    pub pool_name: String,
    pub dataset_name: String,
    pub command_type: FuzzCommandType,
    pub properties: HashMap<String, String>,
    pub raw_args: Vec<String>,
    }
/// ZFS command types for fuzzing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FuzzCommandType {
    CreatePool,
    CreateDataset,
    SetProperty,
    GetProperty,
    Snapshot,
    Clone,
    Destroy,
    List,
    Status,
    RawCommand(String),
    }
/// Network packet structure for fuzzing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuzzNetworkPacket {
    pub protocol_type: NetworkProtocolType,
    pub source_port: u16,
    pub dest_port: u16,
    pub payload: Vec<u8>,
    pub headers: HashMap<String, String>,
    }
/// Network protocol types for fuzzing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkProtocolType {
    Tcp,
    Udp,
    Http,
    Https,
    Websocket,
    Custom(String),
    }
/// Serialization formats for fuzzing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SerializationFormat {
    Json,
    Yaml,
    Toml,
    MessagePack,
    Bincode,
    Protobuf,
    Custom(String),
    }
/// Management manifest structure for fuzzing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuzzBioMeOSManifest {
    pub name: String,
    pub version: String,
    pub dependencies: HashMap<String, String>,
    pub configuration: HashMap<String, serde_json::Value>,
    pub metadata: HashMap<String, String>,
    }
/// Provider configuration for universal adapter fuzzing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuzzProviderConfig {
    pub provider_type: String,
    pub endpoint: String,
    pub capabilities: Vec<String>,
    pub configuration: HashMap<String, serde_json::Value>,
    pub authentication: Option<FuzzAuthConfig>,
    }
/// Authentication configuration for fuzzing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuzzAuthConfig {
    pub auth_type: String,
    pub credentials: HashMap<String, String>,
    pub token_endpoint: Option<String>,
    pub scopes: Vec<String>,
    }
// ==================== SECTION ====================

impl Default for FuzzExtensions {
    fn default() -> Self {
        Self {
            config_parsing: FuzzConfigParsingSettings::default(),
            api_endpoints: FuzzApiEndpointSettings::default(),
            zfs_commands: FuzzZfsCommandSettings::default(),
            network_protocols: FuzzNetworkProtocolSettings::default(),
            path_validation: FuzzPathValidationSettings::default(),
            serialization: FuzzSerializationSettings::default(),
            management_manifests: FuzzBioMeOSManifestSettings::default(),
            universal_adapter: FuzzUniversalAdapterSettings::default(),
    }
    }
    }

impl Default for FuzzConfigParsingSettings {
    fn default() -> Self {
        Self {
            config_format: ConfigFormat::Json,
            config_data: FuzzConfigData::default(),
            malicious_content: Vec::new(),
            enable_resource_exhaustion: true,
            max_parsing_time: 5,
            max_memory_mb: 100,
    }
    }
    }

impl Default for FuzzConfigData {
    fn default() -> Self {
        Self {
            database: FuzzDatabaseConfig::default(),
            logging: FuzzLoggingConfig::default(),
            zfs: FuzzZfsConfigData::default(),
            api: FuzzApiConfigData::default(),
            network: FuzzNetworkConfigData::default(),
            security: FuzzSecurityConfigData::default(),
            raw_fields: HashMap::new(),
    }
    }
    }

impl Default for FuzzDatabaseConfig {
    fn default() -> Self {
        Self {
            url: format!(
// DEPRECATED: PostgreSQL database - migrate to capability-based persistence
// Capability-based discovery implemented
                "postgresql://{}:{}/test",
                crate::constants::addresses::localhost_hostname(),
                crate::constants::port_defaults::POSTGRES_DEFAULT_PORT
            ),
            max_connections: 10,
            timeout_seconds: 30,
            password: "test_password".to_string(),
            connection_pool_size: 5,
            ssl_mode: "prefer".to_string(),
    }
    }
    }

impl Default for FuzzLoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            max_size_mb: 100,
            format: "json".to_string(),
            rotation_policy: "daily".to_string(),
            retention_days: 7,
    }
    }
    }

impl Default for FuzzZfsConfigData {
    fn default() -> Self {
        Self {
            pool_name: "test-pool".to_string(),
            auto_discovery: true,
            health_check_interval: 30,
            properties: HashMap::new(),
            compression: "lz4".to_string(),
            deduplication: false,
    }
    }
    }

impl Default for FuzzApiConfigData {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 8080,
            cors_origins: vec!["*".to_string()],
            rate_limit: 1000,
            timeout_seconds: 30,
    }
    }
    }

impl Default for FuzzNetworkConfigData {
    fn default() -> Self {
        Self {
            bind_endpoint: "0.0.0.0".to_string(),
            port_range: (8000, 9000),
            max_connections: 1000,
            buffer_size: 8192,
            protocol_version: "1.0".to_string(),
    }
    }
    }

impl Default for FuzzSecurityConfigData {
    fn default() -> Self {
        Self {
            enable_tls: true,
            cipher_suites: vec!["TLS_AES_256_GCM_SHA384".to_string()],
            cert_validation: true,
            auth_methods: vec!["bearer".to_string()],
            session_timeout: 3600,
    }
    }
    }

// Default implementations for other settings structs...
impl Default for FuzzApiEndpointSettings {
    fn default() -> Self {
        Self {
            http_requests: Vec::new(),
            attack_vectors: Vec::new(),
            enable_parameter_injection: true,
            enable_header_injection: true,
            max_request_body_size: 1024 * 1024, // 1MB
    }
    }
    }

impl Default for FuzzZfsCommandSettings {
    fn default() -> Self {
        Self {
            zfs_commands: Vec::new(),
            enable_pool_validation: true,
            enable_dataset_validation: true,
            enable_property_validation: true,
            max_command_length: 4096,
    }
    }
    }

impl Default for FuzzNetworkProtocolSettings {
    fn default() -> Self {
        Self {
            network_packets: Vec::new(),
            protocol_types: vec![NetworkProtocolType::Tcp, NetworkProtocolType::Http],
            enable_malformed_packets: true,
            max_packet_size: 65536,
    }
    }
    }

impl Default for FuzzPathValidationSettings {
    fn default() -> Self {
        Self {
            file_paths: Vec::new(),
            enable_path_traversal: true,
            enable_null_byte_injection: true,
            max_path_length: 4096,
    }
    }
    }

impl Default for FuzzSerializationSettings {
    fn default() -> Self {
        Self {
            serialization_formats: vec![SerializationFormat::Json, SerializationFormat::Yaml],
            enable_deserialization_bombs: true,
            enable_type_confusion: true,
            max_serialized_size: 1024 * 1024, // 1MB
    }
    }
    }

impl Default for FuzzBioMeOSManifestSettings {
    fn default() -> Self {
        Self {
            manifests: Vec::new(),
            enable_manifest_validation: true,
            enable_dependency_injection: true,
            max_manifest_size: 1024 * 1024, // 1MB
    }
    }
    }

impl Default for FuzzUniversalAdapterSettings {
    fn default() -> Self {
        Self {
            provider_configs: Vec::new(),
            enable_discovery_testing: true,
            enable_capability_testing: true,
            max_config_complexity: 1000,
    }
    }
    }

// ==================== SECTION ====================

/// Standardized Fuzz configuration
pub type UnifiedFuzzConfig = StandardDomainConfig<FuzzExtensions>;
// ==================== SECTION ====================

impl UnifiedFuzzConfig {
    /// Create a comprehensive fuzz configuration
    #[must_use]
    pub fn comprehensive() -> Self {
        let mut config = StandardDomainConfig::with_service(
            FuzzExtensions::default(),
            "nestgate-fuzz-suite",
            env!("CARGO_PKG_VERSION"),
        );

        // Configure service settings for fuzzing
        config.service.description = "NestGate Comprehensive Fuzz Test Suite".to_string();
        config.service.service_type = UnifiedServiceType::Custom("fuzz-framework".to_string());
        config.service.environment = "fuzzing".to_string();

        // Configure fuzz-specific network settings
        config.network.port = 0; // Random port for fuzzing
        config.network.bind_endpoint = "127.0.0.1".parse().unwrap_or_else(|e| {
    tracing::error!("Expect failed ({}): {:?}", "Valid IP address", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format!("Operation failed - {}: {:?}", "Valid IP address", e)
).into())
);
        // config.network.enable_tls = false; // Field doesn't exist in current UnifiedNetworkConfig
        config.network.max_connections = 1000;

        // Configure fuzz-specific security settings
        config.security.require_auth = false; // Fuzzing typically doesn't require auth
        config.security.enable_tls = false;
        config.security.allowed_origins = vec!["*".to_string()];

        config
    }

    /// Create configuration for API endpoint fuzzing
    #[must_use]
    pub fn api_fuzzing() -> Self {
        let mut config = Self::comprehensive();
        config.service.name = "api-fuzz-test".to_string();
        config.extensions.api_endpoints.enable_parameter_injection = true;
        config.extensions.api_endpoints.enable_header_injection = true;
        config
    }

    /// Create configuration for configuration parsing fuzzing
    #[must_use]
    pub fn config_parsing_fuzzing() -> Self {
        let mut config = Self::comprehensive();
        config.service.name = "config-parsing-fuzz-test".to_string();
        config.extensions.config_parsing.enable_resource_exhaustion = true;
        config.extensions.config_parsing.max_parsing_time = 5;
        config
    }

    /// Create configuration for ZFS command fuzzing
    #[must_use]
    pub fn zfs_command_fuzzing() -> Self {
        let mut config = Self::comprehensive();
        config.service.name = "zfs-command-fuzz-test".to_string();
        config.extensions.zfs_commands.enable_pool_validation = true;
        config.extensions.zfs_commands.enable_dataset_validation = true;
        config
    }

    /// Builder pattern for custom fuzz configurations
    pub fn builder() -> UnifiedFuzzConfigBuilder {
        UnifiedFuzzConfigBuilder::new()
    }
    }

/// Builder for UnifiedFuzzConfig
pub struct UnifiedFuzzConfigBuilder {
    config: UnifiedFuzzConfig,
    }
impl UnifiedFuzzConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: UnifiedFuzzConfig::comprehensive(),
    }
    }

    #[must_use]
    pub fn test_name(mut self, name: &str) -> Self {
        self.config.service.name = name.to_string();
        self
    }

    #[must_use]
    pub fn enable_config_parsing_fuzzing(mut self, enabled: bool) -> Self {
        self.config
            .extensions
            .config_parsing
            .enable_resource_exhaustion = enabled;
        self
    }

    #[must_use]
    pub fn enable_api_fuzzing(mut self, enabled: bool) -> Self {
        self.config
            .extensions
            .api_endpoints
            .enable_parameter_injection = enabled;
        self.config.extensions.api_endpoints.enable_header_injection = enabled;
        self
    }

    #[must_use]
    pub fn enable_zfs_fuzzing(mut self, enabled: bool) -> Self {
        self.config.extensions.zfs_commands.enable_pool_validation = enabled;
        self.config
            .extensions
            .zfs_commands
            .enable_dataset_validation = enabled;
        self
    }

    #[must_use]
    pub fn max_parsing_time(mut self, seconds: u64) -> Self {
        self.config.extensions.config_parsing.max_parsing_time = seconds;
        self
    }

    pub fn build(self) -> UnifiedFuzzConfig {
        self.config
    }
    }
