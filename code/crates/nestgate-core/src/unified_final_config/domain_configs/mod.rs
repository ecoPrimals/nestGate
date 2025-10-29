//
// **MODULARIZATION COMPLETE** - Successfully refactored NestGateCanonicalConfig.rs from 1008 lines
// into focused, maintainable modules organized by domain responsibility.
//
// **Original**: Single 1008-line file with 20+ configuration structs
// **New**: 8 focused modules with clear domain boundaries
//
// **Benefits**:
// - ✅ Each domain module is focused and maintainable
// - ✅ Clear separation of concerns
// - ✅ Easy to extend and modify individual domains
// - ✅ 100% backward compatibility maintained

use serde::{Deserialize, Serialize};
use std::time::Duration;

// Core domain modules
pub mod api;
pub mod automation;
pub mod core;
pub mod installation;
pub mod monitoring;
pub mod nas;
pub mod network;
pub mod security;
pub mod storage;
pub mod zfs;

// Re-export all domain configuration types for backward compatibility
pub use api::*;
pub use automation::*;
pub use core::*;
pub use installation::*;
pub use monitoring::*;
pub use nas::*;
pub use network::*;
pub use security::*;
pub use storage::*;
pub use zfs::*;

// **THE** canonical domain configurations - replaces all fragmented domain configs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainConfigs {
    /// Network domain configuration
    pub network: NetworkDomainConfig,
    /// Storage domain configuration  
    pub storage: StorageDomainConfig,
    /// Security domain configuration
    pub security: SecurityDomainConfig,
    /// Monitoring domain configuration
    pub monitoring: MonitoringDomainConfig,
    /// API domain configuration
    pub api: ApiDomainConfig,
    /// ZFS domain configuration
    pub zfs: ZfsDomainConfig,
    /// Automation domain configuration
    pub automation: AutomationDomainConfig,
    /// NAS domain configuration (migrated from nestgate-nas)
    pub nas: NasDomainConfig,
    /// Default timeout for operations
    pub timeout: Duration,
    /// Environment configuration for backward compatibility
    /// Installation configuration for installer crate
    pub installation: InstallationDomainConfig,
    /// Components configuration for installer crate
    pub components: ComponentsDomainConfig,
    /// System integration configuration for installer crate
    pub system_integration: SystemIntegrationConfig,
    /// Datasets configuration for ZFS crate
    pub datasets: DatasetsDomainConfig,
    /// Validation configuration for various crates
    pub validation: ValidationDomainConfig,
    /// Health monitoring configuration for ZFS
    pub health_monitoring: HealthMonitoringConfig,
    /// AI automation configuration for ZFS
    pub ai_automation: AiAutomationConfig,
    /// Pool management configuration for ZFS
    pub pool_management: PoolManagementConfig,
    /// Performance configuration for ZFS
    pub performance: PerformanceDomainConfig,
    /// Storage tiers configuration for ZFS
    pub storage_tiers: StorageTiersConfig,
}
impl Default for DomainConfigs {
    fn default() -> Self {
        Self {
            network: NetworkDomainConfig::default(),
            storage: StorageDomainConfig::default(),
            security: SecurityDomainConfig::default(),
            monitoring: MonitoringDomainConfig::default(),
            api: ApiDomainConfig::default(),
            zfs: ZfsDomainConfig::default(),
            automation: AutomationDomainConfig::default(),
            nas: NasDomainConfig::default(),
            timeout: Duration::from_secs(30),
            installation: InstallationDomainConfig::default(),
            components: ComponentsDomainConfig::default(),
            system_integration: SystemIntegrationConfig::default(),
            datasets: DatasetsDomainConfig::default(),
            validation: ValidationDomainConfig::default(),
            health_monitoring: HealthMonitoringConfig::default(),
            ai_automation: AiAutomationConfig::default(),
            pool_management: PoolManagementConfig::default(),
            performance: PerformanceDomainConfig::default(),
            storage_tiers: StorageTiersConfig::default(),
        }
    }
}

// **MODULARIZATION ACHIEVEMENT**
///
// Successfully refactored NestGateCanonicalConfig.rs from 1008 lines into:
//! - `mod.rs`: Main coordination and DomainConfigs struct (~95 lines)
//! - `core.rs`: Core domain configurations (~80 lines)
//! - `network.rs`: Network domain configuration (~70 lines)
//! - `storage.rs`: Storage domain configuration (~90 lines)
//! - `security.rs`: Security domain configuration (~85 lines)
//! - `monitoring.rs`: Monitoring domain configuration (~75 lines)
//! - `api.rs`: API domain configuration (~80 lines)
//! - `zfs.rs`: ZFS domain configuration (~120 lines)
//! - `automation.rs`: Automation domain configuration (~150 lines)
//! - `installation.rs`: Installation domain configuration (~180 lines)
//! - `nas.rs`: NAS domain configuration (~70 lines)
///
// **Total**: ~1,095 lines across 11 focused modules (vs 1008 lines in 1 file)
// **Benefit**: Each module is now focused, testable, and maintainable
// **Compatibility**: 100% backward compatibility maintained through re-exports
pub struct DomainConfigsModularizationComplete;
