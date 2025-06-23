use std::env;
use nestgate_orchestrator::OrchestratorConfig;
use nestgate_core::config::{NetworkConfig, EnvironmentConfig};
use nestgate_orchestrator::security::{SecurityConfig, SecurityManager};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    
    println!("=== NestGate Secure Configuration Example ===\n");
    
    // Example 1: Production Configuration with Environment Variables
    println!("1. Production Configuration (Environment Variables)");
    demonstrate_production_config().await?;
    
    // Example 2: Development Configuration (Secure Defaults)
    println!("\n2. Development Configuration (Secure Defaults)");
    demonstrate_development_config().await?;
    
    // Example 3: Security Configuration Validation
    println!("\n3. Security Configuration Validation");
    demonstrate_security_validation().await?;
    
    // Example 4: API Key Management
    println!("\n4. API Key Management");
    demonstrate_api_key_management().await?;
    
    // Example 5: Environment Variable Reference
    println!("\n5. Environment Variable Reference");
    print_environment_variables();
    
    Ok(())
}

async fn demonstrate_production_config() -> Result<(), Box<dyn std::error::Error>> {
    println!("Setting up production environment variables...");
    
    // Set production environment variables
    env::set_var("NESTGATE_ENV", "production");
    env::set_var("NESTGATE_SSL_ENABLED", "true");
    env::set_var("NESTGATE_RATE_LIMITING_ENABLED", "true");
    env::set_var("NESTGATE_ENCRYPTION_ENABLED", "true");
    
    // Generate secure API keys
    let admin_key = SecurityConfig::generate_api_key();
    let readonly_key = SecurityConfig::generate_api_key();
    let operator_key = SecurityConfig::generate_api_key();
    
    env::set_var("NESTGATE_ADMIN_API_KEY", &admin_key);
    env::set_var("NESTGATE_READONLY_API_KEY", &readonly_key);
    env::set_var("NESTGATE_OPERATOR_API_KEY", &operator_key);
    
    // TLS Configuration
    env::set_var("NESTGATE_TLS_CERT_FILE", "/etc/nestgate/tls/cert.pem");
    env::set_var("NESTGATE_TLS_KEY_FILE", "/etc/nestgate/tls/key.pem");
    
    // JWT Configuration
    let jwt_secret = SecurityConfig::generate_api_key();
    env::set_var("NESTGATE_JWT_SECRET", &jwt_secret);
    env::set_var("NESTGATE_JWT_ISSUER", "nestgate-production");
    env::set_var("NESTGATE_JWT_AUDIENCE", "nestgate-services");
    
    // Rate Limiting
    env::set_var("NESTGATE_RATE_LIMIT_RPM", "60");
    
    // IP Whitelist (example)
    env::set_var("NESTGATE_IP_WHITELIST", "127.0.0.1,10.0.0.0/8,192.168.0.0/16");
    
    // Create security configuration
    let security_config = SecurityConfig::from_env()?;
    println!("✅ Security configuration loaded from environment");
    println!("   - SSL/TLS: {}", security_config.ssl.enabled);
    println!("   - Rate Limiting: {}", security_config.rate_limiting.enabled);
    println!("   - Encryption: {}", security_config.encryption.enabled);
    println!("   - API Keys: {}", security_config.api_keys.len());
    
    // Validate configuration
    security_config.validate()?;
    println!("✅ Security configuration validation passed");
    
    // Create orchestrator with production configuration
    let orchestrator_config = OrchestratorConfig::production(false); // Secure by default
    println!("✅ Orchestrator configured for production");
    println!("   - Bind Address: {}", orchestrator_config.bind_address());
    println!("   - Secure: {}", orchestrator_config.is_secure());
    
    // Create security manager
    let security_manager = SecurityManager::new(security_config)?;
    security_manager.initialize().await?;
    println!("✅ Security manager initialized");
    
    // Display generated keys (in real production, these would be stored securely)
    println!("\n🔑 Generated API Keys (store these securely):");
    println!("   Admin Key:    {}...", &admin_key[..16]);
    println!("   Readonly Key: {}...", &readonly_key[..16]);
    println!("   Operator Key: {}...", &operator_key[..16]);
    println!("   JWT Secret:   {}...", &jwt_secret[..16]);
    
    Ok(())
}

async fn demonstrate_development_config() -> Result<(), Box<dyn std::error::Error>> {
    // Clear production environment variables
    env::remove_var("NESTGATE_ENV");
    env::remove_var("NESTGATE_ADMIN_API_KEY");
    env::remove_var("NESTGATE_READONLY_API_KEY");
    env::remove_var("NESTGATE_OPERATOR_API_KEY");
    
    // Set development environment
    env::set_var("NESTGATE_ENV", "development");
    
    println!("Setting up development configuration...");
    
    // Create orchestrator with development configuration
    let orchestrator_config = OrchestratorConfig::development();
    println!("✅ Orchestrator configured for development");
    println!("   - Bind Address: {}", orchestrator_config.bind_address());
    println!("   - Secure: {}", orchestrator_config.is_secure());
    
    // Security configuration will use development defaults
    let security_config = SecurityConfig::default();
    println!("✅ Using development security defaults");
    println!("   - SSL/TLS: {} (disabled for development)", security_config.ssl.enabled);
    println!("   - Rate Limiting: {} (disabled for development)", security_config.rate_limiting.enabled);
    println!("   - Encryption: {} (disabled for development)", security_config.encryption.enabled);
    
    // Note: In development, authentication can be bypassed but this is logged
    println!("⚠️  Development mode: Reduced security (not for production)");
    
    Ok(())
}

async fn demonstrate_security_validation() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing security configuration validation...");
    
    // Test 1: Production without required configuration
    env::set_var("NESTGATE_ENV", "production");
    env::remove_var("NESTGATE_ADMIN_API_KEY");
    
    match SecurityConfig::from_env() {
        Ok(_) => println!("❌ Should have failed validation"),
        Err(e) => println!("✅ Correctly rejected invalid production config: {}", e),
    }
    
    // Test 2: Weak API key
    let weak_key = "short";
    env::set_var("NESTGATE_ADMIN_API_KEY", weak_key);
    
    match SecurityConfig::from_env() {
        Ok(config) => {
            match config.validate() {
                Ok(_) => println!("❌ Should have rejected weak API key"),
                Err(e) => println!("✅ Correctly rejected weak API key: {}", e),
            }
        },
        Err(e) => println!("✅ Configuration loading failed with weak key: {}", e),
    }
    
    // Test 3: Valid configuration
    let strong_key = SecurityConfig::generate_api_key();
    env::set_var("NESTGATE_ADMIN_API_KEY", &strong_key);
    
    match SecurityConfig::from_env() {
        Ok(config) => {
            match config.validate() {
                Ok(_) => println!("✅ Valid configuration accepted"),
                Err(e) => println!("❌ Valid configuration rejected: {}", e),
            }
        },
        Err(e) => println!("❌ Failed to load valid configuration: {}", e),
    }
    
    Ok(())
}

async fn demonstrate_api_key_management() -> Result<(), Box<dyn std::error::Error>> {
    println!("Demonstrating API key management...");
    
    // Set up environment for testing
    env::set_var("NESTGATE_ENV", "development");
    let test_key = SecurityConfig::generate_api_key();
    env::set_var("NESTGATE_ADMIN_API_KEY", &test_key);
    
    // Create security manager
    let security_config = SecurityConfig::from_env()?;
    let security_manager = SecurityManager::new(security_config)?;
    security_manager.initialize().await?;
    
    // Test API key validation
    let valid_key = &test_key;
    let invalid_key = "invalid-key";
    
    // Test authentication with valid key
    match security_manager.authenticate(valid_key).await {
        Ok(context) => {
            println!("✅ Valid API key authenticated");
            println!("   - User: {}", context.username);
            println!("   - Role: {:?}", context.role);
            println!("   - Permissions: {} permissions", context.permissions.len());
        },
        Err(e) => println!("❌ Valid key rejected: {}", e),
    }
    
    // Test authentication with invalid key
    match security_manager.authenticate(invalid_key).await {
        Ok(_) => println!("❌ Invalid key should have been rejected"),
        Err(_) => println!("✅ Invalid API key correctly rejected"),
    }
    
    // Test API key format validation
    println!("✅ API key format validation:");
    println!("   - Valid key format: {}", security_manager.validate_api_key(valid_key));
    println!("   - Invalid key format: {}", security_manager.validate_api_key(invalid_key));
    
    // List API keys (only shows prefixes for security)
    let key_list = security_manager.list_api_keys().await;
    println!("✅ API keys configured: {:?}", key_list);
    
    Ok(())
}

fn print_environment_variables() {
    println!("Environment Variables Reference:");
    println!();
    
    println!("🔐 Security Configuration:");
    println!("   NESTGATE_ENV                     - Environment (development/staging/production)");
    println!("   NESTGATE_SSL_ENABLED             - Enable SSL/TLS (true/false)");
    println!("   NESTGATE_RATE_LIMITING_ENABLED   - Enable rate limiting (true/false)");
    println!("   NESTGATE_ENCRYPTION_ENABLED      - Enable encryption (true/false)");
    println!();
    
    println!("🔑 API Keys:");
    println!("   NESTGATE_ADMIN_API_KEY           - Admin API key (minimum 32 characters)");
    println!("   NESTGATE_READONLY_API_KEY        - Read-only API key");
    println!("   NESTGATE_OPERATOR_API_KEY        - Operator API key");
    println!();
    
    println!("🛡️ TLS Configuration:");
    println!("   NESTGATE_TLS_CERT_FILE           - Path to TLS certificate file");
    println!("   NESTGATE_TLS_KEY_FILE            - Path to TLS private key file");
    println!();
    
    println!("🎫 JWT Configuration:");
    println!("   NESTGATE_JWT_SECRET              - JWT signing secret");
    println!("   NESTGATE_JWT_EXPIRATION_HOURS    - JWT token expiration (hours)");
    println!("   NESTGATE_JWT_ISSUER              - JWT issuer");
    println!("   NESTGATE_JWT_AUDIENCE            - JWT audience");
    println!();
    
    println!("🚦 Rate Limiting:");
    println!("   NESTGATE_RATE_LIMIT_RPM          - Requests per minute limit");
    println!();
    
    println!("🌐 Network Security:");
    println!("   NESTGATE_IP_WHITELIST            - Comma-separated IP whitelist");
    println!();
    
    println!("🔄 Encryption:");
    println!("   NESTGATE_ENCRYPTION_ALGORITHM    - Encryption algorithm (AES-256-GCM)");
    println!("   NESTGATE_KEY_ROTATION_HOURS      - Key rotation interval (hours)");
    println!();
    
    println!("📋 Example Production Setup:");
    println!("   export NESTGATE_ENV=production");
    println!("   export NESTGATE_SSL_ENABLED=true");
    println!("   export NESTGATE_RATE_LIMITING_ENABLED=true");
    println!("   export NESTGATE_ENCRYPTION_ENABLED=true");
    println!("   export NESTGATE_ADMIN_API_KEY=$(openssl rand -hex 32)");
    println!("   export NESTGATE_READONLY_API_KEY=$(openssl rand -hex 32)");
    println!("   export NESTGATE_JWT_SECRET=$(openssl rand -hex 32)");
    println!("   export NESTGATE_TLS_CERT_FILE=/etc/nestgate/tls/cert.pem");
    println!("   export NESTGATE_TLS_KEY_FILE=/etc/nestgate/tls/key.pem");
    println!("   export NESTGATE_RATE_LIMIT_RPM=60");
    println!("   export NESTGATE_IP_WHITELIST=10.0.0.0/8,192.168.0.0/16");
} 