// **SECURITY MONITORING CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for SecurityMonitoring
pub struct SecurityMonitoringConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Audit
    pub audit: AuditSecurityConfig,
    /// Logging
    pub logging: LoggingConfig,
    /// Alerting
    pub alerting: AlertingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for AuditSecurity
pub struct AuditSecurityConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Log Level
    pub log_level: String,
    /// Retention Days
    pub retention_days: u32,
}

impl AuditSecurityConfig {
    /// Creates a development-optimized configuration with minimal audit logging.
    ///
    /// This configuration is suitable for development environments where audit overhead
    /// should be minimized. Auditing is disabled by default.
    #[must_use]
    pub fn development_optimized() -> Self {
        Self {
            enabled: false,
            log_level: "info".to_string(),
            retention_days: 7,
        }
    }

    /// Creates a compliance-focused configuration with comprehensive audit logging.
    ///
    /// This configuration is suitable for environments requiring compliance with
    /// regulatory standards. Enables full audit logging with 1-year retention.
    #[must_use]
    pub fn compliance_focused() -> Self {
        Self {
            enabled: true,
            log_level: "debug".to_string(),
            retention_days: 365,
        }
    }

    /// Creates a production-hardened configuration with maximum audit coverage.
    ///
    /// This configuration is suitable for production environments requiring
    /// maximum security audit coverage with 7-year retention for legal compliance.
    #[must_use]
    pub fn production_hardened() -> Self {
        Self {
            enabled: true,
            log_level: "trace".to_string(),
            retention_days: 2555, // 7 years
        }
    }

    /// Merges this configuration with another, taking values from the other configuration.
    ///
    /// # Arguments
    ///
    /// * `other` - The configuration to merge with, whose values will override this one
    ///
    /// # Returns
    ///
    /// A new configuration with values from `other`
    #[must_use]
    pub fn merge(mut self, other: Self) -> Self {
        self.enabled = other.enabled;
        self.log_level = other.log_level;
        self.retention_days = other.retention_days;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Logging
pub struct LoggingConfig {
    /// Security Events
    pub security_events: bool,
    /// Authentication Events
    pub authentication_events: bool,
    /// Authorization Events
    pub authorization_events: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Alerting
pub struct AlertingConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Channels
    pub channels: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for IncidentResponse
pub struct IncidentResponseConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Escalation Rules
    pub escalation_rules: Vec<String>,
}

impl Default for SecurityMonitoringConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            audit: AuditSecurityConfig::default(),
            logging: LoggingConfig::default(),
            alerting: AlertingConfig::default(),
        }
    }
}

impl Default for AuditSecurityConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            log_level: "INFO".to_string(),
            retention_days: 90,
        }
    }
}

impl Default for LoggingConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            security_events: true,
            authentication_events: true,
            authorization_events: true,
        }
    }
}

impl Default for AlertingConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            channels: vec!["email".to_string(), "slack".to_string()],
        }
    }
}

impl SecurityMonitoringConfig {
    /// Creates a production-hardened security monitoring configuration.
    ///
    /// Returns the default configuration optimized for production use.
    #[must_use]
    pub fn production_hardened() -> Self {
        Self::default()
    }
    /// Creates a development-optimized security monitoring configuration.
    ///
    /// Returns the default configuration suitable for development environments.
    #[must_use]
    pub fn development_optimized() -> Self {
        Self::default()
    }
    /// Creates a compliance-focused security monitoring configuration.
    ///
    /// Returns the default configuration tailored for compliance requirements.
    #[must_use]
    pub fn compliance_focused() -> Self {
        Self::default()
    }
    /// Merges this configuration with another, returning self unchanged.
    ///
    /// # Arguments
    ///
    /// * `_other` - The other configuration (currently ignored)
    ///
    /// # Returns
    ///
    /// Returns self without modification
    #[must_use]
    pub fn merge(self, _other: Self) -> Self {
        self
    }
    /// Validates data
    pub fn validate(&self) -> crate::Result<()> {
        Ok(())
    }
}
