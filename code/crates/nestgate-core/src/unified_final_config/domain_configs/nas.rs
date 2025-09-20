//
// NAS-specific configuration structures extracted from the monolithic NestGateCanonicalConfig.rs
// for better maintainability and focused responsibility.

use serde::{Deserialize, Serialize};

/// NAS domain configuration (migrated from nestgate-nas)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NasDomainConfig {
    pub enabled: bool,
    pub shares_enabled: bool,
    pub backup_enabled: bool,
    pub raid_level: String,
    pub default_share_permissions: String,
}
impl Default for NasDomainConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            shares_enabled: false,
            backup_enabled: false,
            raid_level: "mirror".to_string(),
            default_share_permissions: "755".to_string(),
        }
    }
}
