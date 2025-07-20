use crate::config::InstallerConfig;
use crate::installer::NestGateInstaller;
use anyhow::Result;

/// Run the GUI installer application
///
/// # Errors
///
/// Returns an error if:
/// - GUI framework initialization fails
/// - User interface creation fails
/// - Installation process encounters errors
pub async fn run_gui_installer() -> Result<()> {
    println!("🎨 GUI installer not yet implemented");
    println!("💡 Please use the command line installer instead:");
    println!("   cargo run --bin nestgate-installer");

    // For now, fall back to CLI installer
    let config = InstallerConfig::default();
    let mut installer = NestGateInstaller::new(Some(config.install_path.clone()))?;
    installer
        .install(
            false,               // force
            config.service_mode, // as_service
            false,               // skip_zfs
            true,                // yes (auto-confirm)
        )
        .await?;

    Ok(())
}
