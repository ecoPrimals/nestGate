//! ZFS Pool Setup Demo
//!
//! Demonstrates the pure Rust ZFS pool setup functionality with hardware detection
//! and intelligent configuration recommendations.

use nestgate_zfs::pool_setup::ZfsPoolSetup;
use tracing::error;
use tracing::info;
use tracing::warn;
// Removed unused tracing import

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("🚀 NestGate v2 - ZFS Pool Setup Demo");
    info!("=====================================");

    // Create pool setup manager
    let setup = match ZfsPoolSetup::new().await {
        Ok(setup) => setup,
        Err(e) => {
            error!("Failed to initialize pool setup: {}", e);
            return Err(e.into());
        }
    };

    // Get system report
    let report = setup.get_system_report();

    info!("📊 System Analysis Report:");
    info!("  Total devices found: {}", report.total_devices);
    info!("  Available for ZFS: {}", report.available_devices);

    info!("📋 Device breakdown by type:");
    for (device_type, count) in &report.devices_by_type {
        info!("  {:?}: {} devices", device_type, count);
    }

    info!("⚡ Device breakdown by speed:");
    for (speed_class, count) in &report.devices_by_speed {
        info!("  {:?}: {} devices", speed_class, count);
    }

    info!("🏊 Existing ZFS pools:");
    if report.existing_pools.is_empty() {
        info!("  No existing pools found");
    } else {
        for pool in &report.existing_pools {
            info!("  - {}", pool);
        }
    }

    info!("💡 Recommendations:");
    for recommendation in &report.recommendations {
        info!("  - {}", recommendation);
    }

    // Show available devices in detail
    let available_devices = setup.get_available_devices();
    if !available_devices.is_empty() {
        info!("🔍 Available devices for ZFS:");
        for device in &available_devices {
            let size_gb = device.size_bytes as f64 / (1024.0_f64.powi(3));
            info!("  📀 {} ({:.1} GB)", device.device_path, size_gb);
            info!("     Model: {}", device.model);
            info!("     Type: {:?}", device.device_type);
            info!("     Speed: {:?}", device.speed_class);
            if let Some(ref current_use) = device.current_use {
                info!("     Current use: {}", current_use);
            }
        }
    }

    // Generate recommended configuration
    if report.available_devices > 0 {
        info!("🎯 Generating optimal pool configuration...");

        match setup.recommend_pool_config("nestpool-demo") {
            Ok(config) => {
                info!("✅ Recommended configuration:");
                info!("  Pool name: {}", config.pool_name);
                info!("  Topology: {:?}", config.topology);
                info!("  Devices: {:?}", config.devices);
                info!("  Create tiers: {}", config.create_tiers);

                info!("📋 Pool properties:");
                for (key, value) in &config.properties {
                    info!("    {}: {}", key, value);
                }

                info!("🏗️ Tier mappings:");
                for (tier, device_types) in &config.tier_mappings {
                    info!("    {:?}: {:?}", tier, device_types);
                }

                // Ask user if they want to create the pool
                println!("\n⚠️  WARNING: This will create a real ZFS pool!");
                println!("   This is a DEMO - do not run on production systems!");
                println!("   Press Ctrl+C to abort, or Enter to continue...");

                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;

                if input.trim().is_empty() {
                    warn!("Demo mode: Pool creation skipped for safety");
                    info!("To actually create the pool, use the production setup function");
                } else {
                    info!("Pool creation aborted by user");
                }
            }
            Err(e) => {
                warn!("Could not generate pool configuration: {}", e);
            }
        }
    } else {
        warn!("No available devices found for ZFS pool creation");
        info!("This may be because:");
        info!("  - All devices are currently in use");
        info!("  - No suitable storage devices detected");
        info!("  - Insufficient permissions to access device information");
    }

    info!("🎉 Demo completed!");
    info!("To set up a production ZFS pool with the spare 990 EVO:");
    info!("  1. Ensure the device is not in use");
    info!("  2. Run with sudo privileges");
    info!("  3. Use the setup_production_zfs() function");
}
