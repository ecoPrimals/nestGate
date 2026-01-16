//! NestGate binary main entry point
//!
//! **UniBin Architecture**: One binary, multiple modes
//! - Primary: `nestgate` - CLI commands + daemon mode
//! - Compat: `nestgate-server` - Auto-daemon (backward compatibility)

use clap::Parser;
use nestgate_bin::{cli::Cli, error::BinResult};
use std::path::Path;
use tracing::info;

#[tokio::main]
async fn main() -> BinResult<()> {
    // ═══════════════════════════════════════════════════════════════════════
    // UNIBIN: Binary name detection for auto-routing
    // ═══════════════════════════════════════════════════════════════════════
    
    let bin_name = std::env::args()
        .next()
        .and_then(|p| {
            Path::new(&p)
                .file_name()
                .and_then(|n| n.to_str())
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| "nestgate".to_string());
    
    // Auto-daemon mode for backward compatibility
    if bin_name == "nestgate-server" {
        info!("🏰 NestGate invoked as 'nestgate-server' (legacy mode)");
        info!("💡 TIP: Use 'nestgate daemon' for the modern UniBin interface");
        
        // Setup basic logging for daemon mode
        tracing_subscriber::fmt()
            .with_env_filter("nestgate=info")
            .with_target(false)
            .init();
        
        // Run daemon with defaults
        return nestgate_bin::commands::service::run_daemon(
            nestgate_core::defaults::network::DEFAULT_API_PORT,
            nestgate_core::defaults::network::DEFAULT_BIND_ADDRESS,
            false,
        )
        .await;
    }
    
    // Modern UniBin: Parse CLI and execute
    let cli = Cli::parse();
    cli.run().await
}
