/// Installer Execution Configuration
/// Post-install execution, deployment settings, and installer config implementation methods
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use super::platform::{DeploymentMode, InstallMode, ResourceConstraints};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PostInstallSettings {
    /// Configuration setup
    pub configuration: PostInstallConfigSettings,
    /// Initial data setup
    pub data_setup: InitialDataSettings,
    /// Documentation and help
    pub documentation: DocumentationSettings,
    /// Completion actions
    pub completion: CompletionSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostInstallConfigSettings {
    /// Generate default configuration
    pub generate_default_config: bool,
    /// Configuration template
    pub config_template: Option<String>,
    /// Environment-specific settings
    pub environment_configs: HashMap<String, String>,
    /// Enable configuration wizard
    pub enable_wizard: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitialDataSettings {
    /// Create sample data
    pub create_sample_data: bool,
    /// Initialize databases
    pub initialize_databases: bool,
    /// Setup initial users
    pub setup_initial_users: bool,
    /// Import initial configuration
    pub import_initial_config: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationSettings {
    /// Install documentation
    pub install_docs: bool,
    /// Documentation formats
    pub doc_formats: Vec<String>,
    /// Generate getting started guide
    pub generate_getting_started: bool,
    /// Install man pages
    pub install_man_pages: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionSettings {
    /// Show completion message
    pub show_completion_message: bool,
    /// Launch application after install
    pub launch_after_install: bool,
    /// Open documentation
    pub open_documentation: bool,
    /// Register with system
    pub register_with_system: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentSettings {
    /// Deployment mode
    pub mode: DeploymentMode,
    /// Target environments
    pub targets: Vec<DeploymentTarget>,
    /// Orchestration settings
    pub orchestration: OrchestrationSettings,
    /// Scaling settings
    pub scaling: ScalingSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentTarget {
    /// Target name
    pub name: String,
    /// Target type (local, docker, kubernetes, etc.)
    pub target_type: String,
    /// Connection settings
    pub connection: HashMap<String, String>,
    /// Environment variables
    pub environment: HashMap<String, String>,
    /// Resource constraints
    pub resources: ResourceConstraints,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestrationSettings {
    /// Enable orchestration
    pub enabled: bool,
    /// Orchestrator type
    pub orchestrator: String,
    /// Deployment manifest
    pub manifest: Option<String>,
    /// Rolling update settings
    pub rolling_update: RollingUpdateSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollingUpdateSettings {
    /// Enable rolling updates
    pub enabled: bool,
    /// Update strategy
    pub strategy: String,
    /// Max unavailable
    pub max_unavailable: String,
    /// Max surge
    pub max_surge: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingSettings {
    /// Enable auto-scaling
    pub auto_scaling: bool,
    /// Minimum replicas
    pub min_replicas: u32,
    /// Maximum replicas
    pub max_replicas: u32,
    /// Scaling metrics
    pub metrics: Vec<ScalingMetric>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingMetric {
    /// Metric name
    pub name: String,
    /// Target value
    pub target_value: String,
    /// Metric type
    pub metric_type: String,
}

impl Default for PostInstallConfigSettings {
    fn default() -> Self {
        Self {
            generate_default_config: true,
            config_template: None,
            environment_configs: HashMap::new(),
            enable_wizard: true,
        }
    }
}

impl Default for InitialDataSettings {
    fn default() -> Self {
        Self {
            create_sample_data: false,
            initialize_databases: true,
            setup_initial_users: true,
            import_initial_config: false,
        }
    }
}

impl Default for DocumentationSettings {
    fn default() -> Self {
        Self {
            install_docs: true,
            doc_formats: vec!["html".to_string(), "pdf".to_string()],
            generate_getting_started: true,
            install_man_pages: true,
        }
    }
}

impl Default for CompletionSettings {
    fn default() -> Self {
        Self {
            show_completion_message: true,
            launch_after_install: false,
            open_documentation: false,
            register_with_system: true,
        }
    }
}

impl Default for DeploymentSettings {
    fn default() -> Self {
        Self {
            mode: DeploymentMode::SingleNode,
            targets: Vec::new(),
            orchestration: OrchestrationSettings::default(),
            scaling: ScalingSettings::default(),
        }
    }
}

impl Default for OrchestrationSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            orchestrator: "none".to_string(),
            manifest: None,
            rolling_update: RollingUpdateSettings::default(),
        }
    }
}

impl Default for RollingUpdateSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            strategy: "RollingUpdate".to_string(),
            max_unavailable: "25%".to_string(),
            max_surge: "25%".to_string(),
        }
    }
}

impl Default for ScalingSettings {
    fn default() -> Self {
        Self {
            auto_scaling: false,
            min_replicas: 1,
            max_replicas: 3,
            metrics: Vec::new(),
        }
    }
}

/// Implementation methods for UnifiedInstallerConfig
pub mod implementation {
    use super::*;
    use crate::config::UnifiedInstallerConfig;
    use nestgate_core::unified_config_consolidation::StandardDomainConfig;

    /// Trait providing configuration creation methods
    pub trait UnifiedInstallerConfigExt {
        /// Create a development-focused Installer configuration
        fn development() -> Self;
        /// Create a production-ready Installer configuration
        fn production() -> Self;
        /// Validate Installer-specific configuration
        fn validate_installer_config(&self) -> Result<(), String>;
        /// Set installation mode
        fn set_install_mode(&mut self, mode: InstallMode);
        /// Enable/disable component
        fn set_component_enabled(&mut self, component: &str, enabled: bool);
        /// Get enabled components
        fn get_enabled_components(&self) -> Vec<String>;
        /// Set installation directories
        fn set_install_directories(
            &mut self,
            install_dir: PathBuf,
            config_dir: PathBuf,
            data_dir: PathBuf,
        );
    }

    impl UnifiedInstallerConfigExt for UnifiedInstallerConfig {
        /// Create a development-focused Installer configuration
        fn development() -> Self {
            let mut config = StandardDomainConfig::with_service(
                crate::config::InstallerExtensions::default(),
                "nestgate-installer",
                env!("CARGO_PKG_VERSION"),
            );

            // Development-friendly settings
            config.extensions.installation.mode = InstallMode::Development;
            config.extensions.installation.interactive = true;
            config.extensions.installation.verbose = true;
            config
                .extensions
                .validation
                .pre_install_checks
                .check_network = false;
            config.network.bind_address = nestgate_core::constants::network::addresses::LOCALHOST
                .parse()
                .expect("Valid localhost IP address");
            config.service.environment = "development".to_string();

            config
        }

        /// Create a production-ready Installer configuration
        fn production() -> Self {
            crate::config::migration::create_production_installer_config()
        }

        /// Validate Installer-specific configuration
        fn validate_installer_config(&self) -> Result<(), String> {
            super::super::validation::config_validation::validate_installer_config(self)
        }

        /// Set installation mode
        fn set_install_mode(&mut self, mode: InstallMode) {
            self.extensions.installation.mode = mode;
        }

        /// Enable/disable component
        fn set_component_enabled(&mut self, component: &str, enabled: bool) {
            match component {
                "api" => self.extensions.components.selected_components.install_api = enabled,
                "zfs" => self.extensions.components.selected_components.install_zfs = enabled,
                "network" => {
                    self.extensions
                        .components
                        .selected_components
                        .install_network = enabled
                }
                "monitoring" => {
                    self.extensions
                        .components
                        .selected_components
                        .install_monitoring = enabled
                }
                "security" => {
                    self.extensions
                        .components
                        .selected_components
                        .install_security = enabled
                }
                "automation" => {
                    self.extensions
                        .components
                        .selected_components
                        .install_automation = enabled
                }
                "ui" => self.extensions.components.selected_components.install_ui = enabled,
                "nas" => self.extensions.components.selected_components.install_nas = enabled,
                "fsmonitor" => {
                    self.extensions
                        .components
                        .selected_components
                        .install_fsmonitor = enabled
                }
                "mcp" => self.extensions.components.selected_components.install_mcp = enabled,
                _ => {} // Unknown component
            }
        }

        /// Get enabled components
        fn get_enabled_components(&self) -> Vec<String> {
            let mut components = Vec::new();
            let selection = &self.extensions.components.selected_components;

            if selection.install_api {
                components.push("api".to_string());
            }
            if selection.install_zfs {
                components.push("zfs".to_string());
            }
            if selection.install_network {
                components.push("network".to_string());
            }
            if selection.install_monitoring {
                components.push("monitoring".to_string());
            }
            if selection.install_security {
                components.push("security".to_string());
            }
            if selection.install_automation {
                components.push("automation".to_string());
            }
            if selection.install_ui {
                components.push("ui".to_string());
            }
            if selection.install_nas {
                components.push("nas".to_string());
            }
            if selection.install_fsmonitor {
                components.push("fsmonitor".to_string());
            }
            if selection.install_mcp {
                components.push("mcp".to_string());
            }

            components.extend(selection.custom_components.clone());
            components
        }

        /// Set installation directories
        fn set_install_directories(
            &mut self,
            install_dir: PathBuf,
            config_dir: PathBuf,
            data_dir: PathBuf,
        ) {
            self.extensions.installation.install_dir = install_dir;
            self.extensions.installation.config_dir = config_dir;
            self.extensions.installation.data_dir = data_dir;
        }
    }
}
