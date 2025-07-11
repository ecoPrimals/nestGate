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
        self.configure_features()?;
        self.configure_networking()?;
        self.configure_system_integration()?;
        self.confirm_installation()?;

        Ok(self.config.clone())
    }

    fn configure_installation_path(&mut self) -> Result<()> {
        println!("📁 Installation Directory");

        let default_path = self.config.install_path.to_string_lossy().to_string();
        let custom_path: String = Input::new()
            .with_prompt("Installation directory")
            .default(default_path)
            .interact_text()?;

        self.config.install_path = PathBuf::from(custom_path);

        // Validate the path
        if let Err(e) = self.config.validate() {
            println!("⚠️  Warning: {}", e);
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
            self.config.service_mode = Confirm::new()
                .with_prompt("Install as system service?")
                .default(true)
                .interact()?;

            if self.config.service_mode {
                println!("✅ NestGate will be installed as a system service");
                println!("   - Automatic startup on boot");
                println!("   - Background operation");
                println!("   - System-level privileges");
            }
        } else {
            println!("ℹ️  System service installation not supported on this platform");
            self.config.service_mode = false;
        }

        Ok(())
    }

    fn configure_features(&mut self) -> Result<()> {
        println!("\n⚙️  Feature Configuration");

        // ZFS Support
        self.config.enable_zfs = Confirm::new()
            .with_prompt("Enable ZFS filesystem support?")
            .default(true)
            .interact()?;

        if self.config.enable_zfs {
            println!("✅ ZFS support enabled");
            println!("   - Advanced storage management");
            println!("   - Snapshots and replication");
            println!("   - Data integrity verification");
        }

        // AI Features
        self.config.ai_enabled = Confirm::new()
            .with_prompt("Enable AI-powered features?")
            .default(false)
            .interact()?;

        if self.config.ai_enabled {
            println!("✅ AI features enabled");
            println!("   - Intelligent data organization");
            println!("   - Predictive storage optimization");
            println!("   - Automated maintenance");
        }

        Ok(())
    }

    fn configure_networking(&mut self) -> Result<()> {
        println!("\n🌐 Network Configuration");

        // API Port
        let port_input: String = Input::new()
            .with_prompt("API server port")
            .default(self.config.api_port.to_string())
            .validate_with(|input: &String| -> Result<(), &str> {
                match input.parse::<u16>() {
                    Ok(port) if port > 0 => Ok(()),
                    _ => Err("Please enter a valid port number (1-65535)"),
                }
            })
            .interact_text()?;

        self.config.api_port = port_input.parse().unwrap();

        // Universal Primal Orchestration Integration
        let orchestration_integration = Confirm::new()
            .with_prompt("Configure primal orchestration integration?")
            .default(false)
            .interact()?;

        if orchestration_integration {
            let orchestration_url: String = Input::new()
                .with_prompt("Orchestration primal URL")
                .default(
                    std::env::var("NESTGATE_UI_URL")
                        .unwrap_or_else(|_| "http://localhost:3000".to_string()),
                )
                .interact_text()?;

            self.config.orchestration_url = Some(orchestration_url);

            println!("✅ Primal orchestration integration configured");
            println!("   - Cross-primal networking");
            println!("   - Enhanced security");
            println!("   - Multi-system coordination");
        } else {
            println!("ℹ️  Running in standalone mode");
            println!("   - Local system access only");
            println!("   - Direct network binding");
        }

        Ok(())
    }

    fn configure_system_integration(&mut self) -> Result<()> {
        println!("\n🔗 System Integration");

        // PATH integration
        self.config.add_to_path = Confirm::new()
            .with_prompt("Add NestGate to system PATH?")
            .default(true)
            .interact()?;

        if self.config.add_to_path {
            println!("✅ Will add to PATH - you can run 'nestgate' from anywhere");
        }

        // Desktop shortcut
        self.config.create_desktop_shortcut = Confirm::new()
            .with_prompt("Create desktop shortcut?")
            .default(true)
            .interact()?;

        if self.config.create_desktop_shortcut {
            println!("✅ Desktop shortcut will be created");
        }

        Ok(())
    }

    fn confirm_installation(&self) -> Result<()> {
        println!("\n📋 Installation Summary");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("Installation Path: {}", self.config.install_path.display());
        println!(
            "Service Mode: {}",
            if self.config.service_mode {
                "Yes"
            } else {
                "No"
            }
        );
        println!(
            "ZFS Support: {}",
            if self.config.enable_zfs { "Yes" } else { "No" }
        );
        println!(
            "AI Features: {}",
            if self.config.ai_enabled { "Yes" } else { "No" }
        );
        println!("API Port: {}", self.config.api_port);

        if let Some(url) = &self.config.orchestration_url {
            println!("Orchestration URL: {}", url);
        } else {
            println!("Orchestration: Standalone mode");
        }

        println!(
            "Add to PATH: {}",
            if self.config.add_to_path { "Yes" } else { "No" }
        );
        println!(
            "Desktop Shortcut: {}",
            if self.config.create_desktop_shortcut {
                "Yes"
            } else {
                "No"
            }
        );

        println!("\n🚀 Ready to install NestGate with the above configuration.");

        if !Confirm::new()
            .with_prompt("Proceed with installation?")
            .default(true)
            .interact()?
        {
            anyhow::bail!("Installation cancelled by user");
        }

        Ok(())
    }
}

impl Default for InstallationWizard {
    fn default() -> Self {
        Self::new()
    }
}
