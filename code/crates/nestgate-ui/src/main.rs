use nestgate_ui::run_app;
use tracing::{error, info};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    info!("🚀 Starting NestGate - Production ZFS Storage Management");
    info!("💎 Pure Rust Native UI - Zero Web Dependencies");
    info!("📡 Remote Access: Use universal orchestration/security modules");

    // Run the egui application
    if let Err(e) = run_app() {
        error!("❌ Failed to run NestGate UI: {}", e);
        return Err(e.into());
    }

    info!("✅ NestGate UI terminated successfully");
    Ok(())
}
