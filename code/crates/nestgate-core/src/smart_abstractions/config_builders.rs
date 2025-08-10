/// **SMART CONFIGURATION BUILDERS MODULE**
///
/// This module provides intelligent configuration builders that eliminate
/// the need for scattered configuration helper functions and reduce boilerplate.
///
/// **ABSORBS COMPLEXITY FROM**:
/// - Various config builder patterns across crates
/// - Scattered configuration validation logic
/// - Duplicate environment loading patterns
/// - Manual configuration merging functions
///
/// **PROVIDES**:
/// - Generic configuration builder trait
/// - Smart validation patterns
/// - Environment-driven configuration loading
/// - Configuration merging and composition
use crate::error::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::time::Duration;

/// Type alias for validator functions to reduce complexity
pub type ValidatorFn<T> = Box<dyn Fn(&T) -> Result<()> + Send + Sync>;

// ==================== SMART CONFIGURATION BUILDER TRAIT ====================

/// **SMART CONFIGURATION BUILDER TRAIT**
/// Generic trait for intelligent configuration builders with validation and environment loading
pub trait SmartConfigBuilder<T>: Default + Clone + Debug + Send + Sync
where
    T: Clone + Debug + Send + Sync,
{
    /// Build the configuration with validation
    fn build(self) -> Result<T>;

    /// Build with environment variable overrides
    fn build_with_env(self, prefix: &str) -> Result<T>;

    /// Validate the current builder state
    fn validate(&self) -> Result<()>;

    /// Load from environment variables with prefix
    fn from_env(prefix: &str) -> Result<Self>;

    /// Merge with another builder
    fn merge(self, other: Self) -> Self;

    /// Set a configuration value by key
    fn set_value(self, key: &str, value: serde_json::Value) -> Self;

    /// Get configuration schema for documentation
    fn schema() -> serde_json::Value;
}

// ==================== SMART ENVIRONMENT LOADER ====================

/// **SMART ENVIRONMENT LOADER**
/// Intelligent environment variable loading with type conversion and validation
pub struct SmartEnvLoader {
    prefix: String,
    separator: String,
    case_sensitive: bool,
}

impl SmartEnvLoader {
    /// Create a new environment loader with prefix
    pub fn new(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_uppercase(),
            separator: "_".to_string(),
            case_sensitive: false,
        }
    }

    /// Set the separator for nested keys
    pub fn with_separator(mut self, separator: &str) -> Self {
        self.separator = separator.to_string();
        self
    }

    /// Enable case-sensitive matching
    pub fn case_sensitive(mut self, enabled: bool) -> Self {
        self.case_sensitive = enabled;
        self
    }

    /// Load a string value with default
    pub fn load_string(&self, key: &str, default: Option<&str>) -> String {
        let env_key = self.make_env_key(key);
        env::var(&env_key).unwrap_or_else(|_| default.unwrap_or("").to_string())
    }

    /// Load a numeric value with default
    pub fn load_number<T>(&self, key: &str, default: T) -> T
    where
        T: std::str::FromStr + Copy,
    {
        let env_key = self.make_env_key(key);
        env::var(&env_key)
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    }

    /// Load a boolean value with default
    pub fn load_bool(&self, key: &str, default: bool) -> bool {
        let env_key = self.make_env_key(key);
        env::var(&env_key)
            .ok()
            .and_then(|v| match v.to_lowercase().as_str() {
                "true" | "1" | "yes" | "on" => Some(true),
                "false" | "0" | "no" | "off" => Some(false),
                _ => None,
            })
            .unwrap_or(default)
    }

    /// Load a duration value with default
    pub fn load_duration(&self, key: &str, default: Duration) -> Duration {
        let env_key = self.make_env_key(key);
        env::var(&env_key)
            .ok()
            .and_then(|v| {
                if v.ends_with("ms") {
                    v.strip_suffix("ms")
                        .and_then(|n| n.parse::<u64>().ok())
                        .map(Duration::from_millis)
                } else if v.ends_with("s") {
                    v.strip_suffix("s")
                        .and_then(|n| n.parse::<u64>().ok())
                        .map(Duration::from_secs)
                } else if v.ends_with("m") {
                    v.strip_suffix("m")
                        .and_then(|n| n.parse::<u64>().ok())
                        .map(|n| Duration::from_secs(n * 60))
                } else if v.ends_with("h") {
                    v.strip_suffix("h")
                        .and_then(|n| n.parse::<u64>().ok())
                        .map(|n| Duration::from_secs(n * 3600))
                } else {
                    v.parse::<u64>().ok().map(Duration::from_secs)
                }
            })
            .unwrap_or(default)
    }

    /// Load a list of strings
    pub fn load_string_list(&self, key: &str, default: Vec<String>) -> Vec<String> {
        let env_key = self.make_env_key(key);
        env::var(&env_key)
            .ok()
            .map(|v| v.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or(default)
    }

    /// Load all environment variables with the prefix
    pub fn load_all(&self) -> HashMap<String, String> {
        let prefix_with_sep = format!("{}{}", self.prefix, self.separator);

        env::vars()
            .filter_map(|(key, value)| {
                if key.starts_with(&prefix_with_sep) {
                    let config_key = key.strip_prefix(&prefix_with_sep)?;
                    Some((config_key.to_lowercase(), value))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Create environment key from config key
    fn make_env_key(&self, key: &str) -> String {
        let key = if self.case_sensitive {
            key.to_string()
        } else {
            key.to_uppercase()
        };
        format!("{}{}{}", self.prefix, self.separator, key)
    }
}

// ==================== SMART VALIDATION PATTERNS ====================

/// Smart validator for configuration fields
pub struct SmartValidator<T> {
    value: T,
    field_name: String,
    validators: Vec<ValidatorFn<T>>,
}

impl<T: std::fmt::Debug> std::fmt::Debug for SmartValidator<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SmartValidator")
            .field("value", &self.value)
            .field("field_name", &self.field_name)
            .field(
                "validators",
                &format!("[{} validators]", self.validators.len()),
            )
            .finish()
    }
}

impl<T> SmartValidator<T>
where
    T: Clone + Debug,
{
    /// Create a new validator for a value
    pub fn new(value: T, field_name: &str) -> Self {
        Self {
            value,
            field_name: field_name.to_string(),
            validators: Vec::new(),
        }
    }

    /// Add a validation rule
    pub fn rule<F>(mut self, validator: F) -> Self
    where
        F: Fn(&T) -> Result<()> + Send + Sync + 'static,
    {
        self.validators.push(Box::new(validator));
        self
    }

    /// Validate all rules
    pub fn validate(self) -> Result<T> {
        for validator in &self.validators {
            validator(&self.value)?;
        }
        Ok(self.value)
    }
}

// Common validation rules
impl SmartValidator<String> {
    /// Validate string is not empty
    pub fn not_empty(self) -> Self {
        self.rule(|value| {
            if value.is_empty() {
                Err(NestGateError::Configuration {
                    message: "Value cannot be empty".to_string(),
                    config_source: crate::error::UnifiedConfigSource::Validation(
                        "SmartValidator".to_string(),
                    ),
                    field: Some("value".to_string()),
                    suggested_fix: Some("Provide a non-empty value".to_string()),
                })
            } else {
                Ok(())
            }
        })
    }

    /// Validate string length
    pub fn length_between(self, min: usize, max: usize) -> Self {
        self.rule(move |value| {
            let len = value.len();
            if len < min || len > max {
                Err(NestGateError::Configuration {
                    message: format!("Length must be between {min} and {max}, got {len}"),
                    config_source: crate::error::UnifiedConfigSource::Validation(
                        "SmartValidator".to_string(),
                    ),
                    field: Some("length".to_string()),
                    suggested_fix: Some(format!(
                        "Provide a value with length between {min} and {max}"
                    )),
                })
            } else {
                Ok(())
            }
        })
    }

    /// Validate string matches pattern
    pub fn matches_pattern(self, pattern: &str) -> Self {
        let pattern = pattern.to_string();
        self.rule(move |value| {
            if value.contains(&pattern) {
                Ok(())
            } else {
                Err(NestGateError::Configuration {
                    message: format!("Value must contain pattern: {pattern}"),
                    config_source: crate::error::UnifiedConfigSource::Validation(
                        "SmartValidator".to_string(),
                    ),
                    field: Some("value".to_string()),
                    suggested_fix: Some(format!(
                        "Provide a value that matches the pattern: {pattern}"
                    )),
                })
            }
        })
    }
}

impl SmartValidator<u16> {
    /// Validate port number is in valid range
    pub fn valid_port(self) -> Self {
        self.rule(|value| {
            if *value > 0 {
                Ok(())
            } else {
                Err(NestGateError::Configuration {
                    message: format!("Port must be between 1 and 65535, got {value}"),
                    config_source: crate::error::UnifiedConfigSource::Validation(
                        "SmartValidator".to_string(),
                    ),
                    field: Some("value".to_string()),
                    suggested_fix: Some("Fix the validation error".to_string()),
                })
            }
        })
    }

    /// Validate port is not in reserved range
    pub fn not_reserved(self) -> Self {
        self.rule(|value| {
            if *value < 1024 {
                Err(NestGateError::Configuration {
                    message: format!("Port {value} is in reserved range (< 1024)"),
                    config_source: crate::error::UnifiedConfigSource::Validation(
                        "SmartValidator".to_string(),
                    ),
                    field: Some("value".to_string()),
                    suggested_fix: Some("Fix the validation error".to_string()),
                })
            } else {
                Ok(())
            }
        })
    }
}

impl SmartValidator<u64> {
    /// Validate value is within range
    pub fn range(self, min: u64, max: u64) -> Self {
        self.rule(move |value| {
            if *value >= min && *value <= max {
                Ok(())
            } else {
                Err(NestGateError::Configuration {
                    message: format!("Value must be between {min} and {max}, got {value}"),
                    config_source: crate::error::UnifiedConfigSource::Validation(
                        "SmartValidator".to_string(),
                    ),
                    field: Some("value".to_string()),
                    suggested_fix: Some("Fix the validation error".to_string()),
                })
            }
        })
    }

    /// Validate value is positive
    pub fn positive(self) -> Self {
        self.rule(|value| {
            if *value > 0 {
                Ok(())
            } else {
                Err(NestGateError::Configuration {
                    message: "Value must be positive".to_string(),
                    config_source: crate::error::UnifiedConfigSource::Validation(
                        "SmartValidator".to_string(),
                    ),
                    field: Some("value".to_string()),
                    suggested_fix: Some("Provide a positive value".to_string()),
                })
            }
        })
    }
}

impl SmartValidator<Duration> {
    /// Validate duration is within range
    pub fn duration_range(self, min: Duration, max: Duration) -> Self {
        self.rule(move |value| {
            if *value >= min && *value <= max {
                Ok(())
            } else {
                Err(NestGateError::Configuration {
                    message: format!("Duration must be between {min:?} and {max:?}, got {value:?}"),
                    config_source: crate::error::UnifiedConfigSource::Validation(
                        "SmartValidator".to_string(),
                    ),
                    field: Some("value".to_string()),
                    suggested_fix: Some("Fix the validation error".to_string()),
                })
            }
        })
    }

    /// Validate duration is not zero
    pub fn not_zero(self) -> Self {
        self.rule(|value| {
            if !value.is_zero() {
                Ok(())
            } else {
                Err(NestGateError::Configuration {
                    message: "Duration cannot be zero".to_string(),
                    config_source: crate::error::UnifiedConfigSource::Validation(
                        "SmartValidator".to_string(),
                    ),
                    field: Some("value".to_string()),
                    suggested_fix: Some("Fix the validation error".to_string()),
                })
            }
        })
    }
}

// ==================== SMART CONFIGURATION MERGER ====================

/// **SMART CONFIGURATION MERGER**
/// Intelligent configuration merging with conflict resolution
pub struct SmartConfigMerger<T> {
    base_config: T,
    merge_strategy: MergeStrategy,
    _phantom: PhantomData<T>,
}

/// Configuration merge strategies
#[derive(Debug, Clone)]
pub enum MergeStrategy {
    /// Override base values with new values
    Override,
    /// Keep base values, ignore new values
    KeepBase,
    /// Merge collections, override primitives
    Smart,
    /// Custom merge function
    Custom(fn(&serde_json::Value, &serde_json::Value) -> serde_json::Value),
}

impl<T> SmartConfigMerger<T>
where
    T: Clone + Serialize + for<'de> Deserialize<'de>,
{
    /// Create a new merger with base configuration
    pub fn new(base_config: T) -> Self {
        Self {
            base_config,
            merge_strategy: MergeStrategy::Smart,
            _phantom: PhantomData,
        }
    }

    /// Set merge strategy
    pub fn with_strategy(mut self, strategy: MergeStrategy) -> Self {
        self.merge_strategy = strategy;
        self
    }

    /// Merge with another configuration
    pub fn merge(self, other: T) -> Result<T> {
        let base_json = serde_json::to_value(&self.base_config)?;
        let other_json = serde_json::to_value(&other)?;

        let merged_json = match self.merge_strategy {
            MergeStrategy::Override => other_json,
            MergeStrategy::KeepBase => base_json,
            MergeStrategy::Smart => Self::smart_merge(&base_json, &other_json),
            MergeStrategy::Custom(merge_fn) => merge_fn(&base_json, &other_json),
        };

        let merged_config: T = serde_json::from_value(merged_json)?;
        Ok(merged_config)
    }

    /// Smart merge implementation
    fn smart_merge(base: &serde_json::Value, other: &serde_json::Value) -> serde_json::Value {
        match (base, other) {
            (serde_json::Value::Object(base_obj), serde_json::Value::Object(other_obj)) => {
                let mut result = base_obj.clone();
                for (key, value) in other_obj {
                    if let Some(base_value) = base_obj.get(key) {
                        result.insert(key.clone(), Self::smart_merge(base_value, value));
                    } else {
                        result.insert(key.clone(), value.clone());
                    }
                }
                serde_json::Value::Object(result)
            }
            _ => other.clone(),
        }
    }
}

// ==================== CONFIGURATION PRESETS ====================

/// **SMART CONFIGURATION PRESETS**
/// Pre-defined configuration presets for common scenarios
pub struct SmartConfigPresets;

impl SmartConfigPresets {
    /// Development environment preset
    pub fn development() -> HashMap<String, serde_json::Value> {
        let mut config = HashMap::new();
        config.insert(
            "log_level".to_string(),
            serde_json::Value::String("debug".to_string()),
        );
        config.insert("enable_tracing".to_string(), serde_json::Value::Bool(true));
        config.insert(
            "pool_size".to_string(),
            serde_json::Value::Number(serde_json::Number::from(10)),
        );
        config.insert(
            "timeout_secs".to_string(),
            serde_json::Value::Number(serde_json::Number::from(5)),
        );
        config.insert(
            "enable_hot_reload".to_string(),
            serde_json::Value::Bool(true),
        );
        config
    }

    /// Production environment preset
    pub fn production() -> HashMap<String, serde_json::Value> {
        let mut config = HashMap::new();
        config.insert(
            "log_level".to_string(),
            serde_json::Value::String("info".to_string()),
        );
        config.insert("enable_tracing".to_string(), serde_json::Value::Bool(false));
        config.insert(
            "pool_size".to_string(),
            serde_json::Value::Number(serde_json::Number::from(100)),
        );
        config.insert(
            "timeout_secs".to_string(),
            serde_json::Value::Number(serde_json::Number::from(30)),
        );
        config.insert("enable_metrics".to_string(), serde_json::Value::Bool(true));
        config.insert(
            "enable_health_checks".to_string(),
            serde_json::Value::Bool(true),
        );
        config
    }

    /// Testing environment preset
    pub fn testing() -> HashMap<String, serde_json::Value> {
        let mut config = HashMap::new();
        config.insert(
            "log_level".to_string(),
            serde_json::Value::String("warn".to_string()),
        );
        config.insert("enable_mocking".to_string(), serde_json::Value::Bool(true));
        config.insert(
            "pool_size".to_string(),
            serde_json::Value::Number(serde_json::Number::from(1)),
        );
        config.insert(
            "timeout_secs".to_string(),
            serde_json::Value::Number(serde_json::Number::from(1)),
        );
        config.insert(
            "cleanup_after_tests".to_string(),
            serde_json::Value::Bool(true),
        );
        config
    }

    /// High-performance preset
    pub fn high_performance() -> HashMap<String, serde_json::Value> {
        let mut config = HashMap::new();
        config.insert(
            "worker_threads".to_string(),
            serde_json::Value::Number(serde_json::Number::from(16)),
        );
        config.insert(
            "cache_size_mb".to_string(),
            serde_json::Value::Number(serde_json::Number::from(2048)),
        );
        config.insert(
            "enable_compression".to_string(),
            serde_json::Value::Bool(true),
        );
        config.insert(
            "batch_size".to_string(),
            serde_json::Value::Number(serde_json::Number::from(1000)),
        );
        config.insert(
            "enable_zero_copy".to_string(),
            serde_json::Value::Bool(true),
        );
        config
    }

    /// Security-focused preset
    pub fn secure() -> HashMap<String, serde_json::Value> {
        let mut config = HashMap::new();
        config.insert("enable_tls".to_string(), serde_json::Value::Bool(true));
        config.insert(
            "min_tls_version".to_string(),
            serde_json::Value::String("1.3".to_string()),
        );
        config.insert("enable_auth".to_string(), serde_json::Value::Bool(true));
        config.insert(
            "session_timeout_secs".to_string(),
            serde_json::Value::Number(serde_json::Number::from(1800)),
        );
        config.insert(
            "max_login_attempts".to_string(),
            serde_json::Value::Number(serde_json::Number::from(3)),
        );
        config.insert(
            "enable_audit_logging".to_string(),
            serde_json::Value::Bool(true),
        );
        config
    }
}

// ==================== CONVENIENCE FUNCTIONS ====================

/// Create a smart environment loader
pub fn env_loader(prefix: &str) -> SmartEnvLoader {
    SmartEnvLoader::new(prefix)
}

/// Create a smart validator for a value
pub fn validate<T>(value: T, field_name: &str) -> SmartValidator<T>
where
    T: Clone + Debug,
{
    SmartValidator::new(value, field_name)
}

/// Create a smart configuration merger
pub fn merge_configs<T>(base: T) -> SmartConfigMerger<T>
where
    T: Clone + Serialize + for<'de> Deserialize<'de>,
{
    SmartConfigMerger::new(base)
}

/// Load configuration with environment overrides
pub fn load_config_with_env<T>(base: T, env_prefix: &str) -> Result<T>
where
    T: Clone + Serialize + for<'de> Deserialize<'de>,
{
    let env_loader = SmartEnvLoader::new(env_prefix);
    let env_vars = env_loader.load_all();

    if env_vars.is_empty() {
        return Ok(base);
    }

    // Convert env vars to JSON for merging
    let env_json: serde_json::Value = env_vars
        .into_iter()
        .map(|(k, v)| (k, serde_json::Value::String(v)))
        .collect::<serde_json::Map<_, _>>()
        .into();

    let _base_json = serde_json::to_value(&base)?;
    let merger = SmartConfigMerger::new(base).with_strategy(MergeStrategy::Smart);
    let env_config: T = serde_json::from_value(env_json)?;

    merger.merge(env_config)
}
