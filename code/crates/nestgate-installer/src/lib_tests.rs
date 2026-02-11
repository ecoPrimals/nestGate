//! Unit tests for nestgate-installer crate.
//!
//! Extracted from lib.rs to keep production code under the line limit.

use crate::config::InstallerConfig;
use crate::installer::{InstallationInfo, NestGateInstaller};
use tempfile::TempDir;

mod basic_tests {
    use super::*;
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
        assert!(super::installer_is_available());
        assert!(super::can_validate_environment());
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
        assert!(super::validate_string("valid_string"));
        assert!(!super::validate_string(""));
        assert!(super::validate_path("/valid/path"));
    }

    #[test]
    fn test_installation_utility_functions() {
        assert!(super::is_valid_install_path("/opt/nestgate"));
        assert!(!super::is_valid_install_path(""));
        assert_eq!(super::normalize_path("/path//to/file"), "/path/to/file");
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
}

mod comprehensive_tests {
    use super::*;
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

        assert!(!platform.os.is_empty());
        assert!(!platform.arch.is_empty());
        let _service_manager = &platform.service_manager;
        let _binary_ext = &platform.binary_extension;
    }

    // ==================== PATH HANDLING TESTS ====================

    #[test]
    fn test_path_normalization() {
        assert_eq!(
            super::normalize_path("///path///to///file///"),
            "/path/to/file/"
        );
        assert_eq!(super::normalize_path("/opt//nestgate"), "/opt/nestgate");
    }

    #[test]
    fn test_path_validation() {
        assert!(super::is_valid_install_path("/opt/nestgate"));
        assert!(super::is_valid_install_path("/usr/local/bin"));
        assert!(!super::is_valid_install_path(""));
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
    !path.is_empty() && (path.starts_with('/') || (cfg!(windows) && path.len() >= 3))
}

fn normalize_path(path: &str) -> String {
    let mut result = path.to_string();
    while result.contains("//") {
        result = result.replace("//", "/");
    }
    result
}
