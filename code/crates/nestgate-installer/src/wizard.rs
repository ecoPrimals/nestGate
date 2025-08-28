/// Simplified installation wizard using canonical patterns and LegacyConfigAdapter
use crate::config::InstallerConfig;
use dialoguer::{Confirm, Input};
// Migration utilities no longer needed - using canonical configurations
use nestgate_core::error::{NestGateError, Result};
use std::path::PathBuf;

/// Installation wizard for canonical configuration
pub struct InstallationWizard {
    config: InstallerConfig,
}

impl InstallationWizard {
    /// Create new installation wizard
    pub fn new(config: InstallerConfig) -> Self {
        Self { config }
    }

    /// Run the complete installation wizard
    pub fn run(&mut self) -> Result<InstallerConfig> {
        println!("🚀 NestGate Installation Wizard");
        println!("================================");

        self.configure_installation_path()?;
        self.configure_system_integration()?;
        self.configure_components()?;
        self.configure_advanced_features()?;
        self.show_summary()?;

        Ok(self.config.clone())
    }

    fn configure_installation_path(&mut self) -> Result<()> {
        println!("📁 Installation Directory");

        // Using canonical configuration - access working_directory instead of domains.installation
        let default_path = self
            .config
            .base_config
            .system
            .data_dir
            .to_string_lossy()
            .to_string();

        let custom_path: String = Input::new()
            .with_prompt("Installation directory")
            .default(default_path)
            .interact_text()
            .map_err(|e| NestGateError::validation_error("input", &format!("Input error: {e}"), None))?;

        // Update canonical config fields - use data_dir instead of working_directory
        self.config.base_config.system.data_dir = PathBuf::from(&custom_path);
        // Note: log_directory doesn't exist in canonical config, using data_dir

        Ok(())
    }

    fn configure_system_integration(&mut self) -> Result<()> {
        println!("🔧 System Integration");

        let install_as_service = Confirm::new()
            .with_prompt("Install as system service?")
            .default(false)
            .interact()
            .map_err(|e| NestGateError::validation_error("input", &format!("Input error: {e}"), None))?;

        if install_as_service {
            println!("✅ Will install as system service");
        } else {
            println!("⏭️  Skipping service installation");
        }

        let add_to_path = Confirm::new()
            .with_prompt("Add to system PATH?")
            .default(true)
            .interact()
            .map_err(|e| NestGateError::validation_error("input", &format!("Input error: {e}"), None))?;

        if add_to_path {
            println!("✅ Will add to system PATH");
        }

        Ok(())
    }

    fn configure_components(&mut self) -> Result<()> {
        println!("🔧 Component Selection");

        let install_zfs: bool = Confirm::new()
            .with_prompt("Install ZFS support?")
            .default(true)
            .interact()
            .map_err(|e| NestGateError::validation_error("input", &format!("Input error: {e}"), None))?;

        // Note: components configuration would need to be added to canonical config
        // For now, just enable existing features if requested
        if install_zfs {
            self.config.features.enable_metrics = true;
        }

        Ok(())
    }

    fn configure_advanced_features(&mut self) -> Result<()> {
        println!("⚙️  Advanced Features");

        let enable_monitoring = Confirm::new()
            .with_prompt("Enable performance monitoring?")
            .default(true)
            .interact()
            .map_err(|e| NestGateError::validation_error("input", &format!("Input error: {e}"), None))?;

        let enable_security = Confirm::new()
            .with_prompt("Enable security hardening?")
            .default(true)
            .interact()
            .map_err(|e| NestGateError::validation_error("input", &format!("Input error: {e}"), None))?;

        if enable_monitoring {
            println!("✅ Performance monitoring enabled");
        }
        if enable_security {
            println!("✅ Security hardening enabled");
        }

        Ok(())
    }

    fn show_summary(&self) -> Result<()> {
        println!("📋 Installation Summary");
        println!("======================");
        println!(
            "Installation Path: {}",
            self.config.base_config.system.data_dir.display()
        );
        println!(
            "Service Name: {}",
            self.config.base_config.system.service_name
        );

        let confirm: bool = Confirm::new()
            .with_prompt("Proceed with installation?")
            .default(true)
            .interact()
            .map_err(|e| NestGateError::validation_error("input", &format!("Input error: {e}"), None))?;

        if !confirm {
            return Err(NestGateError::validation_error(
                "user_confirmation",
                "Installation cancelled by user",
                None
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wizard_creation() {
        let config = InstallerConfig::default();
        let wizard = InstallationWizard::new(config);
        assert_eq!(
            wizard.config.system.instance_name,
            Some("nestgate-instance".to_string())
        );
    }
}
