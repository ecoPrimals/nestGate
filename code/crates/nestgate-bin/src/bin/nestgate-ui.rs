//! NestGate Native UI Binary
//!
//! Pure Rust implementation of the NestGate user interface using egui.
//! Replaces the previous TypeScript/React implementation.

use nestgate_api::start_server;
use std::env;

use tracing::error;
// Removed unused tracing import

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("==========================================");
    println!("🚀 NestGate UI Server Starting");
    println!("==========================================");

    let addr = env::var("NESTGATE_UI_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());

    if let Err(e) = start_server(&addr).await {
        error!("Failed to start UI server: {}", e);
        std::process::exit(1);
    }
}
