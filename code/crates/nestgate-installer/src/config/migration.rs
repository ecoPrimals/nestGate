/// Migration and config creation utilities for the installer
use super::*;
use nestgate_core::unified_config_consolidation::StandardDomainConfig;
use std::collections::HashMap;

/// Create a production-ready installer configuration
pub fn create_production_installer_config() -> UnifiedInstallerConfig {
    let extensions = InstallerExtensions {
        installation: InstallationSettings {
            mode: InstallMode::Production,
            install_dir: "/opt/nestgate".into(),
            config_dir: "/etc/nestgate".into(),
            data_dir: "/var/lib/nestgate".into(),
            log_dir: "/var/log/nestgate".into(),
            temp_dir: "/tmp/nestgate".into(),
            force_install: false,
            interactive: false,
            verbose: false,
        },
        components: ComponentSettings {
            selected_components: ComponentSelection {
                install_api: true,
                install_zfs: true,
                install_network: true,
                install_monitoring: true,
                install_security: true,
                install_automation: false,
                install_ui: false,
                install_nas: false,
                install_fsmonitor: false,
                install_mcp: false,
                custom_components: vec![],
            },
            dependencies: HashMap::new(),
            optional_components: vec![],
            component_configs: HashMap::new(),
            validate_components: true,
        },
        system_integration: SystemIntegrationSettings {
            install_as_service: true,
            service_name: "nestgate".to_string(),
            service_user: "nestgate".to_string(),
            service_group: "nestgate".to_string(),
            enable_autostart: true,
            create_firewall_rules: true,
            add_to_path: true,
            desktop_integration: false,
        },
        package_management: PackageManagementSettings {
            package_manager: PackageManagerType::Apt,
            repositories: vec![],
            dependencies: vec!["zfsutils-linux".to_string()],
            optional_packages: vec![],
            auto_updates: false,
            update_channel: "stable".to_string(),
        },
        validation: ValidationSettings::default(),
        post_install: PostInstallSettings::default(),
        deployment: DeploymentSettings::default(),
    };

    StandardDomainConfig::with_service(extensions, "nestgate-installer", env!("CARGO_PKG_VERSION"))
}
