use std::env;
use std::sync::Arc;

/// Configuration for monitoring module, loaded from environment variables.
#[derive(Debug, Clone)]
/// Configuration for MonitoringEnv
pub struct MonitoringEnvConfig {
    log_rotation_size_bytes: usize,
}

/// Type alias for Sharedmonitoringenvconfig
pub type SharedMonitoringEnvConfig = Arc<MonitoringEnvConfig>;

impl MonitoringEnvConfig {
    /// Creates a new `MonitoringEnvConfig` by loading values from environment variables.
    pub fn from_env() -> Self {
        let log_rotation_size_bytes = env::var("NESTGATE_LOG_ROTATION_SIZE_BYTES")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1024 * 1024); // 1MB default

        Self {
            log_rotation_size_bytes,
        }
    }

    /// Log Rotation Size Bytes
    pub fn log_rotation_size_bytes(&self) -> usize {
        self.log_rotation_size_bytes
    }

    // Builder for testing
    pub fn with_log_rotation_size_bytes(mut self, size: usize) -> Self {
        self.log_rotation_size_bytes = size;
        self
    }
}

impl Default for MonitoringEnvConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self::from_env()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = MonitoringEnvConfig::from_env();
        assert_eq!(config.log_rotation_size_bytes(), 1024 * 1024);
    }

    #[test]
    fn test_builder() {
        let config = MonitoringEnvConfig::from_env().with_log_rotation_size_bytes(2048 * 1024);
        assert_eq!(config.log_rotation_size_bytes(), 2048 * 1024);
    }
}
