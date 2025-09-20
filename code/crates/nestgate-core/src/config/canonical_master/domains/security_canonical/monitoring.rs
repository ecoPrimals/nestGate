// **SECURITY MONITORING CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMonitoringConfig {
    pub enabled: bool,
    pub audit: AuditSecurityConfig,
    pub logging: LoggingConfig,
    pub alerting: AlertingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditSecurityConfig {
    pub enabled: bool,
    pub log_level: String,
    pub retention_days: u32,
}

impl AuditSecurityConfig {
    #[must_use]
    pub const fn development_optimized() -> Self {
        Self {
            enabled: false,
            log_level: "info".to_string(),
            retention_days: 7,
        }
    }

    #[must_use]
    pub const fn compliance_focused() -> Self {
        Self {
            enabled: true,
            log_level: "debug".to_string(),
            retention_days: 365,
        }
    }

    #[must_use]
    pub const fn production_hardened() -> Self {
        Self {
            enabled: true,
            log_level: "trace".to_string(),
            retention_days: 2555, // 7 years
        }
    }

    #[must_use]
    pub fn merge(mut self, other: Self) -> Self {
        self.enabled = other.enabled;
        self.log_level = other.log_level;
        self.retention_days = other.retention_days;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub security_events: bool,
    pub authentication_events: bool,
    pub authorization_events: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertingConfig {
    pub enabled: bool,
    pub channels: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentResponseConfig {
    pub enabled: bool,
    pub escalation_rules: Vec<String>,
}

impl Default for SecurityMonitoringConfig {
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
    fn default() -> Self {
        Self {
            enabled: true,
            log_level: "INFO".to_string(),
            retention_days: 90,
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            security_events: true,
            authentication_events: true,
            authorization_events: true,
        }
    }
}

impl Default for AlertingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            channels: vec!["email".to_string(), "slack".to_string()],
        }
    }
}

impl SecurityMonitoringConfig {
    #[must_use]
    pub const fn production_hardened() -> Self {
        Self::default()
    }
    #[must_use]
    pub const fn development_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub const fn compliance_focused() -> Self {
        Self::default()
    }
    #[must_use]
    pub const fn merge(self, _other: Self) -> Self {
        self
    }
    pub const fn validate(&self) -> crate::Result<()> {
        Ok(())
    }
}
