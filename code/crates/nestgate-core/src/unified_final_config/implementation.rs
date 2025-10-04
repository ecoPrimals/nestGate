use crate::error::NestGateError;
use crate::error::CanonicalResult as Result;

impl NestGateFinalConfig {
    /// Create a new configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Load configuration from environment variables and files
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        #[must_use]
        pub fn load() -> Result<Self>  {
        use std::env;
        use tokio::fs;

        let mut config = Self::default();

        // Load from environment variables
        if let Ok(bind_host) = env::var("NESTGATE_BIND_HOST") {
            let addr: std::net::IpAddr = bind_host
                .parse()
                .unwrap_or(std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST));
            config.base.network.bind_endpoint = addr.to_string();
        }

        if let Ok(http_port) = env::var("NESTGATE_HTTP_PORT") {
            if let Ok(port) = http_port.parse() {
                config.base.network.api_port = port;
            }
        }

        if let Ok(main_port) = env::var("NESTGATE_PORT") {
            if let Ok(port) = main_port.parse() {
                config.base.network.port = port;
            }
        }

        // Load from configuration file if it exists
        if let Ok(config_path) = env::var("NESTGATE_CONFIG_PATH") {
            if let Ok(config_content) = fs::read_to_string(&config_path).await {
                if let Ok(file_config) = toml::from_str::<Self>(&config_content) {
                    // Merge file config with environment config (env takes precedence)
                    config = Self::merge_configs(file_config, config);
                }
            }
        }

        config.validate_impl()?;
        Ok(config)
    }

    /// Create development configuration
    #[must_use]
    pub fn development() -> Self {
        let mut config = Self::default();
        config.environment.deployment_environment =
            super::supporting_types::DeploymentEnvironment::Development;
        config
    }

    /// Create production configuration
    #[must_use]
    pub fn production() -> Self {
        let mut config = Self::default();
        config.environment.deployment_environment =
            super::supporting_types::DeploymentEnvironment::Production;
        config
    }

    /// Validate the configuration consistency (implementation specific)
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn validate_impl(&self) -> Result<()>  {
        // Comprehensive validation logic would go here
        // For now, basic validation
        if self
            .system
            .name
            .as_ref()
            .is_none_or(|name| name.is_empty())
        {
            return Err(NestGateError::configuration_error(Some("field".to_string()), "Instance name cannot be empty"));
        }

        Ok(())
    }

    /// Save configuration to file

        let toml_content = toml::to_string_pretty(self).map_err(|e| NestGateError::internal_error(
            location: Some(format!("{})
            context: None)?;

        fs::write(path, toml_content)
            .await
            .map_err(|e| NestGateError::internal_error(
                location: Some(format!("{})
                is_bug: false,
            )?;

        Ok(())
    }

    /// Merge two configurations, with override taking precedence
    fn merge_configs(_base: Self, override_config: Self) -> Self {
        Self {
            system: override_config.system,
            features: override_config.features,
            domains: override_config.domains,
            metadata: override_config.metadata,
            base: override_config.base,
            extensions: override_config.extensions,
            service: override_config.service,
        }
    }
}

// CANONICAL MODERNIZATION: Default implementation moved to core.rs
// This duplicate implementation is removed to prevent conflicts
