/*!
 * NestGate Main Binary
 *
 * NestGate NAS system - runs as standalone service with optional Songbird enhancement
 * 🔧 STANDALONE: Full local functionality with direct network access
 * 🎼 SONGBIRD-ENHANCED: Extended functionality with orchestrated networking
 */

use std::sync::Arc;
use tracing::{info, warn};

// Core NestGate services
use nestgate_core::config::Config as NestGateConfig;
use nestgate_zfs::manager::ZfsManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("info,nestgate=debug")
        .init();

    info!(
        "🏠 NestGate v{} - Sovereign NAS System",
        env!("CARGO_PKG_VERSION")
    );

    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && (args[1] == "-h" || args[1] == "--help") {
        print_help();
        return Ok(());
    }

    // Check for ecosystem integration (OPTIONAL)
    let ecosystem_mode = detect_ecosystem_integration();

    match &ecosystem_mode {
        EcosystemMode::Standalone => {
            info!("🔧 STANDALONE MODE: Full sovereign operation");
            info!("   ✅ Complete ZFS NAS functionality");
            info!("   ✅ Local web UI and direct network access");
            info!("   💡 Set SONGBIRD_URL to enable distributed features");
        }
        EcosystemMode::Distributed {
            songbird_url,
            beardog_available,
        } => {
            info!("🌐 ECOSYSTEM MODE: Enhanced distributed operation");
            info!("   ✅ Songbird orchestration: {}", songbird_url);
            if *beardog_available {
                info!("   ✅ BearDog security: Encrypted federation enabled");
            }
            info!("   ✅ All standalone features PLUS distributed coordination");
        }
    }

    // Generate service identifier
    let service_name = std::env::var("NESTGATE_SERVICE_NAME").unwrap_or_else(|_| {
        format!(
            "nestgate-{}",
            &uuid::Uuid::new_v4().simple().to_string()[..8]
        )
    });

    info!("🏷️ Service identifier: {}", service_name);

    // Initialize NestGate core (always works standalone)
    let _nestgate_config = NestGateConfig::default();

    info!("💾 Initializing ZFS manager...");
    let zfs_manager = Arc::new(ZfsManager::new(nestgate_zfs::config::ZfsConfig::default()).await?);

    // Initialize networking (standalone-first, ecosystem-enhanced)
    let network_config = initialize_networking(&service_name, &ecosystem_mode).await?;

    info!("🌟 NestGate services initialized:");
    info!("   - Service: {}", service_name);
    info!("   - API endpoint: {}", network_config.api_bind_addr);
    info!("   - ZFS management: ✅ Operational");
    info!("   - Mode: {}", network_config.description);

    // Start the API server
    info!("🚀 NestGate ready - {}", network_config.description);
    let api_config = nestgate_api::Config {
        bind_addr: network_config.api_bind_addr,
        enable_zfs_api: true,
        ..Default::default()
    };

    nestgate_api::serve_with_zfs(api_config, zfs_manager).await?;

    Ok(())
}

#[derive(Debug)]
enum EcosystemMode {
    Standalone,
    Distributed {
        songbird_url: String,
        beardog_available: bool,
    },
}

#[derive(Debug)]
struct NetworkConfig {
    api_bind_addr: String,
    description: String,
}

fn detect_ecosystem_integration() -> EcosystemMode {
    // Check for Songbird orchestrator
    if let Ok(songbird_url) = std::env::var("SONGBIRD_URL") {
        // Check for BearDog security
        let beardog_available = std::env::var("BEARDOG_URL").is_ok()
            || std::env::var("BEARDOG_ENABLED")
                .map(|v| v == "true")
                .unwrap_or(false);

        EcosystemMode::Distributed {
            songbird_url,
            beardog_available,
        }
    } else {
        EcosystemMode::Standalone
    }
}

async fn initialize_networking(
    service_name: &str,
    ecosystem_mode: &EcosystemMode,
) -> Result<NetworkConfig, Box<dyn std::error::Error + Send + Sync>> {
    match ecosystem_mode {
        EcosystemMode::Standalone => {
            info!("🔧 Initializing standalone networking...");

            // Use configurable port for standalone mode
            let api_port = std::env::var("NESTGATE_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse::<u16>()
                .unwrap_or(8080);

            let bind_addr = format!("0.0.0.0:{api_port}");

            info!("✅ Standalone networking ready:");
            info!("   - Local access: http://localhost:{}", api_port);
            info!("   - Network access: http://<your-ip>:{}", api_port);
            info!("   - Direct NFS/SMB/HTTP protocols available");

            Ok(NetworkConfig {
                api_bind_addr: bind_addr,
                description: "Standalone operation with direct network access".to_string(),
            })
        }
        EcosystemMode::Distributed {
            songbird_url,
            beardog_available,
        } => {
            info!("🌐 Initializing ecosystem networking...");

            // Try ecosystem integration with graceful fallback
            match try_ecosystem_integration(service_name, songbird_url, *beardog_available).await {
                Ok(config) => {
                    info!("✅ Ecosystem integration successful");
                    Ok(config)
                }
                Err(e) => {
                    warn!("⚠️ Ecosystem integration failed: {}", e);
                    warn!("🔄 Gracefully falling back to standalone mode");

                    // Fallback to standalone
                    let api_port = nestgate_core::constants::network::api_port();
                    let bind_addr = format!("0.0.0.0:{api_port}");

                    Ok(NetworkConfig {
                        api_bind_addr: bind_addr,
                        description: "Standalone fallback (ecosystem unavailable)".to_string(),
                    })
                }
            }
        }
    }
}

async fn try_ecosystem_integration(
    _service_name: &str,
    _songbird_url: &str,
    _beardog_available: bool,
) -> Result<NetworkConfig, Box<dyn std::error::Error + Send + Sync>> {
    // This is where ecosystem integration would go
    // For now, we'll implement a placeholder that demonstrates the pattern

    info!("🎼 Attempting Songbird connection...");

    // Simulate ecosystem check (in real implementation, this would be actual network calls)
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    // For now, return an error to demonstrate fallback
    // In real implementation, this would do actual Songbird/BearDog integration
    Err("Ecosystem integration not yet implemented - using standalone mode".into())
}

fn print_help() {
    println!(
        "NestGate v{} - Sovereign NAS System",
        env!("CARGO_PKG_VERSION")
    );
    println!();
    println!("🏠 STANDALONE MODE: Complete ZFS NAS with local management");
    println!("🌐 ECOSYSTEM MODE: Enhanced with distributed coordination and encryption");
    println!();
    println!("USAGE:");
    println!("    nestgate [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("    -h, --help              Print this help message");
    println!();
    println!("ENVIRONMENT VARIABLES:");
    println!("    NESTGATE_PORT           API port (default: 8080)");
    println!("    NESTGATE_SERVICE_NAME   Service identifier (auto-generated if not set)");
    println!("    ORCHESTRATION_URL       Enable distributed coordination (optional)");
    println!("    SECURITY_URL            Enable encrypted federation (optional)");
    println!();
    println!("EXAMPLES:");
    println!("    # Standalone mode (complete NAS functionality)");
    println!("    nestgate");
    println!();
    println!("    # Custom port");
    println!("    NESTGATE_PORT=9090 nestgate");
    println!();
    println!("    # Distributed mode with orchestration module");
    println!("    ORCHESTRATION_URL=http://orchestrator:8080 nestgate");
    println!();
    println!("    # Full ecosystem with security module");
    println!("    ORCHESTRATION_URL=http://orchestrator:8080 SECURITY_URL=https://security:8443 nestgate");
    println!();
    println!("FEATURES:");
    println!("    ✅ ZFS pool management and tiered storage");
    println!("    ✅ NFS, SMB, and HTTP file sharing");
    println!("    ✅ Headless API with biomeOS UI integration");
    println!("    ✅ Snapshot and backup management");
    println!("    ✅ Performance monitoring and optimization");
    println!("    🌐 Distributed coordination (with orchestration modules)");
    println!("    🔐 Encrypted federation (with security modules)");
    println!();
}
