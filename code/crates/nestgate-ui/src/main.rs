//! # NestGate UI Main Entry Point
//!
//! **Native desktop application entry point for NestGate storage management**
//!
//! This is the main executable entry point for the NestGate native user interface.
//! It initializes logging, starts the application, and handles graceful shutdown.
//!
//! ## Features
//!
//! - **Pure Rust Native UI**: Built with egui for cross-platform native performance
//! - **Zero Web Dependencies**: No browser, HTML, or JavaScript components
//! - **Production Ready**: Complete ZFS storage management capabilities
//! - **Remote Access Ready**: Integrates with universal orchestration modules
//!
//! ## Usage
//!
//! Run the application directly:
//!
//! ```bash
//! cargo run --bin nestgate-ui
//! ```
//!
//! Or build and run the executable:
//!
//! ```bash
//! cargo build --release
//! ./target/release/nestgate-ui
//! ```
//!
//! ## Architecture
//!
//! - Initializes structured logging with [`tracing`]
//! - Launches the egui application via [`nestgate_ui::run_app`]
//! - Handles errors and provides clean shutdown

use nestgate_ui::run_app;

use tracing::error;
use tracing::info;
// Removed unused tracing import

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
