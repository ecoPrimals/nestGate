//
// This module provides cross-platform system detection and validation
// to ensure NestGate can be installed and run properly on the target system.
//
// ## Features
// - Operating system detection (Linux, Windows, macOS)
// - Architecture detection (x86_64, ARM64)
// - System requirements validation
// - Platform-specific installation paths

//! Platform module

use anyhow::Result;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct PlatformInfo {
    /// Os
    pub os: String,
    /// Arch
    pub arch: String,
    /// Supports Systemd
    pub supports_systemd: bool,
    /// Supports Launchd
    pub supports_launchd: bool,
    /// Supports Windows Service
    pub supports_windows_service: bool,
    #[allow(dead_code)] // Reserved for future binary extension support
    /// Binary Extension
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
    use etcetera::BaseStrategy;
    use std::fs::OpenOptions;
    use std::io::Write;

    let shell_rc = if std::env::var("SHELL").unwrap_or_default().contains("zsh") {
        etcetera::base_strategy::choose_base_strategy()
            .ok()
            .map(|strategy| strategy.home_dir().join(".zshrc"))
    } else {
        etcetera::base_strategy::choose_base_strategy()
            .ok()
            .map(|strategy| strategy.home_dir().join(".bashrc"))
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

    // Note: etcetera doesn't have desktop_dir, use XDG standard
    use etcetera::BaseStrategy;
    let desktop_dir: Option<PathBuf> = std::env::var("XDG_DESKTOP_DIR")
        .ok()
        .map(PathBuf::from)
        .or_else(|| {
            etcetera::base_strategy::choose_base_strategy()
                .ok()
                .map(|strategy| strategy.home_dir().join("Desktop"))
        });

    if let Some(desktop_dir) = desktop_dir {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_info_detect() {
        let info = PlatformInfo::detect();
        assert!(!info.os.is_empty());
        assert!(!info.arch.is_empty());
    }

    #[test]
    fn test_platform_info_os_values() {
        let info = PlatformInfo::detect();
        // Should be one of the common OS values
        assert!(
            info.os == "linux"
                || info.os == "macos"
                || info.os == "windows"
                || info.os == "freebsd"
        );
    }

    #[test]
    fn test_platform_info_arch_values() {
        let info = PlatformInfo::detect();
        // Should be a valid architecture
        assert!(
            info.arch == "x86_64"
                || info.arch == "aarch64"
                || info.arch == "arm"
                || info.arch == "i686"
        );
    }

    #[test]
    fn test_platform_info_clone() {
        let info = PlatformInfo::detect();
        let cloned = info.clone();
        assert_eq!(info.os, cloned.os);
        assert_eq!(info.arch, cloned.arch);
        assert_eq!(info.supports_systemd, cloned.supports_systemd);
    }

    #[test]
    fn test_platform_info_debug() {
        let info = PlatformInfo::detect();
        let debug_str = format!("{:?}", info);
        assert!(debug_str.contains("PlatformInfo"));
    }

    #[test]
    fn test_service_install_supported() {
        let info = PlatformInfo::detect();
        let supported = info.service_install_supported();
        // At least one service manager should be supported on common platforms
        assert!(supported);
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_linux_specific_flags() {
        let info = PlatformInfo::detect();
        assert!(info.supports_systemd);
        assert!(!info.supports_launchd);
        assert!(!info.supports_windows_service);
        assert_eq!(info.os, "linux");
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_macos_specific_flags() {
        let info = PlatformInfo::detect();
        assert!(!info.supports_systemd);
        assert!(info.supports_launchd);
        assert!(!info.supports_windows_service);
        assert_eq!(info.os, "macos");
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_windows_specific_flags() {
        let info = PlatformInfo::detect();
        assert!(!info.supports_systemd);
        assert!(!info.supports_launchd);
        assert!(info.supports_windows_service);
        assert_eq!(info.os, "windows");
    }

    #[test]
    fn test_get_binary_name() {
        let info = PlatformInfo::detect();
        let binary = info.get_binary_name("nestgate");
        assert!(binary.contains("nestgate"));
        assert!(binary.ends_with(".exe"));
    }

    #[test]
    fn test_get_binary_name_different_names() {
        let info = PlatformInfo::detect();
        let names = vec!["app", "server", "daemon", "service"];

        for name in names {
            let binary = info.get_binary_name(name);
            assert!(binary.starts_with(name));
        }
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_binary_extension_windows() {
        let info = PlatformInfo::detect();
        assert_eq!(info.binary_extension, ".exe");
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn test_binary_extension_unix() {
        let info = PlatformInfo::detect();
        assert_eq!(info.binary_extension, "");
    }

    #[test]
    fn test_platform_info_multiple_detect_calls() {
        let info1 = PlatformInfo::detect();
        let info2 = PlatformInfo::detect();

        assert_eq!(info1.os, info2.os);
        assert_eq!(info1.arch, info2.arch);
        assert_eq!(info1.supports_systemd, info2.supports_systemd);
    }

    #[test]
    fn test_platform_info_service_manager_exclusivity() {
        let info = PlatformInfo::detect();

        // Only one service manager should be supported at a time
        let count = [
            info.supports_systemd,
            info.supports_launchd,
            info.supports_windows_service,
        ]
        .iter()
        .filter(|&&x| x)
        .count();

        assert_eq!(count, 1, "Exactly one service manager should be supported");
    }

    #[test]
    fn test_platform_info_consistency() {
        let info = PlatformInfo::detect();

        // Verify consistency between OS and service manager support
        if info.os == "linux" {
            assert!(info.supports_systemd);
        } else if info.os == "macos" {
            assert!(info.supports_launchd);
        } else if info.os == "windows" {
            assert!(info.supports_windows_service);
        }
    }
}
