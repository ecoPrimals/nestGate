//! **MIDDLEWARE HANDLER CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiddlewareHandlerConfig {
    pub cors: CorsHandlerConfig,
    pub compression: CompressionHandlerConfig,
    pub security: SecurityMiddlewareConfig,
    pub logging: LoggingMiddlewareConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsHandlerConfig { pub enabled: bool }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionHandlerConfig { pub enabled: bool }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMiddlewareConfig { pub enabled: bool }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingMiddlewareConfig { pub enabled: bool }

impl Default for MiddlewareHandlerConfig {
    fn default() -> Self {
        Self {
            cors: CorsHandlerConfig { enabled: true },
            compression: CompressionHandlerConfig { enabled: true },
            security: SecurityMiddlewareConfig { enabled: true },
            logging: LoggingMiddlewareConfig { enabled: true },
        }
    }
}

impl MiddlewareHandlerConfig {
    pub fn production_optimized() -> Self { Self::default() }
    pub fn development_optimized() -> Self { Self::default() }
    pub fn high_performance() -> Self { Self::default() }
    pub fn merge(self, _other: Self) -> Self { self }
    pub fn validate(&self) -> crate::Result<()> { Ok(()) }
} 