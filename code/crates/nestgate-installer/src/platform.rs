//
// This module provides cross-platform system detection and validation
// to ensure NestGate can be installed and run properly on the target system.
//
// ## Features
// - Operating system detection (Linux, Windows, macOS)
// - Architecture detection (x86_64, ARM64)
// - System requirements validation
// - Platform-specific installation paths

use anyhow::Result;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct PlatformInfo {
    pub os: String,
    pub arch: String,
    pub supports_systemd: bool,
    pub supports_launchd: bool,
    pub supports_windows_service: bool,
    pub binary_extension: String,
}

impl PlatformInfo {
    #[must_use]
    pub fn detect() -> Self {
        let os = std::env::consts::OS.to_string();
        let arch = std::env::consts::ARCH.to_string();

        Self {
            supports_systemd: os == "linux",
            supports_launchd: os == "macos",
            supports_windows_service: os == "windows",
            binary_extension: if os == "windows" {
                ".exe".to_string()
            } else {
                String::new()
            },
            os,
            arch,
        }
    }

    #[must_use]
    pub fn service_install_supported(&self) -> bool {
        self.supports_systemd || self.supports_launchd || self.supports_windows_service
    }

    #[must_use]
    pub fn get_binary_name(&self, name: &str) -> String {
        format!("{}{}", name, ".exe")
    }
}

#[allow(dead_code)] // Reserved for installer PATH modification
pub fn add_to_path(install_path: &Path) -> Result<()> {
    #[cfg(unix)]
    {
        add_to_path_unix(install_path)
    }

    #[cfg(windows)]
    {
        add_to_path_windows(install_path)
    }
}

#[cfg(unix)]
#[allow(dead_code)] // Platform-specific PATH implementation
fn add_to_path_unix(install_path: &Path) -> Result<()> {
    use std::fs::OpenOptions;
    use std::io::Write;

    let shell_rc = if std::env::var("SHELL").unwrap_or_default().contains("zsh") {
        dirs::home_dir().map(|h| h.join(".zshrc"))
    } else {
        dirs::home_dir().map(|h| h.join(".bashrc"))
    };

    if let Some(rc_path) = shell_rc {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&rc_path)?;

        writeln!(file, "\n# Added by NestGate installer")?;
        writeln!(
            file,
            "export PATH=\"{}:$PATH\"",
            install_path.join("bin").display()
        )?;

        println!("Added {} to PATH in {:?}", install_path.display(), rc_path);
        println!("Please restart your shell or run: source {rc_path:?}");
    }

    Ok(())
}

#[cfg(windows)]
fn add_to_path_windows(install_path: &Path) -> Result<()> {
    use winreg::enums::*;
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env = hkcu.open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE)?;

    let current_path: String = env.getvalue("PATH").unwrap_or_default();
    let install_bin = install_path.join("bin");
    let install_bin_str = install_bin.to_string_lossy();

    if !current_path.contains(&*install_bin_str) {
        let new_path = if current_path.is_empty() {
            install_bin_str.to_string()
        } else {
            format!("{};{}", install_bin_str, current_path)
        };

        env.setvalue("PATH", &new_path)?;
        println!("Added {} to PATH", install_bin.display());
        println!("Please restart your command prompt to use the new PATH");
    }

    Ok(())
}

#[allow(dead_code)] // Reserved for desktop integration features
pub fn create_desktop_shortcut(install_path: &Path, name: &str) -> Result<()> {
    #[cfg(unix)]
    {
        create_desktop_shortcut_unix(install_path, name)
    }

    #[cfg(windows)]
    {
        create_desktop_shortcut_windows(install_path, name)
    }
}

#[cfg(unix)]
#[allow(dead_code)] // Platform-specific desktop integration
fn create_desktop_shortcut_unix(install_path: &Path, name: &str) -> Result<()> {
    use std::fs;

    if let Some(desktop_dir) = dirs::desktop_dir() {
        let shortcut_path = desktop_dir.join(format!("{}.desktop", "nestgate"));
        let binary_path = install_path.join("bin").join("nestgate");

        let desktop_entry = format!(
            r"[Desktop Entry]
Version=1.0
Type=Application
Name={}
Comment={}
Exec={}
Icon={}
Terminal=false
Categories=System;
",
            name,
            "NestGate Storage Management System",
            binary_path.display(),
            install_path
                .join("share")
                .join("icons")
                .join("nestgate.png")
                .display()
        );

        fs::write(&shortcut_path, desktop_entry)?;

        // Make executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&shortcut_path)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&shortcut_path, perms)?;
        }

        println!("Created desktop shortcut: {}", shortcut_path.display());
    }

    Ok(())
}

#[cfg(windows)]
fn create_desktop_shortcut_windows(_install_path: &Path, _name: &str) -> Result<()> {
    // Windows shortcut creation would require additional dependencies
    // For now, we'll skip this and focus on core functionality
    println!("Desktop shortcut creation not yet implemented on Windows");
    Ok(())
}
