use nestgate_ui::run_app;
use tracing::{info, error};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("🚀 Starting NestGate - Production ZFS Storage Management");
    info!("💎 Pure Rust Native UI - Zero Web Dependencies");
    info!("📡 Remote Access: Use Songbird/BearDog");
    
    // Run the egui application
    if let Err(e) = run_app() {
        error!("❌ Failed to run NestGate UI: {}", e);
        return Err(e.into());
    }
    
    info!("✅ NestGate UI terminated successfully");
    Ok(())
} 