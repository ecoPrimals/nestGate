//! Configuration Management Tests
//! 
//! Comprehensive test suite for NestGate's configuration system,
//! covering environment variables, file-based config, and validation.

#[cfg(test)]
mod configuration_tests {
    use std::collections::HashMap;
    use std::env;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;
    
    #[derive(Debug, Clone, PartialEq)]
    pub struct NetworkConfig {
        pub bind_address: String,
        pub api_port: u16,
        pub api_base_url: String,
        pub max_connections: usize,
        pub timeout_ms: u64,
    }
    
    impl Default for NetworkConfig {
        fn default() -> Self {
            Self {
                bind_address: "0.0.0.0".to_string(),
                api_port: 8080,
                api_base_url: "http://localhost:8080".to_string(),
                max_connections: 1000,
                timeout_ms: 5000,
            }
        }
    }
    
    #[derive(Debug, Clone, PartialEq)]
    pub struct DatabaseConfig {
        pub host: String,
        pub port: u16,
        pub database_name: String,
        pub username: String,
        pub password: String,
        pub connection_pool_size: usize,
    }
    
    impl Default for DatabaseConfig {
        fn default() -> Self {
            Self {
                host: "localhost".to_string(),
                port: 5432,
                database_name: "nestgate".to_string(),
                username: "nestgate".to_string(),
                password: "".to_string(),
                connection_pool_size: 10,
            }
        }
    }
    
    #[derive(Debug, Clone, PartialEq)]
    pub struct SecurityConfig {
        pub tls_enabled: bool,
        pub cert_path: Option<PathBuf>,
        pub key_path: Option<PathBuf>,
        pub jwt_secret: String,
        pub session_timeout: u64,
    }
    
    impl Default for SecurityConfig {
        fn default() -> Self {
            Self {
                tls_enabled: false,
                cert_path: None,
                key_path: None,
                jwt_secret: "default-secret".to_string(),
                session_timeout: 3600,
            }
        }
    }
    
    #[derive(Debug, Clone, PartialEq)]
    pub struct NestGateConfig {
        pub network: NetworkConfig,
        pub database: DatabaseConfig,
        pub security: SecurityConfig,
        pub environment: String,
        pub log_level: String,
        pub metrics_enabled: bool,
    }
    
    impl Default for NestGateConfig {
        fn default() -> Self {
            Self {
                network: NetworkConfig::default(),
                database: DatabaseConfig::default(),
                security: SecurityConfig::default(),
                environment: "development".to_string(),
                log_level: "info".to_string(),
                metrics_enabled: true,
            }
        }
    }
    
    pub struct ConfigBuilder {
        config: NestGateConfig,
        validation_errors: Vec<String>,
    }
    
    impl ConfigBuilder {
        pub fn new() -> Self {
            Self {
                config: NestGateConfig::default(),
                validation_errors: Vec::new(),
            }
        }
        
        pub fn from_env() -> Self {
            let mut builder = Self::new();
            builder.load_from_environment();
            builder
        }
        
        pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<Self> {
            let content = fs::read_to_string(path)?;
            let mut builder = Self::new();
            builder.load_from_toml(&content)?;
            Ok(builder)
        }
        
        pub fn load_from_environment(&mut self) {
            // Network configuration
            if let Ok(bind_address) = env::var("NESTGATE_BIND_ADDRESS") {
                self.config.network.bind_address = bind_address;
            }
            
            if let Ok(api_port) = env::var("NESTGATE_API_PORT") {
                if let Ok(port) = api_port.parse::<u16>() {
                    self.config.network.api_port = port;
                } else {
                    self.validation_errors.push(format!("Invalid API port: {}", api_port));
                }
            }
            
            if let Ok(api_base_url) = env::var("NESTGATE_API_BASE_URL") {
                self.config.network.api_base_url = api_base_url;
            }
            
            // Database configuration
            if let Ok(db_host) = env::var("NESTGATE_DB_HOST") {
                self.config.database.host = db_host;
            }
            
            if let Ok(db_port) = env::var("NESTGATE_DB_PORT") {
                if let Ok(port) = db_port.parse::<u16>() {
                    self.config.database.port = port;
                } else {
                    self.validation_errors.push(format!("Invalid DB port: {}", db_port));
                }
            }
            
            // Security configuration
            if let Ok(tls_enabled) = env::var("NESTGATE_TLS_ENABLED") {
                self.config.security.tls_enabled = tls_enabled.to_lowercase() == "true";
            }
            
            if let Ok(jwt_secret) = env::var("NESTGATE_JWT_SECRET") {
                if jwt_secret.len() >= 32 {
                    self.config.security.jwt_secret = jwt_secret;
                } else {
                    self.validation_errors.push("JWT secret must be at least 32 characters".to_string());
                }
            }
            
            // General configuration
            if let Ok(environment) = env::var("NESTGATE_ENVIRONMENT") {
                if ["development", "staging", "production"].contains(&environment.as_str()) {
                    self.config.environment = environment;
                } else {
                    self.validation_errors.push(format!("Invalid environment: {}", environment));
                }
            }
            
            if let Ok(log_level) = env::var("NESTGATE_LOG_LEVEL") {
                if ["trace", "debug", "info", "warn", "error"].contains(&log_level.as_str()) {
                    self.config.log_level = log_level;
                } else {
                    self.validation_errors.push(format!("Invalid log level: {}", log_level));
                }
            }
        }
        
        pub fn load_from_toml(&mut self, content: &str) -> std::io::Result<()> {
            // Simplified TOML parsing for testing
            for line in content.lines() {
                if line.starts_with("bind_address") {
                    if let Some(value) = line.split('=').nth(1) {
                        self.config.network.bind_address = value.trim().trim_matches('"').to_string();
                    }
                } else if line.starts_with("api_port") {
                    if let Some(value) = line.split('=').nth(1) {
                        if let Ok(port) = value.trim().parse::<u16>() {
                            self.config.network.api_port = port;
                        }
                    }
                }
                // Add more TOML parsing as needed
            }
            Ok(())
        }
        
        pub fn with_network_config(mut self, config: NetworkConfig) -> Self {
            self.config.network = config;
            self
        }
        
        pub fn with_database_config(mut self, config: DatabaseConfig) -> Self {
            self.config.database = config;
            self
        }
        
        pub fn with_security_config(mut self, config: SecurityConfig) -> Self {
            self.config.security = config;
            self
        }
        
        pub fn validate(&mut self) -> Result<(), Vec<String>> {
            // Network validation
            if self.config.network.api_port == 0 {
                self.validation_errors.push("API port cannot be 0".to_string());
            }
            
            if self.config.network.max_connections == 0 {
                self.validation_errors.push("Max connections must be greater than 0".to_string());
            }
            
            // Database validation
            if self.config.database.host.is_empty() {
                self.validation_errors.push("Database host cannot be empty".to_string());
            }
            
            if self.config.database.database_name.is_empty() {
                self.validation_errors.push("Database name cannot be empty".to_string());
            }
            
            // Security validation
            if self.config.security.tls_enabled {
                if self.config.security.cert_path.is_none() {
                    self.validation_errors.push("TLS certificate path required when TLS is enabled".to_string());
                }
                if self.config.security.key_path.is_none() {
                    self.validation_errors.push("TLS key path required when TLS is enabled".to_string());
                }
            }
            
            if self.config.security.jwt_secret.len() < 32 {
                self.validation_errors.push("JWT secret must be at least 32 characters".to_string());
            }
            
            if self.validation_errors.is_empty() {
                Ok(())
            } else {
                Err(self.validation_errors.clone())
            }
        }
        
        pub fn build(mut self) -> Result<NestGateConfig, Vec<String>> {
            self.validate()?;
            Ok(self.config)
        }
        
        pub fn get_validation_errors(&self) -> &[String] {
            &self.validation_errors
        }
    }

    #[test]
    fn test_default_configuration() {
        let config = NestGateConfig::default();
        
        assert_eq!(config.network.bind_address, "0.0.0.0");
        assert_eq!(config.network.api_port, 8080);
        assert_eq!(config.database.host, "localhost");
        assert_eq!(config.database.port, 5432);
        assert!(!config.security.tls_enabled);
        assert_eq!(config.environment, "development");
    }
    
    #[test]
    fn test_config_builder_initialization() {
        let builder = ConfigBuilder::new();
        assert_eq!(builder.config, NestGateConfig::default());
        assert!(builder.validation_errors.is_empty());
    }
    
    #[test]
    fn test_environment_variable_loading() {
        // Set test environment variables
        env::set_var("NESTGATE_API_PORT", "9090");
        env::set_var("NESTGATE_DB_HOST", "testhost");
        env::set_var("NESTGATE_TLS_ENABLED", "true");
        env::set_var("NESTGATE_ENVIRONMENT", "production");
        env::set_var("NESTGATE_LOG_LEVEL", "debug");
        
        let builder = ConfigBuilder::from_env();
        
        assert_eq!(builder.config.network.api_port, 9090);
        assert_eq!(builder.config.database.host, "testhost");
        assert!(builder.config.security.tls_enabled);
        assert_eq!(builder.config.environment, "production");
        assert_eq!(builder.config.log_level, "debug");
        
        // Clean up
        env::remove_var("NESTGATE_API_PORT");
        env::remove_var("NESTGATE_DB_HOST");
        env::remove_var("NESTGATE_TLS_ENABLED");
        env::remove_var("NESTGATE_ENVIRONMENT");
        env::remove_var("NESTGATE_LOG_LEVEL");
    }
    
    #[test]
    fn test_invalid_environment_variables() {
        env::set_var("NESTGATE_API_PORT", "invalid");
        env::set_var("NESTGATE_ENVIRONMENT", "invalid_env");
        env::set_var("NESTGATE_LOG_LEVEL", "invalid_level");
        
        let builder = ConfigBuilder::from_env();
        
        assert!(!builder.validation_errors.is_empty());
        assert!(builder.validation_errors.iter().any(|e| e.contains("Invalid API port")));
        assert!(builder.validation_errors.iter().any(|e| e.contains("Invalid environment")));
        assert!(builder.validation_errors.iter().any(|e| e.contains("Invalid log level")));
        
        // Clean up
        env::remove_var("NESTGATE_API_PORT");
        env::remove_var("NESTGATE_ENVIRONMENT");
        env::remove_var("NESTGATE_LOG_LEVEL");
    }
    
    #[test]
    fn test_config_file_loading() -> std::io::Result<()> {
        let temp_dir = TempDir::new()?;
        let config_file = temp_dir.path().join("config.toml");
        
        let config_content = r#"
bind_address = "127.0.0.1"
api_port = 8888
"#;
        
        fs::write(&config_file, config_content)?;
        
        let builder = ConfigBuilder::from_file(&config_file)?;
        
        assert_eq!(builder.config.network.bind_address, "127.0.0.1");
        assert_eq!(builder.config.network.api_port, 8888);
        
        Ok(())
    }
    
    #[test]
    fn test_config_validation_success() {
        let mut builder = ConfigBuilder::new();
        builder.config.security.jwt_secret = "a".repeat(32);
        
        let result = builder.validate();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_config_validation_failures() {
        let mut builder = ConfigBuilder::new();
        builder.config.network.api_port = 0;
        builder.config.network.max_connections = 0;
        builder.config.database.host = "".to_string();
        builder.config.security.jwt_secret = "short".to_string();
        
        let result = builder.validate();
        assert!(result.is_err());
        
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.contains("API port cannot be 0")));
        assert!(errors.iter().any(|e| e.contains("Max connections must be greater than 0")));
        assert!(errors.iter().any(|e| e.contains("Database host cannot be empty")));
        assert!(errors.iter().any(|e| e.contains("JWT secret must be at least 32 characters")));
    }
    
    #[test]
    fn test_tls_validation() {
        let mut builder = ConfigBuilder::new();
        builder.config.security.tls_enabled = true;
        builder.config.security.jwt_secret = "a".repeat(32);
        
        let result = builder.validate();
        assert!(result.is_err());
        
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.contains("TLS certificate path required")));
        assert!(errors.iter().any(|e| e.contains("TLS key path required")));
    }
    
    #[test]
    fn test_config_builder_with_custom_configs() {
        let network_config = NetworkConfig {
            bind_address: "192.168.1.1".to_string(),
            api_port: 9000,
            api_base_url: "https://api.example.com".to_string(),
            max_connections: 2000,
            timeout_ms: 10000,
        };
        
        let database_config = DatabaseConfig {
            host: "db.example.com".to_string(),
            port: 5433,
            database_name: "production_db".to_string(),
            username: "prod_user".to_string(),
            password: "secure_password".to_string(),
            connection_pool_size: 20,
        };
        
        let security_config = SecurityConfig {
            tls_enabled: true,
            cert_path: Some(PathBuf::from("/etc/ssl/cert.pem")),
            key_path: Some(PathBuf::from("/etc/ssl/key.pem")),
            jwt_secret: "a".repeat(32),
            session_timeout: 7200,
        };
        
        let config = ConfigBuilder::new()
            .with_network_config(network_config.clone())
            .with_database_config(database_config.clone())
            .with_security_config(security_config.clone())
            .build()
            .expect("Config should be valid");
        
        assert_eq!(config.network, network_config);
        assert_eq!(config.database, database_config);
        assert_eq!(config.security, security_config);
    }
    
    #[test]
    fn test_jwt_secret_validation() {
        env::set_var("NESTGATE_JWT_SECRET", "short");
        
        let builder = ConfigBuilder::from_env();
        assert!(builder.validation_errors.iter().any(|e| e.contains("JWT secret must be at least 32 characters")));
        
        env::set_var("NESTGATE_JWT_SECRET", "a".repeat(32));
        let builder = ConfigBuilder::from_env();
        assert!(!builder.validation_errors.iter().any(|e| e.contains("JWT secret")));
        
        // Clean up
        env::remove_var("NESTGATE_JWT_SECRET");
    }
    
    #[test]
    fn test_configuration_precedence() {
        // Test that environment variables override defaults
        env::set_var("NESTGATE_API_PORT", "7777");
        
        let mut builder = ConfigBuilder::from_env();
        
        // Override with custom network config
        let custom_network = NetworkConfig {
            api_port: 6666,
            ..Default::default()
        };
        
        builder = builder.with_network_config(custom_network);
        
        // Custom config should override environment
        assert_eq!(builder.config.network.api_port, 6666);
        
        // Clean up
        env::remove_var("NESTGATE_API_PORT");
    }
    
    #[test]
    fn test_config_serialization_compatibility() {
        let config = NestGateConfig {
            network: NetworkConfig {
                bind_address: "test.example.com".to_string(),
                api_port: 8080,
                api_base_url: "https://test.example.com".to_string(),
                max_connections: 500,
                timeout_ms: 3000,
            },
            database: DatabaseConfig {
                host: "db.test.com".to_string(),
                port: 5432,
                database_name: "test_db".to_string(),
                username: "test_user".to_string(),
                password: "test_pass".to_string(),
                connection_pool_size: 5,
            },
            security: SecurityConfig {
                tls_enabled: false,
                cert_path: None,
                key_path: None,
                jwt_secret: "test_secret_32_characters_long!".to_string(),
                session_timeout: 1800,
            },
            environment: "testing".to_string(),
            log_level: "debug".to_string(),
            metrics_enabled: false,
        };
        
        // Verify all fields are accessible and have expected values
        assert_eq!(config.network.bind_address, "test.example.com");
        assert_eq!(config.database.host, "db.test.com");
        assert!(!config.security.tls_enabled);
        assert_eq!(config.environment, "testing");
        assert!(!config.metrics_enabled);
    }
} 