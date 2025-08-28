use crate::error::NestGateError;
//
// Provides structured logging and distributed tracing setup.

use crate::{Result};

/// Tracing configuration
#[derive(Debug, Clone)]
pub struct TracingConfig {
    /// Log level filter
    pub level: String,
    /// Enable JSON formatting
    pub json_format: bool,
    /// Enable distributed tracing
    pub distributed_tracing: bool,
    /// Service name for tracing
    pub service_name: String,
    /// Environment name
    pub environment: String,
}

impl Default for TracingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            json_format: false,
            distributed_tracing: false,
            service_name: "nestgate".to_string(),
            environment: "development".to_string(),
        }
    }
}

/// Initialize tracing with the given configuration
pub fn init_tracing(config: TracingConfig) -> Result<()> {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    let level = match config.level.as_str() {
        "trace" => tracing::Level::TRACE,
        "debug" => tracing::Level::DEBUG,
        "info" => tracing::Level::INFO,
        "warn" => tracing::Level::WARN,
        "error" => tracing::Level::ERROR,
        _ => tracing::Level::INFO,
    };

    if config.json_format {
        // JSON structured logging
        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer().json())
            .with(tracing_subscriber::filter::LevelFilter::from_level(level))
            .try_init()
            .map_err(|e| {
                NestGateError::configuration_error_with_field(
                    format!("Failed to initialize JSON tracing: {e}"),
                    "tracing".to_string(),
                )
            })?;
    } else {
        // Human-readable logging
        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer())
            .with(tracing_subscriber::filter::LevelFilter::from_level(level))
            .try_init()
            .map_err(|e| {
                NestGateError::configuration_error_with_field(
                    format!("Failed to initialize tracing: {e}"),
                    "tracing".to_string(),
                )
            })?;
    }

    tracing::info!(
        service = config.service_name,
        environment = config.environment,
        level = config.level,
        "Tracing initialized"
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracing_config_default() {
        let config = TracingConfig::default();
        assert_eq!(config.level, "info");
        assert!(!config.json_format);
        assert_eq!(config.name, "nestgate");
    }
}
