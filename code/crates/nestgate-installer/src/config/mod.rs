/// Installer Configuration - Modular Architecture
/// Split from monolithic unified_installer_config.rs for maintainability and 2000-line compliance
/// Comprehensive installer configuration with platform, validation, and execution modules
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// Import the standardized config pattern
use nestgate_core::unified_config_consolidation::StandardDomainConfig;

// Re-export specialized modules
pub mod execution;
pub mod migration;
pub mod platform;
pub mod validation;

// pub use execution::*; // Currently unused
// pub use migration::*; // Currently unused
pub use platform::{
    ComponentSelection, ComponentSettings, DeploymentSettings, InstallMode, InstallationSettings,
    PackageManagementSettings, PackageManagerType, PostInstallSettings, SystemIntegrationSettings,
};
// Unused imports: DeploymentMode, PlatformType, SystemRequirements
pub use validation::ValidationSettings;
// Unused imports: HealthCheckSettings, PostInstallValidationSettings, PreInstallCheckSettings

// Re-export implementation methods
// pub use execution::implementation::*; // Currently unused

/// Main installer configuration structure
pub type InstallerConfig = StandardDomainConfig<InstallerExtensions>;

// ==================== INSTALLER-SPECIFIC EXTENSIONS ====================

/// Installer-specific configuration extensions
/// Domain-specific fields that don't belong in unified base configs
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstallerExtensions {
    /// Installation mode and behavior
    pub installation: InstallationSettings,
    /// Component selection settings
    pub components: ComponentSettings,
    /// System integration settings
    pub system_integration: SystemIntegrationSettings,
    /// Package management settings
    pub package_management: PackageManagementSettings,
    /// Validation and checks
    pub validation: ValidationSettings,
    /// Post-installation configuration
    pub post_install: PostInstallSettings,
    /// Deployment settings
    pub deployment: DeploymentSettings,
}

impl InstallerExtensions {
    /// Create production-optimized installer extensions
    pub fn production() -> Self {
        let mut config = Self::default();
        // Override specific settings for production
        config.installation.interactive = false;
        config.validation.pre_install_checks.validate_checksums = true;
        config.system_integration.install_as_service = true;
        config.system_integration.enable_autostart = true;
        config
    }
}

// ==================== TYPE ALIAS FOR UNIFIED INSTALLER CONFIG ====================

/// Standardized Installer configuration
pub type UnifiedInstallerConfig = StandardDomainConfig<InstallerExtensions>;

// ==================== MIGRATION UTILITIES ====================

// **DEPRECATED MIGRATION UTILITIES REMOVED**
// Use UnifiedInstallerConfig::development() and UnifiedInstallerConfig::production() builders directly

#[cfg(test)]
mod tests {
    use super::InstallerConfigMethods;
    use super::*;

    #[test]
    fn test_installer_config_creation() {
        let config = UnifiedInstallerConfig::development();
        assert_eq!(config.service.name, "nestgate-installer");
        assert!(config.extensions.installation.interactive);
        assert!(config.extensions.installation.verbose);
    }

    #[test]
    fn test_production_config() {
        let config = UnifiedInstallerConfig::production();
        assert!(matches!(
            config.extensions.installation.mode,
            InstallMode::Production
        ));
        assert!(!config.extensions.installation.interactive);
        assert!(
            config
                .extensions
                .components
                .selected_components
                .install_security
        );
    }

    #[test]
    fn test_config_validation() {
        let mut config = UnifiedInstallerConfig::development();
        config.extensions.validation.system_requirements.min_ram_mb = 2048;

        let result = config.validate_installer_config();
        assert!(result.is_ok());
    }

    #[test]
    fn test_component_management() {
        let mut config = UnifiedInstallerConfig::development();

        config.set_component_enabled("security", true);
        assert!(
            config
                .extensions
                .components
                .selected_components
                .install_security
        );

        config.set_component_enabled("security", false);
        assert!(
            !config
                .extensions
                .components
                .selected_components
                .install_security
        );

        let enabled = config.get_enabled_components();
        assert!(enabled.contains(&"api".to_string()));
        assert!(enabled.contains(&"zfs".to_string()));
        assert!(!enabled.contains(&"security".to_string()));
    }

    #[test]
    fn test_directory_settings() {
        let mut config = UnifiedInstallerConfig::development();

        config.set_install_directories("/custom/install", "/custom/config", "/custom/data");

        assert_eq!(
            config.extensions.installation.install_dir,
            std::path::PathBuf::from("/custom/install")
        );
        assert_eq!(
            config.extensions.installation.config_dir,
            std::path::PathBuf::from("/custom/config")
        );
        assert_eq!(
            config.extensions.installation.data_dir,
            std::path::PathBuf::from("/custom/data")
        );
    }
}

// ==================== INSTALLER-SPECIFIC METHODS ====================

/// Trait for installer-specific configuration methods
pub trait InstallerConfigMethods {
    /// Validate installer-specific configuration
    fn validate_installer_config(&self) -> Result<(), Vec<String>>;

    /// Enable or disable a component
    fn set_component_enabled(&mut self, component: &str, enabled: bool);

    /// Get list of enabled components
    fn get_enabled_components(&self) -> Vec<String>;

    /// Set installation directories
    fn set_install_directories(&mut self, base_path: &str, data_path: &str, config_path: &str);
}

impl InstallerConfigMethods for StandardDomainConfig<InstallerExtensions> {
    fn validate_installer_config(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Validate installation settings
        if !self.extensions.installation.install_dir.exists()
            && !self.extensions.installation.force_install
        {
            errors.push(
                "Installation directory does not exist and force_install is false".to_string(),
            );
        }

        // Validate component settings
        let components = &self.extensions.components.selected_components;
        if !components.install_api && !components.install_zfs && !components.install_network {
            errors.push("At least one component must be selected".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn set_component_enabled(&mut self, component: &str, enabled: bool) {
        match component {
            "api" => self.extensions.components.selected_components.install_api = enabled,
            "zfs" => self.extensions.components.selected_components.install_zfs = enabled,
            "network" => {
                self.extensions
                    .components
                    .selected_components
                    .install_network = enabled
            }
            "monitoring" => {
                self.extensions
                    .components
                    .selected_components
                    .install_monitoring = enabled
            }
            "security" => {
                self.extensions
                    .components
                    .selected_components
                    .install_security = enabled
            }
            "automation" => {
                self.extensions
                    .components
                    .selected_components
                    .install_automation = enabled
            }
            "ui" => self.extensions.components.selected_components.install_ui = enabled,
            "nas" => self.extensions.components.selected_components.install_nas = enabled,
            "fsmonitor" => {
                self.extensions
                    .components
                    .selected_components
                    .install_fsmonitor = enabled
            }
            "mcp" => self.extensions.components.selected_components.install_mcp = enabled,
            _ => {} // Unknown component
        }
    }

    fn get_enabled_components(&self) -> Vec<String> {
        let mut components = Vec::new();
        let selection = &self.extensions.components.selected_components;

        if selection.install_api {
            components.push("api".to_string());
        }
        if selection.install_zfs {
            components.push("zfs".to_string());
        }
        if selection.install_network {
            components.push("network".to_string());
        }
        if selection.install_monitoring {
            components.push("monitoring".to_string());
        }
        if selection.install_security {
            components.push("security".to_string());
        }
        if selection.install_automation {
            components.push("automation".to_string());
        }
        if selection.install_ui {
            components.push("ui".to_string());
        }
        if selection.install_nas {
            components.push("nas".to_string());
        }
        if selection.install_fsmonitor {
            components.push("fsmonitor".to_string());
        }
        if selection.install_mcp {
            components.push("mcp".to_string());
        }

        components
    }

    fn set_install_directories(&mut self, base_path: &str, data_path: &str, config_path: &str) {
        self.extensions.installation.install_dir = PathBuf::from(base_path);
        self.extensions.installation.data_dir = PathBuf::from(data_path);
        self.extensions.installation.config_dir = PathBuf::from(config_path);
    }
}
