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

    pub fn service_install_supported(&self) -> bool {
        self.supports_systemd || self.supports_launchd || self.supports_windows_service
    }

    pub fn get_binary_name(&self, name: &str) -> String {
        format!("{}{}", name, self.binary_extension)
    }
}

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
        println!("Please restart your shell or run: source {:?}", rc_path);
    }

    Ok(())
}

#[cfg(windows)]
fn add_to_path_windows(install_path: &Path) -> Result<()> {
    use winreg::enums::*;
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env = hkcu.open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE)?;

    let current_path: String = env.get_value("PATH").unwrap_or_default();
    let install_bin = install_path.join("bin");
    let install_bin_str = install_bin.to_string_lossy();

    if !current_path.contains(&*install_bin_str) {
        let new_path = if current_path.is_empty() {
            install_bin_str.to_string()
        } else {
            format!("{};{}", current_path, install_bin_str)
        };

        env.set_value("PATH", &new_path)?;
        println!("Added {} to PATH", install_bin.display());
        println!("Please restart your command prompt to use the new PATH");
    }

    Ok(())
}

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
fn create_desktop_shortcut_unix(install_path: &Path, name: &str) -> Result<()> {
    use std::fs;

    if let Some(desktop_dir) = dirs::desktop_dir() {
        let shortcut_path = desktop_dir.join(format!("{}.desktop", name));
        let binary_path = install_path.join("bin").join("nestgate");

        let desktop_entry = format!(
            r#"[Desktop Entry]
Version=1.0
Type=Application
Name={}
Comment=NestGate Storage Management System
Exec={}
Icon={}
Terminal=false
Categories=System;
"#,
            name,
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
