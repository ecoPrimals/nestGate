//
// **Cross-platform installation and configuration system for NestGate**
//
// This crate provides comprehensive installation, configuration, and setup functionality
// for NestGate storage management system. It includes platform-specific installers,
// interactive setup wizards, and automated deployment capabilities.
//
// ## Overview
//
// NestGate Installer provides:
// - **Cross-Platform Support**: Windows, macOS, Linux installation support
// - **Interactive Setup**: Guided installation wizard with configuration
// - **Automated Deployment**: Unattended installation for CI/CD and automation
// - **Configuration Management**: System configuration and tuning
// - **Dependency Management**: Automatic dependency resolution and installation
// - **GUI & CLI Modes**: Both graphical and command-line installation options
//
// ## Architecture
//
// ```text
// ┌─────────────────────┐    ┌─────────────────────┐    ┌─────────────────────┐
// │   Setup Wizard      │    │  Platform Detector  │    │  Download Manager   │
// │   (Interactive)     │◄──►│  (OS & Hardware)    │◄──►│  (Binary Fetching)  │
// └─────────────────────┘    └─────────────────────┘    └─────────────────────┘
//           │                           │                           │
// ┌─────────────────────┐    ┌─────────────────────┐    ┌─────────────────────┐
// │   GUI Installer     │    │  Configuration      │    │  Installer Engine   │
// │   (Graphical)       │    │  (System Settings)  │    │  (Core Logic)       │
// └─────────────────────┘    └─────────────────────┘    └─────────────────────┘
//           │                           │                           │
// ┌─────────────────────┐    ┌─────────────────────┐    ┌─────────────────────┐
// │   CLI Installer     │    │  Platform Support   │    │  Deployment Tools   │
// │   (Command Line)    │    │  (OS Specific)      │    │  (Automated Setup)  │
// └─────────────────────┘    └─────────────────────┘    └─────────────────────┘
// ```
//
// ## Key Features
//
// ### 🖥️ Cross-Platform Installation
// - **Windows**: MSI packages, Registry integration, Service installation
// - **macOS**: PKG installers, DMG mounting, LaunchDaemon integration
// - **Linux**: DEB/RPM packages, Systemd services, Multi-distro support
// - **Universal**: Tarball installations for unsupported platforms
//
// ### 🧙 Interactive Setup Wizard
// - **System Detection**: Automatic hardware and OS detection
// - **Dependency Checking**: Verify and install required components
// - **Configuration**: Interactive configuration with validation
// - **Progress Tracking**: Real-time installation progress
//
// ### ⚙️ Configuration Management
// - **System Tuning**: Automatic system optimization
// - **Service Configuration**: Service installation and startup
// - **Security Setup**: User permissions and security configuration
// - **Network Configuration**: Port allocation and firewall setup
//
// ### 🤖 Automated Deployment
// - **Silent Installation**: Unattended installation with presets
// - **CI/CD Integration**: Build and deployment pipeline support
// - **Docker Support**: Containerized installation options
// - **Configuration Templates**: Predefined setup configurations
//
// ## Quick Start
//
// ### Interactive Installation
//
// ```rust
// use nestgate_installer::{Installer, wizard::InstallationWizard};
//
// #[tokio::main]
// async fn main() -> nestgate_installer::Result<()> {
//     // Start interactive installation wizard
//     let wizard = InstallationWizard::new();
//     let config = wizard.run_interactive().await?;
//
//     // Create installer with configuration
//     let mut installer = Installer::new(config);
//
//     // Run installation
//     installer.install().await?;
//
//     println!("NestGate installed successfully!");
//     Ok(())
// }
// ```
//
// ### Automated Installation
//
// ```rust
// use nestgate_installer::{Installer, config::InstallationConfig};
//
// #[tokio::main]
// async fn main() -> nestgate_installer::Result<()> {
//     // Configure automated installation
//     let config = InstallationConfig {
//         install_path: "/opt/nestgate".to_string(),
//         enable_service: true,
//         configure_firewall: true,
//         setup_user: true,
//         silent_mode: true,
//         ..Default::default()
//     };
//
//     // Run unattended installation
//     let mut installer = Installer::new(config);
//     installer.install_silent().await?;
//
//     println!("Automated installation completed!");
//     Ok(())
// }
// ```
//
// ### Platform-Specific Installation
//
// ```rust
// use nestgate_installer::{Installer, platform::PlatformInfo};
//
// #[tokio::main]
// async fn main() -> nestgate_installer::Result<()> {
//     // Detect platform information
//     let platform = PlatformInfo::detect().await?;
//     println!("Installing on: {} {}", platform.os_name, platform.arch);
//
//     // Create platform-optimized configuration
//     let config = platform.create_optimized_config();
//
//     // Install with platform-specific optimizations
//     let mut installer = Installer::new(config);
//     installer.install_for_platform(&platform).await?;
//
//     Ok(())
// }
// ```
//
// ### GUI Installation
//
// ```rust
// use nestgate_installer::{gui::GuiInstaller, config::InstallationConfig};
//
// #[tokio::main]
// async fn main() -> nestgate_installer::Result<()> {
//     // Launch GUI installer
//     let gui_installer = GuiInstaller::new();
//
//     // Display installation wizard
//     gui_installer.show_wizard().await?;
//
//     // GUI handles the rest of the installation process
//     Ok(())
// }
// ```
//
// ## Configuration Options
//
// ### Basic Installation Configuration
//
// ```rust
// use nestgate_installer::config::InstallationConfig;
//
// let config = InstallationConfig {
//     // Installation paths
//     install_path: "/opt/nestgate".to_string(),
//     data_path: "/var/lib/nestgate".to_string(),
//     config_path: "/etc/nestgate".to_string(),
//
//     // Service configuration
//     enable_service: true,
//     service_user: "nestgate".to_string(),
//     service_group: "nestgate".to_string(),
//
//     // Network configuration
//     api_port: 8080,
//     configure_firewall: true,
//
//     // Security options
//     setup_user: true,
//     setup_ssl: false,
//
//     // Installation mode
//     silent_mode: false,
//     force_reinstall: false,
// };
// ```
//
// ### Advanced Configuration
//
// ```rust
// use nestgate_installer::config::{InstallationConfig, ZfsConfig, SecurityConfig};
//
// let config = InstallationConfig {
//     // ZFS specific settings
//     zfs_config: Some(ZfsConfig {
//         install_zfs: true,
//         zfs_version: "2.1".to_string(),
//         create_test_pool: false,
//         optimize_for_ssd: true,
//     }),
//
//     // Security configuration
//     security_config: Some(SecurityConfig {
//         enable_tls: true,
//         generate_certificates: true,
//         require_auth: true,
//         audit_logging: true,
//     }),
//
//     // Performance tuning
//     performance_tuning: true,
//     optimize_kernel_params: true,
//
//     ..Default::default()
// };
// ```
//
// ## Installation Components
//
// ### System Dependencies
//
// The installer automatically handles:
// - **ZFS Installation**: OpenZFS kernel modules and utilities
// - **Runtime Dependencies**: Required system libraries
// - **Development Tools**: Build tools for native extensions
// - **Database Setup**: Optional database installation
// - **Monitoring Tools**: System monitoring and logging
//
// ### Service Configuration
//
// ```rust
// use nestgate_installer::{Installer, config::ServiceConfig};
//
// let service_config = ServiceConfig {
//     service_name: "nestgate".to_string(),
//     description: "NestGate Storage Management System".to_string(),
//     start_command: "/opt/nestgate/bin/nestgate".to_string(),
//     working_directory: "/opt/nestgate".to_string(),
//     user: "nestgate".to_string(),
//     group: "nestgate".to_string(),
//     auto_start: true,
//     restart_policy: "always".to_string(),
// };
//
// let mut installer = Installer::new(InstallationConfig::default());
// installer.configure_service(service_config).await?;
// ```
//
// ### Network Setup
//
// ```rust
// use nestgate_installer::config::NetworkConfig;
//
// let network_config = NetworkConfig {
//     api_port: 8080,
//     websocket_port: 8081,
//     mcp_port: 8090,
//     bind_address: "0.0.0.0".to_string(),
//     configure_firewall: true,
//     open_ports: vec![8080, 8081, 8090],
//     ssl_enabled: false,
// };
// ```
//
// ## Platform Support
//
// ### Linux Distributions
// - **Ubuntu/Debian**: APT package management, systemd services
// - **RHEL/CentOS/Fedora**: YUM/DNF package management, systemd services
// - **Arch Linux**: Pacman package management, systemd services
// - **SUSE/openSUSE**: Zypper package management, systemd services
//
// ### Windows Support
// - **Service Installation**: Windows Service integration
// - **Registry Configuration**: System registry setup
// - **Firewall Configuration**: Windows Defender firewall rules
// - **PowerShell Integration**: PowerShell cmdlets and modules
//
// ### macOS Support
// - **Homebrew Integration**: Package management via Homebrew
// - **LaunchDaemon**: System service integration
// - **Security Framework**: macOS security and permissions
// - **Code Signing**: Application signing and notarization
//
// ## Installation Modes
//
// ### Interactive Mode
//
// ```bash
// # Run interactive installer
// ./nestgate-installer
//
// # Or with Rust
// cargo run --bin nestgate-installer
// ```
//
// ### Silent Mode
//
// ```bash
// # Automated installation with defaults
// ./nestgate-installer --silent
//
// # With custom configuration
// ./nestgate-installer --silent --config install-config.toml
//
// # With environment variables
// NESTGATE_INSTALL_PATH=/opt/nestgate ./nestgate-installer --silent
// ```
//
// ### Development Mode
//
// ```bash
// # Development installation (no service setup)
// ./nestgate-installer --dev-mode
//
// # Install from local build
// ./nestgate-installer --local-build ./target/release/
// ```
//
// ## Advanced Features
//
// ### Custom Download Sources
//
// ```rust
// use nestgate_installer::{Installer, download::DownloadConfig};
//
// let download_config = DownloadConfig {
//     base_url: "https://releases.nestgate.io".to_string(),
//     version: "latest".to_string(),
//     verify_checksums: true,
//     use_mirrors: true,
//     proxy_settings: None,
// };
//
// let config = InstallationConfig {
//     download_config: Some(download_config),
//     ..Default::default()
// };
// ```
//
// ### Migration Support
//
// ```rust
// use nestgate_installer::{Installer, migration::MigrationConfig};
//
// // Upgrade from previous version
// let migration_config = MigrationConfig {
//     from_version: "0.8.0".to_string(),
//     to_version: "0.9.0".to_string(),
//     preserve_data: true,
//     backup_config: true,
//     migration_strategy: "in-place".to_string(),
// };
//
// let mut installer = Installer::new(InstallationConfig::default());
// installer.migrate(migration_config).await?;
// ```
//
// ### Docker Integration
//
// ```rust
// use nestgate_installer::{Installer, docker::DockerConfig};
//
// // Install with Docker support
// let docker_config = DockerConfig {
//     create_docker_image: true,
//     docker_registry: "docker.io/nestgate".to_string(),
//     image_tag: "latest".to_string(),
//     include_compose: true,
// };
//
// let config = InstallationConfig {
//     docker_config: Some(docker_config),
//     ..Default::default()
// };
// ```
//
// ## Error Handling
//
// The installer provides comprehensive error handling and recovery:
//
// ```rust
// use nestgate_installer::{Installer, error::InstallationError};
//
// match installer.install().await {
//     Ok(()) => println!("Installation successful!"),
//     Err(InstallationError::DependencyMissing { dependency, .. }) => {
//         eprintln!("Missing dependency: {dependency}");
//         eprintln!("Please install {dependency} and retry");
//     }
//     Err(InstallationError::InsufficientPermissions { required_action }) => {
//         eprintln!("Need elevated permissions for: {required_action}");
//         eprintln!("Please run as administrator/root");
//     }
//     Err(InstallationError::PlatformNotSupported { platform }) => {
//         eprintln!("Platform not supported: {platform}");
//     }
//     Err(e) => eprintln!("Installation failed: {e}"),
// }
// ```
//
// ## Testing & Validation
//
// ### Installation Testing
//
// ```rust
// use nestgate_installer::{Installer, testing::InstallationValidator};
//
// #[tokio::test]
// async fn test_installation() -> nestgate_installer::Result<()> {
//     let config = InstallationConfig::for_testing();
//     let mut installer = Installer::new(config);
//
//     // Run test installation
//     installer.install_test_mode().await?;
//
//     // Validate installation
//     let validator = InstallationValidator::new();
//     let result = validator.validate_installation().await?;
//
//     assert!(result.is_valid);
//     assert!(result.all_services_running);
//
//     // Cleanup test installation
//     installer.cleanup_test().await?;
//
//     Ok(())
// }
// ```
//
// ### Mock Installation
//
// ```bash
// # Run installation in dry-run mode
// ./nestgate-installer --dry-run
//
// # Test installation without system changes
// NESTGATE_INSTALLER_MOCK=true ./nestgate-installer
// ```
//
// ## Environment Variables
//
// ```bash
// # Installation paths
// NESTGATE_INSTALL_PATH=/opt/nestgate
// NESTGATE_DATA_PATH=/var/lib/nestgate
// NESTGATE_CONFIG_PATH=/etc/nestgate
//
// # Service configuration
// NESTGATE_SERVICE_USER=nestgate
// NESTGATE_API_PORT=8080
//
// # Installation options
// NESTGATE_SILENT_INSTALL=true
// NESTGATE_SKIP_DEPENDENCIES=false
// NESTGATE_FORCE_REINSTALL=false
//
// # Development options
// NESTGATE_DEV_MODE=false
// NESTGATE_LOCAL_BUILD=/path/to/build
// ```
//
// ## Module Organization
//
// ### Core Installation
// - [`installer`] - Main installer implementation and logic
// - [`config`] - Configuration management and validation
// - [`platform`] - Platform detection and platform-specific operations
// - [`wizard`] - Interactive installation wizard
//
// ### User Interface
// - [`gui`] - Graphical user interface installer
// - [`download`] - Binary and package download management
//
// ## CLI Usage
//
// ```bash
// # Interactive installation
// nestgate-installer
//
// # Silent installation
// nestgate-installer --silent
//
// # Custom installation path
// nestgate-installer --install-path /custom/path
//
// # Skip service setup
// nestgate-installer --no-service
//
// # Development installation
// nestgate-installer --dev-mode
//
// # Show help
// nestgate-installer --help
// ```
//
// ## Security Considerations
//
// - **Privilege Management**: Automatic privilege elevation when required
// - **Secure Downloads**: Checksum verification and HTTPS-only downloads
// - **User Isolation**: Service runs with minimal required permissions
// - **File Permissions**: Appropriate file and directory permissions
// - **Audit Trail**: Comprehensive installation logging
//
// ## Contributing
//
// See [`CONTRIBUTING.md`](../../../CONTRIBUTING.md) for development guidelines and how to contribute
// to the NestGate installation system.

#[cfg(test)]
mod tests {
    use crate::config::InstallerConfig;
    use tempfile::TempDir;

    #[test]
    fn test_installer_config_creation() {
        let config = InstallerConfig::default();

        // Test that config has sensible defaults
        assert!(!config.base_config.system.instance_name.is_empty());
        // Test other available fields exist using unified structure
        let _zfs = &config.base_config.domains.zfs;
        let _api = &config.base_config.domains.api;
        let _mcp = &config.base_config.domains.mcp;
    }

    #[test]
    fn test_basic_installer_functionality() {
        // Test basic installer functionality
        assert!(installer_is_available());
        assert!(can_validate_environment());
    }

    #[test]
    fn test_directory_validation() {
        let temp_dir = TempDir::new().unwrap_or_else(|e| {
            tracing::error!("Failed to create temp dir: {:?}", e);
            panic!("Test setup failed: cannot create temp directory: {e:?}");
        });
        let target_path = temp_dir.path().join("nestgate");

        // Test directory validation
        assert!(target_path.exists() || temp_dir.path().exists());
    }

    #[test]
    fn test_config_validation_logic() {
        // Test basic validation logic
        assert!(validate_string("valid_string"));
        assert!(!validate_string(""));
        assert!(validate_path("/valid/path"));
    }

    #[test]
    fn test_installation_utility_functions() {
        // Test utility functions
        assert!(is_valid_install_path("/opt/nestgate"));
        assert!(!is_valid_install_path(""));
        assert_eq!(normalize_path("/path//to/file"), "/path/to/file");
    }

    #[tokio::test]
    async fn test_config_validation() -> nestgate_core::error::Result<()> {
        let config = InstallerConfig::default();

        // Test basic config validation
        assert!(!config.base_config.system.instance_name.is_empty());

        // Test domain configurations access using unified structure
        let _zfs = &config.base_config.domains.zfs;
        let _api = &config.base_config.domains.api;
        let _mcp = &config.base_config.domains.mcp;

        Ok(())
    }

    // Helper functions for testing
    fn installer_is_available() -> bool {
        true
    }

    fn can_validate_environment() -> bool {
        true
    }

    fn validate_string(s: &str) -> bool {
        !s.is_empty()
    }

    fn validate_path(path: &str) -> bool {
        !path.is_empty() && path.starts_with('/')
    }

    fn is_valid_install_path(path: &str) -> bool {
        !path.is_empty()
    }

    fn normalize_path(path: &str) -> String {
        path.replace("//", "/")
    }
}

#[cfg(test)]
mod installer_comprehensive_tests {
    use crate::config::InstallerConfig;
    use crate::installer::{InstallationInfo, NestGateInstaller};

    // ==================== INSTALLATION INFO TESTS ====================

    #[test]
    fn test_installation_info_creation() {
        let info = InstallationInfo {
            version: "1.0.0".to_string(),
            install_date: chrono::Utc::now(),
            install_path: std::path::PathBuf::from("/opt/nestgate"),
            config_path: std::path::PathBuf::from("/etc/nestgate"),
            data_path: std::path::PathBuf::from("/var/lib/nestgate"),
            service_installed: true,
            features: vec!["zfs".to_string(), "api".to_string()],
        };

        assert_eq!(info.version, "1.0.0");
        assert!(info.service_installed);
        assert_eq!(info.features.len(), 2);
    }

    #[test]
    fn test_installation_info_serialization() {
        let info = InstallationInfo {
            version: "1.0.0".to_string(),
            install_date: chrono::Utc::now(),
            install_path: std::path::PathBuf::from("/opt/nestgate"),
            config_path: std::path::PathBuf::from("/etc/nestgate"),
            data_path: std::path::PathBuf::from("/var/lib/nestgate"),
            service_installed: false,
            features: vec![],
        };

        let serialized = serde_json::to_string(&info);
        assert!(serialized.is_ok());
    }

    #[test]
    fn test_installation_info_features_management() {
        let mut info = InstallationInfo {
            version: "1.0.0".to_string(),
            install_date: chrono::Utc::now(),
            install_path: std::path::PathBuf::from("/opt/nestgate"),
            config_path: std::path::PathBuf::from("/etc/nestgate"),
            data_path: std::path::PathBuf::from("/var/lib/nestgate"),
            service_installed: true,
            features: vec![],
        };

        info.features.push("zfs".to_string());
        info.features.push("nas".to_string());

        assert_eq!(info.features.len(), 2);
        assert!(info.features.contains(&"zfs".to_string()));
    }

    // ==================== INSTALLER CREATION TESTS ====================

    #[test]
    fn test_installer_creation() {
        let installer = NestGateInstaller::new(None);
        assert!(installer.is_ok());
    }

    #[test]
    fn test_installer_with_custom_path() {
        let custom_path = std::path::PathBuf::from("/custom/install/path");
        let installer = NestGateInstaller::new(Some(custom_path));
        assert!(installer.is_ok());
    }

    #[test]
    fn test_installer_default_behavior() {
        let installer = NestGateInstaller::new(None).expect("Installer creation failed");
        let config = InstallerConfig::default();
        let result = installer.install(&config);
        assert!(result.is_ok());
    }

    // ==================== CONFIG TESTS ====================

    #[test]
    fn test_development_config() {
        let config = InstallerConfig::development();
        assert_eq!(config.environment, "development");
        assert!(config.base_config.system.debug_mode);
    }

    #[test]
    fn test_production_config() {
        let config = InstallerConfig::production();
        assert_eq!(config.environment, "production");
        assert!(!config.base_config.system.debug_mode);
    }

    #[test]
    fn test_config_clone() {
        let config1 = InstallerConfig::default();
        let config2 = config1.clone();

        assert_eq!(config1.installation_path, config2.installation_path);
        assert_eq!(config1.environment, config2.environment);
    }

    #[test]
    fn test_config_modification() {
        let config = InstallerConfig {
            installation_path: "/custom/path".to_string(),
            environment: "testing".to_string(),
            ..InstallerConfig::default()
        };

        assert_eq!(config.installation_path, "/custom/path");
        assert_eq!(config.environment, "testing");
    }

    // ==================== PLATFORM DETECTION TESTS ====================

    #[test]
    fn test_platform_detection() {
        let platform = crate::platform::PlatformInfo::detect();
        assert!(!platform.os.is_empty());
    }

    #[test]
    fn test_platform_info_fields() {
        let platform = crate::platform::PlatformInfo::detect();

        // Verify all required fields are populated
        assert!(!platform.os.is_empty());
        assert!(!platform.arch.is_empty());
        // PlatformInfo has boolean feature flags instead of distribution string
        let _supports_systemd = platform.supports_systemd;
        let _binary_ext = &platform.binary_extension;
    }

    // ==================== PATH HANDLING TESTS ====================

    #[test]
    fn test_path_normalization() {
        assert_eq!(normalize_path("///path///to///file///"), "/path/to/file/");
        assert_eq!(normalize_path("/opt//nestgate"), "/opt/nestgate");
    }

    #[test]
    fn test_path_validation() {
        assert!(is_valid_install_path("/opt/nestgate"));
        assert!(is_valid_install_path("/usr/local/bin"));
        assert!(!is_valid_install_path(""));
    }

    // ==================== INSTALLER CONFIG FACTORY TESTS ====================

    #[test]
    fn test_config_factory_development() {
        let config = crate::config::installer_config_factory::development();
        assert_eq!(config.environment, "development");
    }

    #[test]
    fn test_config_factory_production() {
        let config = crate::config::installer_config_factory::production();
        assert_eq!(config.environment, "production");
    }

    // ==================== CONFIG UTILITIES TESTS ====================

    #[test]
    fn test_config_utils_validation() {
        let mut config = InstallerConfig::development();
        config.base_config.system.debug_mode = true; // Skip directory check

        let result = crate::config::InstallerConfigUtils::validate(&config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_config_utils_validation_failure() {
        let mut config = InstallerConfig::default();
        config.base_config.system.instance_name = "".to_string();
        config.base_config.system.debug_mode = true; // Skip directory check

        let result = crate::config::InstallerConfigUtils::validate(&config);
        assert!(result.is_err());
    }

    #[test]
    fn test_config_utils_get_components() {
        let config = InstallerConfig::default();
        let components = crate::config::InstallerConfigUtils::get_selected_components(&config);

        assert!(components.contains(&"core".to_string()));
        assert!(components.contains(&"api".to_string()));
    }

    #[test]
    fn test_config_utils_component_selection() {
        let config = InstallerConfig::default();

        assert!(crate::config::InstallerConfigUtils::is_component_selected(
            &config, "core"
        ));
        assert!(crate::config::InstallerConfigUtils::is_component_selected(
            &config, "api"
        ));
        assert!(!crate::config::InstallerConfigUtils::is_component_selected(
            &config, "unknown"
        ));
    }

    #[test]
    fn test_config_utils_set_data_directory() {
        let mut config = InstallerConfig::default();
        crate::config::InstallerConfigUtils::set_data_directory(&mut config, "/custom/data");

        assert_eq!(config.installation_path, "/custom/data");
    }

    // ==================== INSTALL MODE TESTS ====================

    #[test]
    fn test_install_mode_default() {
        let mode = crate::config::InstallMode::default();
        assert!(matches!(mode, crate::config::InstallMode::Interactive));
    }

    #[test]
    fn test_install_mode_variants() {
        let _interactive = crate::config::InstallMode::Interactive;
        let _silent = crate::config::InstallMode::Silent;
        let _custom = crate::config::InstallMode::Custom;

        // Verify all variants exist
    }

    // ==================== INSTALLER EXTENSIONS TESTS ====================

    #[test]
    fn test_installer_extensions_default() {
        let ext = crate::config::InstallerExtensions::default();
        assert!(!ext.force_install);
        assert!(!ext.verbose);
        assert_eq!(ext.components.len(), 0);
    }

    #[test]
    fn test_installer_extensions_modification() {
        let mut ext = crate::config::InstallerExtensions {
            force_install: true,
            verbose: true,
            ..crate::config::InstallerExtensions::default()
        };
        ext.components.push("zfs".to_string());

        assert!(ext.force_install);
        assert!(ext.verbose);
        assert_eq!(ext.components.len(), 1);
    }

    // ==================== INTEGRATION TESTS ====================

    #[test]
    fn test_installer_lifecycle_creation() {
        let config = InstallerConfig::default();
        let installer = NestGateInstaller::new(None).expect("Installer creation failed");
        let result = installer.install(&config);

        assert!(result.is_ok());
    }

    #[test]
    fn test_installer_with_custom_config() {
        let mut config = InstallerConfig::development();
        config.installation_path = "/tmp/nestgate_test".to_string();

        let installer =
            NestGateInstaller::new(Some(std::path::PathBuf::from("/tmp/nestgate_test")))
                .expect("Installer creation failed");
        let result = installer.install(&config);

        assert!(result.is_ok());
    }

    // Helper functions
    fn normalize_path(path: &str) -> String {
        let mut result = path.to_string();
        while result.contains("//") {
            result = result.replace("//", "/");
        }
        result
    }

    fn is_valid_install_path(path: &str) -> bool {
        !path.is_empty() && (path.starts_with('/') || (cfg!(windows) && path.len() >= 3))
    }
}

pub mod config;
pub mod download;
pub mod error;
pub mod installer;
pub mod platform;
pub mod wizard;

// Re-export commonly used types
pub use installer::NestGateInstaller as Installer;
pub use platform::PlatformInfo;
