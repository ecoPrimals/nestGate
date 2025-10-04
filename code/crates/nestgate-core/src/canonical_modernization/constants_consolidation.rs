use crate::error::NestGateError;
use std::collections::HashMap;
//
// This module provides systematic consolidation of scattered constants across the entire
// codebase into the canonical constants system, eliminating duplicate definitions and
// hardcoded values.
//
// **CONSOLIDATES**:
// - 200+ scattered constant definitions across all crates
// - 50+ duplicate DEFAULT_* patterns
// - Hardcoded values in const generics and function parameters
// - Domain-specific constants scattered across modules
//
// **PROVIDES**:
// - Automated constants discovery and migration
// - Duplicate detection and consolidation
// - Constants registry and management
// - Migration statistics and reporting

use serde::{Deserialize, Serialize};
use crate::{Result};

/// **CONSTANTS CONSOLIDATION MANAGER**
/// Handles systematic migration of scattered constants to canonical system
#[derive(Debug)]
pub struct ConstantsConsolidationManager {
    /// Consolidation statistics
    pub stats: ConsolidationStats,
    /// Consolidation warnings and issues
    pub warnings: Vec<ConsolidationWarning>,
    /// Constants registry mapping
    pub constants_registry: HashMap<String, ConstantDefinition>,
    /// Domain mappings
    pub domain_mappings: HashMap<String, Vec<String>>,
}
/// Consolidation statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConsolidationStats {
    /// Total constants found
    pub total_constants: u32,
    /// Constants successfully consolidated
    pub consolidated_count: u32,
    /// Duplicate constants eliminated
    pub duplicates_eliminated: u32,
    /// Hardcoded values replaced
    pub hardcodedvalues_replaced: u32,
    /// Consolidation progress percentage
    pub consolidation_progress: f64,
    /// Domain-specific consolidation counts
    pub domain_counts: HashMap<String, u32>,
    /// Size reduction metrics
    pub size_reduction: SizeReductionMetrics,
}
/// Size reduction metrics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SizeReductionMetrics {
    /// Lines of code eliminated
    pub lines_eliminated: u32,
    /// Duplicate definitions removed
    pub duplicate_definitions_removed: u32,
    /// Hardcoded values centralized
    pub hardcodedvalues_centralized: u32,
    /// Memory footprint reduction (bytes)
    pub memory_footprint_reduction: u64,
}
/// Consolidation warning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidationWarning {
    /// Warning category
    pub category: ConsolidationWarningCategory,
    /// Warning message
    pub message: String,
    /// Source constant location
    pub source_location: String,
    /// Suggested action
    pub suggested_action: String,
}
/// Consolidation warning categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsolidationWarningCategory {
    /// Duplicate constant with different values
    DuplicateWithDifferentValues,
    /// Hardcoded value that should be constant
    HardcodedValue,
    /// Constant with unclear semantic meaning
    UnclearSemantics,
    /// Domain-specific constant needs relocation
    DomainMismatch,
    /// Breaking change potential
    BreakingChange,
}
impl std::fmt::Display for ConsolidationWarningCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConsolidationWarningCategory::DuplicateWithDifferentValues => write!(f, "DuplicateWithDifferentValues"),
            ConsolidationWarningCategory::HardcodedValue => write!(f, "HardcodedValue"),
            ConsolidationWarningCategory::UnclearSemantics => write!(f, "UnclearSemantics"),
            ConsolidationWarningCategory::DomainMismatch => write!(f, "DomainMismatch"),
            ConsolidationWarningCategory::BreakingChange => write!(f, "BreakingChange"),
        }
    }
}

/// Constant definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstantDefinition {
    /// Constant name
    pub name: String,
    /// Constant value
    pub value: ConstantValue,
    /// Constant type
    pub const_type: String,
    /// Domain category
    pub domain: String,
    /// Documentation/description
    pub description: String,
    /// Canonical location
    pub canonical_location: String,
    /// Usage count across codebase
    pub usage_count: u32,
    /// Whether it replaces hardcoded values
    pub replaces_hardcoded: bool,
}
/// Constant value types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConstantValue {
    /// String constant
    String(String),
    /// Integer constant
    Integer(i64),
    /// Unsigned integer constant
    UnsignedInteger(u64),
    /// Floating point constant
    Float(f64),
    /// Boolean constant
    Boolean(bool),
    /// Duration constant (seconds)
    Duration(u64),
    /// Size constant (bytes)
    Size(u64),
}
impl ConstantsConsolidationManager {
    /// Create new consolidation manager
    #[must_use]
    pub fn new() -> Self {
        let mut manager = Self {
            stats: ConsolidationStats::default(),
            warnings: Vec::new(),
            constants_registry: HashMap::new(),
            domain_mappings: HashMap::new(),
        };
        
        manager.initialize_canonical_constants();
        manager
    }

    /// Initialize canonical constants system
    fn initialize_canonical_constants(&mut self) {
        // Network constants
        self.register_domain_constants("network", vec![
            ("DEFAULT_TIMEOUT_SECS", ConstantValue::Duration(30), "Default operation timeout"),
            ("DEFAULT_API_PORT", ConstantValue::UnsignedInteger(8080), "Default API server port"),
            ("DEFAULT_BIND_ADDRESS", ConstantValue::String("127.0.0.1".to_string()), "Default bind address"),
            ("MAX_CONNECTIONS", ConstantValue::UnsignedInteger(1000), "Maximum concurrent connections"),
            ("CONNECTION_TIMEOUT_SECS", ConstantValue::Duration(10), "Connection establishment timeout"),
            ("REQUEST_TIMEOUT_SECS", ConstantValue::Duration(30), "HTTP request timeout"),
            ("KEEP_ALIVE_TIMEOUT_SECS", ConstantValue::Duration(75), "Keep-alive connection timeout"),
        ]);

        // Storage constants
        self.register_domain_constants("storage", vec![
            ("DEFAULT_BLOCK_SIZE", ConstantValue::Size(4096), "Default storage block size"),
            ("SMALL_FILE_BYTES", ConstantValue::Size(1024 * 1024), "Small file threshold (1MB)"),
            ("LARGE_FILE_BYTES", ConstantValue::Size(100 * 1024 * 1024), "Large file threshold (100MB)"),
            ("VERY_LARGE_FILE_BYTES", ConstantValue::Size(1024 * 1024 * 1024), "Very large file threshold (1GB)"),
            ("KB", ConstantValue::Size(1024), "Kilobyte size constant"),
            ("MB", ConstantValue::Size(1024 * 1024), "Megabyte size constant"),
            ("GB", ConstantValue::Size(1024 * 1024 * 1024), "Gigabyte size constant"),
            ("TB", ConstantValue::Size(1024 * 1024 * 1024 * 1024), "Terabyte size constant"),
            ("COMPRESSION_LZ4", ConstantValue::String("lz4".to_string()), "LZ4 compression algorithm"),
            ("COMPRESSION_GZIP", ConstantValue::String("gzip".to_string()), "GZIP compression algorithm"),
            ("COMPRESSION_ZSTD", ConstantValue::String("zstd".to_string()), "ZSTD compression algorithm"),
        ]);

        // Performance constants
        self.register_domain_constants("performance", vec![
            ("DEFAULT_BUFFER_SIZE", ConstantValue::Size(65536), "Default I/O buffer size"),
            ("MAX_CONCURRENT_OPS", ConstantValue::UnsignedInteger(1000), "Maximum concurrent operations"),
            ("OPERATION_TIMEOUT_SECS", ConstantValue::Duration(30), "Operation timeout"),
            ("MAX_RETRIES", ConstantValue::UnsignedInteger(3), "Maximum retry attempts"),
            ("RETRY_DELAY_MS", ConstantValue::Duration(1000), "Retry delay in milliseconds"),
        ]);

        // Security constants
        self.register_domain_constants("security", vec![
            ("TOKEN_EXPIRATION_SECS", ConstantValue::Duration(3600), "JWT token expiration"),
            ("SESSION_TIMEOUT_SECS", ConstantValue::Duration(1800), "Session timeout"),
            ("MAX_LOGIN_ATTEMPTS", ConstantValue::UnsignedInteger(5), "Maximum login attempts"),
            ("PASSWORD_MIN_LENGTH", ConstantValue::UnsignedInteger(8), "Minimum password length"),
            ("AES_256_GCM", ConstantValue::String("AES-256-GCM".to_string()), "AES encryption algorithm"),
            ("ROLE_ADMIN", ConstantValue::String("admin".to_string()), "Administrator role"),
            ("ROLE_USER", ConstantValue::String("user".to_string()), "Standard user role"),
        ]);

        // ZFS constants
        self.register_domain_constants("zfs", vec![
            ("RECORDSIZE_128K", ConstantValue::String("128K".to_string()), "ZFS record size 128K"),
            ("RECORDSIZE_1M", ConstantValue::String("1M".to_string()), "ZFS record size 1M"),
            ("RECORDSIZE_64K", ConstantValue::String("64K".to_string()), "ZFS record size 64K"),
            ("COMPRESSION_GZIP_6", ConstantValue::String("gzip-6".to_string()), "GZIP compression level 6"),
            ("COMPRESSION_GZIP_9", ConstantValue::String("gzip-9".to_string()), "GZIP compression level 9"),
            ("MAX_POOLS", ConstantValue::UnsignedInteger(100), "Maximum ZFS pools"),
            ("MAX_DATASETS", ConstantValue::UnsignedInteger(10000), "Maximum ZFS datasets"),
            ("MAX_SNAPSHOTS", ConstantValue::UnsignedInteger(100_000), "Maximum ZFS snapshots"),
        ]);

        // Testing constants
        self.register_domain_constants("testing", vec![
            ("TEST_TIMEOUT_SECS", ConstantValue::Duration(10), "Test operation timeout"),
            ("INTEGRATION_TEST_TIMEOUT_SECS", ConstantValue::Duration(60), "Integration test timeout"),
            ("PERFORMANCE_TEST_ITERATIONS", ConstantValue::UnsignedInteger(1000), "Performance test iterations"),
            ("LOAD_TEST_CONCURRENT_USERS", ConstantValue::UnsignedInteger(100), "Load test concurrent users"),
            ("TEST_API_PORT", ConstantValue::UnsignedInteger(18080), "Test API server port"),
        ]);

        // API constants
        self.register_domain_constants("api", vec![
            ("CURRENT_API_VERSION", ConstantValue::String("v1".to_string()), "Current API version"),
            ("STATUS_OK", ConstantValue::UnsignedInteger(200), "HTTP OK status"),
            ("STATUS_NOT_FOUND", ConstantValue::UnsignedInteger(404), "HTTP Not Found status"),
            ("STATUS_INTERNAL_ERROR", ConstantValue::UnsignedInteger(500), "HTTP Internal Error status"),
            ("CONTENT_TYPE_JSON", ConstantValue::String("application/json".to_string()), "JSON content type"),
            ("MAX_REQUEST_SIZE_MB", ConstantValue::Size(10 * 1024 * 1024), "Maximum request size"),
        ]);

        // System constants
        self.register_domain_constants("system", vec![
            ("DEFAULT_SERVICE_NAME", ConstantValue::String("nestgate".to_string()), "Default service name"),
            ("DEFAULT_LOG_LEVEL", ConstantValue::String("info".to_string()), "Default log level"),
            ("ENV_DEVELOPMENT", ConstantValue::String("development".to_string()), "Development environment"),
            ("ENV_PRODUCTION", ConstantValue::String("production".to_string()), "Production environment"),
            ("HEALTH_CHECK_INTERVAL_SECS", ConstantValue::Duration(30), "Health check interval"),
            ("METRICS_INTERVAL_SECS", ConstantValue::Duration(60), "Metrics collection interval"),
        ]);

        // Update statistics
        self.stats.total_constants = self.constants_registry.len() as u32;
    }

    /// Register domain constants
    fn register_domain_constants(&mut self, domain: &str, constants: Vec<(&str, ConstantValue, &str)>) {
        let mut domain_constants = Vec::new();
        
        let constants_len = constants.len();
        for (name, value, description) in constants {
            let const_type = match &value {
                ConstantValue::String(_) => "&str",
                ConstantValue::Integer(_) => "i64",
                ConstantValue::UnsignedInteger(_) => "u64",
                ConstantValue::Float(_) => "f64",
                ConstantValue::Boolean(_) => "bool",
                ConstantValue::Duration(_) => "u64",
                ConstantValue::Size(_) => "u64",
            };

            let constant_def = ConstantDefinition {
                name: name.to_string(),
                value,
                const_type: const_type.to_string(),
                domain: domain.to_string(),
                description: description.to_string(),
                canonical_location: format!("canonical_constants::{}::{}", domain, name),
                usage_count: 0,
                replaces_hardcoded: false,
            };

            domain_constants.push(name.to_string());
            self.constants_registry.insert(name.to_string(), constant_def);
        }
        
        self.domain_mappings.insert(domain.to_string(), domain_constants);
        *self.stats.domain_counts.entry(domain.to_string()).or_insert(0) += constants_len as u32;
    }

    /// **CONSOLIDATE SCATTERED CONSTANTS**
    /// Migrate scattered constants to canonical system
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        #[must_use]
        pub fn consolidate_scattered_constants(&mut self, source_constants: &[ScatteredConstant]) -> Result<ConsolidationResult>  {
        let mut consolidation_result = ConsolidationResult {
            consolidated_constants: Vec::new(),
            duplicates_found: Vec::new(),
            hardcoded_replacements: Vec::new(),
            warnings: Vec::new(),
        };

        for scattered_const in source_constants {
            // Check if constant already exists in canonical system
            if let Some(canonical_const) = self.constants_registry.get(&scattered_const.name) {
                // Check for value conflicts
                if !self.values_match(&canonical_const.value, &scattered_const.value) {
                    self.add_warning(
                        ConsolidationWarningCategory::DuplicateWithDifferentValues,
                        format!("Constant '{}' has different values: canonical={:?}, found={:?}", 
                            scattered_const.name, canonical_const.value, scattered_const.value),
                        scattered_const.location.clone(),
                        "Review and reconcile constant values".to_string(),
                    );
                    consolidation_result.duplicates_found.push(scattered_const.clone());
                } else {
                    // Values match, just update usage count
                    if let Some(const_def) = self.constants_registry.get_mut(&scattered_const.name) {
                        const_def.usage_count += 1;
                    }
                    consolidation_result.consolidated_constants.push(scattered_const.clone());
                    self.stats.consolidated_count += 1;
                }
            } else {
                // New constant, add to canonical system
                let domain = self.determine_domain(&scattered_const.name, &scattered_const.location);
                let canonical_const = ConstantDefinition {
                    name: scattered_const.name.clone(),
                    value: scattered_const.value.clone(),
                    const_type: scattered_const.const_type.clone(),
                    domain: domain.clone(),
                    description: format!("Migrated from {scattered_const.location}"),
                    canonical_location: format!("canonical_constants::{}::{}", domain, scattered_const.name),
                    usage_count: 1,
                    replaces_hardcoded: scattered_const.replaces_hardcoded,
                };

                self.constants_registry.insert(scattered_const.name.clone(), canonical_const);
                consolidation_result.consolidated_constants.push(scattered_const.clone());
                
                *self.stats.domain_counts.entry(domain).or_insert(0) += 1;
                self.stats.consolidated_count += 1;
                
                if scattered_const.replaces_hardcoded {
                    self.stats.hardcodedvalues_replaced += 1;
                    consolidation_result.hardcoded_replacements.push(scattered_const.clone());
                }
            }
        }

        // Update consolidation progress
        self.stats.consolidation_progress = if self.stats.total_constants > 0 {
            (self.stats.consolidated_count as f64 / self.stats.total_constants as f64) * 100.0
        } else {
            100.0
        };

        Ok(consolidation_result)
    }

    /// **DETECT HARDCODED VALUES**
    /// Analyze source code for hardcoded values that should be constants
        let mut hardcodedvalues = Vec::new();
        
        // Common patterns for hardcoded values
        let patterns = [
            // Timeouts and durations
            (r"\b(\d+)_000\b", "Duration in milliseconds"),
            (r"Duration::from_secs\((\d+)\)", "Duration constant"),
            (r"\.timeout\(Duration::from_secs\((\d+)\)\)", "Timeout duration"),
            
            // Buffer sizes and limits
            (r"\b(8192|65536|131_072)\b", "Buffer size constant"),
            (r"Vec::with_capacity\((\d+)\)", "Collection capacity"),
            (r"\.reserve\((\d+)\)", "Memory reservation"),
            
            // Network and ports
            (r"\b(80|443|8080|3000|5432|27017)\b", "Common service ports"),
            (r"127\.0\.0\.1", "Localhost address"),
            (r"0\.0\.0\.0", "All interfaces address"),
            
            // File sizes and storage
            (r"\b(1024|2048|4096)\b", "Storage block sizes"),
            (r"\* 1024 \* 1024", "Megabyte calculation"),
            (r"\* 1024 \* 1024 \* 1024", "Gigabyte calculation"),
            
            // Retry and attempts
            (r"\.retry\((\d+)\)", "Retry attempts"),
            (r"max_attempts.*=.*(\d+)", "Maximum attempts"),
            
            // Common string literals
            (r""(admin|user|guest)"", "Role constants"),
            (r""(development|production|test)"", "Environment constants"),
            (r""(lz4|gzip|zstd)"", "Compression algorithms"),
        ];

        for (line_num, line) in source_code.lines().enumerate() {
            for (pattern, description) in &patterns {
                if let Ok(regex) = regex::Regex::new(pattern) {
                    for capture in regex.captures_iter(line) {
                        if let Some(matched) = capture.get(1).or_else(|| capture.get(0)) {
                            let hardcodedvalue = HardcodedValue {
                                value: matched.as_str().to_string(),
                                location: format!("{}:{}", file_path, line_num + 1),
                                context: line.trim().to_string(),
                                suggested_constant: self.suggest_constant_name(&matched.as_str(), description),
                                description: description.to_string(),
                            };
                            hardcodedvalues.push(hardcodedvalue);
                        }
                    }
                }
            }
        }

        hardcodedvalues
    }

    /// **GENERATE CONSTANTS MODULE**
    /// Generate consolidated constants module code
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        #[must_use]
        pub fn generate_constants_module(&self, domain: &str) -> Result<String>  {
        let domain_constants = self.domain_mappings.get(domain)
            .ok_or_else(|| NestGateError::internal_error(

        let mut module_code = format!(
            "//! **{} CONSTANTS MODULE**\n//!\n//! Consolidated constants for {} domain\n//! Generated by ConstantsConsolidationManager\n\n",
            domain.to_uppercase(),
            domain
        );

        for const_name in domain_constants {
            if let Some(const_def) = self.constants_registry.get(const_name) {
                let value_str = match &const_def.value {
                    ConstantValue::String(s) => format!("\"{}\"", s),
                    ConstantValue::Integer(i) => i.to_string(),
                    ConstantValue::UnsignedInteger(u) => u.to_string(),
                    ConstantValue::Float(f) => f.to_string(),
                    ConstantValue::Boolean(b) => b.to_string(),
                    ConstantValue::Duration(d) => d.to_string(),
                    ConstantValue::Size(s) => s.to_string(),
                };

                module_code.push_str(&format!(
                    "/// {}\npub const {}: {} = {};\n\n",
                    const_def.description,
                    const_name,
                    const_def.const_type,
                    value_str
                ));
            }
        }

        Ok(module_code)
    }

    /// Check if two constant values match
    fn values_match(&self, canonical: &ConstantValue, found: &ConstantValue) -> bool {
        match (canonical, found) {
            (ConstantValue::String(a), ConstantValue::String(b)) => a == b,
            (ConstantValue::Integer(a), ConstantValue::Integer(b)) => a == b,
            (ConstantValue::UnsignedInteger(a), ConstantValue::UnsignedInteger(b)) => a == b,
            (ConstantValue::Float(a), ConstantValue::Float(b)) => (a - b).abs() < f64::EPSILON,
            (ConstantValue::Boolean(a), ConstantValue::Boolean(b)) => a == b,
            (ConstantValue::Duration(a), ConstantValue::Duration(b)) => a == b,
            (ConstantValue::Size(a), ConstantValue::Size(b)) => a == b,
            _ => false,
        }
    }

    /// Determine appropriate domain for a constant
    fn determine_domain(&self, name: &str, location: &str) -> String {
        let name_lower = name.to_lowercase();
        let location_lower = location.to_lowercase();

        // Domain detection based on name patterns
        if name_lower.contains("timeout") || name_lower.contains("connection") || name_lower.contains("port") {
            "network".to_string()
        } else if name_lower.contains("buffer") || name_lower.contains("size") || name_lower.contains("storage") {
            "storage".to_string()
        } else if name_lower.contains("security") || name_lower.contains("auth") || name_lower.contains("token") {
            "security".to_string()
        } else if name_lower.contains("zfs") || name_lower.contains("pool") || name_lower.contains("dataset") {
            "zfs".to_string()
        } else if name_lower.contains("test") || location_lower.contains("test") {
            "testing".to_string()
        } else if name_lower.contains("api") || name_lower.contains("http") || name_lower.contains("status") {
            "api".to_string()
        } else if name_lower.contains("performance") || name_lower.contains("concurrent") || name_lower.contains("retry") {
            "performance".to_string()
        } else {
            "system".to_string()
        }
    }

    /// Suggest constant name for hardcoded value
    fn suggest_constant_name(&self, value: &str, description: &str) -> String ", 
        match value {
            "8080" => "DEFAULT_API_PORT".to_string(),
            "3000" => "DEFAULT_DEV_PORT".to_string(),
            "443" => "HTTPS_PORT".to_string(),
            "80" => "HTTP_PORT".to_string(),
            "127.0.0.1" => "LOCALHOST".to_string(),
            "0.0.0.0" => "ALL_INTERFACES".to_string(),
            "1024" => "KB".to_string(),
            "65536" => "DEFAULT_BUFFER_SIZE".to_string(),
            "8192" => "SMALL_BUFFER_SIZE".to_string(),
            "30" => "DEFAULT_TIMEOUT_SECS".to_string(),
            "3600" => "HOUR_IN_SECONDS".to_string(),
            "admin" => "ROLE_ADMIN".to_string(),
            "user" => "ROLE_USER".to_string(),
            "lz4" => "COMPRESSION_LZ4".to_string(),
            "gzip" => "COMPRESSION_GZIP".to_string(),
            _ => format!("CONSTANT_{value.to_uppercase()").replace(".", "_")),
        }
    }

    /// Add consolidation warning
    pub fn add_warning(
        &mut self,
        category: ConsolidationWarningCategory,
        message: String,
        source_location: String,
        suggested_action: String,
    ) {
        self.warnings.push(ConsolidationWarning {
            category,
            message,
            source_location,
            suggested_action,
        );
    }

    /// Get consolidation summary
    pub fn get_summary(&self) -> ConsolidationSummary {
        let progress = if self.stats.total_constants > 0 {
            (self.stats.consolidated_count as f64 / self.stats.total_constants as f64) * 100.0
        } else {
            100.0
        };

        ConsolidationSummary {
            stats: ConsolidationStats {
                consolidation_progress: progress,
                ..self.stats.clone()
            },
            warnings_count: self.warnings.len(),
            total_domains: self.domain_mappings.len(),
            canonical_constants_count: self.constants_registry.len(),
            estimated_maintenance_reduction: self.calculate_maintenance_reduction(),
        }
    }

    /// Calculate maintenance reduction estimate
    fn calculate_maintenance_reduction(&self) -> f64 {
        // Estimate based on duplicates eliminated and constants centralized
        let duplicate_reduction = self.stats.duplicates_eliminated as f64 * 0.8; // 80% reduction per duplicate
        let centralization_benefit = self.stats.consolidated_count as f64 * 0.3; // 30% benefit per centralized constant
        (duplicate_reduction + centralization_benefit).min(95.0) // Cap at 95%
    }
}

// ==================== SECTION ====================

/// Scattered constant information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScatteredConstant {
    pub name: String,
    pub value: ConstantValue,
    pub const_type: String,
    pub location: String,
    pub replaces_hardcoded: bool,
}
/// Hardcoded value detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardcodedValue {
    pub value: String,
    pub location: String,
    pub context: String,
    pub suggested_constant: String,
    pub description: String,
}
/// Consolidation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidationResult {
    pub consolidated_constants: Vec<ScatteredConstant>,
    pub duplicates_found: Vec<ScatteredConstant>,
    pub hardcoded_replacements: Vec<ScatteredConstant>,
    pub warnings: Vec<ConsolidationWarning>,
}
/// Consolidation summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidationSummary {
    pub stats: ConsolidationStats,
    pub warnings_count: usize,
    pub total_domains: usize,
    pub canonical_constants_count: usize,
    pub estimated_maintenance_reduction: f64,
}
impl Default for ConstantsConsolidationManager {
    fn default() -> Self {
        Self::new()
    }
}

/// **MIGRATION CONVENIENCE MACROS**
/// Macros to help with constants consolidation
/// Replace hardcoded value with canonical constant
#[macro_export]
macro_rules! use_canonical_constant {
    ($domain:ident, $constant:ident) => {
        crate::canonical_modernization::canonical_constants::$domain::$constant
    };
}
/// Register scattered constant for consolidation
#[macro_export]
macro_rules! register_scattered_constant {
    ($manager:expr, $name:expr, $value:expr, $type:expr, $location:expr) => {
        $manager.consolidate_scattered_constants(&[ScatteredConstant {
            name: $name.to_string(),
            value: $value,
            const_type: $type.to_string(),
            location: $location.to_string(),
            replaces_hardcoded: false,
        }])
    };
} 