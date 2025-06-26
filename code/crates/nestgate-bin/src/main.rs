/*!
 * NestGate Main Binary
 * 
 * NestGate NAS system - runs as standalone service with optional Songbird enhancement
 * 🔧 STANDALONE: Full local functionality with direct network access
 * 🎼 SONGBIRD-ENHANCED: Extended functionality with orchestrated networking
 */

use tracing::{info, error, warn};
use tracing_subscriber;
use anyhow::Result;
use std::sync::Arc;
use uuid;
use chrono;

// NestGate services
use nestgate::songbird_integration::{NestGateZfsService, NestGateZfsConfig};
use nestgate_core::config::Config as NestGateConfig;
use nestgate_zfs::manager::ZfsManager;
use nestgate_network::{
    NetworkApi, SongbirdConnectionManager,
    songbird::{SongbirdIntegration, SongbirdConfig},
    ServiceInstance, ServiceStatus,
};
use nestgate_api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("info,nestgate=debug")
        .init();

    info!("🏠 NestGate v{} - Distributed NAS System", env!("CARGO_PKG_VERSION"));

    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 && (args[1] == "-h" || args[1] == "--help") {
        print_help();
        return Ok(());
    }

    // Check for Songbird URL (OPTIONAL)
    let songbird_url = std::env::var("SONGBIRD_URL")
        .or_else(|_| {
            // Check command line arguments
            for i in 0..args.len() {
                if args[i] == "--songbird-url" && i + 1 < args.len() {
                    return Ok(args[i + 1].clone());
                }
            }
            Err(std::env::VarError::NotPresent)
        })
        .ok();

    // Determine operation mode
    let operation_mode = if let Some(ref url) = songbird_url {
        info!("🎼 SONGBIRD-ENHANCED MODE: Connecting to orchestrator at {}", url);
        info!("   ✅ Enhanced networking, service discovery, and automation");
        OperationMode::SongbirdEnhanced(url.clone())
    } else {
        info!("🔧 STANDALONE MODE: Running without Songbird orchestrator");
        info!("   ✅ Full local functionality with direct network access");
        info!("   💡 Add SONGBIRD_URL to enable enhanced orchestration");
        OperationMode::Standalone
    };

    // Get service name
    let service_name = std::env::var("NESTGATE_SERVICE_NAME")
        .unwrap_or_else(|_| format!("nestgate-{}", uuid::Uuid::new_v4().to_string()[..8].to_string()));

    info!("🏷️ Service name: {}", service_name);

    // Initialize NestGate core
    let nestgate_config = NestGateConfig::default();
    
    // Initialize ZFS configuration
    let zfs_config = ZfsServiceConfig::default();
    
    info!("💾 Initializing ZFS manager...");
    let zfs_manager = Arc::new(
        ZfsManager::new(nestgate_zfs::config::ZfsConfig::default()).await?
    );

    // Initialize networking based on operation mode
    let network_setup = match operation_mode {
        OperationMode::Standalone => {
            initialize_standalone_networking(&service_name).await?
        }
        OperationMode::SongbirdEnhanced(url) => {
            initialize_songbird_networking(&service_name, &url).await?
        }
    };

    info!("🌟 NestGate services started:");
    info!("   - Service: {}", service_name);
    info!("   - API endpoint: {}", network_setup.api_bind_addr);
    info!("   - ZFS management: ✅ Enabled");
    info!("   - Operation mode: {}", network_setup.mode_description);

    // Run the API server
    info!("🚀 NestGate ready - {}", network_setup.mode_description);
    let api_config = nestgate_api::Config {
        bind_addr: network_setup.api_bind_addr,
        enable_zfs_api: true,
        ..Default::default()
    };

    nestgate_api::serve_with_zfs(api_config, zfs_manager).await?;

    Ok(())
}

#[derive(Debug)]
enum OperationMode {
    Standalone,
    SongbirdEnhanced(String),
}

#[derive(Debug)]
struct NetworkSetup {
    api_bind_addr: String,
    mode_description: String,
    _connection_manager: Option<SongbirdConnectionManager>,
}

async fn initialize_standalone_networking(service_name: &str) -> Result<NetworkSetup> {
    info!("🔧 Initializing standalone networking...");
    
    // Use standard localhost binding for standalone mode
    let api_port = std::env::var("NESTGATE_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);
    
    let bind_addr = format!("127.0.0.1:{}", api_port);
    
    info!("✅ Standalone networking initialized:");
    info!("   - Local access: http://{}", bind_addr);
    info!("   - Direct network binding (no orchestration)");
    
    Ok(NetworkSetup {
        api_bind_addr: bind_addr,
        mode_description: "Standalone mode with local access".to_string(),
        _connection_manager: None,
    })
}

async fn initialize_songbird_networking(service_name: &str, songbird_url: &str) -> Result<NetworkSetup> {
    info!("🎼 Initializing Songbird-enhanced networking...");
    
    // Initialize Songbird integration
    let songbird_config = SongbirdConfig {
        orchestrator_url: songbird_url.to_string(),
        registration_interval: 30,
        health_check_interval: 30,
        discovery_interval: 60,
        auto_port_allocation: true,
        service_metadata: {
            let mut metadata = std::collections::HashMap::new();
            metadata.insert("service_name".to_string(), service_name.to_string());
            metadata.insert("version".to_string(), env!("CARGO_PKG_VERSION").to_string());
            metadata.insert("capabilities".to_string(), "zfs,nfs,smb,iscsi,s3".to_string());
            metadata
        },
    };

    let mut songbird_integration = SongbirdIntegration::new(songbird_config);

    // Try to initialize Songbird (graceful fallback to standalone)
    match songbird_integration.initialize().await {
        Ok(()) => {
            info!("✅ Songbird integration successful");
            
            // Initialize connection manager
            let connection_manager = SongbirdConnectionManager::new(
                songbird_url.to_string(),
                service_name.to_string(),
            );

            // Initialize network API with Songbird
            let mut network_api = NetworkApi::new();
            network_api.initialize_with_songbird(songbird_url.to_string()).await?;

            // Register service and allocate ports
            let service_instance = ServiceInstance {
                id: uuid::Uuid::new_v4().to_string(),
                name: service_name.to_string(),
                host: service_name.to_string(), // Use service name
                port: 0, // Let Songbird allocate
                status: ServiceStatus::Running,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };

            network_api.register_service(service_instance).await?;
            let api_port = network_api.allocate_port(service_name, "api").await?;
            
            let bind_addr = format!("{}:{}", service_name, api_port);
            
            info!("✅ Songbird-enhanced networking initialized:");
            info!("   - Service endpoint: {} (Songbird-managed)", bind_addr);
            info!("   - Port allocation: {} (orchestrated)", api_port);
            
            Ok(NetworkSetup {
                api_bind_addr: bind_addr,
                mode_description: "Songbird-enhanced with orchestrated networking".to_string(),
                _connection_manager: Some(connection_manager),
            })
        }
        Err(e) => {
            warn!("⚠️ Songbird connection failed: {}", e);
            warn!("🔄 Falling back to standalone mode");
            
            // Graceful fallback to standalone
            initialize_standalone_networking(service_name).await
        }
    }
}

// Configuration structures for NestGate services
#[derive(Debug, Clone)]
pub struct ZfsServiceConfig {
    pub pools_path: String,
    pub enable_snapshots: bool,
    pub tier_management: bool,
}

impl Default for ZfsServiceConfig {
    fn default() -> Self {
        Self {
            pools_path: "/dev/disk/by-id".to_string(),
            enable_snapshots: true,
            tier_management: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CoreServiceConfig {
    pub api_port: u16,
    pub enable_metrics: bool,
}

impl Default for CoreServiceConfig {
    fn default() -> Self {
        Self {
            api_port: 8080,
            enable_metrics: true,
        }
    }
}

fn print_help() {
    println!("NestGate v{} - Distributed NAS System", env!("CARGO_PKG_VERSION"));
    println!();
    println!("🔧 STANDALONE MODE: Full local functionality with direct network access");
    println!("🎼 SONGBIRD-ENHANCED MODE: Extended functionality with orchestrated networking");
    println!();
    println!("USAGE:");
    println!("    nestgate [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("    -h, --help              Print this help message");
    println!("    --songbird-url URL      Enable Songbird orchestration (optional)");
    println!();
    println!("EXAMPLES:");
    println!("    # Standalone mode (local access only)");
    println!("    nestgate");
    println!("    nestgate NESTGATE_PORT=8080");
    println!("    ");
    println!("    # Songbird-enhanced mode (orchestrated networking)");
    println!("    nestgate --songbird-url http://songbird-orchestrator:8000");
    println!("    export SONGBIRD_URL=http://10.0.1.100:8000 && nestgate");
    println!();
    println!("ENVIRONMENT VARIABLES:");
    println!("    SONGBIRD_URL            Songbird orchestrator URL (enables enhanced mode)");
    println!("    NESTGATE_PORT           API port for standalone mode (default: 8080)");
    println!("    NESTGATE_SERVICE_NAME   Service name for identification");
    println!("    RUST_LOG                Logging level (info, debug, trace)");
    println!();
    println!("OPERATION MODES:");
    println!();
    println!("  🔧 STANDALONE MODE (Default):");
    println!("    • Full ZFS functionality locally accessible");
    println!("    • Direct network binding (127.0.0.1:8080)");
    println!("    • No external dependencies");
    println!("    • Perfect for single-node deployments");
    println!("    • Access via: http://localhost:8080");
    println!();
    println!("  🎼 SONGBIRD-ENHANCED MODE (Optional):");
    println!("    • All standalone functionality PLUS:");
    println!("    • Service discovery and registration");
    println!("    • Orchestrated port allocation");
    println!("    • Inter-service communication");
    println!("    • Network security management");
    println!("    • Multi-node coordination");
    println!("    • Graceful fallback to standalone if Songbird unavailable");
    println!();
    println!("API ENDPOINTS (Both Modes):");
    println!("    GET  /api/v1/health              # Service health check");
    println!("    GET  /api/v1/zfs/pools           # List ZFS storage pools");
    println!("    GET  /api/v1/zfs/datasets        # List ZFS datasets"); 
    println!("    GET  /api/v1/zfs/snapshots       # List ZFS snapshots");
    println!("    POST /api/v1/zfs/pools           # Create ZFS pool");
    println!("    POST /api/v1/zfs/datasets        # Create ZFS dataset");
    println!();
    println!("SECURITY MODEL:");
    println!("    • Standalone: Local access only (127.0.0.1)");
    println!("    • Songbird-enhanced: Orchestrator-managed security");
    println!("    • Friend's tower scenario: Songbird controls access permissions");
} 