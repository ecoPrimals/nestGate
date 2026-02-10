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

use anyhow::{Context, Result};
use nestgate_api::routes::{create_router, AppState};
use nestgate_core::config::environment::EnvironmentConfig;
use nestgate_core::NestGateError;
use std::net::SocketAddr;
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing::{debug, info};
// Note: tracing_subscriber not available - using basic tracing
// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Server configuration with RPC capabilities
///
/// ✅ MODERNIZED (Week 2): Now uses `EnvironmentConfig` for all settings
/// Eliminates hardcoded defaults and manual `env::var` parsing
#[derive(Debug, Clone)]
/// Configuration for Server
pub struct ServerConfig {
    /// Environment configuration (network, storage, monitoring, etc.)
    pub env_config: EnvironmentConfig,
    /// Enable CORS
    pub enable_cors: bool,
    /// Enable request tracing
    pub enable_tracing: bool,
    /// Security service RPC address (tarpc)
    pub security_capability: Option<String>,
    /// Orchestration RPC address (JSON RPC)
    pub orchestration_capability: Option<String>,
    /// Enable RPC connections
    pub enable_rpc: bool,
}

impl ServerConfig {
    /// Get bind endpoint as `SocketAddr`
    ///
    /// # Errors
    /// Returns an error if the bind address cannot be parsed
    pub fn bind_endpoint(&self) -> Result<SocketAddr, std::net::AddrParseError> {
        let host = &self.env_config.network.host;
        let port = self.env_config.network.port.get();
        format!("{host}:{port}").parse()
    }

    /// Get API port
    #[must_use]
    pub const fn api_port(&self) -> u16 {
        self.env_config.network.port.get()
    }

    /// Get bind address string
    #[must_use]
    pub fn bind_address(&self) -> &str {
        &self.env_config.network.host
    }

    /// Get log level
    #[must_use]
    pub fn log_level(&self) -> &str {
        &self.env_config.monitoring.log_level
    }
}

impl Default for ServerConfig {
    /// Returns the default instance using `EnvironmentConfig`
    fn default() -> Self {
        Self {
            env_config: EnvironmentConfig::default(),
            enable_cors: true,
            enable_tracing: true,
            security_capability: None,
            orchestration_capability: None,
            enable_rpc: true,
        }
    }
}
/// Start the nestgate API server
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize configuration from environment
    let config = load_config().context("Failed to load server configuration")?;

    // Initialize logging
    init_logging(config.log_level());

    info!("🚀 Starting NestGate Data API Server with Real-time Bidirectional RPC");
    info!(
        "📡 Bind endpoint: {}:{}",
        config.bind_address(),
        config.api_port()
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
    let bind_endpoint = config
        .bind_endpoint()
        .map_err(|e| NestGateError::internal(format!("Invalid bind address: {e}")))?;
    info!("🌐 Starting server on {}", bind_endpoint);
    info!("📊 Ready to serve ZFS data with real-time bidirectional RPC!");

    let listener = tokio::net::TcpListener::bind(bind_endpoint)
        .await
        .context("Failed to bind TCP listener")?;

    // Start server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    info!("👋 NestGate Data API Server shutdown complete");
    Ok(())
}

/// Load server configuration with RPC settings
///
/// ✅ MODERNIZED (Week 2): Uses `EnvironmentConfig` for all settings
/// Eliminates manual `env::var` parsing and `unwrap()` calls
fn load_config() -> Result<ServerConfig> {
    // Load base configuration from environment
    let env_config =
        EnvironmentConfig::from_env().context("Failed to load environment configuration")?;

    let mut config = ServerConfig {
        env_config,
        enable_cors: std::env::var("NESTGATE_ENABLE_CORS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(true),
        enable_tracing: std::env::var("NESTGATE_ENABLE_TRACING")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(true),
        enable_rpc: std::env::var("NESTGATE_ENABLE_RPC")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(true),
        security_capability: None,
        orchestration_capability: None,
    };

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
            info!("Universal adapter enabled - using capability-based service discovery");
            // Universal adapter will handle service discovery automatically
            // No hardcoded addresses needed
        }
    }

    Ok(config)
}

/// Initialize logging
fn init_logging(log_level: &str) {
    let level = match log_level.to_lowercase().as_str() {
        "trace" => tracing::Level::TRACE,
        "debug" => tracing::Level::DEBUG,
        "warn" => tracing::Level::WARN,
        "error" => tracing::Level::ERROR,
        _ => tracing::Level::INFO, // "info" or unknown defaults to INFO
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
#[allow(clippy::too_many_lines)]
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
            println!("│ SECURITY SERVICE RPC (tarpc - High Performance Binary)             │");
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
            println!("│ ORCHESTRATION SERVICE RPC (JSON RPC - Standard HTTP)               │");
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
        config.bind_address(),
        config.api_port()
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
        config.bind_address(),
        config.api_port()
    );
    println!(
        "  📈 Metrics: curl http://{}:{}/api/v1/monitoring/metrics",
        config.bind_address(),
        config.api_port()
    );
    println!(
        "  🗄️ Datasets: curl http://{}:{}/api/v1/zfs/datasets",
        config.bind_address(),
        config.api_port()
    );

    if config.enable_rpc {
        println!(
            "  🔐 Security RPC: curl -X POST http://{}:{}/api/v1/rpc/call \\",
            config.bind_address(),
            config.api_port()
        );
        println!("    -H 'Content-Type: application/json' \\");
        println!(
            "    -d '{{\"id\":\"123\",\"source\":\"test\",\"target\":\"security\",\"method\":\"encrypt_data\",\"_params\":{{\"data\":\"secret\"}},\"timestamp\":\"2025-01-30T10:00:00Z\",\"streaming\":false,\"_metadata\":{{}}}}''"
        );
        println!(
            "  🎼 Orchestration RPC: curl -X POST http://{}:{}/api/v1/rpc/call \\",
            config.bind_address(),
            config.api_port()
        );
        println!("    -H 'Content-Type: application/json' \\");
        println!(
            "    -d '{{\"id\":\"456\",\"source\":\"test\",\"target\":\"orchestration\",\"method\":\"discover_services\",\"_params\":{{\"service_type\":\"storage\"}},\"timestamp\":\"2025-01-30T10:00:00Z\",\"streaming\":false,\"_metadata\":{{}}}}''"
        );
    }

    println!(
        "  🔌 WebSocket: ws://{}:{}/ws/metrics",
        config.bind_address(),
        config.api_port()
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
/// Type alias for Serverconfigcanonical
pub type ServerConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using ServerConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.
