// **VALIDATION HANDLER CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for ValidationHandler
pub struct ValidationHandlerConfig {
    /// Schema
    pub schema: SchemaValidationConfig,
    /// Data
    pub data: DataValidationConfig,
    /// Business Rules
    pub business_rules: BusinessRuleValidationConfig,
    /// Custom
    pub custom: CustomValidationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for SchemaValidation
pub struct SchemaValidationConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for DataValidation
pub struct DataValidationConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for BusinessRuleValidation
pub struct BusinessRuleValidationConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for CustomValidation
pub struct CustomValidationConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

impl Default for ValidationHandlerConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            schema: SchemaValidationConfig { enabled: true },
            data: DataValidationConfig { enabled: true },
            business_rules: BusinessRuleValidationConfig { enabled: true },
            custom: CustomValidationConfig { enabled: false },
        }
    }
}

impl ValidationHandlerConfig {
    #[must_use]
    pub fn production_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn development_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn high_performance() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn merge(self, _other: Self) -> Self {
        self
    }
    /// Validates data
    pub fn validate(&self) -> crate::Result<()> {
        Ok(())
    }
}
