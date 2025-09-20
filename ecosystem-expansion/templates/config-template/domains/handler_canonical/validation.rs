//! **VALIDATION HANDLER CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationHandlerConfig {
    pub schema: SchemaValidationConfig,
    pub data: DataValidationConfig,
    pub business_rules: BusinessRuleValidationConfig,
    pub custom: CustomValidationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaValidationConfig { pub enabled: bool }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataValidationConfig { pub enabled: bool }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessRuleValidationConfig { pub enabled: bool }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomValidationConfig { pub enabled: bool }

impl Default for ValidationHandlerConfig {
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
    pub fn production_optimized() -> Self { Self::default() }
    pub fn development_optimized() -> Self { Self::default() }
    pub fn high_performance() -> Self { Self::default() }
    pub fn merge(self, _other: Self) -> Self { self }
    pub fn validate(&self) -> crate::Result<()> { Ok(()) }
} 