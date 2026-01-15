//! **UNIX SOCKET SERVER EXAMPLE**
//!
//! Demonstrates TRUE PRIMAL transport with Unix sockets + JSON-RPC 2.0.
//!
//! ## Usage
//!
//! ```bash
//! # Terminal 1: Start server
//! NESTGATE_FAMILY_ID=example cargo run --example unix_socket_server
//!
//! # Terminal 2: Test with curl (requires socat)
//! echo '{"jsonrpc":"2.0","method":"health.ping","params":{},"id":1}' | socat - UNIX-CONNECT:/tmp/nestgate-example.sock
//! echo '{"jsonrpc":"2.0","method":"identity.get","params":{},"id":2}' | socat - UNIX-CONNECT:/tmp/nestgate-example.sock
//! ```

use nestgate_api::transport::{NestGateRpcHandler, TransportConfig, TransportServer};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    
    // Create configuration from environment
    let config = TransportConfig::from_env()?;
    
    println!("🚀 Starting NestGate TRUE PRIMAL Server");
    println!("   Family: {}", config.family_id);
    println!("   Socket: {}", config.socket_path.display());
    println!();
    println!("📡 Listening for JSON-RPC 2.0 requests...");
    println!();
    println!("Try these methods:");
    println!("  - health.ping");
    println!("  - health.status");
    println!("  - identity.get");
    println!("  - identity.capabilities");
    println!("  - system.info");
    println!();
    
    // Create RPC handler
    let handler = NestGateRpcHandler::new();
    
    // Create and start server
    let server = TransportServer::new(config, handler)?;
    
    // Setup Ctrl+C handler
    let server_clone = server.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.ok();
        println!("\n📡 Shutting down...");
        server_clone.shutdown();
    });
    
    // Start server
    server.start().await?;
    
    println!("✅ Server stopped");
    Ok(())
}
