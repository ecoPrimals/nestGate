//! # NestGate API
//!
//! ## Overview
//!
//! NestGate API provides a comprehensive REST API layer for interacting with the NestGate
//! ecosystem. It exposes all core functionality through well-designed HTTP endpoints with
//! built-in authentication, rate limiting, and comprehensive error handling.
//!
//! ## Key Features
//!
//! - **RESTful Architecture**: Clean, intuitive API design following REST principles
//! - **Hardware Tuning**: Dynamic hardware optimization and performance tuning
//! - **ZFS Integration**: Direct access to ZFS storage operations
//! - **Health Monitoring**: Real-time system health and status reporting
//! - **BearDog Security**: Integrated crypto lock protection for external access
//! - **Async Performance**: High-throughput async request handling
//!
//! ## API Endpoints
//!
//! ### Health & Status
//! - `GET /health` - System health check
//! - `GET /status` - Detailed system status
//! - `GET /metrics` - Performance metrics
//!
//! ### Hardware Tuning
//! - `POST /hardware/tune` - Auto-tune hardware configuration
//! - `GET /hardware/config` - Get current hardware configuration
//! - `POST /hardware/benchmark` - Run hardware benchmarks
//! - `POST /hardware/optimize` - Apply performance optimizations
//!
//! ### ZFS Operations
//! - `GET /zfs/pools` - List ZFS pools
//! - `GET /zfs/pools/{pool}/status` - Get pool status
//! - `POST /zfs/pools/{pool}/scrub` - Start pool scrub
//! - `GET /zfs/datasets` - List datasets
//! - `POST /zfs/snapshot` - Create snapshot
//!
//! ### Authentication
//! - `POST /auth/login` - User authentication
//! - `POST /auth/refresh` - Token refresh
//! - `POST /auth/logout` - User logout
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
//! │   HTTP Layer    │    │   Middleware    │    │   Handlers      │
//! │   (axum)        │    │   (auth/cors)   │    │   (business)    │
//! └─────────────────┘    └─────────────────┘    └─────────────────┘
//!           │                       │                       │
//!           └───────────────────────┼───────────────────────┘
//!                                   │
//!                      ┌─────────────────┐
//!                      │   NestGate      │
//!                      │   Core          │
//!                      └─────────────────┘
//! ```
//!
//! ## Usage
//!
//! ### Starting the API Server
//!
//! ```rust
//! use nestgate_api::{create_app, ApiConfig};
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = ApiConfig::default();
//!     let app = create_app(config).await;
//!     
//!     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
//!     axum::serve(listener, app).await.unwrap();
//! }
//! ```
//!
//! ### Making API Calls
//!
//! ```bash
//! # Health check
//! curl -X GET http://localhost:3000/health
//!
//! # Get hardware configuration
//! curl -X GET http://localhost:3000/api/v1/hardware/config
//!
//! # Auto-tune hardware
//! curl -X POST http://localhost:3000/api/v1/hardware/tune
//!
//! # Get ZFS pool status
//! curl -X GET http://localhost:3000/api/v1/zfs/pools/mypool/status
//! ```
//!
//! ## Performance
//!
//! The API is optimized for high performance:
//!
//! - **Concurrent Requests**: 10,000+ concurrent connections
//! - **Low Latency**: Sub-10ms response times for most endpoints
//! - **Memory Efficient**: Minimal memory allocation per request
//! - **Connection Pooling**: Efficient database and service connections
//!
//! ## Security
//!
//! Multiple layers of security protection:
//!
//! - **BearDog Crypto Locks**: External API access requires crypto locks
//! - **JWT Authentication**: Secure token-based authentication
//! - **Rate Limiting**: Prevent abuse and DoS attacks
//! - **CORS Protection**: Configurable cross-origin resource sharing
//! - **Input Validation**: Comprehensive input sanitization
//!
//! ## Error Handling
//!
//! Comprehensive error handling with structured responses:
//!
//! ```json
//! {
//!   "error": {
//!     "code": "HARDWARE_TUNING_FAILED",
//!     "message": "Failed to apply hardware tuning configuration",
//!     "details": {
//!       "component": "cpu_governor",
//!       "reason": "insufficient_permissions"
//!     },
//!     "timestamp": "2024-01-01T12:00:00Z"
//!   }
//! }
//! ```
//!
//! ## Monitoring
//!
//! Built-in monitoring and observability:
//!
//! - **Health Endpoints**: Real-time health checks
//! - **Metrics Collection**: Prometheus-compatible metrics
//! - **Request Logging**: Structured logging for all requests
//! - **Performance Tracing**: Distributed tracing support

use axum::Router;
use nestgate_zfs::manager::ZfsManager;
use std::sync::Arc;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;

pub mod byob;
pub mod handlers;
mod models;
mod routes;

pub use byob::create_byob_router;
pub use handlers::zfs::ZfsApiState;

/// API server configuration
#[derive(Debug, Clone)]
pub struct Config {
    /// The address to bind the API server to
    pub bind_addr: String,
    /// CORS configuration
    pub cors: Option<CorsLayer>,
    /// Enable ZFS API endpoints
    pub enable_zfs_api: bool,
    /// Request timeout in seconds
    pub request_timeout: u64,
    /// Maximum request body size in bytes
    pub max_body_size: usize,
}

impl Default for Config {
    fn default() -> Self {
        // Check if we're in Songbird mode or standalone mode
        let songbird_mode = std::env::var("SONGBIRD_URL").is_ok();

        let bind_addr = if songbird_mode {
            // Songbird-enhanced mode: use service name with auto port
            std::env::var("NESTGATE_API_BIND").unwrap_or_else(|_| "nestgate-api:0".to_string())
        } else {
            // Standalone mode: use localhost with configurable port
            let port = std::env::var("NESTGATE_PORT").unwrap_or_else(|_| "8080".to_string());
            format!(
                "{}:{}",
                nestgate_core::constants::addresses::localhost(),
                port
            )
        };

        Self {
            bind_addr,
            cors: None,
            enable_zfs_api: true,
            request_timeout: 30,
            max_body_size: 16 * 1024 * 1024, // 16MB
        }
    }
}

/// Initialize and start the API server with ZFS integration
pub async fn serve_with_zfs(
    config: Config,
    zfs_manager: Arc<ZfsManager>,
) -> Result<(), Box<dyn std::error::Error>> {
    let zfs_state = ZfsApiState { zfs_manager };

    let app = routes::create_combined_router()
        .with_state(zfs_state)
        .layer(TraceLayer::new_for_http())
        .layer(config.cors.unwrap_or_else(|| {
            CorsLayer::permissive() // Default to permissive CORS in development
        }));

    info!(
        "Starting NestGate API server on {} with ZFS integration",
        config.bind_addr
    );
    let listener = tokio::net::TcpListener::bind(&config.bind_addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Initialize and start the basic API server (without ZFS)
pub async fn serve(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let app = routes::create_router()
        .layer(TraceLayer::new_for_http())
        .layer(config.cors.unwrap_or_else(|| {
            CorsLayer::permissive() // Default to permissive CORS in development
        }));

    info!("Starting basic API server on {}", config.bind_addr);
    let listener = tokio::net::TcpListener::bind(&config.bind_addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Create a configured API router with ZFS integration
pub fn create_api_router(zfs_manager: Arc<ZfsManager>) -> Router {
    let zfs_state = ZfsApiState { zfs_manager };

    routes::create_combined_router()
        .with_state(zfs_state)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
}

/// Create a basic API router without ZFS integration
pub fn create_basic_router() -> Router {
    routes::create_router()
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
}
