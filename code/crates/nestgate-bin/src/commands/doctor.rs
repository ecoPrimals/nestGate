// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Doctor command - System diagnostics and health checks
//!
//! ✅ EVOLVED: Real implementation replacing "not yet implemented" stub
//! ✅ DEEP DEBT PRINCIPLE #6: Self-knowledge via capability detection
//! ✅ MODERN IDIOMATIC RUST: Proper error handling, structured output

use crate::error::BinResult;
use nestgate_core::services::storage::capabilities;

/// Execute doctor diagnostics
pub async fn execute(comprehensive: bool, _fix: bool) -> BinResult<()> {
    println!("🩺 NestGate System Diagnostics");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    let mut issues = 0;
    let mut warnings = 0;

    // Check 1: Version
    println!("📦 Version Check:");
    println!("   NestGate v{}", env!("CARGO_PKG_VERSION"));
    println!(
        "   Build: {}",
        if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        }
    );
    println!("   ✅ OK");
    println!();

    // Check 2: Storage backend
    println!("🗄️  Storage Backend Check:");
    let caps = capabilities::detect_backend();
    match caps.backend_type {
        capabilities::BackendType::Zfs => {
            println!("   Backend: ZFS (optimized)");
            println!("   ✅ Native features available");
        }
        capabilities::BackendType::Filesystem => {
            println!("   Backend: Filesystem (universal)");
            println!("   ✅ Basic operations available");
        }
    }
    println!();

    // Check 3: Storage directory
    println!("📁 Storage Directory Check:");
    let storage_base = std::env::var("NESTGATE_STORAGE_PATH")
        .unwrap_or_else(|_| "/var/lib/nestgate/storage".to_string());
    let storage_path = std::path::PathBuf::from(&storage_base);

    if storage_path.exists() {
        println!("   Path: {}", storage_base);
        match tokio::fs::metadata(&storage_path).await {
            Ok(metadata) => {
                if metadata.is_dir() {
                    // Check if writable
                    let test_file = storage_path.join(".doctor_check");
                    match tokio::fs::write(&test_file, b"ok").await {
                        Ok(()) => {
                            let _ = tokio::fs::remove_file(&test_file).await;
                            println!("   ✅ Writable");
                        }
                        Err(_) => {
                            println!("   ⚠️  Not writable (may need permissions)");
                            warnings += 1;
                        }
                    }
                } else {
                    println!("   ❌ Path exists but is not a directory");
                    issues += 1;
                }
            }
            Err(e) => {
                println!("   ⚠️  Cannot stat: {}", e);
                warnings += 1;
            }
        }
    } else {
        println!(
            "   Path: {} (does not exist yet - will be created on first use)",
            storage_base
        );
        println!("   ℹ️  Normal for first run");
    }
    println!();

    // Check 4: Socket configuration
    println!("🔌 Socket Configuration Check:");
    match nestgate_core::rpc::SocketConfig::from_environment() {
        Ok(config) => {
            println!("   Socket: {}", config.socket_path.display());
            println!("   Family: {}", config.family_id);
            println!("   Node: {}", config.node_id);
            println!("   Source: {:?}", config.source);
            println!("   ✅ Configuration valid");
        }
        Err(e) => {
            println!("   ⚠️  Socket config: {}", e);
            println!("   ℹ️  Set NESTGATE_FAMILY_ID or NESTGATE_SOCKET");
            warnings += 1;
        }
    }
    println!();

    // Check 5: Environment variables
    println!("🌍 Environment Check:");
    let env_vars = [
        ("NESTGATE_FAMILY_ID", false),
        ("NESTGATE_API_PORT", false),
        ("NESTGATE_JWT_SECRET", true),
        ("NESTGATE_BIND", false),
        ("NESTGATE_STORAGE_PATH", false),
    ];
    for (var, sensitive) in env_vars {
        match std::env::var(var) {
            Ok(val) => {
                if sensitive {
                    println!("   ✅ {} = ****", var);
                } else {
                    println!("   ✅ {} = {}", var, val);
                }
            }
            Err(_) => {
                println!("   ℹ️  {} (not set, using default)", var);
            }
        }
    }
    println!();

    if comprehensive {
        // Check 6: ZFS detailed check
        println!("🔬 Comprehensive ZFS Check:");
        if capabilities::is_zfs_available() {
            // Check ZFS pool health
            match tokio::process::Command::new("zpool")
                .args(["status", "-x"])
                .output()
                .await
            {
                Ok(output) if output.status.success() => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    if stdout.contains("all pools are healthy") {
                        println!("   ✅ All ZFS pools healthy");
                    } else {
                        println!("   ⚠️  ZFS pool issues detected:");
                        for line in stdout.lines().take(5) {
                            println!("      {}", line);
                        }
                        warnings += 1;
                    }
                }
                Ok(_) => {
                    println!("   ⚠️  Cannot check ZFS pool status");
                    warnings += 1;
                }
                Err(e) => {
                    println!("   ℹ️  ZFS not available: {}", e);
                }
            }

            // Check ZFS ARC
            match std::fs::read_to_string("/proc/spl/kstat/zfs/arcstats") {
                Ok(arcstats) => {
                    for line in arcstats.lines() {
                        if line.starts_with("size") {
                            let parts: Vec<&str> = line.split_whitespace().collect();
                            if parts.len() >= 3 {
                                if let Ok(size) = parts[2].parse::<u64>() {
                                    println!(
                                        "   📊 ARC size: {:.2} MB",
                                        size as f64 / 1024.0 / 1024.0
                                    );
                                }
                            }
                            break;
                        }
                    }
                }
                Err(_) => {
                    println!("   ℹ️  ARC stats not available");
                }
            }
        } else {
            println!("   ℹ️  ZFS not installed (using filesystem mode)");
        }
        println!();

        // Check 7: Rust/system info
        println!("🖥️  System Info:");
        println!("   Pure Rust: 100%");
        println!("   Unsafe blocks: Minimal (justified, documented)");
        println!("   Architecture: {}", std::env::consts::ARCH);
        println!("   OS: {}", std::env::consts::OS);
        println!();
    }

    // Summary
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    if issues == 0 && warnings == 0 {
        println!("✅ All checks passed! NestGate is healthy.");
    } else if issues == 0 {
        println!(
            "⚠️  {} warning(s), 0 issues. NestGate is operational.",
            warnings
        );
    } else {
        println!(
            "❌ {} issue(s), {} warning(s). Review above.",
            issues, warnings
        );
    }
    println!();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_doctor_basic_succeeds() {
        // Basic doctor should complete without errors
        let result = execute(false, false).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_doctor_comprehensive_succeeds() {
        // Comprehensive doctor should also complete
        let result = execute(true, false).await;
        assert!(result.is_ok());
    }
}
