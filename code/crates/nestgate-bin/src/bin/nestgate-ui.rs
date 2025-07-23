//! NestGate Native UI Binary
//!
//! Pure Rust implementation of the NestGate user interface using egui.
//! Replaces the previous TypeScript/React implementation.

use nestgate_ui::run_app;

use tracing::error;
// Removed unused tracing import

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    println!("🚀 Starting NestGate Native UI (Pure Rust)");
    println!("==========================================");

    if let Err(e) = run_app() {
        error!("Failed to start UI: {}", e);
        std::process::exit(1);
    }
}
