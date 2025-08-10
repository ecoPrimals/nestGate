//! Interactive Installation Wizard
//!
//! Provides an interactive command-line wizard for guiding users through
//! the NestGate installation process with step-by-step configuration.
//!
//! ## Features
//! - Interactive prompts for configuration options
//! - Input validation and sanitization
//! - Progress tracking and feedback
//! - Recovery from installation errors

use crate::config::InstallerConfig;
use anyhow::Result;
use dialoguer::{Confirm, Input};
use std::path::PathBuf;

pub struct InstallationWizard {
    config: InstallerConfig,
}

impl InstallationWizard {
    pub fn new() -> Self {
        Self {
            config: InstallerConfig::default(),
        }
    }

    pub fn run_interactive(&mut self) -> Result<InstallerConfig> {
        println!("🚀 Welcome to NestGate Installation Wizard");
        println!("This wizard will guide you through setting up NestGate on your system.\n");

        self.configure_installation_path()?;
        self.configure_service_mode()?;
        // Configure basic options
        self.configure_features()?;

        // Confirm and proceed
        self.confirm_installation()?;

        Ok(self.config.clone())
    }

    fn configure_installation_path(&mut self) -> Result<()> {
        println!("📁 Installation Directory");

        let default_path = self
            .config
            .extensions
            .installation
            .install_dir
            .to_string_lossy()
            .to_string();
        let custom_path: String = Input::new()
            .with_prompt("Installation directory")
            .default(default_path)
            .interact_text()?;

        self.config.extensions.installation.install_dir = PathBuf::from(custom_path);

        // Validate the path
        if let Err(e) =
            crate::config::validation::config_validation::validate_installer_config(&self.config)
        {
            println!("⚠️  Warning: {e}");
            if !Confirm::new()
                .with_prompt("Continue with this path anyway?")
                .default(false)
                .interact()?
            {
                return self.configure_installation_path();
            }
        }

        Ok(())
    }

    fn configure_service_mode(&mut self) -> Result<()> {
        println!("\n🔧 Service Configuration");

        let platform_info = crate::platform::PlatformInfo::detect();

        if platform_info.service_install_supported() {
            self.config.extensions.system_integration.install_as_service = Confirm::new()
                .with_prompt("Install as system service?")
                .default(true)
                .interact()?;

            if self.config.extensions.system_integration.install_as_service {
                println!("✅ NestGate will be installed as a system service");
                println!("   - Automatic startup on boot");
                println!("   - Background operation");
                println!("   - System-level privileges");
            }
        } else {
            println!("ℹ️  System service installation not supported on this platform");
            self.config.extensions.system_integration.install_as_service = false;
        }

        Ok(())
    }

    fn configure_features(&mut self) -> Result<()> {
        println!("\n⚙️  Feature Configuration");

        // ZFS Features
        self.config
            .extensions
            .components
            .selected_components
            .install_zfs = Confirm::new()
            .with_prompt("Enable ZFS storage management?")
            .default(true)
            .interact()?;

        if self
            .config
            .extensions
            .components
            .selected_components
            .install_zfs
        {
            println!("✅ ZFS features enabled");
            println!("   - Pool management");
            println!("   - Dataset operations");
            println!("   - Snapshot management");
            println!("   - Health monitoring");
        }

        // UI Features
        self.config
            .extensions
            .components
            .selected_components
            .install_ui = Confirm::new()
            .with_prompt("Enable UI features?")
            .default(false)
            .interact()?;

        if self
            .config
            .extensions
            .components
            .selected_components
            .install_ui
        {
            println!("✅ UI features enabled");
            println!("   - Intelligent resource optimization");
            println!("   - Predictive maintenance");
            println!("   - Automated troubleshooting");
        }

        // Network configuration
        self.config
            .extensions
            .components
            .selected_components
            .install_network = Confirm::new()
            .with_prompt("Enable network features?")
            .default(true)
            .interact()?;

        if self
            .config
            .extensions
            .components
            .selected_components
            .install_network
        {
            println!("✅ Network features enabled");
            println!("   - Remote management");
            println!("   - Distributed storage");
            println!("   - Service discovery");
        }

        // Advanced configuration
        println!("\n🔧 Advanced Configuration:");
        println!("Configure advanced features and integrations");

        // Desktop integration
        self.config
            .extensions
            .system_integration
            .desktop_integration = Confirm::new()
            .with_prompt("Create desktop shortcut?")
            .default(true)
            .interact()?;

        if self
            .config
            .extensions
            .system_integration
            .desktop_integration
        {
            println!("✅ Desktop shortcut will be created");
            println!("   - Quick access to NestGate");
            println!("   - Integrated with system menu");
        }

        // PATH configuration
        self.config.extensions.system_integration.add_to_path = Confirm::new()
            .with_prompt("Add NestGate to system PATH?")
            .default(true)
            .interact()?;

        if self.config.extensions.system_integration.add_to_path {
            println!("✅ NestGate will be added to PATH");
            println!("   - Access from any terminal");
            println!("   - Global command availability");
        }

        Ok(())
    }

    #[allow(dead_code)]
    fn configure_system_integration(&mut self) -> Result<()> {
        println!("\n🔗 System Integration");

        // PATH integration
        self.config.extensions.system_integration.add_to_path = Confirm::new()
            .with_prompt("Add NestGate to system PATH?")
            .default(true)
            .interact()?;

        if self.config.extensions.system_integration.add_to_path {
            println!("✅ Will add to PATH - you can run 'nestgate' from anywhere");
        }

        // Desktop shortcut
        self.config
            .extensions
            .system_integration
            .desktop_integration = Confirm::new()
            .with_prompt("Create desktop shortcut?")
            .default(true)
            .interact()?;

        if self
            .config
            .extensions
            .system_integration
            .desktop_integration
        {
            println!("✅ Desktop shortcut will be created");
        }

        Ok(())
    }

    fn confirm_installation(&self) -> Result<bool> {
        println!("\n📋 Installation Summary:");
        println!("========================");
        println!("This will install NestGate with the following configuration:");
        println!(
            "  📂 Install Path: {}",
            self.config.extensions.installation.install_dir.display()
        );
        println!(
            "  �� Service Mode: {}",
            if self.config.extensions.system_integration.install_as_service {
                "Enabled"
            } else {
                "Disabled"
            }
        );
        println!(
            "  💾 ZFS Support: {}",
            if self
                .config
                .extensions
                .components
                .selected_components
                .install_zfs
            {
                "Enabled"
            } else {
                "Disabled"
            }
        );
        println!(
            "  🎨 UI Components: {}",
            if self
                .config
                .extensions
                .components
                .selected_components
                .install_ui
            {
                "Enabled"
            } else {
                "Disabled"
            }
        );
        println!(
            "  🌐 Network Features: {}",
            if self
                .config
                .extensions
                .components
                .selected_components
                .install_network
            {
                "Enabled"
            } else {
                "Disabled"
            }
        );

        if self
            .config
            .extensions
            .system_integration
            .desktop_integration
        {
            println!("  🖥️  Desktop Entry: Will be created");
        } else {
            println!("  🖥️  Desktop Entry: Will not be created");
        }

        if self.config.extensions.system_integration.add_to_path {
            println!("  🛤️  PATH: Will be added to system PATH");
        } else {
            println!("  🛤️  PATH: Will not be added to system PATH");
        }

        if !Confirm::new()
            .with_prompt("Proceed with installation?")
            .default(true)
            .interact()?
        {
            anyhow::bail!("Installation cancelled by user");
        }

        Ok(true)
    }
}

impl Default for InstallationWizard {
    fn default() -> Self {
        Self::new()
    }
}
