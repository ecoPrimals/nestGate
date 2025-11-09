/// Platform-specific installation settings, component selection, and system integration
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
// Type alias to reduce complexity
#[allow(dead_code)] // Reserved for future component configuration
type ComponentConfigMap = HashMap<String, HashMap<String, String>>;

#[allow(dead_code)] // Reserved for future installation settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallationSettings {
    /// Installation mode
    pub mode: InstallMode,
    /// Installation directory
    pub install_dir: PathBuf,
    /// Configuration directory
    pub config_dir: PathBuf,
    /// Data directory
    pub data_dir: PathBuf,
    /// Log directory
    pub log_dir: PathBuf,
    /// Temporary directory for installation
    pub temp_dir: PathBuf,
    /// Enable force installation (overwrite existing)
    pub force_install: bool,
    /// Enable interactive mode
    pub interactive: bool,
    /// Enable verbose output
    pub verbose: bool,
}
#[allow(dead_code)] // Reserved for future component settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentSettings {
    /// Component selection
    pub selected_components: ComponentSelection,
    /// Component dependencies
    pub dependencies: HashMap<String, Vec<String>>,
    /// Optional components
    pub optional_components: Vec<String>,
    /// Component-specific configurations
    pub component_configs: ComponentConfigMap,
    /// Enable component validation
    pub validate_components: bool,
}
#[allow(dead_code)] // Reserved for future component selection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentSelection {
    /// Install API component
    pub install_api: bool,
    /// Install ZFS component
    pub install_zfs: bool,
    /// Install Network component
    pub install_network: bool,
    /// Install Monitoring component
    pub install_monitoring: bool,
    /// Install Security component
    pub install_security: bool,
    /// Install Automation component
    pub install_automation: bool,
    /// Install UI component
    pub install_ui: bool,
    /// Install NAS component
    pub install_nas: bool,
    /// Install `FSMonitor` component
    pub install_fsmonitor: bool,
    /// Install MCP component
    pub install_mcp: bool,
    /// Custom components
    pub custom_components: Vec<String>,
}
#[allow(dead_code)] // Reserved for future system integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemIntegrationSettings {
    /// Install as system service
    pub install_as_service: bool,
    /// Service name
    pub service_name: String,
    /// Service user
    pub service_user: String,
    /// Service group
    pub service_group: String,
    /// Enable autostart
    pub enable_autostart: bool,
    /// Create firewall rules
    pub create_firewall_rules: bool,
    /// System path integration
    pub add_to_path: bool,
    /// Desktop integration
    pub desktop_integration: bool,
}
#[allow(dead_code)] // Reserved for future package management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageManagementSettings {
    /// Package manager type
    pub package_manager: PackageManagerType,
    /// Package repositories
    pub repositories: Vec<String>,
    /// Package dependencies
    pub dependencies: Vec<String>,
    /// Optional packages
    pub optional_packages: Vec<String>,
    /// Enable automatic updates
    pub auto_updates: bool,
    /// Update channel
    pub update_channel: String,
}

/// Installation mode enumeration
#[allow(dead_code)] // Reserved for future installation modes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstallMode {
    /// Default installation
    Default,
    /// Minimal installation
    Minimal,
    /// Development installation
    Development,
    /// Production installation
    Production,
    /// Distributed installation across multiple nodes
    Distributed,
    /// Standalone installation (single-node)
    Standalone,
    /// Custom installation with user choices
    Custom,
    /// Container deployment
    Container,
    /// Cloud deployment
    Cloud,
}
/// Deployment mode enumeration
#[allow(dead_code)] // Reserved for future deployment modes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeploymentMode {
    /// Single node deployment
    SingleNode,
    /// Multi-node cluster
    Cluster,
    /// Docker container
    Docker,
    /// Kubernetes
    Kubernetes,
    /// Cloud native
    CloudNative,
    /// Hybrid deployment
    Hybrid,
}
/// Supported platform types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlatformType {
    /// Linux `x86_64`
    LinuxX64,
    /// Linux ARM64
    LinuxArm64,
    /// Windows `x86_64`
    WindowsX64,
    /// macOS `x86_64`
    MacOsX64,
    /// macOS ARM64 (Apple Silicon)
    MacOsArm64,
    /// FreeBSD
    FreeBsd,
}
/// Package manager types
#[allow(dead_code)] // Reserved for future package manager support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PackageManagerType {
    /// Debian/Ubuntu APT
    Apt,
    /// Red Hat/CentOS YUM
    Yum,
    /// Red Hat/Fedora DNF
    Dnf,
    /// Arch Linux Pacman
    Pacman,
    /// openSUSE Zypper
    Zypper,
    /// FreeBSD PKG
    Pkg,
    /// Homebrew (macOS/Linux)
    Homebrew,
    /// Chocolatey (Windows)
    Chocolatey,
    /// Manual installation
    Manual,
}
/// System resource requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemRequirements {
    /// Platform compatibility
    pub supported_platforms: Vec<PlatformType>,
    /// Minimum RAM in MB
    pub min_ram_mb: u64,
    /// Recommended RAM in MB
    pub recommended_ram_mb: u64,
    /// Minimum disk space in MB
    pub min_disk_space_mb: u64,
    /// Recommended disk space in MB
    pub recommended_disk_space_mb: u64,
    /// Minimum CPU cores
    pub min_cpu_cores: u32,
    /// Required kernel version (Linux)
    pub min_kernel_version: Option<String>,
    /// Required dependencies
    pub required_packages: Vec<String>,
    /// Optional dependencies
    pub optional_packages: Vec<String>,
}
#[allow(dead_code)] // Reserved for future resource constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceConstraints {
    /// CPU limit
    pub cpu_limit: Option<String>,
    /// Memory limit
    pub memory_limit: Option<String>,
    /// Disk limit
    pub disk_limit: Option<String>,
    /// Network limit
    pub network_limit: Option<String>,
}
impl Default for InstallationSettings {
    fn default() -> Self {
        Self {
            mode: InstallMode::Standalone,
            install_dir: PathBuf::from(
                std::env::var("NESTGATE_INSTALL_DIR")
                    .unwrap_or_else(|_| "/opt/nestgate".to_string()),
            ),
            config_dir: PathBuf::from("/etc/nestgate"),
            data_dir: PathBuf::from("/var/lib/nestgate"),
            log_dir: PathBuf::from(
                std::env::var("NESTGATE_LOG_DIR")
                    .unwrap_or_else(|_| "/var/log/nestgate".to_string()),
            ),
            temp_dir: std::env::temp_dir().join("nestgate-install"),
            force_install: false,
            interactive: true,
            verbose: false,
        }
    }
}

impl Default for ComponentSettings {
    fn default() -> Self {
        Self {
            selected_components: ComponentSelection::default(),
            dependencies: HashMap::new(),
            optional_components: Vec::new(),
            component_configs: HashMap::new(),
            validate_components: true,
        }
    }
}

impl Default for ComponentSelection {
    fn default() -> Self {
        Self {
            install_api: true,
            install_zfs: true,
            install_network: true,
            install_monitoring: true,
            install_security: false,
            install_automation: false,
            install_ui: false,
            install_nas: false,
            install_fsmonitor: false,
            install_mcp: false,
            custom_components: Vec::new(),
        }
    }
}

impl Default for SystemIntegrationSettings {
    fn default() -> Self {
        Self {
            install_as_service: true,
            service_name: "nestgate".to_string(),
            service_user: "nestgate".to_string(),
            service_group: "nestgate".to_string(),
            enable_autostart: true,
            create_firewall_rules: false,
            add_to_path: true,
            desktop_integration: false,
        }
    }
}

impl Default for PackageManagementSettings {
    fn default() -> Self {
        Self {
            package_manager: PackageManagerType::Manual,
            repositories: Vec::new(),
            dependencies: Vec::new(),
            optional_packages: Vec::new(),
            auto_updates: false,
            update_channel: "stable".to_string(),
        }
    }
}

impl Default for SystemRequirements {
    fn default() -> Self {
        Self {
            supported_platforms: vec![
                PlatformType::LinuxX64,
                PlatformType::LinuxArm64,
                PlatformType::MacOsX64,
                PlatformType::MacOsArm64,
            ],
            min_ram_mb: 2048,
            recommended_ram_mb: 4096,
            min_disk_space_mb: 5120,
            recommended_disk_space_mb: 10240,
            min_cpu_cores: 2,
            min_kernel_version: None,
            required_packages: Vec::new(),
            optional_packages: Vec::new(),
        }
    }
}

/// Post-installation settings
#[allow(dead_code)] // Reserved for future post-install settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostInstallSettings {
    /// Run initial setup
    pub run_initial_setup: bool,
    /// Create sample config
    pub create_sample_config: bool,
    /// Start services automatically
    pub start_services: bool,
}
impl Default for PostInstallSettings {
    fn default() -> Self {
        Self {
            run_initial_setup: true,
            create_sample_config: true,
            start_services: true,
        }
    }
}

/// Deployment settings
#[allow(dead_code)] // Reserved for future deployment settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentSettings {
    /// Deployment mode
    pub deployment_mode: DeploymentMode,
    /// Backup existing installation
    pub backup_existing: bool,
    /// Rollback on failure
    pub rollback_on_failure: bool,
}
impl Default for DeploymentSettings {
    fn default() -> Self {
        Self {
            deployment_mode: DeploymentMode::SingleNode,
            backup_existing: true,
            rollback_on_failure: true,
        }
    }
}
