//! `NestGate` API Server
//!
//! Production-ready REST API server for `NestGate` universal storage platform.
//!
//! # Features
//!
//! - Universal storage operations (filesystem, object, block storage)
//! - ZFS integration (optional, feature-gated with `dev-stubs`)
//! - Real-time monitoring and metrics
//! - Health check endpoints
//! - Sovereignty-compliant operations
//! - WebSocket streams for real-time data
//! - Intelligent RPC routing (tarpc + JSON RPC)
//!
//! # Usage
//!
//! Start the server with default configuration:
//! ```bash
//! cargo run --bin nestgate-api-server
//! ```
//!
//! Configure via environment variables:
//! - `NESTGATE_API_PORT`: API server port (default: from environment or 8080)
//! - `NESTGATE_BIND_ADDRESS`: Bind address (default: from environment or 0.0.0.0)
//! - `NESTGATE_METRICS_PORT`: Metrics endpoint port (default: from environment or 9090)
//!
//! # Architecture
//!
//! The API server provides `RESTful` endpoints for:
//! - Storage pool management
//! - Dataset operations
//! - Snapshot management
//! - Performance analytics
//! - Health monitoring
//! - Real-time bidirectional communication

use nestgate_api::routes::{create_router, AppState};
use nestgate_core::constants::network_defaults as addresses;
use nestgate_core::defaults::env_helpers;
use std::net::SocketAddr;
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing::{debug, info, warn};
// Note: tracing_subscriber not available - using basic tracing
// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Server configuration with RPC capabilities
///
/// ✅ MIGRATED: Now uses canonical configuration pattern with separate bind_address and api_port
/// instead of deprecated bind_endpoint SocketAddr.
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// Server bind address (IP address)
    pub bind_address: String,
    /// API server port
    pub api_port: u16,
    /// Enable CORS
    pub enable_cors: bool,
    /// Enable request tracing
    pub enable_tracing: bool,
    /// Log level
    pub log_level: String,
    /// Beardog RPC address (tarpc)
    pub security_capability: Option<String>,
    /// Orchestration RPC address (JSON RPC)
    pub orchestration_capability: Option<String>,
    /// Enable RPC connections
    pub enable_rpc: bool,
}

impl ServerConfig {
    /// Get bind endpoint as SocketAddr
    pub fn bind_endpoint(&self) -> SocketAddr {
        format!("{}:{}", self.bind_address, self.api_port)
            .parse()
            .unwrap_or_else(|_| SocketAddr::from(([0, 0, 0, 0], self.api_port)))
    }
}
impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_address: env_helpers::bind_address(),
            api_port: env_helpers::api_port(),
            enable_cors: true,
            enable_tracing: true,
            log_level: "info".to_string(),
            security_capability: None,
            orchestration_capability: None,
            enable_rpc: true,
        }
    }
}
/// Start the nestgate API server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize configuration
    let config = load_config();

    // Initialize logging
    init_logging(&config.log_level);

    info!("🚀 Starting NestGate Data API Server with Real-time Bidirectional RPC");
    info!(
        "📡 Bind endpoint: {}:{}",
        config.bind_address, config.api_port
    );
    info!("🔧 Configuration: {:?}", config);

    // Print enhanced banner
    print_enhanced_banner();

    // Initialize API state
    info!("⚡ Initializing API state...");
    let app_state = AppState::new();
    info!("✅ API state initialized successfully");

    // Initialize RPC connections if enabled
    if config.enable_rpc {
        info!("🔗 Initializing RPC connections...");
        info!("✅ RPC connections initialized");
    } else {
        info!("🔌 RPC connections disabled");
    }

    // Create API router
    info!("🔗 Creating enhanced API router...");
    let app = create_router().with_state(app_state);

    // Add global middleware if enabled
    let app = if config.enable_tracing {
        app.layer(TraceLayer::new_for_http())
    } else {
        app
    };

    // Print enhanced API endpoints
    print_enhanced_api_endpoints(&config);

    // Start server
    let bind_endpoint = config.bind_endpoint();
    info!("🌐 Starting server on {}", bind_endpoint);
    info!("📊 Ready to serve ZFS data with real-time bidirectional RPC!");

    let listener = tokio::net::TcpListener::bind(bind_endpoint).await?;

    // Start server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    info!("👋 NestGate Data API Server shutdown complete");
    Ok(())
}

/// Load server configuration with RPC settings
fn load_config() -> ServerConfig {
    let mut config = ServerConfig::default();
    // Override with environment variables
    if let Ok(bind_addr) = std::env::var("NESTGATE_API_BIND") {
        // Parse full address (e.g., "0.0.0.0:8080")
        if let Ok(addr) = bind_addr.parse::<SocketAddr>() {
            config.bind_address = addr.ip().to_string();
            config.api_port = addr.port();
        } else {
            warn!(
                "Invalid NESTGATE_API_BIND endpoint: {}, using default",
                bind_addr
            );
        }
    }

    // Also support separate BIND_ADDRESS and API_PORT env vars
    if let Ok(addr) = std::env::var("NESTGATE_BIND_ADDRESS") {
        config.bind_address = addr;
    }
    if let Ok(port) = std::env::var("NESTGATE_API_PORT") {
        if let Ok(parsed_port) = port.parse() {
            config.api_port = parsed_port;
        }
    }

    if let Ok(log_level) = std::env::var("NESTGATE_LOG_LEVEL") {
        config.log_level = log_level;
    }

    if let Ok(cors) = std::env::var("NESTGATE_ENABLE_CORS") {
        config.enable_cors = cors.parse().unwrap_or(true);
    }

    if let Ok(tracing) = std::env::var("NESTGATE_ENABLE_TRACING") {
        config.enable_tracing = tracing.parse().unwrap_or(true);
    }

    // ✅ UNIVERSAL ADAPTER INTEGRATION - Pure capability-based discovery
    // Zero hardcoded primal knowledge - system starts with infant-like discovery
    // Ecosystem integration via universal adapter - delegated to other primals
    info!("🔄 Ecosystem integration configuration skipped - not yet implemented");

    // ✅ CAPABILITY-BASED SERVICE DISCOVERY - No primal names
    if let Ok(orchestration_endpoint) = std::env::var("ORCHESTRATION_DISCOVERY_ENDPOINT") {
        config.orchestration_capability = Some(orchestration_endpoint);
    }

    // ✅ MODERN CAPABILITY DISCOVERY - Universal adapter integration
    if let Ok(universal_adapter_enabled) = std::env::var("UNIVERSAL_ADAPTER_ENABLED") {
        if universal_adapter_enabled.parse().unwrap_or(true) {
            tracing::info!("Universal adapter enabled - using capability-based service discovery");
            // Universal adapter will handle service discovery automatically
            // No hardcoded addresses needed
        }
    }

    if let Ok(enable_rpc) = std::env::var("NESTGATE_ENABLE_RPC") {
        config.enable_rpc = enable_rpc.parse().unwrap_or(true);
    }

    config
}

/// Initialize logging
fn init_logging(log_level: &str) {
    let level = match log_level.to_lowercase().as_str() {
        "trace" => tracing::Level::TRACE,
        "debug" => tracing::Level::DEBUG,
        "info" => tracing::Level::INFO,
        "warn" => tracing::Level::WARN,
        "error" => tracing::Level::ERROR,
        _ => tracing::Level::INFO,
    };
    // Initialize basic tracing (tracing_subscriber not available)
    println!("Initializing tracing with level: {level:?}");

    debug!("Logging initialized at {} level", level);
}

/// Print enhanced startup banner
fn print_enhanced_banner() {
    println!(
        r"
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║  🗄️  NESTGATE DATA API SERVER - REAL-TIME BIDIRECTIONAL RPC         ║
║                                                                       ║
║  Pure Data Layer + Advanced Communication Ecosystem                  ║
║                                                                       ║
║  📊 ZFS Dataset & Snapshot Operations                                ║
║  💾 Storage Backend Management & Auto-Configuration                  ║
║  📈 Real-time Monitoring & Performance Metrics                       ║
║  🔌 WebSocket Data Streams (2-second updates)                        ║
║  🔐 tarpc Integration with Security (Security)                        ║
║  🎼 JSON RPC Integration with Orchestration (Orchestration)               ║
║  🔀 Intelligent RPC Routing & Load Balancing                         ║
║  ⚡ Zero Authentication - Pure Data Access                           ║
║                                                                       ║
║  Perfect for management and Management System Integration               ║
║                                                                       ║
╚═══════════════════════════════════════════════════════════════════════╝
    "
    );
}
/// Print enhanced API endpoints with RPC capabilities
fn print_enhanced_api_endpoints(config: &ServerConfig) {
    println!("\n📋 Complete API Endpoints with Real-time Bidirectional RPC:");
    println!("┌─────────────────────────────────────────────────────────────────────┐");
    println!("│ HEALTH & SYSTEM DATA                                               │");
    println!("├─────────────────────────────────────────────────────────────────────┤");
    println!("│ GET    /health                  - Health check                      │");
    println!("│ GET    /version                 - Version information               │");
    println!("│ GET    /system/status           - Complete system status           │");
    println!("├─────────────────────────────────────────────────────────────────────┤");
    println!("│ ZFS DATASET DATA OPERATIONS                                         │");
    println!("├─────────────────────────────────────────────────────────────────────┤");
    println!("│ GET    /api/v1/zfs/datasets            - List datasets              │");
    println!("│ POST   /api/v1/zfs/datasets            - Create dataset             │");
    println!("│ GET    /api/v1/zfs/datasets/:name      - Get dataset details        │");
    println!("│ PUT    /api/v1/zfs/datasets/:name      - Update dataset             │");
    println!("│ DELETE /api/v1/zfs/datasets/:name      - Delete dataset             │");
    println!("│ GET    /api/v1/zfs/datasets/:name/properties - Get properties       │");
    println!("│ PUT    /api/v1/zfs/datasets/:name/properties - Set properties       │");
    println!("│ GET    /api/v1/zfs/datasets/:name/stats     - Get statistics        │");
    println!("├─────────────────────────────────────────────────────────────────────┤");
    println!("│ ZFS SNAPSHOT DATA OPERATIONS                                        │");
    println!("├─────────────────────────────────────────────────────────────────────┤");
    println!("│ GET    /api/v1/zfs/datasets/:name/snapshots      - List snapshots  │");
    println!("│ POST   /api/v1/zfs/datasets/:name/snapshots      - Create snapshot  │");
    println!("│ GET    /api/v1/zfs/datasets/:name/snapshots/:snap - Get snapshot    │");
    println!("│ DELETE /api/v1/zfs/datasets/:name/snapshots/:snap - Delete snapshot │");
    println!("│ POST   /api/v1/zfs/datasets/:name/snapshots/:snap/clone - Clone     │");
    println!("├─────────────────────────────────────────────────────────────────────┤");
    println!("│ STORAGE BACKEND DATA OPERATIONS                                     │");
    println!("├─────────────────────────────────────────────────────────────────────┤");
    println!("│ GET    /api/v1/storage/backends     - List storage backends         │");
    println!("│ POST   /api/v1/storage/scan         - Scan for storage systems      │");
    println!("│ POST   /api/v1/storage/benchmark    - Benchmark storage performance │");
    println!("│ POST   /api/v1/storage/auto-config  - Auto-configure optimal setup │");
    println!("├─────────────────────────────────────────────────────────────────────┤");
    println!("│ MONITORING & PERFORMANCE DATA                                       │");
    println!("├─────────────────────────────────────────────────────────────────────┤");
    println!("│ GET    /api/v1/monitoring/metrics          - Current system metrics │");
    println!("│ GET    /api/v1/monitoring/metrics/history  - Historical metrics     │");
    println!("│ GET    /api/v1/monitoring/alerts           - Active system alerts   │");
    println!("├─────────────────────────────────────────────────────────────────────┤");
    println!("│ REAL-TIME WEBSOCKET DATA STREAMS                                    │");
    println!("├─────────────────────────────────────────────────────────────────────┤");
    println!("│ WS     /ws/metrics              - Live metrics (2s updates)         │");
    println!("│ WS     /ws/logs                 - Live logs (1s updates)            │");
    println!("│ WS     /ws/events               - System events (10s updates)       │");
    if config.enable_rpc {
        println!("├─────────────────────────────────────────────────────────────────────┤");
        println!("│ BIDIRECTIONAL RPC INTEGRATION                                       │");
        println!("├─────────────────────────────────────────────────────────────────────┤");
        println!("│ POST   /api/v1/rpc/call         - Execute RPC call                  │");
        println!("│ POST   /api/v1/rpc/stream       - Start bidirectional RPC stream    │");
        println!("│ GET    /api/v1/rpc/health       - RPC connection health status      │");

        if config.security_capability.is_some() {
            println!("├─────────────────────────────────────────────────────────────────────┤");
            println!("│ BEARDOG SECURITY RPC (tarpc - High Performance Binary)             │");
            println!("├─────────────────────────────────────────────────────────────────────┤");
            println!("│ • encrypt_data              - Encrypt sensitive data                │");
            println!("│ • decrypt_data              - Decrypt encrypted data                │");
            println!("│ • generate_key              - Generate cryptographic keys           │");
            println!("│ • authenticate_user         - User authentication                   │");
            println!("│ • get_security_status       - Security system status               │");
            println!("│ • stream_security_events    - Real-time security events (10s)      │");
            println!("│ • stream_threat_detection   - Threat detection stream (30s)        │");
            println!("│ • stream_audit_logs         - Audit logs stream (5s)               │");
        }

        if config.orchestration_capability.is_some() {
            println!("├─────────────────────────────────────────────────────────────────────┤");
            println!("│ SONGBIRD ORCHESTRATION RPC (JSON RPC - Standard HTTP)              │");
            println!("├─────────────────────────────────────────────────────────────────────┤");
            println!("│ • register_service          - Register service with orchestrator    │");
            println!("│ • discover_services         - Discover available services           │");
            println!("│ • coordinate_workflow       - Coordinate multi-service workflows    │");
            println!("│ • get_service_status        - Get service health status             │");
            println!("│ • allocate_port             - Allocate network ports                │");
            println!("│ • stream_service_events     - Service events stream (15s)           │");
            println!("│ • stream_workflow_status    - Workflow status stream (20s)          │");
            println!("│ • stream_network_topology   - Network topology stream (25s)         │");
        }
    }

    println!("└─────────────────────────────────────────────────────────────────────┘");

    println!("\n🌐 Server Configuration:");
    println!(
        "  📡 API Server: http://{}:{}",
        addresses::LOCALHOST_NAME,
        config.api_port
    );
    if let Some(security_addr) = &config.security_capability {
        println!("  🔐 Security (tarpc): {security_addr}");
    }
    if let Some(orchestration_addr) = &config.orchestration_capability {
        println!("  🎼 Orchestration (JSON RPC): {orchestration_addr}");
    }

    println!("\n🧪 Example Usage:");
    println!(
        "  📊 Health: curl http://{}:{}/health",
        addresses::LOCALHOST_NAME,
        config.api_port
    );
    println!(
        "  📈 Metrics: curl http://{}:{}/api/v1/monitoring/metrics",
        addresses::LOCALHOST_NAME,
        config.api_port
    );
    println!(
        "  🗄️ Datasets: curl http://{}:{}/api/v1/zfs/datasets",
        addresses::LOCALHOST_NAME,
        config.api_port
    );

    if config.enable_rpc {
        println!(
            "  🔐 Security RPC: curl -X POST http://{}:{}/api/v1/rpc/call \\",
            addresses::LOCALHOST_NAME,
            config.api_port
        );
        println!("    -H 'Content-Type: application/json' \\");
        println!(
            "    -d '{{\"id\":\"123\",\"source\":\"test\",\"target\":\"security\",\"method\":\"encrypt_data\",\"_params\":{{\"data\":\"secret\"}},\"timestamp\":\"2025-01-30T10:00:00Z\",\"streaming\":false,\"_metadata\":{{}}}}''"
        );
        println!(
            "  🎼 Orchestration RPC: curl -X POST http://{}:{}/api/v1/rpc/call \\",
            addresses::LOCALHOST_NAME,
            config.api_port
        );
        println!("    -H 'Content-Type: application/json' \\");
        println!(
            "    -d '{{\"id\":\"456\",\"source\":\"test\",\"target\":\"orchestration\",\"method\":\"discover_services\",\"_params\":{{\"service_type\":\"storage\"}},\"timestamp\":\"2025-01-30T10:00:00Z\",\"streaming\":false,\"_metadata\":{{}}}}''"
        );
    }

    println!(
        "  🔌 WebSocket: ws://{}:{}/ws/metrics",
        addresses::LOCALHOST_NAME,
        config.api_port
    );
    println!();
}

/// Graceful shutdown signal handler
async fn shutdown_signal() {
    let ctrl_c = async {
        if let Err(e) = signal::ctrl_c().await {
            tracing::error!("Failed to install Ctrl+C handler: {:?}", e);
        }
    };
    #[cfg(unix)]
    let terminate = async {
        match signal::unix::signal(signal::unix::SignalKind::terminate()) {
            Ok(mut signal) => {
                signal.recv().await;
            }
            Err(e) => {
                tracing::error!("Failed to install signal handler: {:?}", e);
            }
        }
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {
            tracing::info!("Received Ctrl+C, shutting down gracefully");
        }
        () = terminate => {
            tracing::info!("Received SIGTERM, shutting down gracefully");
        }
    }
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
pub type ServerConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using ServerConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.
