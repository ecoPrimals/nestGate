// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use nestgate_types::error::NestGateError;
//
// Provides structured logging and distributed tracing setup.

use nestgate_types::Result;

/// Tracing configuration
#[derive(Debug, Clone)]
/// Configuration for Tracing
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
    /// Returns the default instance
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            json_format: false,
            distributed_tracing: false,
            service_name: nestgate_config::constants::system::DEFAULT_SERVICE_NAME.to_string(),
            environment: "development".to_string(),
        }
    }
}

/// Initialize tracing with the given configuration.
///
/// This function can be called multiple times safely - if tracing is already
/// initialized, it will return Ok without error. This makes it safe to use
/// in test environments where multiple tests may attempt initialization.
///
/// # Errors
///
/// Returns when tracing subscriber initialization fails for a reason other than
/// "already initialized".
pub fn init_tracing(config: &TracingConfig) -> Result<()> {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
    let level = match config.level.as_str() {
        "trace" => tracing::Level::TRACE,
        "debug" => tracing::Level::DEBUG,
        "warn" => tracing::Level::WARN,
        "error" => tracing::Level::ERROR,
        _ => tracing::Level::INFO,
    };

    let result = if config.json_format {
        // JSON structured logging
        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer().json())
            .with(tracing_subscriber::filter::LevelFilter::from_level(level))
            .try_init()
    } else {
        // Human-readable logging
        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer())
            .with(tracing_subscriber::filter::LevelFilter::from_level(level))
            .try_init()
    };

    // Handle already initialized case gracefully - this is expected in tests
    // where multiple tests may attempt to initialize tracing
    match result {
        Ok(()) => {
            tracing::info!(
                service = config.service_name,
                environment = config.environment,
                level = config.level,
                "Tracing initialized"
            );
            Ok(())
        }
        Err(e) => {
            // Check if this is the "already initialized" error
            let err_str = e.to_string();
            if err_str.contains("already") || err_str.contains("SetGlobalDefaultError") {
                // Tracing already initialized - this is OK, especially in tests
                tracing::debug!("Tracing already initialized, skipping re-initialization");
                Ok(())
            } else {
                // Some other error occurred
                Err(NestGateError::configuration_error(
                    "tracing",
                    format!("Failed to initialize tracing: {e}"),
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracing_config_default() {
        let config = TracingConfig::default();
        assert_eq!(config.level, "info");
        assert!(!config.json_format);
        assert_eq!(config.service_name, "nestgate");
    }

    #[test]
    fn init_tracing_levels_and_json_branch() {
        for level in ["trace", "debug", "info", "warn", "error", "other"] {
            let mut c = TracingConfig::default();
            c.level = level.to_string();
            let _ = init_tracing(&c);
        }
        let mut json_cfg = TracingConfig::default();
        json_cfg.json_format = true;
        json_cfg.level = "debug".to_string();
        let _ = init_tracing(&json_cfg);
    }
}
