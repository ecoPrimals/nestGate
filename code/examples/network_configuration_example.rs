use nestgate_orchestrator::{Orchestrator, OrchestratorConfig};
use nestgate_core::config::{NetworkConfig, EnvironmentConfig, RuntimeEnvironment, default_ports};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    
    // Example 1: Development configuration (secure localhost only)
    println!("=== Development Configuration ===");
    let dev_config = OrchestratorConfig::development();
    println!("Development bind address: {}", dev_config.bind_address());
    println!("Is secure (localhost only): {}", dev_config.is_secure());
    
    // Example 2: Production configuration with external access
    println!("\n=== Production Configuration (External Access) ===");
    let prod_config = OrchestratorConfig::production(true);
    println!("Production bind address: {}", prod_config.bind_address());
    println!("Is secure (localhost only): {}", prod_config.is_secure());
    
    // Example 3: Production configuration with localhost only
    println!("\n=== Production Configuration (Localhost Only) ===");
    let secure_prod_config = OrchestratorConfig::production(false);
    println!("Secure production bind address: {}", secure_prod_config.bind_address());
    println!("Is secure (localhost only): {}", secure_prod_config.is_secure());
    
    // Example 4: Custom network configuration
    println!("\n=== Custom Network Configuration ===");
    let mut custom_config = OrchestratorConfig::default();
    custom_config.network = NetworkConfig::custom_host("192.168.1.100", 9090);
    println!("Custom bind address: {}", custom_config.bind_address());
    println!("Is secure (localhost only): {}", custom_config.is_secure());
    
    // Example 5: Environment-based configuration from environment variables
    println!("\n=== Environment-based Configuration ===");
    let env_config = get_config_from_environment();
    println!("Environment bind address: {}", env_config.bind_address());
    println!("Is secure (localhost only): {}", env_config.is_secure());
    
    // Example 6: Testing configuration
    println!("\n=== Testing Configuration ===");
    let test_config = OrchestratorConfig::testing();
    println!("Testing bind address: {}", test_config.bind_address());
    println!("Is secure (localhost only): {}", test_config.is_secure());
    
    // Example 7: Service-specific network configurations
    println!("\n=== Service-specific Network Configurations ===");
    demonstrate_service_configs();
    
    // Example 8: Security validation
    println!("\n=== Security Validation ===");
    validate_security_configurations();
    
    // Example 9: Start orchestrator with development config
    println!("\n=== Starting Orchestrator (Development Mode) ===");
    let orchestrator = Orchestrator::new(dev_config).await?;
    orchestrator.start().await?;
    
    println!("Orchestrator started successfully!");
    println!("Access at: http://{}/health", orchestrator.config.bind_address());
    
    // Give it a moment to start
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    
    // Test the health endpoint
    let client = reqwest::Client::new();
    match client.get(&format!("http://{}/health", orchestrator.config.bind_address())).send().await {
        Ok(response) => {
            println!("Health check response: {}", response.status());
            if let Ok(body) = response.text().await {
                println!("Response body: {}", body);
            }
        }
        Err(e) => println!("Health check failed: {}", e),
    }
    
    orchestrator.shutdown().await?;
    println!("Orchestrator shut down successfully!");
    
    Ok(())
}

fn get_config_from_environment() -> OrchestratorConfig {
    let environment = env::var("NESTGATE_ENV").unwrap_or_else(|_| "development".to_string());
    let allow_external = env::var("NESTGATE_ALLOW_EXTERNAL").unwrap_or_else(|_| "false".to_string()) == "true";
    let custom_host = env::var("NESTGATE_HOST").ok();
    let custom_port = env::var("NESTGATE_PORT").ok().and_then(|p| p.parse::<u16>().ok());
    
    let mut config = match environment.as_str() {
        "production" => OrchestratorConfig::production(allow_external),
        "staging" => {
            let mut config = OrchestratorConfig::default();
            config.environment.environment = RuntimeEnvironment::Staging;
            config.environment.allow_external_access = allow_external;
            config.network = config.environment.default_network_config(default_ports::ORCHESTRATOR);
            config
        }
        "testing" => OrchestratorConfig::testing(),
        _ => OrchestratorConfig::development(),
    };
    
    // Override with custom host/port if provided
    if let Some(host) = custom_host {
        let port = custom_port.unwrap_or(default_ports::ORCHESTRATOR);
        config.network = NetworkConfig::custom_host(&host, port);
    }
    
    config
}

fn demonstrate_service_configs() {
    println!("API Service config: {}", NetworkConfig::localhost(default_ports::API).bind_address());
    println!("MCP Service config: {}", NetworkConfig::localhost(default_ports::MCP).bind_address());
    println!("WebSocket config: {}", NetworkConfig::localhost(default_ports::WEBSOCKET).bind_address());
    println!("Metrics config: {}", NetworkConfig::localhost(default_ports::METRICS).bind_address());
    println!("Health config: {}", NetworkConfig::localhost(default_ports::HEALTH).bind_address());
    println!("ZFS API config: {}", NetworkConfig::localhost(default_ports::ZFS_API).bind_address());
    
    // External access versions
    println!("\nExternal access versions:");
    println!("API Service (external): {}", NetworkConfig::all_interfaces(default_ports::API).bind_address());
    println!("WebSocket (external): {}", NetworkConfig::all_interfaces(default_ports::WEBSOCKET).bind_address());
}

fn validate_security_configurations() {
    let configs = vec![
        ("Development", OrchestratorConfig::development()),
        ("Production (secure)", OrchestratorConfig::production(false)),
        ("Production (external)", OrchestratorConfig::production(true)),
        ("Testing", OrchestratorConfig::testing()),
    ];
    
    for (name, config) in configs {
        println!("{}: secure={}, external_access={}", 
                 name, 
                 config.is_secure(), 
                 config.network.is_externally_accessible());
        
        // Security warning for external access
        if config.network.is_externally_accessible() {
            println!("  ⚠️  WARNING: {} allows external network access!", name);
        } else {
            println!("  ✅ {} is secure (localhost only)", name);
        }
    }
} 